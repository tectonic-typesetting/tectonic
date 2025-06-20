// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

#![allow(missing_docs)]

//! The low-level primitive commands provided by the engine.

use std::{collections::BTreeMap, io::Write};
use tectonic_errors::prelude::*;

use crate::{
    format::Format,
    symbols::{HasSymbol, SymbolCategory, SymbolTable},
    FormatVersion,
};

pub type CommandCode = i16;
pub type CommandArgument = i32;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CommandGroup {
    /// Category code commands. Last is MAX_CHAR_CODE.
    Group1 = 1,

    /// "Regular command": no \the, no \global, yes in big_switch
    Group2 = 2,

    /// First "internal" commands: expandable with \the, non-prefixed (no
    /// \global). First is MIN_INTERNAL. Last is MAX_NON_PREFIXED_COMMAND.
    Group3 = 3,

    /// Prefixable internal commands (yes \the, yes \global). Last is
    /// MAX_INTERNAL.
    Group4 = 4,

    /// Prefixable non-internal commands (no \the, yes \global). Last is
    /// MAX_COMMAND.
    Group5 = 5,

    /// Special commands: cannot make it to big_switch
    Group6 = 6,
}

/// A TeX primitive associated with a command.
#[derive(Clone, Copy, Debug)]
struct CommandPrimitive {
    pub name: &'static str,

    pub arg: ArgKind,

    pub init: PrimitiveExtraInit,
}

/// An argument value associated with a primitive.
#[derive(Clone, Copy, Debug)]
enum ArgKind {
    /// An integer value without an associated symbolic name.
    Unnamed(isize),

    /// A value with an associated symbolic name
    Symbol(&'static str),
}

impl ArgKind {
    pub fn get_value(&self, symbols: &SymbolTable) -> CommandArgument {
        match *self {
            ArgKind::Unnamed(x) => x as CommandArgument,
            ArgKind::Symbol(s) => symbols.lookup(s) as CommandArgument,
        }
    }
}

/// Special initialization to be done after a primitive is created.
///
/// These operations usually involve initialization of the special "frozen"
/// primitives in the eqtb.
#[derive(Clone, Copy, Debug)]
enum PrimitiveExtraInit {
    /// No extra initialization.
    None,

    /// This is `\par`: initialize `par_loc` and `par_token`.
    Par,

    /// This is `\write`: initialize `write_loc`
    Write,

    /// This is a frozen primitive: initialize a frozen copy
    Frozen(&'static str),
}

trait CommandBehavior: Sized {
    fn build(version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self>;
}

trait HasMetadata {
    fn metadata() -> CommandMetadata;
}

struct CommandMetadata {
    symbol: &'static str,
    parser_overload_name: Option<&'static str>,
    macro_overload_name: Option<&'static str>,
    since: FormatVersion,
    group: CommandGroup,
}

trait Command: HasSymbol + std::fmt::Debug {
    fn describe(&self, _arg: CommandArgument) -> String;
    fn primitives(&self) -> Vec<CommandPrimitive>;

