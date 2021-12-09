// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//#![deny(missing_docs)]

//! The low-level primitive commands provided by the engine.

use std::io::{Result, Write};

use super::FormatVersion;

pub type CommandCode = i16;
pub type CommandArgument = i32;

/// Information about commands.
#[derive(Clone, Copy, Debug)]
pub struct Command {
    /// The symbolic name of the command in WEB2C.
    pub web2cname: &'static str,

    /// An alternative name for the command used in TeX's parser. Some commands
    /// can never make it out of the parser, so their associated codes are
    /// re-used deeper inside the engine.
    pub parser_overload_name: Option<&'static str>,

    /// An alternative name for the command used in macro token lists.
    pub macro_overload_name: Option<&'static str>,

    /// The semantics of the command argument value stored in TeX's `cur_chr`
    /// variable.
    pub arg_type: ArgumentType,

    /// The TeX primitives associated with the command.
    pub primitives: &'static [CommandPrimitive],

    /// The first format version in which the command was introduced.
    pub since: FormatVersion,
}

/// Different TeX command argument semantics.
#[derive(Clone, Copy, Debug)]
pub enum ArgumentType {
    /// This command has not yet had its argument type properly annotated.
    Unspecified,

    /// The argument is unused (and should probably be zero).
    Unused,

    /// The argument is a character (that is, a Unicode Scalar Value).
    Character,
}

/// A TeX primitive associated with a command.
#[derive(Clone, Copy, Debug)]
pub struct CommandPrimitive {
    pub name: &'static str,

    pub arg: &'static str,

    pub init: PrimitiveExtraInit,
}

/// Special initialization to be done after a primitive is created.
///
/// These operations usually involve initialization of the special "frozen"
/// primitives in the eqtb.
#[derive(Clone, Copy, Debug)]
pub enum PrimitiveExtraInit {
    /// No extra initialization.
    None,

    /// This is `\par`: initialize `par_loc` and `par_token`.
    Par,

    /// This is `\write`: initialize `write_loc`
    Write,

