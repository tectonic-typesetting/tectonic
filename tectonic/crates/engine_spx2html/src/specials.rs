// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

//! TeX `\special` items recognized by the spx2html emitter.

use std::fmt::{Display, Error, Formatter};
use tectonic_status_base::{tt_warning, StatusBackend};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum Special<'a> {
    AddTemplate(&'a str),
    AutoStartParagraph,
    AutoEndParagraph,
    CanvasEnd(&'a str),
    CanvasStart(&'a str),
    ContentFinished,
    DirectText(&'a str),
    EndDefineFontFamily,
    EndFontFamilyTagAssociations,
    Emit,
    ManualEnd(&'a str),
    ManualFlexibleStart(&'a str),
    ProvideFile(&'a str),
    ProvideSpecial(&'a str),
    SetOutputPath(&'a str),
    SetTemplate(&'a str),
    SetTemplateVariable(&'a str),
    StartDefineFontFamily,
    StartFontFamilyTagAssociations,
}

impl<'a> Special<'a> {
    pub(crate) fn parse(text: &'a str, status: &mut dyn StatusBackend) -> Option<Self> {
        // str.split_once() would be nice but it was introduced in 1.52 which is
        // a bit recent for us.

        let mut pieces = text.splitn(2, ' ');

        let (cmd, remainder) = if let Some(p) = pieces.next() {
            if let Some(cmd) = p.strip_prefix("tdux:") {
                (cmd, pieces.next().unwrap_or_default())
            } else {
                return None;
            }
        } else {
            return None;
        };

        Some(match cmd {
            "asp" => Special::AutoStartParagraph,
            "aep" => Special::AutoEndParagraph,
            "cs" => Special::CanvasStart(remainder),
            "ce" => Special::CanvasEnd(remainder),
            "mfs" => Special::ManualFlexibleStart(remainder),
            "me" => Special::ManualEnd(remainder),
            "dt" => Special::DirectText(remainder),
            "emit" => Special::Emit,
            "addTemplate" => Special::AddTemplate(remainder),
            "setTemplate" => Special::SetTemplate(remainder),
            "setOutputPath" => Special::SetOutputPath(remainder),
            "setTemplateVariable" => Special::SetTemplateVariable(remainder),
            "provideFile" => Special::ProvideFile(remainder),
            "provideSpecial" => Special::ProvideSpecial(remainder),
            "contentFinished" => Special::ContentFinished,
            "startDefineFontFamily" => Special::StartDefineFontFamily,
            "endDefineFontFamily" => Special::EndDefineFontFamily,
            "startFontFamilyTagAssociations" => Special::StartFontFamilyTagAssociations,
            "endFontFamilyTagAssociations" => Special::EndFontFamilyTagAssociations,
            _ => {
                tt_warning!(
                    status,
                    "ignoring unrecognized Tectonic special: tdux:{} {}",
                    cmd,
                    remainder
                );
                return None;
            }
        })
    }

    pub fn ends_initialization(&self) -> bool {
        matches!(
            self,
            Special::Emit
                | Special::ProvideFile(_)
                | Special::ProvideSpecial(_)
                | Special::AutoStartParagraph
                | Special::AutoEndParagraph
                | Special::CanvasStart(_)
                | Special::CanvasEnd(_)
                | Special::ManualFlexibleStart(_)
                | Special::ManualEnd(_)
                | Special::DirectText(_)
        )
    }
}

impl<'a> Display for Special<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let (cmd, rest) = match self {
            Special::AddTemplate(t) => ("addTemplate", Some(t)),
            Special::AutoStartParagraph => ("asp", None),
            Special::AutoEndParagraph => ("aep", None),
            Special::CanvasEnd(t) => ("ce", Some(t)),
            Special::CanvasStart(t) => ("cs", Some(t)),
            Special::ContentFinished => ("contentFinished", None),
            Special::DirectText(t) => ("dt", Some(t)),
            Special::EndDefineFontFamily => ("endDefineFontFamily", None),
            Special::EndFontFamilyTagAssociations => ("endFontFamilyTagAssociations", None),
            Special::Emit => ("emit", None),
            Special::ManualEnd(t) => ("me", Some(t)),
            Special::ManualFlexibleStart(t) => ("mfs", Some(t)),
            Special::ProvideFile(t) => ("provideFile", Some(t)),
            Special::ProvideSpecial(t) => ("provideSpecial", Some(t)),
            Special::SetOutputPath(t) => ("setOutputPath", Some(t)),
            Special::SetTemplate(t) => ("setTemplate", Some(t)),
            Special::SetTemplateVariable(t) => ("setTemplateVariable", Some(t)),
            Special::StartDefineFontFamily => ("startDefineFontFamily", None),
            Special::StartFontFamilyTagAssociations => ("startFontFamilyTagAssociations", None),
        };

        if let Some(t) = rest {
            write!(f, "tdux:{cmd} {t}")
        } else {
            write!(f, "tdux:{cmd}")
        }
    }
}
