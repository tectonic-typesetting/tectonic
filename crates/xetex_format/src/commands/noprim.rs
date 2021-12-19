// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//#![deny(missing_docs)]

//! Commands without primitives.

use tectonic_errors::prelude::*;

use super::{Command, CommandArgument, CommandBehavior, CommandPrimitive};
use crate::{symbols::SymbolTable, FormatVersion};

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

// Shorthands
declare! { CharGiven }
declare! { MathGiven }
declare! { XetexMathGiven }

// All the rest.
declare! { EndV }
declare! { UndefinedCs }
declare! { Call }
declare! { LongCall }
declare! { OuterCall }
declare! { LongOuterCall }
declare! { EndTemplate }
declare! { DontExpand }
declare! { GlueRef }
declare! { ShapeRef }
declare! { BoxRef }
declare! { Data }
