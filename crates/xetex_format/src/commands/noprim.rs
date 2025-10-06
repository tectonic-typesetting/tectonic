// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Commands without primitives.

use tectonic_errors::prelude::*;

use super::{Command, CommandArgument, CommandBehavior, CommandPrimitive};
use crate::{format::Format, symbols::SymbolTable, FormatVersion};

macro_rules! declare {
    ($typename:ident) => {
        #[derive(Debug)]
        pub struct $typename {}

        impl CommandBehavior for $typename {
            fn build(_version: FormatVersion, _symbols: &mut SymbolTable) -> Result<Self> {
                Ok($typename {})
            }
        }

        impl Command for $typename {
            fn describe(&self, arg: CommandArgument) -> String {
                format!("[{:?} {}]", self, arg)
            }

            fn primitives(&self) -> Vec<CommandPrimitive> {
                Vec::new()
            }
        }
    };
}

// TODO THESE ACTUALLY HAVE PRIMITIVES!!!! FROM PARAMETERS!!!
declare! { AssignToks }
declare! { AssignInt }
declare! { AssignDimen }
declare! { AssignGlue }
declare! { AssignMuGlue }
declare! { SetShape } // "locals"

// Macro calls -- we customize these to provide `extended_info()` content

macro_rules! declare_call {
    ($typename:ident) => {
        #[derive(Debug)]
        pub struct $typename {}

        impl CommandBehavior for $typename {
            fn build(_version: FormatVersion, _symbols: &mut SymbolTable) -> Result<Self> {
                Ok($typename {})
            }
        }

        impl Command for $typename {
            fn describe(&self, arg: CommandArgument) -> String {
                format!("[{:?} {}]", self, arg)
            }

            fn primitives(&self) -> Vec<CommandPrimitive> {
                Vec::new()
            }

            fn extended_info(&self, arg: CommandArgument, format: &Format) -> Option<String> {
                Some(format.fmt_toklist(arg, true))
            }
        }
    };
}

declare_call! { Call }
declare_call! { LongCall }
declare_call! { OuterCall }
declare_call! { LongOuterCall }

// Shorthands
declare! { CharGiven }
declare! { MathGiven }
declare! { XetexMathGiven }

// All the rest.
declare! { EndV }
declare! { UndefinedCs }
declare! { EndTemplate }
declare! { DontExpand }
declare! { GlueRef }
declare! { ShapeRef }
declare! { BoxRef }
declare! { Data }
