// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! "Dynamic" enumerations have version-dependent variants and values that
//! depend on other symbols.

use std::collections::BTreeMap;
use tectonic_errors::prelude::*;

use super::HasPrimitive;
use crate::{
    symbols::{HasSymbol, SymbolCategory, SymbolTable},
    FormatVersion,
};

/// For a variant of a dynamic enum, where its numerical value comes from.
enum DynamicEnumValueSource {
    // The previous value, plus one.
    Next,

    // The same numerical value as the previous value.
    //
    // This might seem like an odd option, but this is useful for the "virtual"
    // enum variants that are associated with symbols but not actual variants in
    // the Rust enum. The more complex dynamic enumerations can have several
    // groups of variant values, and often there are symbols specifying the
    // numerical start and end values of those groups.
    Same,

    // The previous numerical value, plus a delta.
    //
    // Clearly the `Next` and `Same` variants can be expressed with this, but
    // it's convenient to have special variants for those common cases.
    Delta(isize),

    // A symbol plus a delta.
    Symbol(&'static str, isize),

    // A symbol times a prefactor plus a delta.
    ScaledSymbol(&'static str, isize, isize),
}

/// A description of an item/variant of a dynamic enum.
///
/// The type parameter is the Rust enum associated with this dynamic enum.
///
/// Some "items" in the enum description do not correspond to actual variants that
/// we realize in the Rust enum. All items, however, have associated symbols that
/// are entered in the symbol table.
struct DynamicEnumItemDescription<T> {
    /// The value of the abstract Rust enum associated with this item, if
    /// present.
    ///
    /// If None, this particular item is used to define a useful C preprocessor
    /// symbol but isn't actually a valid value for the enum.
    absval: Option<T>,

    /// The name of the symbol-table symbol associated with this item.
    symbol_name: &'static str,

    /// The format version in which this item was introduced.
    since: FormatVersion,

    /// The source of the numerical value of this item, relative to the previous
    /// item.
    value_source: DynamicEnumValueSource,
}

/// A data structure for resolving numerical enumeration values to an abstract
/// Rust enumeration type.
///
/// This resolution must be done at runtime because the specific numerical
/// values depend on the version of the engine format that we're considering.
/// New versions can add new enumeration variants, sometimes in the middle of
/// pre-existing numerical sequences.
#[derive(Debug)]
pub struct DynamicEnumResolver<T> {
    values: BTreeMap<isize, T>,
}

impl<T: Copy + std::fmt::Debug> DynamicEnumResolver<T> {
    /// Create a resolver for a specific engine version.
    ///
    /// Besides creating the resolver type, this populates the symbol table with
    /// the symbols defined by this enum.
    fn build(
        version: FormatVersion,
        symbols: &mut SymbolTable,
        cat: SymbolCategory,
        items: &[DynamicEnumItemDescription<T>],
    ) -> Result<Self> {
        let mut cur_val = -1;
        let mut values = BTreeMap::new();

        for item in items {
            // The whole point of this mess: skip items that are too new for the
            // version we're considering.
            if item.since > version {
                continue;
            }

            let next_val = match item.value_source {
                DynamicEnumValueSource::Next => cur_val + 1,
                DynamicEnumValueSource::Same => cur_val,
                DynamicEnumValueSource::Delta(d) => cur_val + d,
                DynamicEnumValueSource::Symbol(sym, d) => symbols.lookup(sym) + d,
                DynamicEnumValueSource::ScaledSymbol(sym, f, d) => f * symbols.lookup(sym) + d,
            };

            ensure!(
                next_val >= cur_val,
                "dynamic enum values must be nondecreasing"
            );

            cur_val = next_val;
            symbols.add(cat, item.symbol_name.to_owned(), cur_val)?;

            if let Some(v) = item.absval {
                ensure!(
                    values.insert(cur_val, v).is_none(),
                    "duplicated dynamic enum variant value"
                );
            }
        }

        Ok(DynamicEnumResolver { values })
    }

    /// Resolve a numerical enumeration value to its associated abstract Rust
    /// value.
    pub fn resolve(&self, value: isize) -> Option<T> {
        self.values.get(&value).copied()
    }

    /// Get an iterator over the enumeration values that are "active" and
    /// actually present in the resolved version of the enum.
    pub fn active(&self) -> impl Iterator<Item = (&isize, &T)> {
        self.values.iter()
    }
}

macro_rules! inner_declare_primitive_value {
    (_) => {
        None
    };
    ($primname:ident) => {
        Some(stringify!($primname))
    };
}

/// Helper macro: declare the "abstract" Rust enumeration type associated with
/// a particular dynamic enum.
///
/// Usage:
///
/// ```text
/// {
///     var $VARNAME1 $SYMNAME1 $SINCE1 [$VALUESOURCE1],
///     var $VARNAME2 $SYMNAME2 $SINCE2 [$VALUESOURCE2],
///     not $IGNORED $SYMNAME3 $SINCE3 [$VALUESOURCE3],
/// }
/// ```
///
/// basically becomes
///
/// ```text
/// pub enum Enumeration {
///     $VARNAME1,
///     $VARNAME2,
/// }
/// ```
///
/// FIXME: I, uh, don't understand how this filtering pattern actually works.
/// Copied from: https://users.rust-lang.org/t/filter-a-list-with-macros/64185/2
/// It filters out the records starting with `not var` and preserves the ones
/// starting with `var`. (Compared to the forum post, we use `var` instead of
/// `keep` and `not` instead of `drop`.)
macro_rules! inner_declare_enum {
    (@inner $enumname:ident $doc:expr; $($(var $($v:literal)?)? $(not)?: $varname:tt $symname:ident $primname:tt,)*) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum $enumname { $($($($v)? $varname,)?)* }

        impl HasSymbol for $enumname {
            fn symbol(&self) -> &'static str {
                match *self {
                    $($($($v)? Self::$varname => stringify!($symname),)?)*
                }
            }
        }

        impl HasPrimitive for $enumname {
            fn primitive(&self) -> Option<&'static str> {
                match *self {
                    $($($($v)? Self::$varname => inner_declare_primitive_value!($primname),)?)*
                }
            }
        }
    };

    ($enumname:ident $doc:expr; $($x:ident $varname:tt $symname:ident $primname:tt $since:literal [$($val:tt)+],)*) => {
        inner_declare_enum! { @inner $enumname $doc; $($x: $varname $symname $primname,)* }
    };
}

