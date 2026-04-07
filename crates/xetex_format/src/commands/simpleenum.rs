// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Commands that have have primitives with argument codes associated with
//! "simple" enums, whose numerical values we know at compile-time.

use tectonic_errors::prelude::*;

use super::{
    ArgKind, Command, CommandArgument, CommandBehavior, CommandPrimitive, PrimitiveExtraInit,
};
use crate::{
    enums::simple::*,
    symbols::{HasSymbol, SymbolTable},
    FormatVersion,
};

macro_rules! declare {
    ($typename:tt $enumname:ident { $($primname:ident $varname:ident [$($init:tt)+],)+ }) => {
        #[derive(Debug)]
        pub struct $typename {}

        impl CommandBehavior for $typename {
            fn build(_version: FormatVersion, _symbols: &mut SymbolTable) -> Result<Self> {
                Ok($typename {})
            }
        }

        impl Command for $typename {
            #[allow(non_upper_case_globals)]
            fn describe(&self, arg: CommandArgument) -> String {
                $(
                    const $varname: CommandArgument = $enumname::$varname as CommandArgument;
                )+

                let s = match arg {
                    $(
                        $varname => stringify!($primname),
                    )+
                    _ => { return format!("[{:?}?? {}]", self, arg) }
                };
                format!("[{}]", s)
            }

            fn primitives(&self) -> Vec<CommandPrimitive> {
                let mut prims = Vec::new();

                $(
                    let name = stringify!($primname);
                    let arg: ArgKind = ArgKind::Symbol($enumname::$varname.symbol());
                    let init: PrimitiveExtraInit = PrimitiveExtraInit::$($init)+;
                    prims.push(CommandPrimitive { name, arg, init });
                )+

                prims
            }
        }
    };
}

declare! {
    XRay XrayCodes {
        show Show [None],
        showbox ShowBox [None],
        showthe ShowThe [None],
        showlists ShowLists [None],
        showgroups ShowGroups [None],
        showtokens ShowTokens [None],
        showifs ShowIfs [None],
    }
}

declare! {
    RemoveItem NodeTypes {
        unskip Glue [None],
        unkern Kern [None],
        unpenalty Penalty [None],
    }
}

declare! {
    HSkip SkipCodes {
        hfil Fill1 [None],
        hfill Fill2 [None],
        hss StretchOrShrink [None],
        hfilneg NegativeFill1 [None],
        hskip Skip [None],
    }
}

declare! {
    VSkip SkipCodes {
        vfil Fill1 [None],
        vfill Fill2 [None],
        vss StretchOrShrink [None],
        vfilneg NegativeFill1 [None],
        vskip Skip [None],
    }
}

declare! {
    MSkip SkipCodes {
        mskip MathSkip [None],
    }
}

declare! {
    Kern KernNodeSubtypes {
        kern Explicit [None],
    }
}

declare! {
    MathKern GlueNodeSubtypes {
        mkern MuGlueOrShipout [None],
    }
}

declare! {
    LeaderShip GlueNodeSubtypes {
        shipout MuGlueOrShipout [None],
        leaders AlignedLeaders [None],
        cleaders CenteredLeaders [None],
        xleaders ExpandedLeaders [None],
    }
}

declare! {
    VAlign MathNodeSubtypes {
        valign Before [None], // quasi-hack: should be "unnamed(0)"
        beginL BeginL [None],
        endL EndL [None],
        beginR BeginR [None],
        endR EndR [None],
    }
}

declare! {
    LimitSwitch OpNoadSubtypes {
        displaylimits Normal [None],
        limits Limits [None],
        nolimits NoLimits [None],
    }
}

declare! {
    Above AboveCodes {
        above Above [None],
        over Over [None],
        atop Atop [None],
        abovewithdelims AboveWithDelims [None],
        overwithdelims OverWithDelims [None],
        atopwithdelims AtopWithDelims [None],
    }
}

declare! {
    MathStyle StyleNodeSubtypes {
        displaystyle Display [None],
        textstyle Text [None],
        scriptstyle Script [None],
        scriptscriptstyle ScriptScript [None],
    }
}

declare! {
    SetBoxDimen SetBoxDimenCodes {
        wd Width [None],
        dp Depth [None],
        ht Height [None],
    }
}

declare! {
    SetInteraction InteractionModes {
        batchmode Batch [None],
        nonstopmode Nonstop [None],
        scrollmode Scroll [None],
        errorstopmode ErrorStop [None],
    }
}

