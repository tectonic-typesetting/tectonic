// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! The first 16 special character-code commands

use tectonic_errors::prelude::*;

use super::{
    ArgKind, Command, CommandArgument, CommandBehavior, CommandPrimitive, PrimitiveExtraInit,
};
use crate::{symbols::SymbolTable, FormatVersion};

/// Relax!
#[derive(Debug)]
pub struct Relax {
    too_big_usv: CommandArgument,
}

impl CommandBehavior for Relax {
    fn build(_version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self> {
        let too_big_usv = symbols.lookup("TOO_BIG_USV") as CommandArgument;
        Ok(Relax { too_big_usv })
    }
}

impl Command for Relax {
    fn describe(&self, arg: CommandArgument) -> String {
        if arg == self.too_big_usv {
            "[relax]".to_owned()
        } else {
            format!("[relax {}]", crate::format::fmt_usv(arg))
        }
    }

    fn primitives(&self) -> Vec<CommandPrimitive> {
        vec![CommandPrimitive {
            name: "relax",
            arg: ArgKind::Symbol("TOO_BIG_USV"),
            init: PrimitiveExtraInit::Frozen("FROZEN_RELAX"),
        }]
    }
}

/// Tab mark
#[derive(Debug)]
pub struct TabMark {
    span_code: CommandArgument,
}

impl CommandBehavior for TabMark {
    fn build(_version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self> {
        let span_code = symbols.lookup("SPAN_CODE") as CommandArgument;
        Ok(TabMark { span_code })
    }
}

impl Command for TabMark {
    fn describe(&self, arg: CommandArgument) -> String {
        if arg == self.span_code {
            "[span]".to_owned()
        } else {
            format!("[tab-mark {}]", crate::format::fmt_usv(arg))
        }
    }

    fn primitives(&self) -> Vec<CommandPrimitive> {
        vec![CommandPrimitive {
            name: "span",
            arg: ArgKind::Symbol("SPAN_CODE"),
            init: PrimitiveExtraInit::None,
        }]
    }
}

/// Carriage return
#[derive(Debug)]
pub struct CarRet {
    cr_code: CommandArgument,
    cr_cr_code: CommandArgument,
}

impl CommandBehavior for CarRet {
    fn build(_version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self> {
        let cr_code = symbols.lookup("CR_CODE") as CommandArgument;
        let cr_cr_code = symbols.lookup("CR_CR_CODE") as CommandArgument;
        Ok(CarRet {
            cr_code,
            cr_cr_code,
        })
    }
}

impl Command for CarRet {
    fn describe(&self, arg: CommandArgument) -> String {
        if arg == self.cr_code {
            "[cr]".to_owned()
        } else if arg == self.cr_cr_code {
            "[crcr]".to_owned()
        } else {
            format!("[car-ret {}]", crate::format::fmt_usv(arg))
        }
    }

    fn primitives(&self) -> Vec<CommandPrimitive> {
        vec![
            CommandPrimitive {
                name: "cr",
                arg: ArgKind::Symbol("CR_CODE"),
                init: PrimitiveExtraInit::Frozen("FROZEN_CR"),
            },
            CommandPrimitive {
                name: "crcr",
                arg: ArgKind::Symbol("CR_CR_CODE"),
                init: PrimitiveExtraInit::None,
            },
        ]
    }
}

/// End of paragraph
#[derive(Debug)]
pub struct ParEnd {
    too_big_usv: CommandArgument,
}

impl CommandBehavior for ParEnd {
    fn build(_version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self> {
        let too_big_usv = symbols.lookup("TOO_BIG_USV") as CommandArgument;
        Ok(ParEnd { too_big_usv })
    }
}

impl Command for ParEnd {
    fn describe(&self, arg: CommandArgument) -> String {
        if arg == self.too_big_usv {
            "[par]".to_owned()
        } else {
            format!("[par {}]", crate::format::fmt_usv(arg))
        }
    }

    fn primitives(&self) -> Vec<CommandPrimitive> {
        vec![CommandPrimitive {
            name: "par",
            arg: ArgKind::Symbol("TOO_BIG_USV"),
            init: PrimitiveExtraInit::Par,
        }]
    }
}

macro_rules! declare {
    ($typename:ident $desc:literal) => {
        #[derive(Debug)]
        pub struct $typename {}

        impl CommandBehavior for $typename {
            fn build(_version: FormatVersion, _symbols: &mut SymbolTable) -> Result<Self> {
                Ok($typename {})
            }
        }

        impl Command for $typename {
            fn describe(&self, arg: CommandArgument) -> String {
                format!("[{} {}]", $desc, crate::format::fmt_usv(arg))
            }

            fn primitives(&self) -> Vec<CommandPrimitive> {
                Vec::new()
            }
        }
    };
}

declare! { LeftBrace "left-brace" }
declare! { RightBrace "right-brace" }
declare! { MathShift "math-shift" }
declare! { MacroParam "mac-param" }
declare! { SupMark "sup" }
declare! { SubMark "sub" }
declare! { Spacer "space" }
declare! { Letter "letter" }
declare! { OtherChar "other" }