    /// This is a frozen primitive: initialize a frozen copy
    Frozen(&'static str),
}

const COMMANDS: &[Command] = &[
    Command {
        web2cname: "RELAX",
        parser_overload_name: Some("ESCAPE"),
        macro_overload_name: None,
        arg_type: ArgumentType::Unused,
        primitives: &[CommandPrimitive {
            name: "relax",
            arg: "TOO_BIG_USV",
            init: PrimitiveExtraInit::Frozen("FROZEN_RELAX"),
        }],
        since: 0,
    },
    Command {
        web2cname: "LEFT_BRACE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "RIGHT_BRACE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "MATH_SHIFT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "TAB_MARK",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[CommandPrimitive {
            name: "span",
            arg: "SPAN_CODE",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "CAR_RET",
        parser_overload_name: None,
        macro_overload_name: Some("OUT_PARAM"),
        arg_type: ArgumentType::Character,
        primitives: &[
            CommandPrimitive {
                name: "cr",
                arg: "CR_CODE",
                init: PrimitiveExtraInit::Frozen("FROZEN_CR"),
            },
            CommandPrimitive {
                name: "crcr",
                arg: "CR_CR_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "MAC_PARAM",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "SUP_MARK",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "SUB_MARK",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "ENDV",
        parser_overload_name: Some("IGNORE"),
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "SPACER",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "LETTER",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "OTHER_CHAR",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Character,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "PAR_END",
        parser_overload_name: Some("ACTIVE_CHAR"),
        macro_overload_name: Some("MATCH"),
        arg_type: ArgumentType::Character,
        primitives: &[CommandPrimitive {
            name: "par",
            arg: "TOO_BIG_USV",
            init: PrimitiveExtraInit::Par,
        }],
        since: 0,
    },
    Command {
        web2cname: "STOP",
        parser_overload_name: Some("COMMENT"),
        macro_overload_name: Some("END_MATCH"),
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "end",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "dump",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "DELIM_NUM",
        parser_overload_name: Some("INVALID_CHAR"),
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "delimiter",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Udelimiter",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXdelimiter",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "CHAR_NUM",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "char",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "MATH_CHAR_NUM",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "mathchar",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Umathcharnum",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXmathcharnum",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Umathchar",
                arg: "2",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXmathchar",
                arg: "2",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "MARK",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "mark",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "marks",
                arg: "5",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "XRAY",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "show",
                arg: "SHOW_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "showbox",
                arg: "SHOW_BOX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "showthe",
                arg: "SHOW_THE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "showlists",
                arg: "SHOW_LISTS",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "showgroups",
                arg: "SHOW_GROUPS",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "showtokens",
                arg: "SHOW_TOKENS",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "showifs",
                arg: "SHOW_IFS",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "MAKE_BOX",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "box",
                arg: "BOX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "copy",
                arg: "COPY_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "lastbox",
                arg: "LAST_BOX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "vsplit",
                arg: "VSPLIT_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "vtop",
                arg: "VTOP_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "vbox",
                arg: "VTOP_CODE + VMODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "hbox",
                arg: "VTOP_CODE + HMODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "HMOVE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "moveright",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "moveleft",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "VMOVE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "lower",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "raise",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "UN_HBOX",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "unhbox",
                arg: "BOX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "unhcopy",
                arg: "COPY_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "UN_VBOX",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "unvbox",
                arg: "BOX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "unvcopy",
                arg: "COPY_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pagediscards",
                arg: "LAST_BOX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "splitdiscards",
                arg: "VSPLIT_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "REMOVE_ITEM",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "unskip",
                arg: "GLUE_NODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "unkern",
                arg: "KERN_NODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "unpenalty",
                arg: "PENALTY_NODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "HSKIP",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "hfil",
                arg: "FIL_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "hfill",
                arg: "FILL_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "hss",
                arg: "SS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "hfilneg",
                arg: "FIL_NEG_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "hskip",
                arg: "SKIP_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "VSKIP",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "vfil",
                arg: "FIL_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "vfill",
                arg: "FILL_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "vss",
                arg: "SS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "vfilneg",
                arg: "FIL_NEG_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "vskip",
                arg: "SKIP_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "MSKIP",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "mskip",
            arg: "MSKIP_CODE",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "KERN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "kern",
            arg: "EXPLICIT",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "MKERN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "mkern",
            arg: "MU_GLUE",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "LEADER_SHIP",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "shipout",
                arg: "A_LEADERS - 1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "leaders",
                arg: "A_LEADERS",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "cleaders",
                arg: "C_LEADERS",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "xleaders",
                arg: "X_LEADERS",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "HALIGN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "halign",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "VALIGN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "valign",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "beginL",
                arg: "BEGIN_L_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "endL",
                arg: "END_L_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "beginR",
                arg: "BEGIN_R_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "endR",
                arg: "END_R_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "NO_ALIGN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "noalign",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "VRULE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "vrule",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "HRULE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "hrule",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "INSERT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "insert",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "VADJUST",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "vadjust",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "IGNORE_SPACES",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "ignorespaces",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "AFTER_ASSIGNMENT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "afterassignment",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "AFTER_GROUP",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "aftergroup",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "BREAK_PENALTY",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "penalty",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "START_PAR",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "noindent",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "indent",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "ITAL_CORR",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "/",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "ACCENT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "accent",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "MATH_ACCENT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "mathaccent",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Umathaccent",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXmathaccent",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "DISCRETIONARY",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "discretionary",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "-",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "EQ_NO",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "eqno",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "leqno",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "LEFT_RIGHT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "middle",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "left",
                arg: "LEFT_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "right",
                arg: "RIGHT_NOAD",
                init: PrimitiveExtraInit::Frozen("FROZEN_RIGHT"),
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "MATH_COMP",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "mathord",
                arg: "ORD_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathop",
                arg: "OP_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathbin",
                arg: "BIN_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathrel",
                arg: "REL_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathopen",
                arg: "OPEN_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathclose",
                arg: "CLOSE_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathpunct",
                arg: "PUNCT_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathinner",
                arg: "INNER_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "underline",
                arg: "UNDER_NOAD",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "overline",
                arg: "OVER_NOAD",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "LIMIT_SWITCH",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "displaylimits",
                arg: "NORMAL",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "limits",
                arg: "LIMITS",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "nolimits",
                arg: "NO_LIMITS",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "ABOVE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "above",
                arg: "ABOVE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "over",
                arg: "OVER_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "atop",
                arg: "ATOP_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "abovewithdelims",
                arg: "DELIMITED_CODE + ABOVE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "overwithdelims",
                arg: "DELIMITED_CODE + OVER_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "atopwithdelims",
                arg: "DELIMITED_CODE + ATOP_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "MATH_STYLE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "displaystyle",
                arg: "DISPLAY_STYLE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "textstyle",
                arg: "TEXT_STYLE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "scriptstyle",
                arg: "SCRIPT_STYLE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "scriptscriptstyle",
                arg: "SCRIPT_SCRIPT_STYLE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "MATH_CHOICE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "mathchoice",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "NON_SCRIPT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "nonscript",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "VCENTER",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "vcenter",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "CASE_SHIFT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "lowercase",
                arg: "LC_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "uppercase",
                arg: "UC_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "MESSAGE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "message",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "errmessage",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "EXTENSION",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "openout",
                arg: "OPEN_NODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "write",
                arg: "WRITE_NODE",
                init: PrimitiveExtraInit::Write,
            },
            CommandPrimitive {
                name: "closeout",
                arg: "CLOSE_NODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "special",
                arg: "SPECIAL_NODE",
                init: PrimitiveExtraInit::Frozen("FROZEN_SPECIAL"),
            },
            CommandPrimitive {
                name: "immediate",
                arg: "IMMEDIATE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "setlanguage",
                arg: "SET_LANGUAGE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pdfsavepos",
                arg: "PDF_SAVE_POS_NODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "resettimer",
                arg: "RESET_TIMER_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "setrandomseed",
                arg: "SET_RANDOM_SEED_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXdefaultencoding",
                arg: "XETEX_DEFAULT_ENCODING_EXTENSION_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXglyph",
                arg: "GLYPH_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXinputencoding",
                arg: "XETEX_INPUT_ENCODING_EXTENSION_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXlinebreaklocale",
                arg: "XETEX_LINEBREAK_LOCALE_EXTENSION_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXpdffile",
                arg: "PDF_FILE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXpicfile",
                arg: "PIC_FILE_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "IN_STREAM",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "closein",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "openin",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "BEGIN_GROUP",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "begingroup",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "END_GROUP",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "endgroup",
            arg: "0",
            init: PrimitiveExtraInit::Frozen("FROZEN_END_GROUP"),
        }],
        since: 0,
    },
    Command {
        web2cname: "OMIT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "omit",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "EX_SPACE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: " ",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "NO_BOUNDARY",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "noboundary",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "RADICAL",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "radical",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Uradical",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXradical",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "END_CS_NAME",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "endcsname",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        // this is MIN_INTERNAL
        web2cname: "CHAR_GIVEN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "MATH_GIVEN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "XETEX_MATH_GIVEN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        // this is MAX_NON_PREFIXED_COMMAND
        web2cname: "LAST_ITEM",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "badness",
                arg: "BADNESS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "currentgrouplevel",
                arg: "CURRENT_GROUP_LEVEL_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "currentgrouptype",
                arg: "CURRENT_GROUP_TYPE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "currentifbranch",
                arg: "CURRENT_IF_BRANCH_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "currentiflevel",
                arg: "CURRENT_IF_LEVEL_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "currentiftype",
                arg: "CURRENT_IF_TYPE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "dimexpr",
                arg: "ETEX_EXPR + 1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "elapsedtime",
                arg: "ELAPSED_TIME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "eTeXversion",
                arg: "ETEX_VERSION_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "fontchardp",
                arg: "FONT_CHAR_DP_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "fontcharht",
                arg: "FONT_CHAR_HT_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "fontcharic",
                arg: "FONT_CHAR_IC_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "fontcharwd",
                arg: "FONT_CHAR_WD_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "glueexpr",
                arg: "ETEX_EXPR + 2",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "glueshrink",
                arg: "GLUE_SHRINK_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "glueshrinkorder",
                arg: "GLUE_SHRINK_ORDER_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "gluestretch",
                arg: "GLUE_STRETCH_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "gluestretchorder",
                arg: "GLUE_STRETCH_ORDER_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "gluetomu",
                arg: "GLUE_TO_MU_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "inputlineno",
                arg: "INPUT_LINE_NO_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "lastkern",
                arg: "DIMEN_VAL",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "lastnodetype",
                arg: "LAST_NODE_TYPE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "lastpenalty",
                arg: "INT_VAL",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "lastskip",
                arg: "GLUE_VAL",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "muexpr",
                arg: "ETEX_EXPR + 3",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mutoglue",
                arg: "MU_TO_GLUE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "numexpr",
                arg: "ETEX_EXPR + 0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "parshapedimen",
                arg: "PAR_SHAPE_DIMEN_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "parshapeindent",
                arg: "PAR_SHAPE_INDENT_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "parshapelength",
                arg: "PAR_SHAPE_LENGTH_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pdflastxpos",
                arg: "PDF_LAST_X_POS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pdflastypos",
                arg: "PDF_LAST_Y_POS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "randomseed",
                arg: "RANDOM_SEED_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "shellescape",
                arg: "PDF_SHELL_ESCAPE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXcharglyph",
                arg: "XETEX_MAP_CHAR_TO_GLYPH_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXcountfeatures",
                arg: "XETEX_COUNT_FEATURES_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXcountglyphs",
                arg: "XETEX_COUNT_GLYPHS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXcountselectors",
                arg: "XETEX_COUNT_SELECTORS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXcountvariations",
                arg: "XETEX_COUNT_VARIATIONS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXfeaturecode",
                arg: "XETEX_FEATURE_CODE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXfindfeaturebyname",
                arg: "XETEX_FIND_FEATURE_BY_NAME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXfindselectorbyname",
                arg: "XETEX_FIND_SELECTOR_BY_NAME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXfindvariationbyname",
                arg: "XETEX_FIND_VARIATION_BY_NAME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXfirstfontchar",
                arg: "XETEX_FIRST_CHAR_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXfonttype",
                arg: "XETEX_FONT_TYPE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXglyphbounds",
                arg: "XETEX_GLYPH_BOUNDS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXglyphindex",
                arg: "XETEX_GLYPH_INDEX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXisdefaultselector",
                arg: "XETEX_IS_DEFAULT_SELECTOR_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXisexclusivefeature",
                arg: "XETEX_IS_EXCLUSIVE_FEATURE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXlastfontchar",
                arg: "XETEX_LAST_CHAR_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXOTcountfeatures",
                arg: "XETEX_OT_COUNT_FEATURES_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXOTcountlanguages",
                arg: "XETEX_OT_COUNT_LANGUAGES_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXOTcountscripts",
                arg: "XETEX_OT_COUNT_SCRIPTS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXOTfeaturetag",
                arg: "XETEX_OT_FEATURE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXOTlanguagetag",
                arg: "XETEX_OT_LANGUAGE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXOTscripttag",
                arg: "XETEX_OT_SCRIPT_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXpdfpagecount",
                arg: "XETEX_PDF_PAGE_COUNT_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXselectorcode",
                arg: "XETEX_SELECTOR_CODE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXvariationdefault",
                arg: "XETEX_VARIATION_DEFAULT_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXvariationmax",
                arg: "XETEX_VARIATION_MAX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXvariationmin",
                arg: "XETEX_VARIATION_MIN_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXvariation",
                arg: "XETEX_VARIATION_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXversion",
                arg: "XETEX_VERSION_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        // The following commands are assignment commands and
        // can therefore be prefixed by \global
        web2cname: "TOKS_REGISTER",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "toks",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "ASSIGN_TOKS",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[], // populated by local pars
        since: 0,
    },
    Command {
        web2cname: "ASSIGN_INT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[], // populated by intpars
        since: 0,
    },
    Command {
        web2cname: "ASSIGN_DIMEN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[], // populated by dimenpars
        since: 0,
    },
    Command {
        web2cname: "ASSIGN_GLUE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[], // populated by gluepars
        since: 0,
    },
    Command {
        web2cname: "ASSIGN_MU_GLUE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[], // populated by gluepars
        since: 0,
    },
    Command {
        web2cname: "ASSIGN_FONT_DIMEN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "fontdimen",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "ASSIGN_FONT_INT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "hyphenchar",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "skewchar",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "lpcode",
                arg: "2",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "rpcode",
                arg: "3",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "SET_AUX",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "prevdepth",
                arg: "VMODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "spacefactor",
                arg: "HMODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "SET_PREV_GRAF",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "prevgraf",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "SET_PAGE_DIMEN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "pagegoal",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pagetotal",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pagestretch",
                arg: "2",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pagefilstretch",
                arg: "3",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pagefillstretch",
                arg: "4",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pagefilllstretch",
                arg: "5",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pageshrink",
                arg: "6",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "pagedepth",
                arg: "7",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "SET_PAGE_INT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "deadcycles",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "insertpenalties",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "interactionmode",
                arg: "2",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "SET_BOX_DIMEN",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "wd",
                arg: "WIDTH_OFFSET",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "dp",
                arg: "DEPTH_OFFSET",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ht",
                arg: "HEIGHT_OFFSET",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "SET_SHAPE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "DEF_CODE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "catcode",
                arg: "CAT_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "lccode",
                arg: "LC_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "uccode",
                arg: "UC_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "sfcode",
                arg: "SF_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathcode",
                arg: "MATH_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "delcode",
                arg: "DEL_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "XETEX_DEF_CODE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "XeTeXcharclass",
                arg: "SF_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Umathcodenum",
                arg: "MATH_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXmathcodenum",
                arg: "MATH_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Umathcode",
                arg: "MATH_CODE_BASE + 1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXmathcode",
                arg: "MATH_CODE_BASE + 1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Udelcodenum",
                arg: "DEL_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXdelcodenum",
                arg: "DEL_CODE_BASE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Udelcode",
                arg: "DEL_CODE_BASE + 1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXdelcode",
                arg: "DEL_CODE_BASE + 1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "DEF_FAMILY",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "textfont",
                arg: "MATH_FONT_BASE + TEXT_SIZE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "scriptfont",
                arg: "MATH_FONT_BASE + SCRIPT_SIZE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "scriptscriptfont",
                arg: "MATH_FONT_BASE + SCRIPT_SCRIPT_SIZE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "SET_FONT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "nullfont",
            arg: "FONT_BASE",
            init: PrimitiveExtraInit::Frozen("FROZEN_NULL_FONT"),
        }],
        since: 0,
    },
    Command {
        web2cname: "DEF_FONT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "font",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        // this is MAX_INTERNAL
        web2cname: "REGISTER",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "count",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "dimen",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "skip",
                arg: "2",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "muskip",
                arg: "3",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "ADVANCE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "advance",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "MULTIPLY",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "multiply",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "DIVIDE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "divide",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "PREFIX",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "long",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "outer",
                arg: "2",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "global",
                arg: "4",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "protected",
                arg: "8",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "LET",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "let",
                arg: "NORMAL",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "futurelet",
                arg: "NORMAL + 1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "SHORTHAND_DEF",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "chardef",
                arg: "CHAR_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mathchardef",
                arg: "MATH_CHAR_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "countdef",
                arg: "COUNT_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "dimendef",
                arg: "DIMEN_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "skipdef",
                arg: "SKIP_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "muskipdef",
                arg: "MU_SKIP_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "toksdef",
                arg: "TOKS_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Umathcharnumdef",
                arg: "XETEX_MATH_CHAR_NUM_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXmathcharnumdef",
                arg: "XETEX_MATH_CHAR_NUM_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Umathchardef",
                arg: "XETEX_MATH_CHAR_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXmathchardef",
                arg: "XETEX_MATH_CHAR_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "READ_TO_CS",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "read",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "readline",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "DEF",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "def",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "gdef",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "edef",
                arg: "2",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "xdef",
                arg: "3",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "SET_BOX",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "setbox",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "HYPH_DATA",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "hyphenation",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "patterns",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        // This is also MAX_COMMAND: "the largest command code seen at
        // big_switch"
        web2cname: "SET_INTERACTION",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "batchmode",
                arg: "BATCH_MODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "nonstopmode",
                arg: "NONSTOP_MODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "scrollmode",
                arg: "SCROLL_MODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "errorstopmode",
                arg: "ERROR_STOP_MODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        // This command, and the remainders, are handled by the scanner
        // but can't make it through to the main control routine.
        web2cname: "UNDEFINED_CS",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "EXPAND_AFTER",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "expandafter",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "unless",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "NO_EXPAND",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "noexpand",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "primitive",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "INPUT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "input",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "endinput",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "scantokens",
                arg: "2",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "IF_TEST",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "if",
                arg: "IF_CHAR_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifcat",
                arg: "IF_CAT_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifnum",
                arg: "IF_INT_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifdim",
                arg: "IF_DIM_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifodd",
                arg: "IF_ODD_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifvmode",
                arg: "IF_VMODE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifhmode",
                arg: "IF_HMODE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifmmode",
                arg: "IF_MMODE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifinner",
                arg: "IF_INNER_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifvoid",
                arg: "IF_VOID_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifhbox",
                arg: "IF_HBOX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifvbox",
                arg: "IF_VBOX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifx",
                arg: "IFX_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifeof",
                arg: "IF_EOF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "iftrue",
                arg: "IF_TRUE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "iffalse",
                arg: "IF_FALSE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifcase",
                arg: "IF_CASE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifdefined",
                arg: "IF_DEF_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifcsname",
                arg: "IF_CS_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "iffontchar",
                arg: "IF_FONT_CHAR_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifincsname",
                arg: "IF_IN_CSNAME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "ifprimitive",
                arg: "IF_PRIMITIVE_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "FI_OR_ELSE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "fi",
                arg: "FI_CODE",
                init: PrimitiveExtraInit::Frozen("FROZEN_FI"),
            },
            CommandPrimitive {
                name: "else",
                arg: "ELSE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "or",
                arg: "OR_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "CS_NAME",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[CommandPrimitive {
            name: "csname",
            arg: "0",
            init: PrimitiveExtraInit::None,
        }],
        since: 0,
    },
    Command {
        web2cname: "CONVERT",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "number",
                arg: "NUMBER_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "romannumeral",
                arg: "ROMAN_NUMERAL_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "string",
                arg: "STRING_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "meaning",
                arg: "MEANING_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "fontname",
                arg: "FONT_NAME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "eTeXrevision",
                arg: "ETEX_REVISION_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "expanded",
                arg: "EXPANDED_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "leftmarginkern",
                arg: "LEFT_MARGIN_KERN_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "rightmarginkern",
                arg: "RIGHT_MARGIN_KERN_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "strcmp",
                arg: "PDF_STRCMP_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "creationdate",
                arg: "PDF_CREATION_DATE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "filemoddate",
                arg: "PDF_FILE_MOD_DATE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "filesize",
                arg: "PDF_FILE_SIZE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "mdfivesum",
                arg: "PDF_MDFIVE_SUM_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "filedump",
                arg: "PDF_FILE_DUMP_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "uniformdeviate",
                arg: "UNIFORM_DEVIATE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "normaldeviate",
                arg: "NORMAL_DEVIATE_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXvariationname",
                arg: "XETEX_VARIATION_NAME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXrevision",
                arg: "XETEX_REVISION_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXfeaturename",
                arg: "XETEX_FEATURE_NAME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXselectorname",
                arg: "XETEX_SELECTOR_NAME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "XeTeXglyphname",
                arg: "XETEX_GLYPH_NAME_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Uchar",
                arg: "XETEX_UCHAR_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "Ucharcat",
                arg: "XETEX_UCHARCAT_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "jobname",
                arg: "JOB_NAME_CODE",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "THE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "the",
                arg: "0",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "unexpanded",
                arg: "1",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "detokenize",
                arg: "SHOW_TOKENS",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "TOP_BOT_MARK",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[
            CommandPrimitive {
                name: "topmark",
                arg: "TOP_MARK_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "firstmark",
                arg: "FIRST_MARK_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "botmark",
                arg: "BOT_MARK_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "splitfirstmark",
                arg: "SPLIT_FIRST_MARK_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "splitbotmark",
                arg: "SPLIT_BOT_MARK_CODE",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "topmarks",
                arg: "TOP_MARK_CODE + 5",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "firstmarks",
                arg: "FIRST_MARK_CODE + 5",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "botmarks",
                arg: "BOT_MARK_CODE + 5",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "splitfirstmarks",
                arg: "SPLIT_FIRST_MARK_CODE + 5",
                init: PrimitiveExtraInit::None,
            },
            CommandPrimitive {
                name: "splitbotmarks",
                arg: "SPLIT_BOT_MARK_CODE + 5",
                init: PrimitiveExtraInit::None,
            },
        ],
        since: 0,
    },
    Command {
        web2cname: "CALL",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "LONG_CALL",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "OUTER_CALL",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "LONG_OUTER_CALL",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "END_TEMPLATE",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "DONT_EXPAND",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "GLUE_REF",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "SHAPE_REF",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "BOX_REF",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
    Command {
        web2cname: "DATA",
        parser_overload_name: None,
        macro_overload_name: None,
        arg_type: ArgumentType::Unspecified,
        primitives: &[],
        since: 0,
    },
];