/// Helper macro: declare one DynamicEnumItemDescription item.
///
/// Usage:
///
/// ```text
/// (var $VARNAME $SYMNAME $SINCE [$VALUESOURCE])
/// ```
///
/// becomes a `DynamicEnumItemDescription` initializer with `absval`
/// of `Some(Enumeration::$VARNAME)`, while
///
/// ```text
/// (not ....)
/// ```
///
/// becomes the same with an `absval` of `None`.
macro_rules! inner_declare_desc_item {
    (var $varname:tt $symname:ident $since:literal [$($val:tt)+]) => {
        DynamicEnumItemDescription {
            absval: Some(Self::$varname),
            symbol_name: stringify!($symname),
            since: $since,
            value_source: DynamicEnumValueSource::$($val)+,
        }
    };

    (not $ignore:tt $symname:ident $since:literal [$($val:tt)+]) => {
        DynamicEnumItemDescription {
            absval: None,
            symbol_name: stringify!($symname),
            since: $since,
            value_source: DynamicEnumValueSource::$($val)+,
        }
    };
}

/// Helper macro: declare a constant `DESCRIPTION` of DynamicEnumItemDescription
/// values.
macro_rules! inner_declare_desc {
    ($enumname:ident; $($flag:ident $varname:tt $symname:ident $primname:tt $since:literal [$($val:tt)+],)*) => {
        impl $enumname {
            fn get_description() -> &'static [DynamicEnumItemDescription<Self>] {
                &[
                    $(
                        inner_declare_desc_item!($flag $varname $symname $since [$($val)+]),
                    )*
                ]
            }
        }
    };
}