    fn extended_info(&self, _arg: CommandArgument, _format: &Format) -> Option<String> {
        None
    }
}

macro_rules! foreach_command {
    ($callback:ident) => {
        $callback! { Relax RELAX ESCAPE _ 0 Group1 }
        $callback! { LeftBrace LEFT_BRACE _ _ 0 Group1 }
        $callback! { RightBrace RIGHT_BRACE _ _ 0 Group1 }
        $callback! { MathShift MATH_SHIFT _ _ 0 Group1 }
        $callback! { TabMark TAB_MARK _ _ 0 Group1 }
        $callback! { CarRet CAR_RET _ OUT_PARAM 0 Group1 }
        $callback! { MacroParam MAC_PARAM _ _ 0 Group1 }
        $callback! { SupMark SUP_MARK _ _ 0 Group1 }
        $callback! { SubMark SUB_MARK _ _ 0 Group1 }
        $callback! { EndV ENDV IGNORE _ 0 Group1 }
        $callback! { Spacer SPACER _ _ 0 Group1 }
        $callback! { Letter LETTER _ _ 0 Group1 }
        $callback! { OtherChar OTHER_CHAR _ _ 0 Group1 }
        $callback! { ParEnd PAR_END ACTIVE_CHAR MATCH  0 Group1 }
        $callback! { Stop STOP COMMENT END_MATCH 0 Group1 }
        $callback! { DelimNum DELIM_NUM INVALID_CHAR _ 0 Group1 }
        $callback! { CharNum CHAR_NUM _ _ 0 Group2 }
        $callback! { MathCharNum MATH_CHAR_NUM _ _ 0 Group2 }
        $callback! { Mark MARK _ _ 0 Group2 }
        $callback! { XRay XRAY _ _ 0 Group2 }
        $callback! { MakeBox MAKE_BOX _ _ 0 Group2 }
        $callback! { HMove HMOVE _ _ 0 Group2 }
        $callback! { VMove VMOVE _ _ 0 Group2 }
        $callback! { UnHBox UN_HBOX _ _ 0 Group2 }
        $callback! { UnVBox UN_VBOX _ _ 0 Group2 }
        $callback! { RemoveItem REMOVE_ITEM _ _ 0 Group2 }
        $callback! { HSkip HSKIP _ _ 0 Group2 }
        $callback! { VSkip VSKIP _ _ 0 Group2 }
        $callback! { MSkip MSKIP _ _ 0 Group2 }
        $callback! { Kern KERN _ _ 0 Group2 }
        $callback! { MathKern MKERN _ _ 0 Group2 }
        $callback! { LeaderShip LEADER_SHIP _ _ 0 Group2 }
        $callback! { HAlign HALIGN _ _ 0 Group2 }
        $callback! { VAlign VALIGN _ _ 0 Group2 }
        $callback! { NoAlign NO_ALIGN _ _ 0 Group2 }
        $callback! { VRule VRULE _ _ 0 Group2 }
        $callback! { HRule HRULE _ _ 0 Group2 }
        $callback! { Insert INSERT _ _ 0 Group2 }
        $callback! { VAdjust VADJUST _ _ 0 Group2 }
        $callback! { IgnoreSpaces IGNORE_SPACES _ _ 0 Group2 }
        $callback! { AfterAssignment AFTER_ASSIGNMENT _ _ 0 Group2 }
        $callback! { AfterGroup AFTER_GROUP _ _ 0 Group2 }
        $callback! { BreakPenalty BREAK_PENALTY _ _ 0 Group2 }
        $callback! { StartPar START_PAR _ _ 0 Group2 }
        $callback! { ItalicCorrection ITAL_CORR _ _ 0 Group2 }
        $callback! { Accent ACCENT _ _ 0 Group2 }
        $callback! { MathAccent MATH_ACCENT _ _ 0 Group2 }
        $callback! { Discretionary DISCRETIONARY _ _ 0 Group2 }
        $callback! { EquationNumber EQ_NO _ _ 0 Group2 }
        $callback! { LeftRight LEFT_RIGHT _ _ 0 Group2 }
        $callback! { MathComp MATH_COMP _ _ 0 Group2 }
        $callback! { LimitSwitch LIMIT_SWITCH _ _ 0 Group2 }
        $callback! { Above ABOVE _ _ 0 Group2 }
        $callback! { MathStyle MATH_STYLE _ _ 0 Group2 }
        $callback! { MathChoice MATH_CHOICE _ _ 0 Group2 }
        $callback! { NonScript NON_SCRIPT _ _ 0 Group2 }
        $callback! { VCenter VCENTER _ _ 0 Group2 }
        $callback! { CaseShift CASE_SHIFT _ _ 0 Group2 }
        $callback! { Message MESSAGE _ _ 0 Group2 }
        $callback! { Extension EXTENSION _ _ 0 Group2 }
        $callback! { InStream IN_STREAM _ _ 0 Group2 }
        $callback! { BeginGroup BEGIN_GROUP _ _ 0 Group2 }
        $callback! { EndGroup END_GROUP _ _ 0 Group2 }
        $callback! { Omit OMIT _ _ 0 Group2 }
        $callback! { ExSpace EX_SPACE _ _ 0 Group2 }
        $callback! { NoBoundary NO_BOUNDARY _ _ 0 Group2 }
        $callback! { Radical RADICAL _ _ 0 Group2 }
        $callback! { EndCsName END_CS_NAME _ _ 0 Group2 }
        $callback! { CharGiven CHAR_GIVEN _ _ 0 Group3 }
        $callback! { MathGiven MATH_GIVEN _ _ 0 Group3 }
        $callback! { XetexMathGiven XETEX_MATH_GIVEN _ _ 0 Group3 }
        $callback! { LastItem LAST_ITEM _ _ 0 Group3 }
        $callback! { ToksRegister TOKS_REGISTER _ _ 0 Group4 }
        $callback! { AssignToks ASSIGN_TOKS _ _ 0 Group4 }
        $callback! { AssignInt ASSIGN_INT _ _ 0 Group4 }
        $callback! { AssignDimen ASSIGN_DIMEN _ _ 0 Group4 }
        $callback! { AssignGlue ASSIGN_GLUE _ _ 0 Group4 }
        $callback! { AssignMuGlue ASSIGN_MU_GLUE _ _ 0 Group4 }
        $callback! { AssignFontDimen ASSIGN_FONT_DIMEN _ _ 0 Group4 }
        $callback! { AssignFontInt ASSIGN_FONT_INT _ _ 0 Group4 }
        $callback! { SetAux SET_AUX _ _ 0 Group4 }
        $callback! { SetPrevGraf SET_PREV_GRAF _ _ 0 Group4 }
        $callback! { SetPageDimen SET_PAGE_DIMEN _ _ 0 Group4 }
        $callback! { SetPageInt SET_PAGE_INT _ _ 0 Group4 }
        $callback! { SetBoxDimen SET_BOX_DIMEN _ _ 0 Group4 }
        $callback! { SetShape SET_SHAPE _ _ 0 Group4 }
        $callback! { DefCode DEF_CODE _ _ 0 Group4 }
        $callback! { XetexDefCode XETEX_DEF_CODE _ _ 0 Group4 }
        $callback! { DefFamily DEF_FAMILY _ _ 0 Group4 }
        $callback! { SetFont SET_FONT _ _ 0 Group4 }
        $callback! { DefFont DEF_FONT _ _ 0 Group4 }
        $callback! { Register REGISTER _ _ 0 Group4 }
        $callback! { Advance ADVANCE _ _ 0 Group5 }
        $callback! { Multiply MULTIPLY _ _ 0 Group5 }
        $callback! { Divide DIVIDE _ _ 0 Group5 }
        $callback! { Prefix PREFIX _ _ 0 Group5 }
        $callback! { Let LET _ _ 0 Group5 }
        $callback! { ShorthandDef SHORTHAND_DEF _ _ 0 Group5 }
        $callback! { ReadToCs READ_TO_CS _ _ 0 Group5 }
        $callback! { Def DEF _ _ 0 Group5 }
        $callback! { SetBox SET_BOX _ _ 0 Group5 }
        $callback! { HyphData HYPH_DATA _ _ 0 Group5 }
        $callback! { SetInteraction SET_INTERACTION _ _ 0 Group5 }
        $callback! { UndefinedCs UNDEFINED_CS _ _ 0 Group6 }
        $callback! { ExpandAfter EXPAND_AFTER _ _ 0 Group6 }
        $callback! { NoExpand NO_EXPAND _ _ 0 Group6 }
        $callback! { Input INPUT _ _ 0 Group6 }
        $callback! { IfTest IF_TEST _ _ 0 Group6 }
        $callback! { FiOrElse FI_OR_ELSE _ _ 0 Group6 }
        $callback! { CsName CS_NAME _ _ 0 Group6 }
        $callback! { Convert CONVERT _ _ 0 Group6 }
        $callback! { The THE _ _ 0 Group6 }
        $callback! { TopBotMark TOP_BOT_MARK _ _ 0 Group6 }
        $callback! { Call CALL _ _ 0 Group6 }
        $callback! { LongCall LONG_CALL _ _ 0 Group6 }
        $callback! { OuterCall OUTER_CALL _ _ 0 Group6 }
        $callback! { LongOuterCall LONG_OUTER_CALL _ _ 0 Group6 }
        $callback! { EndTemplate END_TEMPLATE _ _ 0 Group6 }
        $callback! { DontExpand DONT_EXPAND _ _ 0 Group6 }
        $callback! { GlueRef GLUE_REF _ _ 0 Group6 }
        $callback! { ShapeRef SHAPE_REF _ _ 0 Group6 }
        $callback! { BoxRef BOX_REF _ _ 0 Group6 }
        $callback! { Data DATA _ _ 0 Group6 }
    };
}

macro_rules! inner_emit_maybe_str {
    ( _ ) => {
        None
    };
    ($sym:ident) => {
        Some(stringify!($sym))
    };
}

macro_rules! inner_emit_metadata_impls {
    ($typename:ident $symname:ident $parseroverload:tt $macrooverload:tt $since:literal $group:ident) => {
        impl HasMetadata for $typename {
            fn metadata() -> CommandMetadata {
                CommandMetadata {
                    symbol: stringify!($symname),
                    parser_overload_name: inner_emit_maybe_str!($parseroverload),
                    macro_overload_name: inner_emit_maybe_str!($macrooverload),
                    since: $since,
                    group: CommandGroup::$group,
                }
            }
        }

        impl HasSymbol for $typename {
            fn symbol(&self) -> &'static str {
                stringify!($symname)
            }
        }
    };
}

foreach_command! {inner_emit_metadata_impls}

fn foreach_metadata<F>(mut cb: F)
where
    F: FnMut(&CommandMetadata),
{
    macro_rules! foreach_metadata_helper {
        ($typename:ident $symname:ident $parseroverload:tt $macrooverload:tt $since:literal $group:ident) => {
            cb(&$typename::metadata());
        };
    }

    foreach_command!(foreach_metadata_helper);
}

fn foreach_build<F>(version: FormatVersion, symbols: &mut SymbolTable, mut cb: F) -> Result<()>
where
    F: FnMut(&CommandMetadata, Box<dyn Command>),
{
    macro_rules! foreach_resolved_helper {
        ($typename:ident $symname:ident $parseroverload:tt $macrooverload:tt $since:literal $group:ident) => {
            cb(
                &$typename::metadata(),
                Box::new($typename::build(version, symbols)?),
            );
        };
    }

    foreach_command!(foreach_resolved_helper);
    Ok(())
}

pub fn initialize_command_code_symbols(version: FormatVersion, symbols: &mut SymbolTable) {
    let mut prev_group = CommandGroup::Group1;
    let mut n = 0;

    foreach_metadata(|md| {
        if version >= md.since {
            symbols.add(SymbolCategory::Commands, md.symbol, n).unwrap();

            if let Some(o) = md.parser_overload_name {
                symbols.add(SymbolCategory::Commands, o, n).unwrap();
            }

            if let Some(o) = md.macro_overload_name {
                symbols.add(SymbolCategory::Commands, o, n).unwrap();
            }

            let (bonus_symbol, delta) = match (prev_group, md.group) {
                (CommandGroup::Group1, CommandGroup::Group2) => (Some("MAX_CHAR_CODE"), -1),
                (CommandGroup::Group2, CommandGroup::Group3) => (Some("MIN_INTERNAL"), 0),
                (CommandGroup::Group3, CommandGroup::Group4) => {
                    (Some("MAX_NON_PREFIXED_COMMAND"), -1)
                }
                (CommandGroup::Group4, CommandGroup::Group5) => (Some("MAX_INTERNAL"), -1),
                (CommandGroup::Group5, CommandGroup::Group6) => (Some("MAX_COMMAND"), -1),
                _ => (None, 0),
            };

            if let Some(bs) = bonus_symbol {
                symbols
                    .add(SymbolCategory::Commands, bs, n + delta)
                    .unwrap();
            }

            n += 1;
            prev_group = md.group;
        }
    });
}

#[derive(Debug)]
pub struct Commands {
    codes: BTreeMap<CommandCode, Box<dyn Command>>,
}

impl Commands {
    pub fn get_for_version(version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self> {
        let mut cur_code = 0;
        let mut codes = BTreeMap::default();

        foreach_build(version, symbols, |md, cmd| {
            if version >= md.since {
                codes.insert(cur_code, cmd);
                cur_code += 1;
            }
        })?;

        Ok(Commands { codes })
    }

    pub fn describe(&self, code: CommandCode, arg: CommandArgument) -> String {
        if let Some(cmd) = self.codes.get(&code) {
            cmd.describe(arg)
        } else {
            format!("[??? {code} {arg}]")
        }
    }

    pub fn describe_extended(
        &self,
        code: CommandCode,
        arg: CommandArgument,
        format: &Format,
    ) -> (String, Option<String>) {
        if let Some(cmd) = self.codes.get(&code) {
            (cmd.describe(arg), cmd.extended_info(arg, format))
        } else {
            (format!("[??? {code} {arg}]"), None)
        }
    }

    /// Emit the beginning of the C header information for the commands and
    /// primitives.
    pub fn emit_c_header_beginning<W: Write>(&self, mut stream: W) -> Result<()> {
        writeln!(
            stream,
            "\n/* Primitives */

enum xetex_format_primitive_extra_init_t {{
    xf_prim_init_none = 0,
    xf_prim_init_par = 1,
    xf_prim_init_write = 2
    /* Other values should be used to set up a \"frozen\" primitive */
}};

typedef struct xetex_format_primitive_def_t {{
    char const *name;
    eight_bits cmd;
    int32_t chr;
    int32_t extra_init;
}} xetex_format_primitive_def_t;

#define XETEX_FORMAT_PRIMITIVE_INITIALIZERS \\"
        )?;

        for cmd in self.codes.values() {
            for prim in cmd.primitives() {
                let arg = match prim.arg {
                    ArgKind::Unnamed(v) => format!("{v}"),
                    ArgKind::Symbol(s) => s.to_string(),
                };

                let extra_init = match prim.init {
                    PrimitiveExtraInit::None => "xf_prim_init_none",
                    PrimitiveExtraInit::Par => "xf_prim_init_par",
                    PrimitiveExtraInit::Write => "xf_prim_init_write",
                    PrimitiveExtraInit::Frozen(s) => s,
                };

                writeln!(
                    stream,
                    "    {{ \"{}\", {}, {}, {} }}, \\",
                    prim.name,
                    cmd.symbol(),
                    arg,
                    extra_init
                )?;
            }
        }

        Ok(())
    }

    /// Emit the ending of the C header information for the commands and
    /// primitives.
    pub fn emit_c_header_ending<W: Write>(&self, mut stream: W) -> Result<()> {
        // We just need to terminate the primitives macro definition
        writeln!(stream, "    {{ NULL, 0, 0, 0 }}")?;
        Ok(())
    }
}

// Actual commands!!!

pub mod noprim;
pub use noprim::*;

pub mod simple;
pub use simple::*;

pub mod charcode;
pub use charcode::*;

pub mod simpleenum;
pub use simpleenum::*;

pub mod dynamicenum;
pub use dynamicenum::*;

pub mod symbolic;
pub use symbolic::*;
