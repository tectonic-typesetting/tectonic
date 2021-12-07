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
            },
            CommandPrimitive {
                name: "crcr",
                arg: "CR_CR_CODE",
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
            },
            CommandPrimitive {
                name: "dump",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "Udelimiter",
                arg: "1",
            },
            CommandPrimitive {
                name: "XeTeXdelimiter",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "Umathcharnum",
                arg: "1",
            },
            CommandPrimitive {
                name: "XeTeXmathcharnum",
                arg: "1",
            },
            CommandPrimitive {
                name: "Umathchar",
                arg: "2",
            },
            CommandPrimitive {
                name: "XeTeXmathchar",
                arg: "2",
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
            },
            CommandPrimitive {
                name: "marks",
                arg: "5",
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
            },
            CommandPrimitive {
                name: "showbox",
                arg: "SHOW_BOX_CODE",
            },
            CommandPrimitive {
                name: "showthe",
                arg: "SHOW_THE_CODE",
            },
            CommandPrimitive {
                name: "showlists",
                arg: "SHOW_LISTS",
            },
            CommandPrimitive {
                name: "showgroups",
                arg: "SHOW_GROUPS",
            },
            CommandPrimitive {
                name: "showtokens",
                arg: "SHOW_TOKENS",
            },
            CommandPrimitive {
                name: "showifs",
                arg: "SHOW_IFS",
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
            },
            CommandPrimitive {
                name: "copy",
                arg: "COPY_CODE",
            },
            CommandPrimitive {
                name: "lastbox",
                arg: "LAST_BOX_CODE",
            },
            CommandPrimitive {
                name: "vsplit",
                arg: "VSPLIT_CODE",
            },
            CommandPrimitive {
                name: "vtop",
                arg: "VTOP_CODE",
            },
            CommandPrimitive {
                name: "vbox",
                arg: "VTOP_CODE + VMODE",
            },
            CommandPrimitive {
                name: "hbox",
                arg: "VTOP_CODE + HMODE",
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
            },
            CommandPrimitive {
                name: "moveleft",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "raise",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "unhcopy",
                arg: "COPY_CODE",
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
            },
            CommandPrimitive {
                name: "unvcopy",
                arg: "COPY_CODE",
            },
            CommandPrimitive {
                name: "pagediscards",
                arg: "LAST_BOX_CODE",
            },
            CommandPrimitive {
                name: "splitdiscards",
                arg: "VSPLIT_CODE",
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
            },
            CommandPrimitive {
                name: "unkern",
                arg: "KERN_NODE",
            },
            CommandPrimitive {
                name: "unpenalty",
                arg: "PENALTY_NODE",
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
            },
            CommandPrimitive {
                name: "hfill",
                arg: "FILL_CODE",
            },
            CommandPrimitive {
                name: "hss",
                arg: "SS_CODE",
            },
            CommandPrimitive {
                name: "hfilneg",
                arg: "FIL_NEG_CODE",
            },
            CommandPrimitive {
                name: "hskip",
                arg: "SKIP_CODE",
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
            },
            CommandPrimitive {
                name: "vfill",
                arg: "FILL_CODE",
            },
            CommandPrimitive {
                name: "vss",
                arg: "SS_CODE",
            },
            CommandPrimitive {
                name: "vfilneg",
                arg: "FIL_NEG_CODE",
            },
            CommandPrimitive {
                name: "vskip",
                arg: "SKIP_CODE",
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
            },
            CommandPrimitive {
                name: "leaders",
                arg: "A_LEADERS",
            },
            CommandPrimitive {
                name: "cleaders",
                arg: "C_LEADERS",
            },
            CommandPrimitive {
                name: "xleaders",
                arg: "X_LEADERS",
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
            },
            CommandPrimitive {
                name: "beginL",
                arg: "BEGIN_L_CODE",
            },
            CommandPrimitive {
                name: "endL",
                arg: "END_L_CODE",
            },
            CommandPrimitive {
                name: "beginR",
                arg: "BEGIN_R_CODE",
            },
            CommandPrimitive {
                name: "endR",
                arg: "END_R_CODE",
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
            },
            CommandPrimitive {
                name: "indent",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "Umathaccent",
                arg: "1",
            },
            CommandPrimitive {
                name: "XeTeXmathaccent",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "-",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "leqno",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "left",
                arg: "LEFT_NOAD",
            },
            CommandPrimitive {
                name: "right",
                arg: "RIGHT_NOAD",
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
            },
            CommandPrimitive {
                name: "mathop",
                arg: "OP_NOAD",
            },
            CommandPrimitive {
                name: "mathbin",
                arg: "BIN_NOAD",
            },
            CommandPrimitive {
                name: "mathrel",
                arg: "REL_NOAD",
            },
            CommandPrimitive {
                name: "mathopen",
                arg: "OPEN_NOAD",
            },
            CommandPrimitive {
                name: "mathclose",
                arg: "CLOSE_NOAD",
            },
            CommandPrimitive {
                name: "mathpunct",
                arg: "PUNCT_NOAD",
            },
            CommandPrimitive {
                name: "mathinner",
                arg: "INNER_NOAD",
            },
            CommandPrimitive {
                name: "underline",
                arg: "UNDER_NOAD",
            },
            CommandPrimitive {
                name: "overline",
                arg: "OVER_NOAD",
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
            },
            CommandPrimitive {
                name: "limits",
                arg: "LIMITS",
            },
            CommandPrimitive {
                name: "nolimits",
                arg: "NO_LIMITS",
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
            },
            CommandPrimitive {
                name: "over",
                arg: "OVER_CODE",
            },
            CommandPrimitive {
                name: "atop",
                arg: "ATOP_CODE",
            },
            CommandPrimitive {
                name: "abovewithdelims",
                arg: "DELIMITED_CODE + ABOVE_CODE",
            },
            CommandPrimitive {
                name: "overwithdelims",
                arg: "DELIMITED_CODE + OVER_CODE",
            },
            CommandPrimitive {
                name: "atopwithdelims",
                arg: "DELIMITED_CODE + ATOP_CODE",
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
            },
            CommandPrimitive {
                name: "textstyle",
                arg: "TEXT_STYLE",
            },
            CommandPrimitive {
                name: "scriptstyle",
                arg: "SCRIPT_STYLE",
            },
            CommandPrimitive {
                name: "scriptscriptstyle",
                arg: "SCRIPT_SCRIPT_STYLE",
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
            },
            CommandPrimitive {
                name: "uppercase",
                arg: "UC_CODE_BASE",
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
            },
            CommandPrimitive {
                name: "errmessage",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "write",
                arg: "WRITE_NODE",
            },
            CommandPrimitive {
                name: "closeout",
                arg: "CLOSE_NODE",
            },
            CommandPrimitive {
                name: "special",
                arg: "SPECIAL_NODE",
            },
            CommandPrimitive {
                name: "immediate",
                arg: "IMMEDIATE_CODE",
            },
            CommandPrimitive {
                name: "setlanguage",
                arg: "SET_LANGUAGE_CODE",
            },
            CommandPrimitive {
                name: "pdfsavepos",
                arg: "PDF_SAVE_POS_NODE",
            },
            CommandPrimitive {
                name: "resettimer",
                arg: "RESET_TIMER_CODE",
            },
            CommandPrimitive {
                name: "setrandomseed",
                arg: "SET_RANDOM_SEED_CODE",
            },
            CommandPrimitive {
                name: "XeTeXdefaultencoding",
                arg: "XETEX_DEFAULT_ENCODING_EXTENSION_CODE",
            },
            CommandPrimitive {
                name: "XeTeXglyph",
                arg: "GLYPH_CODE",
            },
            CommandPrimitive {
                name: "XeTeXinputencoding",
                arg: "XETEX_INPUT_ENCODING_EXTENSION_CODE",
            },
            CommandPrimitive {
                name: "XeTeXlinebreaklocale",
                arg: "XETEX_LINEBREAK_LOCALE_EXTENSION_CODE",
            },
            CommandPrimitive {
                name: "XeTeXpdffile",
                arg: "PDF_FILE_CODE",
            },
            CommandPrimitive {
                name: "XeTeXpicfile",
                arg: "PIC_FILE_CODE",
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
            },
            CommandPrimitive {
                name: "openin",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "Uradical",
                arg: "1",
            },
            CommandPrimitive {
                name: "XeTeXradical",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "currentgrouplevel",
                arg: "CURRENT_GROUP_LEVEL_CODE",
            },
            CommandPrimitive {
                name: "currentgrouptype",
                arg: "CURRENT_GROUP_TYPE_CODE",
            },
            CommandPrimitive {
                name: "currentifbranch",
                arg: "CURRENT_IF_BRANCH_CODE",
            },
            CommandPrimitive {
                name: "currentiflevel",
                arg: "CURRENT_IF_LEVEL_CODE",
            },
            CommandPrimitive {
                name: "currentiftype",
                arg: "CURRENT_IF_TYPE_CODE",
            },
            CommandPrimitive {
                name: "dimexpr",
                arg: "ETEX_EXPR + 1",
            },
            CommandPrimitive {
                name: "elapsedtime",
                arg: "ELAPSED_TIME_CODE",
            },
            CommandPrimitive {
                name: "eTeXversion",
                arg: "ETEX_VERSION_CODE",
            },
            CommandPrimitive {
                name: "fontchardp",
                arg: "FONT_CHAR_DP_CODE",
            },
            CommandPrimitive {
                name: "fontcharht",
                arg: "FONT_CHAR_HT_CODE",
            },
            CommandPrimitive {
                name: "fontcharic",
                arg: "FONT_CHAR_IC_CODE",
            },
            CommandPrimitive {
                name: "fontcharwd",
                arg: "FONT_CHAR_WD_CODE",
            },
            CommandPrimitive {
                name: "glueexpr",
                arg: "ETEX_EXPR + 2",
            },
            CommandPrimitive {
                name: "glueshrink",
                arg: "GLUE_SHRINK_CODE",
            },
            CommandPrimitive {
                name: "glueshrinkorder",
                arg: "GLUE_SHRINK_ORDER_CODE",
            },
            CommandPrimitive {
                name: "gluestretch",
                arg: "GLUE_STRETCH_CODE",
            },
            CommandPrimitive {
                name: "gluestretchorder",
                arg: "GLUE_STRETCH_ORDER_CODE",
            },
            CommandPrimitive {
                name: "gluetomu",
                arg: "GLUE_TO_MU_CODE",
            },
            CommandPrimitive {
                name: "inputlineno",
                arg: "INPUT_LINE_NO_CODE",
            },
            CommandPrimitive {
                name: "lastkern",
                arg: "DIMEN_VAL",
            },
            CommandPrimitive {
                name: "lastnodetype",
                arg: "LAST_NODE_TYPE_CODE",
            },
            CommandPrimitive {
                name: "lastpenalty",
                arg: "INT_VAL",
            },
            CommandPrimitive {
                name: "lastskip",
                arg: "GLUE_VAL",
            },
            CommandPrimitive {
                name: "muexpr",
                arg: "ETEX_EXPR + 3",
            },
            CommandPrimitive {
                name: "mutoglue",
                arg: "MU_TO_GLUE_CODE",
            },
            CommandPrimitive {
                name: "numexpr",
                arg: "ETEX_EXPR + 0",
            },
            CommandPrimitive {
                name: "parshapedimen",
                arg: "PAR_SHAPE_DIMEN_CODE",
            },
            CommandPrimitive {
                name: "parshapeindent",
                arg: "PAR_SHAPE_INDENT_CODE",
            },
            CommandPrimitive {
                name: "parshapelength",
                arg: "PAR_SHAPE_LENGTH_CODE",
            },
            CommandPrimitive {
                name: "pdflastxpos",
                arg: "PDF_LAST_X_POS_CODE",
            },
            CommandPrimitive {
                name: "pdflastypos",
                arg: "PDF_LAST_Y_POS_CODE",
            },
            CommandPrimitive {
                name: "randomseed",
                arg: "RANDOM_SEED_CODE",
            },
            CommandPrimitive {
                name: "shellescape",
                arg: "PDF_SHELL_ESCAPE_CODE",
            },
            CommandPrimitive {
                name: "XeTeXcharglyph",
                arg: "XETEX_MAP_CHAR_TO_GLYPH_CODE",
            },
            CommandPrimitive {
                name: "XeTeXcountfeatures",
                arg: "XETEX_COUNT_FEATURES_CODE",
            },
            CommandPrimitive {
                name: "XeTeXcountglyphs",
                arg: "XETEX_COUNT_GLYPHS_CODE",
            },
            CommandPrimitive {
                name: "XeTeXcountselectors",
                arg: "XETEX_COUNT_SELECTORS_CODE",
            },
            CommandPrimitive {
                name: "XeTeXcountvariations",
                arg: "XETEX_COUNT_VARIATIONS_CODE",
            },
            CommandPrimitive {
                name: "XeTeXfeaturecode",
                arg: "XETEX_FEATURE_CODE_CODE",
            },
            CommandPrimitive {
                name: "XeTeXfindfeaturebyname",
                arg: "XETEX_FIND_FEATURE_BY_NAME_CODE",
            },
            CommandPrimitive {
                name: "XeTeXfindselectorbyname",
                arg: "XETEX_FIND_SELECTOR_BY_NAME_CODE",
            },
            CommandPrimitive {
                name: "XeTeXfindvariationbyname",
                arg: "XETEX_FIND_VARIATION_BY_NAME_CODE",
            },
            CommandPrimitive {
                name: "XeTeXfirstfontchar",
                arg: "XETEX_FIRST_CHAR_CODE",
            },
            CommandPrimitive {
                name: "XeTeXfonttype",
                arg: "XETEX_FONT_TYPE_CODE",
            },
            CommandPrimitive {
                name: "XeTeXglyphbounds",
                arg: "XETEX_GLYPH_BOUNDS_CODE",
            },
            CommandPrimitive {
                name: "XeTeXglyphindex",
                arg: "XETEX_GLYPH_INDEX_CODE",
            },
            CommandPrimitive {
                name: "XeTeXisdefaultselector",
                arg: "XETEX_IS_DEFAULT_SELECTOR_CODE",
            },
            CommandPrimitive {
                name: "XeTeXisexclusivefeature",
                arg: "XETEX_IS_EXCLUSIVE_FEATURE_CODE",
            },
            CommandPrimitive {
                name: "XeTeXlastfontchar",
                arg: "XETEX_LAST_CHAR_CODE",
            },
            CommandPrimitive {
                name: "XeTeXOTcountfeatures",
                arg: "XETEX_OT_COUNT_FEATURES_CODE",
            },
            CommandPrimitive {
                name: "XeTeXOTcountlanguages",
                arg: "XETEX_OT_COUNT_LANGUAGES_CODE",
            },
            CommandPrimitive {
                name: "XeTeXOTcountscripts",
                arg: "XETEX_OT_COUNT_SCRIPTS_CODE",
            },
            CommandPrimitive {
                name: "XeTeXOTfeaturetag",
                arg: "XETEX_OT_FEATURE_CODE",
            },
            CommandPrimitive {
                name: "XeTeXOTlanguagetag",
                arg: "XETEX_OT_LANGUAGE_CODE",
            },
            CommandPrimitive {
                name: "XeTeXOTscripttag",
                arg: "XETEX_OT_SCRIPT_CODE",
            },
            CommandPrimitive {
                name: "XeTeXpdfpagecount",
                arg: "XETEX_PDF_PAGE_COUNT_CODE",
            },
            CommandPrimitive {
                name: "XeTeXselectorcode",
                arg: "XETEX_SELECTOR_CODE_CODE",
            },
            CommandPrimitive {
                name: "XeTeXvariationdefault",
                arg: "XETEX_VARIATION_DEFAULT_CODE",
            },
            CommandPrimitive {
                name: "XeTeXvariationmax",
                arg: "XETEX_VARIATION_MAX_CODE",
            },
            CommandPrimitive {
                name: "XeTeXvariationmin",
                arg: "XETEX_VARIATION_MIN_CODE",
            },
            CommandPrimitive {
                name: "XeTeXvariation",
                arg: "XETEX_VARIATION_CODE",
            },
            CommandPrimitive {
                name: "XeTeXversion",
                arg: "XETEX_VERSION_CODE",
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
            },
            CommandPrimitive {
                name: "skewchar",
                arg: "1",
            },
            CommandPrimitive {
                name: "lpcode",
                arg: "2",
            },
            CommandPrimitive {
                name: "rpcode",
                arg: "3",
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
            },
            CommandPrimitive {
                name: "spacefactor",
                arg: "HMODE",
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
            },
            CommandPrimitive {
                name: "pagetotal",
                arg: "1",
            },
            CommandPrimitive {
                name: "pagestretch",
                arg: "2",
            },
            CommandPrimitive {
                name: "pagefilstretch",
                arg: "3",
            },
            CommandPrimitive {
                name: "pagefillstretch",
                arg: "4",
            },
            CommandPrimitive {
                name: "pagefilllstretch",
                arg: "5",
            },
            CommandPrimitive {
                name: "pageshrink",
                arg: "6",
            },
            CommandPrimitive {
                name: "pagedepth",
                arg: "7",
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
            },
            CommandPrimitive {
                name: "insertpenalties",
                arg: "1",
            },
            CommandPrimitive {
                name: "interactionmode",
                arg: "2",
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
            },
            CommandPrimitive {
                name: "dp",
                arg: "DEPTH_OFFSET",
            },
            CommandPrimitive {
                name: "ht",
                arg: "HEIGHT_OFFSET",
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
            },
            CommandPrimitive {
                name: "lccode",
                arg: "LC_CODE_BASE",
            },
            CommandPrimitive {
                name: "uccode",
                arg: "UC_CODE_BASE",
            },
            CommandPrimitive {
                name: "sfcode",
                arg: "SF_CODE_BASE",
            },
            CommandPrimitive {
                name: "mathcode",
                arg: "MATH_CODE_BASE",
            },
            CommandPrimitive {
                name: "delcode",
                arg: "DEL_CODE_BASE",
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
            },
            CommandPrimitive {
                name: "Umathcodenum",
                arg: "MATH_CODE_BASE",
            },
            CommandPrimitive {
                name: "XeTeXmathcodenum",
                arg: "MATH_CODE_BASE",
            },
            CommandPrimitive {
                name: "Umathcode",
                arg: "MATH_CODE_BASE + 1",
            },
            CommandPrimitive {
                name: "XeTeXmathcode",
                arg: "MATH_CODE_BASE + 1",
            },
            CommandPrimitive {
                name: "Udelcodenum",
                arg: "DEL_CODE_BASE",
            },
            CommandPrimitive {
                name: "XeTeXdelcodenum",
                arg: "DEL_CODE_BASE",
            },
            CommandPrimitive {
                name: "Udelcode",
                arg: "DEL_CODE_BASE + 1",
            },
            CommandPrimitive {
                name: "XeTeXdelcode",
                arg: "DEL_CODE_BASE + 1",
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
            },
            CommandPrimitive {
                name: "scriptfont",
                arg: "MATH_FONT_BASE + SCRIPT_SIZE",
            },
            CommandPrimitive {
                name: "scriptscriptfont",
                arg: "MATH_FONT_BASE + SCRIPT_SCRIPT_SIZE",
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
            },
            CommandPrimitive {
                name: "dimen",
                arg: "1",
            },
            CommandPrimitive {
                name: "skip",
                arg: "2",
            },
            CommandPrimitive {
                name: "muskip",
                arg: "3",
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
            },
            CommandPrimitive {
                name: "outer",
                arg: "2",
            },
            CommandPrimitive {
                name: "global",
                arg: "4",
            },
            CommandPrimitive {
                name: "protected",
                arg: "8",
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
            },
            CommandPrimitive {
                name: "futurelet",
                arg: "NORMAL + 1",
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
            },
            CommandPrimitive {
                name: "mathchardef",
                arg: "MATH_CHAR_DEF_CODE",
            },
            CommandPrimitive {
                name: "countdef",
                arg: "COUNT_DEF_CODE",
            },
            CommandPrimitive {
                name: "dimendef",
                arg: "DIMEN_DEF_CODE",
            },
            CommandPrimitive {
                name: "skipdef",
                arg: "SKIP_DEF_CODE",
            },
            CommandPrimitive {
                name: "muskipdef",
                arg: "MU_SKIP_DEF_CODE",
            },
            CommandPrimitive {
                name: "toksdef",
                arg: "TOKS_DEF_CODE",
            },
            CommandPrimitive {
                name: "Umathcharnumdef",
                arg: "XETEX_MATH_CHAR_NUM_DEF_CODE",
            },
            CommandPrimitive {
                name: "XeTeXmathcharnumdef",
                arg: "XETEX_MATH_CHAR_NUM_DEF_CODE",
            },
            CommandPrimitive {
                name: "Umathchardef",
                arg: "XETEX_MATH_CHAR_DEF_CODE",
            },
            CommandPrimitive {
                name: "XeTeXmathchardef",
                arg: "XETEX_MATH_CHAR_DEF_CODE",
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
            },
            CommandPrimitive {
                name: "readline",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "gdef",
                arg: "1",
            },
            CommandPrimitive {
                name: "edef",
                arg: "2",
            },
            CommandPrimitive {
                name: "xdef",
                arg: "3",
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
            },
            CommandPrimitive {
                name: "patterns",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "nonstopmode",
                arg: "NONSTOP_MODE",
            },
            CommandPrimitive {
                name: "scrollmode",
                arg: "SCROLL_MODE",
            },
            CommandPrimitive {
                name: "errorstopmode",
                arg: "ERROR_STOP_MODE",
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
            },
            CommandPrimitive {
                name: "unless",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "primitive",
                arg: "1",
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
            },
            CommandPrimitive {
                name: "endinput",
                arg: "1",
            },
            CommandPrimitive {
                name: "scantokens",
                arg: "2",
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
            },
            CommandPrimitive {
                name: "ifcat",
                arg: "IF_CAT_CODE",
            },
            CommandPrimitive {
                name: "ifnum",
                arg: "IF_INT_CODE",
            },
            CommandPrimitive {
                name: "ifdim",
                arg: "IF_DIM_CODE",
            },
            CommandPrimitive {
                name: "ifodd",
                arg: "IF_ODD_CODE",
            },
            CommandPrimitive {
                name: "ifvmode",
                arg: "IF_VMODE_CODE",
            },
            CommandPrimitive {
                name: "ifhmode",
                arg: "IF_HMODE_CODE",
            },
            CommandPrimitive {
                name: "ifmmode",
                arg: "IF_MMODE_CODE",
            },
            CommandPrimitive {
                name: "ifinner",
                arg: "IF_INNER_CODE",
            },
            CommandPrimitive {
                name: "ifvoid",
                arg: "IF_VOID_CODE",
            },
            CommandPrimitive {
                name: "ifhbox",
                arg: "IF_HBOX_CODE",
            },
            CommandPrimitive {
                name: "ifvbox",
                arg: "IF_VBOX_CODE",
            },
            CommandPrimitive {
                name: "ifx",
                arg: "IFX_CODE",
            },
            CommandPrimitive {
                name: "ifeof",
                arg: "IF_EOF_CODE",
            },
            CommandPrimitive {
                name: "iftrue",
                arg: "IF_TRUE_CODE",
            },
            CommandPrimitive {
                name: "iffalse",
                arg: "IF_FALSE_CODE",
            },
            CommandPrimitive {
                name: "ifcase",
                arg: "IF_CASE_CODE",
            },
            CommandPrimitive {
                name: "ifdefined",
                arg: "IF_DEF_CODE",
            },
            CommandPrimitive {
                name: "ifcsname",
                arg: "IF_CS_CODE",
            },
            CommandPrimitive {
                name: "iffontchar",
                arg: "IF_FONT_CHAR_CODE",
            },
            CommandPrimitive {
                name: "ifincsname",
                arg: "IF_IN_CSNAME_CODE",
            },
            CommandPrimitive {
                name: "ifprimitive",
                arg: "IF_PRIMITIVE_CODE",
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
            },
            CommandPrimitive {
                name: "else",
                arg: "ELSE_CODE",
            },
            CommandPrimitive {
                name: "or",
                arg: "OR_CODE",
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
            },
            CommandPrimitive {
                name: "romannumeral",
                arg: "ROMAN_NUMERAL_CODE",
            },
            CommandPrimitive {
                name: "string",
                arg: "STRING_CODE",
            },
            CommandPrimitive {
                name: "meaning",
                arg: "MEANING_CODE",
            },
            CommandPrimitive {
                name: "fontname",
                arg: "FONT_NAME_CODE",
            },
            CommandPrimitive {
                name: "eTeXrevision",
                arg: "ETEX_REVISION_CODE",
            },
            CommandPrimitive {
                name: "expanded",
                arg: "EXPANDED_CODE",
            },
            CommandPrimitive {
                name: "leftmarginkern",
                arg: "LEFT_MARGIN_KERN_CODE",
            },
            CommandPrimitive {
                name: "rightmarginkern",
                arg: "RIGHT_MARGIN_KERN_CODE",
            },
            CommandPrimitive {
                name: "strcmp",
                arg: "PDF_STRCMP_CODE",
            },
            CommandPrimitive {
                name: "creationdate",
                arg: "PDF_CREATION_DATE_CODE",
            },
            CommandPrimitive {
                name: "filemoddate",
                arg: "PDF_FILE_MOD_DATE_CODE",
            },
            CommandPrimitive {
                name: "filesize",
                arg: "PDF_FILE_SIZE_CODE",
            },
            CommandPrimitive {
                name: "mdfivesum",
                arg: "PDF_MDFIVE_SUM_CODE",
            },
            CommandPrimitive {
                name: "filedump",
                arg: "PDF_FILE_DUMP_CODE",
            },
            CommandPrimitive {
                name: "uniformdeviate",
                arg: "UNIFORM_DEVIATE_CODE",
            },
            CommandPrimitive {
                name: "normaldeviate",
                arg: "NORMAL_DEVIATE_CODE",
            },
            CommandPrimitive {
                name: "XeTeXvariationname",
                arg: "XETEX_VARIATION_NAME_CODE",
            },
            CommandPrimitive {
                name: "XeTeXrevision",
                arg: "XETEX_REVISION_CODE",
            },
            CommandPrimitive {
                name: "XeTeXfeaturename",
                arg: "XETEX_FEATURE_NAME_CODE",
            },
            CommandPrimitive {
                name: "XeTeXselectorname",
                arg: "XETEX_SELECTOR_NAME_CODE",
            },
            CommandPrimitive {
                name: "XeTeXglyphname",
                arg: "XETEX_GLYPH_NAME_CODE",
            },
            CommandPrimitive {
                name: "Uchar",
                arg: "XETEX_UCHAR_CODE",
            },
            CommandPrimitive {
                name: "Ucharcat",
                arg: "XETEX_UCHARCAT_CODE",
            },
            CommandPrimitive {
                name: "jobname",
                arg: "JOB_NAME_CODE",
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
            },
            CommandPrimitive {
                name: "unexpanded",
                arg: "1",
            },
            CommandPrimitive {
                name: "detokenize",
                arg: "SHOW_TOKENS",
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
            },
            CommandPrimitive {
                name: "firstmark",
                arg: "FIRST_MARK_CODE",
            },
            CommandPrimitive {
                name: "botmark",
                arg: "BOT_MARK_CODE",
            },
            CommandPrimitive {
                name: "splitfirstmark",
                arg: "SPLIT_FIRST_MARK_CODE",
            },
            CommandPrimitive {
                name: "splitbotmark",
                arg: "SPLIT_BOT_MARK_CODE",
            },
            CommandPrimitive {
                name: "topmarks",
                arg: "TOP_MARK_CODE + 5",
            },
            CommandPrimitive {
                name: "firstmarks",
                arg: "FIRST_MARK_CODE + 5",
            },
            CommandPrimitive {
                name: "botmarks",
                arg: "BOT_MARK_CODE + 5",
            },
            CommandPrimitive {
                name: "splitfirstmarks",
                arg: "SPLIT_FIRST_MARK_CODE + 5",
            },
            CommandPrimitive {
                name: "splitbotmarks",
                arg: "SPLIT_BOT_MARK_CODE + 5",
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

typedef struct tectonic_primitive_def_t {{
    char *name;
    eight_bits cmd;
    int32_t chr;
}} tectonic_primitive_def_t;

#define TECTONIC_PRIMITIVE_INITIALIZERS \\"
    )?;

    for cmd in cmds {
        for prim in cmd.primitives {
            writeln!(
                stream,
                "    {{ \"{}\", {}, {} }}, \\",
                prim.name, cmd.web2cname, prim.arg
            )?;
        }
    }

    Ok(())
}

/// Emit the ending of the C header information for the commands and
/// primitives.
pub fn emit_c_header_ending<W: Write>(_cmds: &[Command], mut stream: W) -> Result<()> {
    // We just need to terminate the primitives macro definition
    writeln!(stream, "    {{ NULL, 0, 0 }}")?;
    Ok(())
}