/// Declare a dynamic enum.
///
/// Usage:
///
/// ```text
/// declare! {
///     EnumName enum_name {
///         var First  FIRST_CODE     10 [Next],
///         not var    SPECIAL_SYMBOL 0  [Same],
///         var Second SECOND_CODE    0  [Next],
///      }
/// }
/// ```
///
/// evaluates to something along the lines of
///
/// ```text
/// pub mod enum_name {
///     pub enum Enumeration {
///         First,
///         Second
///     }
///
///     impl Enumeration {
///         pub fn build_resolver(...) -> Result<DynamicEnumResolver<Self>> { ... }
///     }
/// }
///
/// pub use enum_name::Enumeration as EnumName;
/// ```
///
/// Here the `First` variation is only provided in engine format versions >= 10,
/// and the `SPECIAL_SYMBOL` symbol gets entered into the symbol table value
/// with the same value as the previous symbol, but is not associated with a
/// variant in the "abstract" Rust enum.
macro_rules! declare {
    (#[doc = $doc:expr] $enumname:ident {
        $($contents:tt)+
    }) => {
        inner_declare_enum! { $enumname $doc; $($contents)+ }
        inner_declare_desc! { $enumname; $($contents)+ }

        impl $enumname {
            pub fn build_resolver(
                version: FormatVersion,
                symbols: &mut SymbolTable,
            ) -> Result<DynamicEnumResolver<Self>> {
                DynamicEnumResolver::build(version, symbols, SymbolCategory::$enumname, Self::get_description())
            }
        }
    }
}

declare! {
    /// Subcommand codes for the box-related commands.
    // Primitives here are for MAKE_BOX, but this enum is also used
    // by UN_HBOX and UN_VBOX.
    BoxCodes {
        var Box BOX_CODE box 0 [Next],
        var Copy COPY_CODE copy 0 [Next],
        var LastBox LAST_BOX_CODE lastbox 0 [Next],
        var VSplit VSPLIT_CODE vsplit 0 [Next],
        var VTop VTOP_CODE vtop 0 [Next],
        var VBox TT_VBOX_CODE vbox 0 [Symbol("VMODE", 4)],
        var HBox TT_HBOX_CODE hbox 0 [Symbol("HMODE", 4)],
    }
}

declare! {
    /// Subcommand codes for the CONVERT command.
    ConvertCodes {
        var Number NUMBER_CODE number 0 [Next],
        var RomanNumeral ROMAN_NUMERAL_CODE romannumeral 0 [Next],
        var String STRING_CODE string 0 [Next],
        var Meaning MEANING_CODE meaning 0 [Next],
        var FontName FONT_NAME_CODE fontname 0 [Next],
        not var ETEX_CONVERT_BASE _ 0 [Next],
        var EtexRevision ETEX_REVISION_CODE eTeXrevision 0 [Same],
        not var ETEX_CONVERT_CODES _ 0 [Next],
        var Expanded EXPANDED_CODE expanded 0 [Same],
        not var PDFTEX_FIRST_EXPAND_CODE _ 0 [Next], // 7
        var LeftMarginKern LEFT_MARGIN_KERN_CODE leftmarginkern 0 [Delta(9)], // 16
        var RightMarginKern RIGHT_MARGIN_KERN_CODE rightmarginkern 0 [Next],
        var PdfStrcmp PDF_STRCMP_CODE strcmp 0 [Next], // 18
        var PdfCreationDate PDF_CREATION_DATE_CODE creationdate 0 [Delta(4)], // 22
        var PdfFileModDate PDF_FILE_MOD_DATE_CODE filemoddate 0 [Next],
        var PdfFileSize PDF_FILE_SIZE_CODE filesize 0 [Next],
        var PdfMd5Sum PDF_MDFIVE_SUM_CODE mdfivesum 0 [Next],
        var PdfFileDump PDF_FILE_DUMP_CODE filedump 0 [Next], // 26
        var UniformDeviate UNIFORM_DEVIATE_CODE uniformdeviate 0 [Delta(3)], // 29
        var NormalDeviate NORMAL_DEVIATE_CODE normaldeviate 0 [Next], // 30
        not var PDFTEX_CONVERT_CODES _ 0 [Delta(3)], // 33
        not var XETEX_FIRST_EXPAND_CODE _ 0 [Same], // 33
        var XetexRevision XETEX_REVISION_CODE XeTeXrevision 0 [Same], // 33
        var XetexVariationName XETEX_VARIATION_NAME_CODE XeTeXvariationname 0 [Next],
        var XetexFeatureName XETEX_FEATURE_NAME_CODE XeTeXfeaturename 0 [Next],
        var XetexSelectorName XETEX_SELECTOR_NAME_CODE XeTeXselectorname 0 [Next],
        var XetexGlyphName XETEX_GLYPH_NAME_CODE XeTeXglyphname 0 [Next],
        var XetexUChar XETEX_UCHAR_CODE Uchar 0 [Next],
        var XetexUCharCat XETEX_UCHARCAT_CODE Ucharcat 0 [Next],
        var JobName JOB_NAME_CODE jobname 0 [Next],
        not var XETEX_CONVERT_CODES _ 0 [Same],
    }
}

