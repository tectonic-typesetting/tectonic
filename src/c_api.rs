// src/c_api.rs -- exposing the C/C++ API
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use libc;

use file_format::FileFormat;


// The API of the C/C++ library that we interact with to drive the engine
// under the hood.

extern {
    pub fn tt_get_error_message() -> *const i8;
    pub fn tt_misc_initialize(dump_name: *const i8) -> ();
    pub fn tt_set_int_variable(var_name: *const u8, value: libc::c_int) -> libc::c_int;
    //pub fn tt_set_string_variable(var_name: *const u8, value: *const i8) -> libc::c_int;
    pub fn tt_run_engine(input_file_name: *const i8) -> libc::c_int;

    pub fn dvipdfmx_simple_main(dviname: *const i8, pdfname: *const i8) -> libc::c_int;
}


pub fn c_format_to_rust (format: libc::c_int) -> Option<FileFormat> {
    // See the kpse_file_format_type enum.
    match format {
        1 => Some(FileFormat::Pk),
        3 => Some(FileFormat::TFM),
        4 => Some(FileFormat::AFM),
        10 => Some(FileFormat::Format),
        11 => Some(FileFormat::FontMap),
        20 => Some(FileFormat::Ofm),
        25 => Some(FileFormat::Pict),
        26 => Some(FileFormat::Tex),
        33 => Some(FileFormat::Vf),
        36 => Some(FileFormat::TrueType),
        41 => Some(FileFormat::MiscFonts),
        44 => Some(FileFormat::Enc),
        45 => Some(FileFormat::Cmap),
        46 => Some(FileFormat::Sfd),
        _ => None
    }
}
