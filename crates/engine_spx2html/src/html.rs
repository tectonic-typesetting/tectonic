// Copyright 2021-2022 the Tectonic Project
// Licensed under the MIT License.

//! Helpers for working with HTML5 elements.

use std::str::FromStr;

macro_rules! emit_element_data {
    ($([
        $vname:ident
        $tagname:literal
        deprecated($deprecated:literal)
        empty($empty:literal)
        autoclosed($($acvname:ident),*)
    ],)+) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub enum Element {
            $($vname,)+
            Other(String),
        }

        impl FromStr for Element {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $($tagname => Element::$vname,)+
                    other => Element::Other(other.to_owned())
                })
            }
        }

        impl Element {
            pub fn name(&self) -> &str {
                match self {
                    $(Element::$vname => $tagname,)+
                    Element::Other(ref name) => name,
                }
            }

            pub fn is_deprecated(&self) -> bool {
                match self {
                    $(Element::$vname => $deprecated,)+
                    Element::Other(_) => false,
                }
            }

            pub fn is_empty(&self) -> bool {
                match self {
                    $(Element::$vname => $empty,)+
                    Element::Other(_) => false,
                }
            }

            pub fn is_autoclosed_by(&self, other: &Element) -> bool {
                match self {
                    $(
                        Element::$vname => {
                            match other {
                                $(Element::$acvname => true,)*
                                _ => false
                            }
                        },
                    )+
                    Element::Other(_) => false,
                }
            }
        }
    }
}

