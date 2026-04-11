// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Commands whose arguments come from the symbol table, but not one of the
//! enumerations.

use std::collections::HashMap;
use tectonic_errors::prelude::*;

use super::{
    ArgKind, Command, CommandArgument, CommandBehavior, CommandPrimitive, PrimitiveExtraInit,
};
use crate::{symbols::SymbolTable, FormatVersion};

/// An extra offset that can be applied to the base symbol.
enum Offset {
    None,
    Delta(isize),
    Symbol(&'static str),
}

macro_rules! declare {
    ($typename:tt { $($primname:ident $symname:ident [$($offset:tt)+] [$($init:tt)+],)+ } ) => {
        #[derive(Debug)]
        pub struct $typename {
            args: HashMap<CommandArgument, String>,
        }

        impl CommandBehavior for $typename {
            fn build(_version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self> {
                let mut args = HashMap::new();

                $(
                    let offset = Offset::$($offset)+;
                    let code = symbols.lookup(stringify!($symname)) + match offset {
                        Offset::None => 0,
                        Offset::Delta(d) => d,
                        Offset::Symbol(s) => symbols.lookup(s),
                    };
                    args.insert(code as CommandArgument, stringify!($primname).to_owned());
                )+

                Ok($typename { args })
            }
        }

        impl Command for $typename {
            fn describe(&self, arg: CommandArgument) -> String {
                if let Some(p) = self.args.get(&arg) {
                    format!("[{}]", p)
                } else {
                    format!("[{:?}?? {}]", self, arg)
                }
            }

            fn primitives(&self) -> Vec<CommandPrimitive> {
                let mut prims = Vec::new();

                $(
                    let offset = Offset::$($offset)+;
                    let name = stringify!($primname);
                    let arg = match offset {
                        Offset::None => ArgKind::Symbol(stringify!($symname)),
                        _ => {
                            // Hack for XetexDefCode, where there are multiple
                            // primitives with the same arg. They're always Uxxx
                            // and XeTeXxxx, and XeTeX comes second.
                            let tblname = name.replace("U", "XeTeX");
                            // Silly and inefficient. Oh well.
                            let v = self.args
                                        .iter()
                                        .find_map(|(arg, prim)| if prim == &tblname { Some(*arg) } else { None });
                            ArgKind::Unnamed(v.unwrap() as isize)
                        }
                    };
                    let init: PrimitiveExtraInit = PrimitiveExtraInit::$($init)+;
                    prims.push(CommandPrimitive { name, arg, init });
                )+

                prims
            }
        }
    };
}

declare! {
    CaseShift {
        lowercase LC_CODE_BASE [None] [None],
        uppercase UC_CODE_BASE [None] [None],
    }
}

declare! {
    DefCode {
        catcode CAT_CODE_BASE [None] [None],
        lccode LC_CODE_BASE [None] [None],
        uccode UC_CODE_BASE [None] [None],
        sfcode SF_CODE_BASE [None] [None],
        mathcode MATH_CODE_BASE [None] [None],
        delcode DEL_CODE_BASE [None] [None],
    }
}

declare! {
    XetexDefCode {
        XeTeXcharclass SF_CODE_BASE [None] [None],
        Umathcodenum MATH_CODE_BASE [None] [None],
        XeTeXmathcodenum MATH_CODE_BASE [None] [None],
        Umathcode MATH_CODE_BASE [Delta(1)] [None],
        XeTeXmathcode MATH_CODE_BASE [Delta(1)] [None],
        Udelcodenum DEL_CODE_BASE [None] [None],
        XeTeXdelcodenum DEL_CODE_BASE [None] [None],
        Udelcode DEL_CODE_BASE [Delta(1)] [None],
        XeTeXdelcode DEL_CODE_BASE [Delta(1)] [None],
    }
}

declare! {
    DefFamily {
        textfont MATH_FONT_BASE [Symbol("TEXT_SIZE")] [None],
        scriptfont MATH_FONT_BASE [Symbol("SCRIPT_SIZE")] [None],
        scriptscriptfont MATH_FONT_BASE [Symbol("SCRIPT_SCRIPT_SIZE")] [None],
    }
}
