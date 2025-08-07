// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Commands that have a simple set of associated primitives and arguments.

use std::collections::BTreeMap;
use tectonic_errors::prelude::*;

use super::{
    ArgKind, Command, CommandArgument, CommandBehavior, CommandPrimitive, PrimitiveExtraInit,
};
use crate::{symbols::SymbolTable, FormatVersion};

macro_rules! inner_parse_primname {
    ([$n:literal]) => {
        $n
    };
    ($n:ident) => {
        stringify!($n)
    };
}

macro_rules! declare {
    ($typename:tt { $($primname:tt [$($argkind:tt)+] [$($init:tt)+],)+ }) => {
        #[derive(Debug)]
        pub struct $typename {
            args: BTreeMap<CommandArgument, String>,
        }

        impl CommandBehavior for $typename {
            fn build(_version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self> {
                let mut args = BTreeMap::new();

                $(
                    let argkind: ArgKind = ArgKind::$($argkind)+;
                    args.insert(argkind.get_value(symbols), inner_parse_primname!($primname).to_owned());
                )+

                Ok($typename { args } )
            }
        }

        impl Command for $typename {
            fn describe(&self, arg: CommandArgument) -> String {
                if let Some(s) = self.args.get(&arg) {
                    format!("[{}]", s)
                } else {
                    format!("[{}?? {}]", stringify!($typename), arg)
                }
            }

            fn primitives(&self) -> Vec<CommandPrimitive> {
                let mut prims = Vec::new();

                $(
                    let name = inner_parse_primname!($primname);
                    let arg: ArgKind = ArgKind::$($argkind)+;
                    let init: PrimitiveExtraInit = PrimitiveExtraInit::$($init)+;
                    prims.push(CommandPrimitive { name, arg, init
                    });
                )+

                prims
            }
        }
    };
}

declare! {
    Stop {
        end [Unnamed(0)] [None],
        dump [Unnamed(1)] [None],
    }
}

declare! {
    DelimNum {
        delimiter [Unnamed(0)] [None],
        Udelimiter [Unnamed(1)] [None],
        XeTeXdelimiter [Unnamed(1)] [None],
    }
}

declare! {
    CharNum {
        char [Unnamed(0)] [None],
    }
}

declare! {
    MathCharNum {
        mathchar [Unnamed(0)] [None],
        Umathcharnum [Unnamed(1)] [None],
        XeTeXmathcharnum [Unnamed(1)] [None],
        Umathchar [Unnamed(2)] [None],
        XeTeXmathchar [Unnamed(2)] [None],
    }
}

declare! {
    Mark {
        mark [Unnamed(0)] [None],
        marks [Unnamed(5)] [None],
    }
}

declare! {
    HMove {
        moveright [Unnamed(0)] [None],
        moveleft [Unnamed(1)] [None],
    }
}

declare! {
    VMove {
        lower [Unnamed(0)] [None],
        raise [Unnamed(1)] [None],
    }
}

declare! {
    HAlign {
        halign [Unnamed(0)] [None],
    }
}

declare! {
    NoAlign {
        noalign [Unnamed(0)] [None],
    }
}

declare! {
    VRule {
        vrule [Unnamed(0)] [None],
    }
}

declare! {
    HRule {
        hrule [Unnamed(0)] [None],
    }
}

declare! {
    Insert {
        insert [Unnamed(0)] [None],
    }
}

declare! {
    VAdjust {
        vadjust [Unnamed(0)] [None],
    }
}

declare! {
    IgnoreSpaces {
        ignorespaces [Unnamed(0)] [None],
    }
}

declare! {
    AfterAssignment {
        afterassignment [Unnamed(0)] [None],
    }
}

declare! {
    AfterGroup {
        aftergroup [Unnamed(0)] [None],
    }
}

declare! {
    BreakPenalty {
        penalty [Unnamed(0)] [None],
    }
}

declare! {
    StartPar {
        noindent [Unnamed(0)] [None],
        indent [Unnamed(1)] [None],
    }
}

declare! {
    ItalicCorrection {
        ["/"] [Unnamed(0)] [None],
    }
}

declare! {
    Accent {
        accent [Unnamed(0)] [None],
    }
}

declare! {
    MathAccent {
        mathaccent [Unnamed(0)] [None],
        Umathaccent [Unnamed(1)] [None],
        XeTeXmathaccent [Unnamed(1)] [None],
    }
}

declare! {
    Discretionary {
        discretionary [Unnamed(0)] [None],
        ["-"] [Unnamed(1)] [None],
    }
}

declare! {
    EquationNumber {
        eqno [Unnamed(0)] [None],
        leqno [Unnamed(1)] [None],
    }
}

declare! {
    MathChoice {
        mathchoice [Unnamed(0)] [None],
    }
}

declare! {
    NonScript {
        nonscript [Unnamed(0)] [None],
    }
}

declare! {
    VCenter {
        vcenter [Unnamed(0)] [None],
    }
}

declare! {
    Message {
        message [Unnamed(0)] [None],
        errmessage [Unnamed(1)] [None],
    }
}

declare! {
    InStream {
        closein [Unnamed(0)] [None],
        openin [Unnamed(1)] [None],
    }
}

declare! {
    BeginGroup {
        begingroup [Unnamed(0)] [None],
    }
}

declare! {
    EndGroup {
        endgroup [Unnamed(0)] [Frozen("FROZEN_END_GROUP")],
    }
}

declare! {
    Omit {
        omit [Unnamed(0)] [None],
    }
}

declare! {
    ExSpace {
        [" "] [Unnamed(0)] [None],
    }
}

declare! {
    NoBoundary {
        noboundary [Unnamed(0)] [None],
    }
}

declare! {
    Radical {
        radical [Unnamed(0)] [None],
        Uradical [Unnamed(1)] [None],
        XeTeXradical [Unnamed(1)] [None],
    }
}

declare! {
    EndCsName {
        endcsname [Unnamed(0)] [None],
    }
}

declare! {
    ToksRegister {
        toks [Unnamed(0)] [None],
    }
}

declare! {
    AssignFontDimen {
        fontdimen [Unnamed(0)] [None],
    }
}

declare! {
    AssignFontInt {
        hyphenchar [Unnamed(0)] [None],
        skewchar [Unnamed(1)] [None],
        lpcode [Unnamed(2)] [None],
        rpcode [Unnamed(3)] [None],
    }
}

declare! {
    SetPrevGraf {
        prevgraf [Unnamed(0)] [None],
    }
}

declare! {
    SetPageDimen {
        pagegoal [Unnamed(0)] [None],
        pagetotal [Unnamed(1)] [None],
        pagestretch [Unnamed(2)] [None],
        pagefilstretch [Unnamed(3)] [None],
        pagefillstretch [Unnamed(4)] [None],
        pagefilllstretch [Unnamed(5)] [None],
        pageshrink [Unnamed(6)] [None],
        pagedepth [Unnamed(7)] [None],
    }
}

declare! {
    SetPageInt {
        deadcycles [Unnamed(0)] [None],
        insertpenalties [Unnamed(1)] [None],
        interactionmode [Unnamed(2)] [None],
    }
}

declare! {
    DefFont {
        font [Unnamed(0)] [None],
    }
}

declare! {
    Register {
        count [Unnamed(0)] [None],
        dimen [Unnamed(1)] [None],
        skip [Unnamed(2)] [None],
        muskip [Unnamed(3)] [None],
    }
}

declare! {
    Advance {
        advance [Unnamed(0)] [None],
    }
}

declare! {
    Multiply {
        multiply [Unnamed(0)] [None],
    }
}

declare! {
    Divide {
        divide [Unnamed(0)] [None],
    }
}

declare! {
    Prefix {
        long [Unnamed(1)] [None],
        outer [Unnamed(2)] [None],
        global [Unnamed(4)] [None],
        protected [Unnamed(8)] [None],
    }
}

declare! {
    Let {
        let [Unnamed(0)] [None],
        futurelet [Unnamed(1)] [None],
    }
}

declare! {
    ReadToCs {
        read [Unnamed(0)] [None],
        readline [Unnamed(1)] [None],
    }
}

declare! {
    Def {
        def [Unnamed(0)] [None],
        gdef [Unnamed(1)] [None],
        edef [Unnamed(2)] [None],
        xdef [Unnamed(3)] [None],
    }
}

declare! {
    SetBox {
        setbox [Unnamed(0)] [None],
    }
}

declare! {
    HyphData {
        hyphenation [Unnamed(0)] [None],
        patterns [Unnamed(1)] [None],
    }
}

declare! {
    ExpandAfter {
        expandafter [Unnamed(0)] [None],
        unless [Unnamed(1)] [None],
    }
}

declare! {
    NoExpand {
        noexpand [Unnamed(0)] [None],
        primitive [Unnamed(1)] [None],
    }
}

declare! {
    Input {
        input [Unnamed(0)] [None],
        endinput [Unnamed(1)] [None],
        scantokens [Unnamed(2)] [None],
    }
}

declare! {
    CsName {
        csname [Unnamed(0)] [None],
    }
}

// The 0 argument is nominally FONT_BASE.
//
// TODO: non-zero args correspond to fonts that have been loaded into memory.
declare! {
    SetFont {
        nullfont [Unnamed(0)] [Frozen("FROZEN_NULL_FONT")],
    }
}