declare! {
    IfTest IfCodes {
        if Char [None],
        ifcat Cat [None],
        ifnum Int [None],
        ifdim Dim [None],
        ifodd Odd [None],
        ifvmode VMode [None],
        ifhmode HMode [None],
        ifmmode MMode [None],
        ifinner Inner [None],
        ifvoid Void [None],
        ifhbox HBox [None],
        ifvbox VBox [None],
        ifx IfX [None],
        ifeof Eof [None],
        iftrue True [None],
        iffalse False [None],
        ifcase Case [None],
        ifdefined Defined [None],
        ifcsname CSName [None],
        iffontchar FontChar [None],
        ifincsname InCSName [None],
        ifprimitive Primitive [None],
    }
}

declare! {
    FiOrElse FiOrElseCodes {
        fi Fi [Frozen("FROZEN_FI")],
        else Else [None],
        or Or [None],
    }
}

declare! {
    The XrayCodes {
        the Show [None], // quasi-hack; should be 0
        unexpanded ShowBox [None], // quasi-hack; should be 1
        detokenize ShowTokens [None],
    }
}

declare! {
    TopBotMark TopBotMarkCodes {
        topmark TopMark [None],
        firstmark FirstMark [None],
        botmark BotMark [None],
        splitfirstmark SplitFirstMark [None],
        splitbotmark SplitBotMark [None],
        topmarks TopMarks [None],
        firstmarks FirstMarks [None],
        botmarks BotMarks [None],
        splitfirstmarks SplitFirstMarks [None],
        splitbotmarks SplitBotMarks [None],
    }
}

// This is *almost* a simple enum, but we have multiple primitives for the same
// argument.
#[derive(Debug)]
pub struct ShorthandDef {}

impl CommandBehavior for ShorthandDef {
    fn build(_version: FormatVersion, _symbols: &mut SymbolTable) -> Result<Self> {
        Ok(ShorthandDef {})
    }
}

impl Command for ShorthandDef {
    #[allow(non_upper_case_globals)]
    fn describe(&self, arg: CommandArgument) -> String {
        const CHAR: CommandArgument = ShorthandDefCodes::Char as CommandArgument;
        const MATH_CHAR: CommandArgument = ShorthandDefCodes::MathChar as CommandArgument;
        const COUNT: CommandArgument = ShorthandDefCodes::Count as CommandArgument;
        const DIMEN: CommandArgument = ShorthandDefCodes::Dimen as CommandArgument;
        const SKIP: CommandArgument = ShorthandDefCodes::Skip as CommandArgument;
        const MU_SKIP: CommandArgument = ShorthandDefCodes::MuSkip as CommandArgument;
        const TOKS: CommandArgument = ShorthandDefCodes::Tokens as CommandArgument;
        const XETEX_MATH_CHAR_NUM: CommandArgument =
            ShorthandDefCodes::XetexMathCharNum as CommandArgument;
        const XETEX_MATH_CHAR: CommandArgument =
            ShorthandDefCodes::XetexMathChar as CommandArgument;

        let s = match arg {
            CHAR => "chardef",
            MATH_CHAR => "mathchardef",
            COUNT => "countdef",
            DIMEN => "dimendef",
            SKIP => "skipdef",
            MU_SKIP => "muskipdef",
            TOKS => "toksdef",
            XETEX_MATH_CHAR_NUM => "XeTeXmathcharnumdef",
            XETEX_MATH_CHAR => "XeTeXmathchardef",
            _ => return format!("[{self:?}?? {arg}]"),
        };
        format!("[{s}]")
    }

    fn primitives(&self) -> Vec<CommandPrimitive> {
        vec![
            CommandPrimitive {
                name: "chardef",
                arg: ArgKind::Symbol(ShorthandDefCodes::Char.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathchardef",
                arg: ArgKind::Symbol(ShorthandDefCodes::MathChar.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "countdef",
                arg: ArgKind::Symbol(ShorthandDefCodes::Count.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "dimendef",
                arg: ArgKind::Symbol(ShorthandDefCodes::Dimen.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "skipdef",
                arg: ArgKind::Symbol(ShorthandDefCodes::Skip.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "muskipdef",
                arg: ArgKind::Symbol(ShorthandDefCodes::MuSkip.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "toksdef",
                arg: ArgKind::Symbol(ShorthandDefCodes::Tokens.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Umathcharnumdef",
                arg: ArgKind::Symbol(ShorthandDefCodes::XetexMathCharNum.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXmathcharnumdef",
                arg: ArgKind::Symbol(ShorthandDefCodes::XetexMathCharNum.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Umathchardef",
                arg: ArgKind::Symbol(ShorthandDefCodes::XetexMathChar.symbol()),
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXmathchardef",
                arg: ArgKind::Symbol(ShorthandDefCodes::XetexMathChar.symbol()),
                init: PrimitiveExtraInit::None,
            },
        ]
    }
}
