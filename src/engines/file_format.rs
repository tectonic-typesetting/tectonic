// src/engines/file_format.rs -- different file formats that the C code cares about
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.


#[derive(Clone,Copy,Debug)]
pub enum FileFormat {
    AFM,
    Cmap,
    Enc,
    Format,
    FontMap,
    MiscFonts,
    Ofm,
    OpenType,
    Ovf,
    Pict,
    Pk,
    ProgramData,
    Sfd,
    Tex,
    TexPsHeader,
    TFM,
    TrueType,
    Type1,
    Vf,
}

pub fn format_to_extension (format: FileFormat) -> &'static str {
    match format {
        FileFormat::AFM => ".afm",
        FileFormat::Cmap => ".cmap", /* XXX: kpathsea doesn't define any suffixes for this */
        FileFormat::Enc => ".enc",
        FileFormat::Format => ".fmt",
        FileFormat::FontMap => ".map",
        FileFormat::MiscFonts => ".miscfonts", /* XXX: no kpathsea suffixes */
        FileFormat::Ofm => ".ofm", /* XXX: also .tfm */
        FileFormat::OpenType => ".otf", /* XXX: also OTF */
        FileFormat::Ovf => ".ovf", /* XXX: also .vf */
        FileFormat::Pict => ".pdf", /* XXX: also .eps, .epsi, ... */
        FileFormat::Pk => ".pk",
        FileFormat::ProgramData => ".programdata", /* XXX no suffixes */
        FileFormat::Sfd => ".sfd",
        FileFormat::Tex => ".tex", /* also .{sty,cls,fd,aux,bbl,def,clo,ldf} */
        FileFormat::TexPsHeader => ".pro",
        FileFormat::TFM => ".tfm",
        FileFormat::TrueType => ".ttf", /* XXX: also .ttc, .TTF, .TTC, .dfont */
        FileFormat::Type1 => ".pfa", /* XXX: also .pfb */
        FileFormat::Vf => ".vf",
    }
}
