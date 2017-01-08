// src/file_format.rs -- different file formats that the C code cares about
// Copyright 2016 the Tectonic Project
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
    Pict,
    Pk,
    Sfd,
    Tex,
    TFM,
    TrueType,
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
        FileFormat::Pict => ".pdf", /* XXX: also .eps, .epsi, ... */
        FileFormat::Pk => ".pk",
        FileFormat::Sfd => ".sfd",
        FileFormat::Tex => ".tex", /* also .{sty,cls,fd,aux,bbl,def,clo,ldf} */
        FileFormat::TFM => ".tfm",
        FileFormat::TrueType => ".ttf", /* XXX: also .ttc, .TTF, .TTC, .dfont */
        FileFormat::Vf => ".vf",
    }
}
