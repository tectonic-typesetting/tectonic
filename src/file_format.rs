// src/file_format.rs -- different file formats that the C code cares about
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.


#[derive(Clone,Copy,Debug)]
pub enum FileFormat {
    TFM,
    Pict,
    Tex,
    Format,
}

pub fn format_to_extension (format: FileFormat) -> &'static str {
    match format {
        FileFormat::TFM => ".tfm",
        FileFormat::Pict => ".pdf", /* XXX */
        FileFormat::Tex => ".tex",
        FileFormat::Format => ".fmt",
    }
}
