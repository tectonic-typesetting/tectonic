// Copyright 2021-2022 the Tectonic Project
// Licensed under the MIT License.

//! Helpers for working with HTML5 tags.

use std::str::FromStr;

macro_rules! emit_tag_data {
    ($([$vname:ident $tagname:literal deprecated($deprecated:literal), autoclosed($($acvname:ident),*)],)+) => {
        pub enum Tag {
            $($vname,)+
            Other(String),
        }

        impl FromStr for Tag {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $($tagname => Tag::$vname,)+
                    other => Tag::Other(other.to_owned())
                })
            }
        }

        impl Tag {
            pub fn is_deprecated(&self) -> bool {
                match self {
                    $(Tag::$vname => $deprecated,)+
                    Tag::Other(_) => false,
                }
            }

            pub fn is_other(&self) -> bool {
                matches!(self, Tag::Other(_))
            }

            pub fn is_autoclosed_by(&self, other: &Tag) -> bool {
                match self {
                    $(
                        Tag::$vname => {
                            match other {
                                $(Tag::$acvname => true,)*
                                _ => false
                            }
                        },
                    )+
                    Tag::Other(_) => false,
                }
            }
        }
    }
}

emit_tag_data! {
    [A "a" deprecated(false), autoclosed()],
    [Abbr "abbr" deprecated(false), autoclosed()],
    [Acronym "acroynm" deprecated(true), autoclosed()],
    [Address "address" deprecated(false), autoclosed()],
    [Applet "applet" deprecated(true), autoclosed()],
    [Area "area" deprecated(false), autoclosed()],
    [Article "article" deprecated(false), autoclosed()],
    [Aside "aside" deprecated(false), autoclosed()],
    [Audio "audio" deprecated(false), autoclosed()],
    [B "b" deprecated(false), autoclosed()],
    [Base "base" deprecated(false), autoclosed()],
    [Bdi "bdi" deprecated(false), autoclosed()],
    [Bdo "bdo" deprecated(false), autoclosed()],
    [Bgsound "bgsound" deprecated(true), autoclosed()],
    [Big "big" deprecated(true), autoclosed()],
    [Blink "blink" deprecated(true), autoclosed()],
    [Blockquote "blockquote" deprecated(false), autoclosed()],
    [Body "body" deprecated(false), autoclosed()],
    [Br "br" deprecated(false), autoclosed()],
    [Button "button" deprecated(false), autoclosed()],
    [Canvas "canvas" deprecated(false), autoclosed()],
    [Caption "caption" deprecated(false), autoclosed()],
    [Center "center" deprecated(true), autoclosed()],
    [Cite "cite" deprecated(false), autoclosed()],
    [Code "code" deprecated(false), autoclosed()],
    [Col "col" deprecated(false), autoclosed()],
    [Colgroup "colgroup" deprecated(false), autoclosed()],
    [Content "content" deprecated(true), autoclosed()],
    [Data "data" deprecated(false), autoclosed()],
    [Datalist "datalist" deprecated(false), autoclosed()],
    [Dd "dd" deprecated(false), autoclosed()],
    [Del "del" deprecated(false), autoclosed()],
    [Details "details" deprecated(false), autoclosed()],
    [Dfn "dfn" deprecated(false), autoclosed()],
    [Dialog "dialog" deprecated(false), autoclosed()],
    [Dir "dir" deprecated(true), autoclosed()],
    [Div "div" deprecated(false), autoclosed()],
    [Dl "dl" deprecated(false), autoclosed()],
    [Dt "dt" deprecated(false), autoclosed()],
    [Em "em" deprecated(false), autoclosed()],
    [Embed "embed" deprecated(false), autoclosed()],
    [Fieldset "fieldset" deprecated(false), autoclosed()],
    [Figcaption "figcaption" deprecated(false), autoclosed()],
    [Figure "figure" deprecated(false), autoclosed()],
    [Font "font" deprecated(true), autoclosed()],
    [Footer "footer" deprecated(false), autoclosed()],
    [Form "form" deprecated(false), autoclosed()],
    [Frame "frame" deprecated(true), autoclosed()],
    [Frameset "frameset" deprecated(true), autoclosed()],
    [H1 "h1" deprecated(false), autoclosed()],
    [H2 "h2" deprecated(false), autoclosed()],
    [H3 "h3" deprecated(false), autoclosed()],
    [H4 "h4" deprecated(false), autoclosed()],
    [H5 "h5" deprecated(false), autoclosed()],
    [H6 "h6" deprecated(false), autoclosed()],
    [Head "head" deprecated(false), autoclosed()],
    [Header "header" deprecated(false), autoclosed()],
    [Hgroup "hgroup" deprecated(true), autoclosed()],
    [Hr "hr" deprecated(false), autoclosed()],
    [Html "html" deprecated(false), autoclosed()],
    [I "i" deprecated(false), autoclosed()],
    [Iframe "iframe" deprecated(false), autoclosed()],
    [Image "image" deprecated(true), autoclosed()],
    [Img "img" deprecated(false), autoclosed()],
    [Input "input" deprecated(false), autoclosed()],
    [Ins "ins" deprecated(false), autoclosed()],
    [Kbd "kbd" deprecated(false), autoclosed()],
    [Keygen "keygen" deprecated(true), autoclosed()],
    [Label "label" deprecated(false), autoclosed()],
    [Legend "legend" deprecated(false), autoclosed()],
    [Li "li" deprecated(false), autoclosed()],
    [Link "link" deprecated(false), autoclosed()],
    [Main "main" deprecated(false), autoclosed()],
    [Map "map" deprecated(false), autoclosed()],
    [Mark "mark" deprecated(false), autoclosed()],
    [Marquee "marquee" deprecated(true), autoclosed()],
    [Math "math" deprecated(false), autoclosed()],
    [Menu "menu" deprecated(false), autoclosed()],
    [Menuitem "menuitem" deprecated(true), autoclosed()],
    [Meta "meta" deprecated(false), autoclosed()],
    [Meter "meter" deprecated(false), autoclosed()],
    [Nav "nav" deprecated(false), autoclosed()],
    [Nobr "nobr" deprecated(true), autoclosed()],
    [Noembed "noembed" deprecated(true), autoclosed()],
    [Noframes "noframes" deprecated(true), autoclosed()],
    [Noscript "noscript" deprecated(false), autoclosed()],
    [Object "object" deprecated(false), autoclosed()],
    [Ol "ol" deprecated(false), autoclosed()],
    [Optgroup "optgroup" deprecated(false), autoclosed()],
    [Option "option" deprecated(false), autoclosed()],
    [Output "output" deprecated(false), autoclosed()],
    [P "p" deprecated(false), autoclosed(
        Address,
        Article,
        Aside,
        Blockquote,
        Div,
        Dl,
        Fieldset,
        Footer,
        Form,
        H1,
        H2,
        H3,
        H4,
        H5,
        H6,
        Header,
        Hr,
        Menu,
        Nav,
        Ol,
        Pre,
        Section,
        Table,
        Ul,
        P
    )],
    [Param "param" deprecated(true), autoclosed()],
    [Picture "picture" deprecated(false), autoclosed()],
    [Plaintext "plaintext" deprecated(true), autoclosed()],
    [Portal "portal" deprecated(false), autoclosed()],
    [Pre "pre" deprecated(false), autoclosed()],
    [Progress "progress" deprecated(false), autoclosed()],
    [Q "q" deprecated(false), autoclosed()],
    [Rb "rb" deprecated(true), autoclosed()],
    [Rp "rp" deprecated(false), autoclosed()],
    [Rt "rt" deprecated(false), autoclosed()],
    [Rtc "rtc" deprecated(true), autoclosed()],
    [Ruby "ruby" deprecated(false), autoclosed()],
    [S "s" deprecated(false), autoclosed()],
    [Samp "samp" deprecated(false), autoclosed()],
    [Script "script" deprecated(false), autoclosed()],
    [Section "section" deprecated(false), autoclosed()],
    [Select "select" deprecated(false), autoclosed()],
    [Shadow "shadow" deprecated(true), autoclosed()],
    [Slot "slot" deprecated(false), autoclosed()],
    [Small "small" deprecated(false), autoclosed()],
    [Spacer "spacer" deprecated(true), autoclosed()],
    [Source "source" deprecated(false), autoclosed()],
    [Span "span" deprecated(false), autoclosed()],
    [Strike "strike" deprecated(true), autoclosed()],
    [Strong "strong" deprecated(false), autoclosed()],
    [Style "style" deprecated(false), autoclosed()],
    [Sub "sub" deprecated(false), autoclosed()],
    [Summary "summary" deprecated(false), autoclosed()],
    [Sup "sup" deprecated(false), autoclosed()],
    [Svg "svg" deprecated(false), autoclosed()],
    [Table "table" deprecated(false), autoclosed()],
    [Tbody "tbody" deprecated(false), autoclosed()],
    [Td "td" deprecated(false), autoclosed()],
    [Template "template" deprecated(false), autoclosed()],
    [Textarea "textarea" deprecated(false), autoclosed()],
    [Tfoot "tfoot" deprecated(false), autoclosed()],
    [Th "th" deprecated(false), autoclosed()],
    [Thead "thead" deprecated(false), autoclosed()],
    [Time "time" deprecated(false), autoclosed()],
    [Title "title" deprecated(false), autoclosed()],
    [Tr "tr" deprecated(false), autoclosed()],
    [Track "track" deprecated(false), autoclosed()],
    [Tt "tt" deprecated(true), autoclosed()],
    [U "u" deprecated(false), autoclosed()],
    [Ul "ul" deprecated(false), autoclosed()],
    [Var "var" deprecated(false), autoclosed()],
    [Video "video" deprecated(false), autoclosed()],
    [Wbr "wbr" deprecated(false), autoclosed()],
    [Xmp "xmp" deprecated(true), autoclosed()],
}