declare! {
    /// Subcommand codes for the EXTENSION command.
    ExtensionCodes {
        var Open OPEN_NODE openout 0 [Next], // also WhatsitNodeSubtype
        var Write WRITE_NODE write 0 [Next], // also WhatsitNodeSubtype
        var Close CLOSE_NODE closeout 0 [Next], // also WhatsitNodeSubtype
        var Special SPECIAL_NODE special 0 [Next], // also WhatsitNodeSubtype
        var Immediate IMMEDIATE_CODE immediate 0 [Next],
        var SetLanguage SET_LANGUAGE_CODE setlanguage 0 [Next],
        var PdfSavePos PDF_SAVE_POS_NODE pdfsavepos 0 [Symbol("PDF_SAVE_POS_NODE", 0)], // 21
        var ResetTimer RESET_TIMER_CODE resettimer 0 [Delta(10)], // 31
        var SetRandomSeed SET_RANDOM_SEED_CODE setrandomseed 0 [Delta(2)], // 33
        var PicFile PIC_FILE_CODE XeTeXpicfile 0 [Delta(8)], // 41; not to be confused with similar WhatsitNodeSubtype
        var PdfFile PDF_FILE_CODE XeTeXpdffile 0 [Next], // not to be confused with similar WhatsitNodeSubtype
        var Glyph GLYPH_CODE XeTeXglyph 0 [Next], // not to be confused with similar WhatsitNodeSubtype
        var XetexInputEncoding XETEX_INPUT_ENCODING_EXTENSION_CODE XeTeXinputencoding 0 [Next],
        var XetexDefaultEncoding XETEX_DEFAULT_ENCODING_EXTENSION_CODE XeTeXdefaultencoding 0 [Next],
        var XetexLinebreakLocale XETEX_LINEBREAK_LOCALE_EXTENSION_CODE XeTeXlinebreaklocale 0 [Next],
    }
}

declare! {
    /// Kinds of math "noads".
    //
    // "An mlist is represented internally as a linked list consisting chiefly of
    // “noads” (pronounced “no-adds”), to distinguish them from the somewhat
    // similar “nodes” in hlists and vlists. Certain kinds of ordinary nodes are
    // allowed to appear in mlists together with the noads; TEX tells the diﬀerence
    // by means of the type field, since a noad’s type is always greater than that
    // of a node. An mlist does not contain character nodes, hlist nodes, vlist
    // nodes, math nodes, ligature nodes, or unset nodes; in particular, each mlist
    // item appears in the variable-size part of mem , so the type field is always
    // present."
    MathNoadTypes {
        var LeftRightMiddleMode TT_LEFT_RIGHT_MIDDLE_MODE _ 0 [Delta(2)], // hack for LEFT_RIGHT command
        var Ordinary ORD_NOAD _ 0 [Symbol("UNSET_NODE", 3)], // 16
        var BigOperator OP_NOAD _ 0 [Next],
        var BinaryOperator BIN_NOAD _ 0 [Next],
        var Relation REL_NOAD _ 0 [Next],
        var Opening OPEN_NOAD _ 0 [Next],
        var Closing CLOSE_NOAD _ 0 [Next],
        var Punctuation PUNCT_NOAD _ 0 [Next],
        var Inner INNER_NOAD _ 0 [Next],
        var Radical RADICAL_NOAD _ 0 [Next],
        var Fraction FRACTION_NOAD _ 0 [Next],
        var Underline UNDER_NOAD _ 0 [Next],
        var Overline OVER_NOAD _ 0 [Next],
        var Accented ACCENT_NOAD _ 0 [Next],
        var VCenter VCENTER_NOAD _ 0 [Next],
        var Left LEFT_NOAD _ 0 [Next],
        var Right RIGHT_NOAD _ 0 [Next],
    }
}

