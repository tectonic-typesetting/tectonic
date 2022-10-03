// Copyright 2021-2022 the Tectonic Project
// Licensed under the MIT License.

//! "Simple" enumerations have variants and values that do not depend on the
//! engine version. They declare symbols for each of their variants.

use tectonic_errors::prelude::*;

use crate::symbols::{DeclaresSymbols, HasSymbol, SymbolCategory, SymbolTable};

macro_rules! declare {
    ($enum:ident { $($varname:ident $symname:ident $value:literal,)+ }) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $enum {
            $($varname = $value,)+
        }

        impl DeclaresSymbols for $enum {
            fn declare_symbols(symbols: &mut SymbolTable) -> Result<()> {
                $(
                    symbols.add(SymbolCategory::$enum, stringify!($symname), $value)?;
                )+
                Ok(())
            }
        }

        impl HasSymbol for $enum {
            fn symbol(&self) -> &'static str {
                match *self {
                    $(
                        $enum::$varname => stringify!($symname),
                    )+
                }
            }
        }
    };
}

declare! {
    AboveCodes {
        Above ABOVE_CODE 0,
        Over OVER_CODE 1,
        Atop ATOP_CODE 2,
        AboveWithDelims TT_ABOVE_WITH_DELIMS 3,
        OverWithDelims TT_OVER_WITH_DELIMS 4,
        AtopWithDelims TT_ATOP_WITH_DELIMS 5,
    }
}

declare! {
    CharacterConstants {
        TooBigUsv TOO_BIG_USV 0x11_0000,
        SpecialChar SPECIAL_CHAR 0x11_0001,
    }
}

declare! {
    FiOrElseCodes {
        Fi FI_CODE 2,
        Else ELSE_CODE 3,
        Or OR_CODE 4,
    }
}

declare! {
    GlueNodeSubtypes {
        Normal NORMAL 0,
        MuGlueOrShipout MU_GLUE 99,
        AlignedLeaders A_LEADERS 100,
        CenteredLeaders C_LEADERS 101,
        ExpandedLeaders X_LEADERS 102,
    }
}

declare! {
    IfCodes {
        Char IF_CHAR_CODE 0,
        Cat IF_CAT_CODE 1,
        Int IF_INT_CODE 2,
        Dim IF_DIM_CODE 3,
        Odd IF_ODD_CODE 4,
        VMode IF_VMODE_CODE 5,
        HMode IF_HMODE_CODE 6,
        MMode IF_MMODE_CODE 7,
        Inner IF_INNER_CODE 8,
        Void IF_VOID_CODE 9,
        HBox IF_HBOX_CODE 10,
        VBox IF_VBOX_CODE 11,
        IfX IFX_CODE 12,
        Eof IF_EOF_CODE 13,
        True IF_TRUE_CODE 14,
        False IF_FALSE_CODE 15,
        Case IF_CASE_CODE 16,
        Defined IF_DEF_CODE 17,
        CSName IF_CS_CODE 18,
        FontChar IF_FONT_CHAR_CODE 19,
        InCSName IF_IN_CSNAME_CODE 20,
        Primitive IF_PRIMITIVE_CODE 21,
    }
}

declare! {
    InteractionModes {
        Batch BATCH_MODE 0,
        Nonstop NONSTOP_MODE 1,
        Scroll SCROLL_MODE 2,
        ErrorStop ERROR_STOP_MODE 3,
    }
}

declare! {
    KernNodeSubtypes {
        Explicit EXPLICIT 1,
        Accent ACC_KERN 2,
        SpaceAdjustment SPACE_ADJUSTMENT 3,
    }
}

// In principle this should be dynamic depending on NUMBER_MATH_FAMILIES,
// but who's got time for that?
declare! {
    MathFontSizes {
        Text TEXT_SIZE 0,
        Script SCRIPT_SIZE 256,
        ScriptScript SCRIPT_SCRIPT_SIZE 512,
    }
}

declare! {
    MathNodeSubtypes {
        Before BEFORE 0,
        After AFTER 1,
        BeginM BEGIN_M_CODE 2,
        EndM END_M_CODE 3,
        BeginL BEGIN_L_CODE 6,
        EndL END_L_CODE 7,
        BeginR BEGIN_R_CODE 10,
        EndR END_R_CODE 11,
    }
}

