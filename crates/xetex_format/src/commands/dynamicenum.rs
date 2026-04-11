// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Commands whose arguments map to the values of dynamic enums.

use tectonic_errors::prelude::*;

use super::{
    ArgKind, Command, CommandArgument, CommandBehavior, CommandPrimitive, PrimitiveExtraInit,
};
use crate::{
    enums::{dynamic::*, HasPrimitive},
    symbols::{HasSymbol, SymbolTable},
    FormatVersion,
};

macro_rules! declare_exhaustive {
    ($typename:tt $enumname:ident) => {
        #[derive(Debug)]
        pub struct $typename {
            codes: DynamicEnumResolver<$enumname>,
        }

        impl CommandBehavior for $typename {
            fn build(version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self> {
                let codes = $enumname::build_resolver(version, symbols)?;
                Ok($typename { codes })
            }
        }

        impl Command for $typename {
            fn describe(&self, arg: CommandArgument) -> String {
                match self
                    .codes
                    .resolve(arg as isize)
                    .map(|v| v.primitive())
                    .flatten()
                {
                    Some(p) => format!("[{}]", p),
                    None => format!("[{:?}?? {}]", self, arg),
                }
            }

            fn primitives(&self) -> Vec<CommandPrimitive> {
                let mut prims = Vec::new();

                for (_code, variant) in self.codes.active() {
                    if let Some(prim) = variant.primitive() {
                        let init = if prim == "write" {
                            PrimitiveExtraInit::Write
                        } else if prim == "special" {
                            PrimitiveExtraInit::Frozen("FROZEN_SPECIAL")
                        } else {
                            PrimitiveExtraInit::None
                        };

                        prims.push(CommandPrimitive {
                            name: prim,
                            arg: ArgKind::Symbol(variant.symbol()),
                            init,
                        });
                    }
                }

                prims
            }
        }
    };
}

declare_exhaustive! { LastItem LastItemCodes }
declare_exhaustive! { MakeBox BoxCodes }
declare_exhaustive! { Extension ExtensionCodes }
declare_exhaustive! { Convert ConvertCodes }

macro_rules! declare_partial {
    ($typename:tt $enumname:ident { $($primname:ident $varname:ident [$($init:tt)+],)+ } ) => {
        #[derive(Debug)]
        pub struct $typename {
            codes: DynamicEnumResolver<$enumname>,
        }

        impl CommandBehavior for $typename {
            fn build(version: FormatVersion, symbols: &mut SymbolTable) -> Result<Self> {
                let codes = $enumname::build_resolver(version, symbols)?;
                Ok($typename { codes })
            }
        }

        impl Command for $typename {
            fn describe(&self, arg: CommandArgument) -> String {
                let s = match self
                    .codes
                    .resolve(arg as isize)
                {
                    $(
                        Some($enumname::$varname) => stringify!($primname),
                    )+
                    Some(other) =>{ return format!("[{:?}?? {:?}]", self, other) },
                    None => { return format!("[{:?}?? {}]", self, arg) },
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

declare_partial! {
    UnHBox BoxCodes {
        unhbox Box [None],
        unhcopy Copy [None],
    }
}

declare_partial! {
    UnVBox BoxCodes {
        unvbox Box [None],
        unvcopy Copy [None],
        pagediscards LastBox [None],
        splitdiscards VSplit [None],
    }
}

// We've hackily added a new option to MathNoadTypes in order to make it
// possible to express this command in this formalism.
declare_partial! {
    LeftRight MathNoadTypes {
        middle LeftRightMiddleMode [None],
        left Left [None],
        right Right [Frozen("FROZEN_RIGHT")],
    }
}

declare_partial! {
    MathComp MathNoadTypes {
        mathord Ordinary [None],
        mathop BigOperator [None],
        mathbin BinaryOperator [None],
        mathrel Relation [None],
        mathopen Opening [None],
        mathclose Closing [None],
        mathpunct Punctuation [None],
        mathinner Inner [None],
        underline Underline [None],
        overline Overline [None],
    }
}

declare_partial! {
    SetAux Modes {
        prevdepth Vertical [None],
        spacefactor Horizontal [None],
    }
}