declare! {
    /// Subcommand codes for the LAST_ITEM command.
    LastItemCodes {
        // Plain TeX LAST_ITEM possibilities
        var IntVal INT_VAL lastpenalty 0 [Next],
        var DimenVal DIMEN_VAL lastkern 0 [Next],
        var GlueVal GLUE_VAL lastskip 0 [Next],
        var LastNodeType LAST_NODE_TYPE_CODE lastnodetype 0 [Next],
        var InputLineNo INPUT_LINE_NO_CODE inputlineno 0 [Next],
        var Badness BADNESS_CODE badness 0 [Next],

        // Various integers added by pdftex. It looks like XeTeX only implements
        // a few of these and but skips numbers to keep consistent numbering
        // around the ones that it doesn't use.
        not var PDFTEX_FIRST_RINT_CODE _ 0 [Next],
        var PdfLastXPos PDF_LAST_X_POS_CODE pdflastxpos 0 [Delta(6)],
        var PdfLastYPos PDF_LAST_Y_POS_CODE pdflastypos 0 [Next],
        var ElapsedTime ELAPSED_TIME_CODE elapsedtime 0 [Delta(3)],
        var PdfShellEscape PDF_SHELL_ESCAPE_CODE shellescape 0 [Next],
        var RandomSeed RANDOM_SEED_CODE randomseed 0 [Next],

        // Integers added by e-TeX.
        not var ETEX_INT _ 0 [Next],
        var ETexVersion ETEX_VERSION_CODE eTeXversion 0 [Same],
        var CurrentGroupLevel CURRENT_GROUP_LEVEL_CODE currentgrouplevel 0 [Next],
        var CurrentGroupType CURRENT_GROUP_TYPE_CODE currentgrouptype 0 [Next],
        var CurrentIfLevel CURRENT_IF_LEVEL_CODE currentiflevel 0 [Next],
        var CurrentIfType CURRENT_IF_TYPE_CODE currentiftype 0 [Next],
        var CurrentIfBranch CURRENT_IF_BRANCH_CODE currentifbranch 0 [Next],
        var GlueStretchOrder GLUE_STRETCH_ORDER_CODE gluestretchorder 0 [Next],
        var GlueShrinkOrder GLUE_SHRINK_ORDER_CODE glueshrinkorder 0 [Next],

        // Integers added by XeTeX
        not var XETEX_INT _ 0 [Next],
        var XetexVersion XETEX_VERSION_CODE XeTeXversion 0 [Same],
        var XetexCountGlyphs XETEX_COUNT_GLYPHS_CODE XeTeXcountglyphs 0 [Next],
        var XetexCountVariatons XETEX_COUNT_VARIATIONS_CODE XeTeXcountvariations 0 [Next],
        var XetexVariation XETEX_VARIATION_CODE XeTeXvariation 0 [Next],
        var XetexFindVariationByName XETEX_FIND_VARIATION_BY_NAME_CODE XeTeXfindvariationbyname 0 [Next],
        var XetexVariationMin XETEX_VARIATION_MIN_CODE XeTeXvariationmin 0 [Next],
        var XetexVariationMax XETEX_VARIATION_MAX_CODE XeTeXvariationmax 0 [Next],
        var XetexVariationDefault XETEX_VARIATION_DEFAULT_CODE XeTeXvariationdefault 0 [Next],
        var XetexCountFeatures XETEX_COUNT_FEATURES_CODE XeTeXcountfeatures 0 [Next],
        var XetexFeatureCode XETEX_FEATURE_CODE_CODE XeTeXfeaturecode 0 [Next],
        var XetexFindFeatureByName XETEX_FIND_FEATURE_BY_NAME_CODE XeTeXfindfeaturebyname 0 [Next],
        var XetexIsExclusiveFeature XETEX_IS_EXCLUSIVE_FEATURE_CODE XeTeXisexclusivefeature 0 [Next],
        var XetexCountSelectors XETEX_COUNT_SELECTORS_CODE XeTeXcountselectors 0 [Next],
        var XetexSelectorCode XETEX_SELECTOR_CODE_CODE XeTeXselectorcode 0 [Next],
        var XetexFindSelectorByName XETEX_FIND_SELECTOR_BY_NAME_CODE XeTeXfindselectorbyname 0 [Next],
        var XetexIsDefaultSelector XETEX_IS_DEFAULT_SELECTOR_CODE XeTeXisdefaultselector 0 [Next],
        var XetexOTCountScripts XETEX_OT_COUNT_SCRIPTS_CODE XeTeXOTcountscripts 0 [Next],
        var XetexOTCountLanguages XETEX_OT_COUNT_LANGUAGES_CODE XeTeXOTcountlanguages 0 [Next],
        var XetexOTCountFeatures XETEX_OT_COUNT_FEATURES_CODE XeTeXOTcountfeatures 0 [Next],
        var XetexOTScript XETEX_OT_SCRIPT_CODE XeTeXOTscripttag 0 [Next],
        var XetexOTLanguage XETEX_OT_LANGUAGE_CODE XeTeXOTlanguagetag 0 [Next],
        var XetexOTFeature XETEX_OT_FEATURE_CODE XeTeXOTfeaturetag 0 [Next],
        var XetexMapCharToGlyph XETEX_MAP_CHAR_TO_GLYPH_CODE XeTeXcharglyph 0 [Next],
        var XetexGlyphIndex XETEX_GLYPH_INDEX_CODE XeTeXglyphindex 0 [Next],
        var XetexFontType XETEX_FONT_TYPE_CODE XeTeXfonttype 0 [Next],
        var XetexFirstChar XETEX_FIRST_CHAR_CODE XeTeXfirstfontchar 0 [Next],
        var XetexLastChar XETEX_LAST_CHAR_CODE XeTeXlastfontchar 0 [Next],
        var XetexPdfPageCount XETEX_PDF_PAGE_COUNT_CODE XeTeXpdfpagecount 0 [Next],
        not var XETEX_LAST_ITEM_CODES _ 0 [Same],

        // Dimension value(s) added by XeTeX
        not var XETEX_DIM _ 0 [Next],
        var XetexGlyphBounds XETEX_GLYPH_BOUNDS_CODE XeTeXglyphbounds 0 [Same],
        not var XETEX_LAST_DIM_CODES _ 0 [Same],

        // Dimension value(s) added by e-TeX
        not var ETEX_DIM _ 0 [Next],
        var FontCharWidth FONT_CHAR_WD_CODE fontcharwd 0 [Same],
        var FontCharHeight FONT_CHAR_HT_CODE fontcharht 0 [Next],
        var FontCharDepth FONT_CHAR_DP_CODE fontchardp 0 [Next],
        var FontCharItalicCorrection FONT_CHAR_IC_CODE fontcharic 0 [Next],
        var ParShapeLength PAR_SHAPE_LENGTH_CODE parshapelength 0 [Next],
        var ParShapeIndent PAR_SHAPE_INDENT_CODE parshapeindent 0 [Next],
        var ParShapeDimen PAR_SHAPE_DIMEN_CODE parshapedimen 0 [Next],
        var GlueStretch GLUE_STRETCH_CODE gluestretch 0 [Next],
        var GlueShrink GLUE_SHRINK_CODE glueshrink 0 [Next],

        // Glue value(s) added by e-TeX
        not var ETEX_GLUE _ 0 [Next],
        var MuToGlue MU_TO_GLUE_CODE mutoglue 0 [Same],

        // Mu-glue value(s) added by e-TeX
        not var ETEX_MU _ 0 [Next],
        var GluetoMu GLUE_TO_MU_CODE gluetomu 0 [Same],

        // Expression evaluators added by e-TeX
        not var ETEX_EXPR _ 0 [Next],
        var EtexNumExpr TT_ETEX_NUM_EXPR_CODE numexpr 0 [Same],
        var EtexDimExpr TT_ETEX_DIM_EXPR_CODE dimexpr 0 [Next],
        var EtexGlueExpr TT_ETEX_GLUE_EXPR_CODE glueexpr 0 [Next],
        var EtexMuExpr TT_ETEX_MU_EXPR_CODE muexpr 0 [Next],
    }
}

declare! {
    /// Major processing modes of the engine
    Modes {
        var Vertical VMODE _ 0 [Delta(2)],
        var Horizontal HMODE _ 0 [Symbol("MAX_COMMAND", 2)],
        var Math MMODE _ 0 [ScaledSymbol("MAX_COMMAND", 2, 3)],
    }
}

declare! {
    /// Subcommand codes for the TAB_MARK and CAR_RET commands
    TabCrCodes {
        var Span SPAN_CODE _ 0 [Symbol("SPECIAL_CHAR", 0)],
        var Cr CR_CODE _ 0 [Symbol("SPECIAL_CHAR", 1)],
        var CrCr CR_CR_CODE _ 0 [Symbol("SPECIAL_CHAR", 2)],
    }
}