declare! {
    NodeTypes {
        HList HLIST_NODE 0,
        VList VLIST_NODE 1,
        DeltaRule RULE_NODE 2,
        Insert INS_NODE 3,
        Mark MARK_NODE 4,
        Adjustment ADJUST_NODE 5,
        Ligature LIGATURE_NODE 6,
        Discretionary DISC_NODE 7,
        Whatsit WHATSIT_NODE 8,
        Math MATH_NODE 9,
        Glue GLUE_NODE 10,
        Kern KERN_NODE 11,
        Penalty PENALTY_NODE 12,
        Unset UNSET_NODE 13,
        Style STYLE_NODE 14,
        Choice CHOICE_NODE 15,
        MarginKern MARGIN_KERN_NODE 40,
    }
}

declare! {
    OpNoadSubtypes {
        Normal NORMAL 0,
        Limits LIMITS 1,
        NoLimits NO_LIMITS 2,
    }
}

declare! {
    SetBoxDimenCodes {
        Width WIDTH_OFFSET 1,
        Depth DEPTH_OFFSET 2,
        Height HEIGHT_OFFSET 3,
    }
}

// As of version 33, CHAR_SUB_DEF_CODE is no longer
// used, but to make life easy we still define it here.
declare! {
    ShorthandDefCodes {
        Char CHAR_DEF_CODE 0,
        MathChar MATH_CHAR_DEF_CODE 1,
        Count COUNT_DEF_CODE 2,
        Dimen DIMEN_DEF_CODE 3,
        Skip SKIP_DEF_CODE 4,
        MuSkip MU_SKIP_DEF_CODE 5,
        Tokens TOKS_DEF_CODE 6,
        CharSub CHAR_SUB_DEF_CODE 7,
        XetexMathCharNum XETEX_MATH_CHAR_NUM_DEF_CODE 8,
        XetexMathChar XETEX_MATH_CHAR_DEF_CODE 9,
    }
}

declare! {
    SkipCodes {
        Fill1 FIL_CODE 0,
        Fill2 FILL_CODE 1,
        StretchOrShrink SS_CODE 2,
        NegativeFill1 FIL_NEG_CODE 3,
        Skip SKIP_CODE 4,
        MathSkip MSKIP_CODE 5,
    }
}

declare! {
    StyleNodeSubtypes {
        Display DISPLAY_STYLE 0,
        Text TEXT_STYLE 2,
        Script SCRIPT_STYLE 4,
        ScriptScript SCRIPT_SCRIPT_STYLE 6,
    }
}

declare! {
    TopBotMarkCodes {
        TopMark TOP_MARK_CODE 0,
        FirstMark FIRST_MARK_CODE 1,
        BotMark BOT_MARK_CODE 2,
        SplitFirstMark SPLIT_FIRST_MARK_CODE 3,
        SplitBotMark SPLIT_BOT_MARK_CODE 4,
        TopMarks TT_TOP_MARKS_CODE 5,
        FirstMarks TT_FIRST_MARKS_CODE 6,
        BotMarks TT_BOT_MARKS_CODE 7,
        SplitFirstMarks TT_SPLIT_FIRST_MARKS_CODE 8,
        SplitBotMarks TT_SPLIT_BOT_MARKS_CODE 9,
    }
}

declare! {
    WhatsitNodeSubtypes {
        Open OPEN_NODE 0,
        Write WRITE_NODE 1,
        Close CLOSE_NODE 2,
        Special SPECIAL_NODE 3,
        Language LANGUAGE_NODE 4,
        PdfSavePos PDF_SAVE_POS_NODE 21,
        NativeWord NATIVE_WORD_NODE 40,
        NativeWordActualText NATIVE_WORD_NODE_AT 41,
        Glyph GLYPH_NODE 42, // Not to be confused with ExtensionCodes::Glyph
        Picture PIC_NODE 43, // Not to be confused with ExtensionCodes::PicFile
        Pdf PDF_NODE 44, // Not to be confused with ExtensionCodes::pdfFile
    }
}

declare! {
    XrayCodes {
        Show SHOW_CODE 0,
        ShowBox SHOW_BOX_CODE 1,
        ShowThe SHOW_THE_CODE 2,
        ShowLists SHOW_LISTS 3,
        ShowGroups SHOW_GROUPS 4,
        ShowTokens SHOW_TOKENS 5,
        ShowIfs SHOW_IFS 6,
    }
}