emit_element_data! {
    [A "a" deprecated(false) empty(false) autoclosed()],
    [Abbr "abbr" deprecated(false) empty(false) autoclosed()],
    [Acronym "acroynm" deprecated(true) empty(false) autoclosed()],
    [Address "address" deprecated(false) empty(false) autoclosed()],
    [Applet "applet" deprecated(true) empty(false) autoclosed()],
    [Area "area" deprecated(false) empty(true) autoclosed()],
    [Article "article" deprecated(false) empty(false) autoclosed()],
    [Aside "aside" deprecated(false) empty(false) autoclosed()],
    [Audio "audio" deprecated(false) empty(false) autoclosed()],
    [B "b" deprecated(false) empty(false) autoclosed()],
    [Base "base" deprecated(false) empty(true) autoclosed()],
    [Bdi "bdi" deprecated(false) empty(false) autoclosed()],
    [Bdo "bdo" deprecated(false) empty(false) autoclosed()],
    [Bgsound "bgsound" deprecated(true) empty(false) autoclosed()],
    [Big "big" deprecated(true) empty(false) autoclosed()],
    [Blink "blink" deprecated(true) empty(false) autoclosed()],
    [Blockquote "blockquote" deprecated(false) empty(false) autoclosed()],
    [Body "body" deprecated(false) empty(false) autoclosed()],
    [Br "br" deprecated(false) empty(true) autoclosed()],
    [Button "button" deprecated(false) empty(false) autoclosed()],
    [Canvas "canvas" deprecated(false) empty(false) autoclosed()],
    [Caption "caption" deprecated(false) empty(false) autoclosed()],
    [Center "center" deprecated(true) empty(false) autoclosed()],
    [Cite "cite" deprecated(false) empty(false) autoclosed()],
    [Code "code" deprecated(false) empty(false) autoclosed()],
    [Col "col" deprecated(false) empty(true) autoclosed()],
    [Colgroup "colgroup" deprecated(false) empty(false) autoclosed()],
    [Content "content" deprecated(true) empty(false) autoclosed()],
    [Data "data" deprecated(false) empty(false) autoclosed()],
    [Datalist "datalist" deprecated(false) empty(false) autoclosed()],
    [Dd "dd" deprecated(false) empty(false) autoclosed(Dd, Dt)],
    [Del "del" deprecated(false) empty(false) autoclosed()],
    [Details "details" deprecated(false) empty(false) autoclosed()],
    [Dfn "dfn" deprecated(false) empty(false) autoclosed()],
    [Dialog "dialog" deprecated(false) empty(false) autoclosed()],
    [Dir "dir" deprecated(true) empty(false) autoclosed()],
    [Div "div" deprecated(false) empty(false) autoclosed()],
    [Dl "dl" deprecated(false) empty(false) autoclosed()],
    [Dt "dt" deprecated(false) empty(false) autoclosed(Dd, Dt)],
    [Em "em" deprecated(false) empty(false) autoclosed()],
    [Embed "embed" deprecated(false) empty(true) autoclosed()],
    [Fieldset "fieldset" deprecated(false) empty(false) autoclosed()],
    [Figcaption "figcaption" deprecated(false) empty(false) autoclosed()],
    [Figure "figure" deprecated(false) empty(false) autoclosed()],
    [Font "font" deprecated(true) empty(false) autoclosed()],
    [Footer "footer" deprecated(false) empty(false) autoclosed()],
    [Form "form" deprecated(false) empty(false) autoclosed()],
    [Frame "frame" deprecated(true) empty(false) autoclosed()],
    [Frameset "frameset" deprecated(true) empty(false) autoclosed()],
    [H1 "h1" deprecated(false) empty(false) autoclosed()],
    [H2 "h2" deprecated(false) empty(false) autoclosed()],
    [H3 "h3" deprecated(false) empty(false) autoclosed()],
    [H4 "h4" deprecated(false) empty(false) autoclosed()],
    [H5 "h5" deprecated(false) empty(false) autoclosed()],
    [H6 "h6" deprecated(false) empty(false) autoclosed()],
    [Head "head" deprecated(false) empty(false) autoclosed()],
    [Header "header" deprecated(false) empty(false) autoclosed()],
    [Hgroup "hgroup" deprecated(true) empty(false) autoclosed()],
    [Hr "hr" deprecated(false) empty(true) autoclosed()],
    [Html "html" deprecated(false) empty(false) autoclosed()],
    [I "i" deprecated(false) empty(false) autoclosed()],
    [Iframe "iframe" deprecated(false) empty(false) autoclosed()],
    [Image "image" deprecated(true) empty(false) autoclosed()],
    [Img "img" deprecated(false) empty(true) autoclosed()],
    [Input "input" deprecated(false) empty(true) autoclosed()],
    [Ins "ins" deprecated(false) empty(false) autoclosed()],
    [Kbd "kbd" deprecated(false) empty(false) autoclosed()],
    [Keygen "keygen" deprecated(true) empty(true) autoclosed()],
    [Label "label" deprecated(false) empty(false) autoclosed()],
    [Legend "legend" deprecated(false) empty(false) autoclosed()],
    [Li "li" deprecated(false) empty(false) autoclosed(Li)],
    [Link "link" deprecated(false) empty(true) autoclosed()],
    [Main "main" deprecated(false) empty(false) autoclosed()],
    [Map "map" deprecated(false) empty(false) autoclosed()],
    [Mark "mark" deprecated(false) empty(false) autoclosed()],
    [Marquee "marquee" deprecated(true) empty(false) autoclosed()],
    [Math "math" deprecated(false) empty(false) autoclosed()],
    [Menu "menu" deprecated(false) empty(false) autoclosed()],
    [Menuitem "menuitem" deprecated(true) empty(false) autoclosed()],
    [Meta "meta" deprecated(false) empty(true) autoclosed()],
    [Meter "meter" deprecated(false) empty(false) autoclosed()],
    [Nav "nav" deprecated(false) empty(false) autoclosed()],
    [Nobr "nobr" deprecated(true) empty(false) autoclosed()],
    [Noembed "noembed" deprecated(true) empty(false) autoclosed()],
    [Noframes "noframes" deprecated(true) empty(false) autoclosed()],
    [Noscript "noscript" deprecated(false) empty(false) autoclosed()],
    [Object "object" deprecated(false) empty(false) autoclosed()],
    [Ol "ol" deprecated(false) empty(false) autoclosed()],
    [Optgroup "optgroup" deprecated(false) empty(false) autoclosed(Optgroup)],
    [Option "option" deprecated(false) empty(false) autoclosed()],
    [Output "output" deprecated(false) empty(false) autoclosed()],
    [P "p" deprecated(false) empty(false) autoclosed(
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
    [Param "param" deprecated(true) empty(true) autoclosed()],
    [Picture "picture" deprecated(false) empty(false) autoclosed()],
    [Plaintext "plaintext" deprecated(true) empty(false) autoclosed()],
    [Portal "portal" deprecated(false) empty(false) autoclosed()],
    [Pre "pre" deprecated(false) empty(false) autoclosed()],
    [Progress "progress" deprecated(false) empty(false) autoclosed()],
    [Q "q" deprecated(false) empty(false) autoclosed()],
    [Rb "rb" deprecated(true) empty(false) autoclosed()],
    [Rp "rp" deprecated(false) empty(false) autoclosed()],
    [Rt "rt" deprecated(false) empty(false) autoclosed()],
    [Rtc "rtc" deprecated(true) empty(false) autoclosed()],
    [Ruby "ruby" deprecated(false) empty(false) autoclosed()],
    [S "s" deprecated(false) empty(false) autoclosed()],
    [Samp "samp" deprecated(false) empty(false) autoclosed()],
    [Script "script" deprecated(false) empty(false) autoclosed()],
    [Section "section" deprecated(false) empty(false) autoclosed()],
    [Select "select" deprecated(false) empty(false) autoclosed()],
    [Shadow "shadow" deprecated(true) empty(false) autoclosed()],
    [Slot "slot" deprecated(false) empty(false) autoclosed()],
    [Small "small" deprecated(false) empty(false) autoclosed()],
    [Spacer "spacer" deprecated(true) empty(false) autoclosed()],
    [Source "source" deprecated(false) empty(true) autoclosed()],
    [Span "span" deprecated(false) empty(false) autoclosed()],
    [Strike "strike" deprecated(true) empty(false) autoclosed()],
    [Strong "strong" deprecated(false) empty(false) autoclosed()],
    [Style "style" deprecated(false) empty(false) autoclosed()],
    [Sub "sub" deprecated(false) empty(false) autoclosed()],
    [Summary "summary" deprecated(false) empty(false) autoclosed()],
    [Sup "sup" deprecated(false) empty(false) autoclosed()],
    [Svg "svg" deprecated(false) empty(false) autoclosed()],
    [Table "table" deprecated(false) empty(false) autoclosed()],
    [Tbody "tbody" deprecated(false) empty(false) autoclosed()],
    [Td "td" deprecated(false) empty(false) autoclosed(Td, Th)],
    [Template "template" deprecated(false) empty(false) autoclosed()],
    [Textarea "textarea" deprecated(false) empty(false) autoclosed()],
    [Tfoot "tfoot" deprecated(false) empty(false) autoclosed()],
    [Th "th" deprecated(false) empty(false) autoclosed(Td, Th)],
    [Thead "thead" deprecated(false) empty(false) autoclosed(Tbody, Tfoot)],
    [Time "time" deprecated(false) empty(false) autoclosed()],
    [Title "title" deprecated(false) empty(false) autoclosed()],
    [Tr "tr" deprecated(false) empty(false) autoclosed(Tr)],
    [Track "track" deprecated(false) empty(true) autoclosed()],
    [Tt "tt" deprecated(true) empty(false) autoclosed()],
    [U "u" deprecated(false) empty(false) autoclosed()],
    [Ul "ul" deprecated(false) empty(false) autoclosed()],
    [Var "var" deprecated(false) empty(false) autoclosed()],
    [Video "video" deprecated(false) empty(false) autoclosed()],
    [Wbr "wbr" deprecated(false) empty(true) autoclosed()],
    [Xmp "xmp" deprecated(true) empty(false) autoclosed()],
}