/// Get information about the commands used in the latest engine format.
pub fn get_latest_commands() -> &'static [Command] {
    COMMANDS
}

/// Get information about the commands used in a specific engine format
/// version.
pub fn get_commands_for_version(version: FormatVersion) -> Vec<Command> {
    let mut r = Vec::new();

    for p in COMMANDS {
        if version >= p.since {
            r.push(*p)
        }
    }

    r
}

/// Emit the beginning of the C header information for the commands and
/// primitives.
pub fn emit_c_header_beginning<W: Write>(cmds: &[Command], mut stream: W) -> Result<()> {
    writeln!(
        stream,
        "/* Commands */

#undef IGNORE /* Windows OS headers sometimes define this */
"
    )?;

    for (index, cmd) in cmds.iter().enumerate() {
        writeln!(stream, "#define {} {}", cmd.web2cname, index)?;

        if let Some(o) = cmd.parser_overload_name {
            writeln!(
                stream,
                "#define {} {} /* Overloaded usage in parser */",
                o, cmd.web2cname
            )?;
        }

        if let Some(o) = cmd.macro_overload_name {
            writeln!(
                stream,
                "#define {} {} /* Overloaded usage in macro evaluation */",
                o, cmd.web2cname
            )?;
        }
    }

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

    for cmd in cmds {
        for prim in cmd.primitives {
            let extra_init = match prim.init {
                PrimitiveExtraInit::None => "xf_prim_init_none",
                PrimitiveExtraInit::Par => "xf_prim_init_par",
                PrimitiveExtraInit::Write => "xf_prim_init_write",
                PrimitiveExtraInit::Frozen(s) => s,
            };

            writeln!(
                stream,
                "    {{ \"{}\", {}, {}, {} }}, \\",
                prim.name, cmd.web2cname, prim.arg, extra_init
            )?;
        }
    }

    Ok(())
}

/// Emit the ending of the C header information for the commands and
/// primitives.
pub fn emit_c_header_ending<W: Write>(_cmds: &[Command], mut stream: W) -> Result<()> {
    // We just need to terminate the primitives macro definition
    writeln!(stream, "    {{ NULL, 0, 0, 0 }}")?;
    Ok(())
}
