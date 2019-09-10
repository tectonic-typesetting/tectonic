#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate c2rust_bitfields;
extern crate libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* Global symbols that route through the global API variable. Hopefully we
     * will one day eliminate all of the global state and get rid of all of
     * these. */
    #[no_mangle]
    fn ttstub_issue_warning(format: *const i8, _: ...);
    #[no_mangle]
    fn ttstub_issue_error(format: *const i8, _: ...);
    #[no_mangle]
    fn ttstub_fprintf(handle: rust_output_handle_t, format: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn ttstub_output_open(path: *const i8, is_gz: i32) -> rust_output_handle_t;
    #[no_mangle]
    fn ttstub_output_close(handle: rust_output_handle_t) -> i32;
    /* tectonic/core-memory.h: basic dynamic memory helpers
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    #[no_mangle]
    fn xstrdup(s: *const i8) -> *mut i8;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(old_address: *mut libc::c_void, new_size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn gettexstring(_: str_number) -> *mut i8;
    /* Needed here for UFILE */
    /* variables! */
    /* All the following variables are defined in xetexini.c */
    #[no_mangle]
    static mut eqtb: *mut memory_word;
    #[no_mangle]
    static mut mem: *mut memory_word;
    #[no_mangle]
    static mut cur_input: input_state_t;
    #[no_mangle]
    static mut job_name: str_number;
    #[no_mangle]
    static mut total_pages: i32;
    #[no_mangle]
    static mut rule_ht: scaled_t;
    #[no_mangle]
    static mut rule_dp: scaled_t;
    #[no_mangle]
    static mut rule_wd: scaled_t;
    #[no_mangle]
    static mut cur_h: scaled_t;
    #[no_mangle]
    static mut cur_v: scaled_t;
    #[no_mangle]
    static mut synctex_enabled: i32;
    #[no_mangle]
    static mut name_of_input_file: *mut i8;
}
pub type size_t = u64;
pub type rust_output_handle_t = *mut libc::c_void;
pub type scaled_t = i32;
pub type str_number = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2_le_t {
    pub s0: i32,
    pub s1: i32,
}
/* The annoying `memory_word` type. We have to make sure the byte-swapping
 * that the (un)dumping routines do suffices to put things in the right place
 * in memory.
 *
 * This set of data used to be a huge mess (see comment after the
 * definitions). It is now (IMO) a lot more reasonable, but there will no
 * doubt be carryover weird terminology around the code.
 *
 * ## ENDIANNESS (cheat sheet because I'm lame)
 *
 * Intel is little-endian. Say that we have a 32-bit integer stored in memory
 * with `p` being a `uint8` pointer to its location. In little-endian land,
 * `p[0]` is least significant byte and `p[3]` is its most significant byte.
 *
 * Conversely, in big-endian land, `p[0]` is its most significant byte and
 * `p[3]` is its least significant byte.
 *
 * ## MEMORY_WORD LAYOUT
 *
 * Little endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [lsb......s0.......msb] [lsb......s1.......msb]
 *   b16:   [l..s0...m] [l..s1...m] [l..s2...m] [l..s3...m]
 *
 * Big endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [msb......s1.......lsb] [msb......s0.......lsb]
 *   b16:   [m..s3...l] [m..s2...l] [m..s1...l] [m...s0..l]
 *
 */
pub type b32x2 = b32x2_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4_le_t {
    pub s0: u16,
    pub s1: u16,
    pub s2: u16,
    pub s3: u16,
}
pub type b16x4 = b16x4_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: f64,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_state_t {
    pub state: u16,
    pub index: u16,
    pub start: i32,
    pub loc: i32,
    pub limit: i32,
    pub name: i32,
    pub synctex_tag: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _flags {
    #[bitfield(name = "content_ready", ty = "u32", bits = "0..=0")]
    #[bitfield(name = "off", ty = "u32", bits = "1..=1")]
    #[bitfield(name = "not_void", ty = "u32", bits = "2..=2")]
    #[bitfield(name = "warn", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "output_p", ty = "u32", bits = "4..=4")]
    pub content_ready_off_not_void_warn_output_p: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
/* recorders know how to record a node */
/*  Here are all the local variables gathered in one "synchronization context"  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub file: rust_output_handle_t,
    pub root_name: *mut i8,
    pub count: i32,
    pub node: i32,
    pub recorder: synctex_recorder_t,
    pub tag: i32,
    pub line: i32,
    pub curh: i32,
    pub curv: i32,
    pub magnification: i32,
    pub unit: i32,
    pub total_length: i32,
    pub lastv: i32,
    pub form_depth: i32,
    pub synctex_tag_counter: u32,
    pub flags: _flags,
}
/*  For non-GCC compilation.  */
/*  UNIT is the scale. TeX coordinates are very accurate and client won't need
 *  that, at leat in a first step.  1.0 <-> 2^16 = 65536.
 *  The TeX unit is sp (scaled point) or pt/65536 which means that the scale
 *  factor to retrieve a bp unit (a postscript) is 72/72.27/65536 =
 *  1/4096/16.06 = 1/8192/8.03
 *  Here we use 1/SYNCTEX_UNIT_FACTOR as scale factor, then we can limit ourselves to
 *  integers. This default value assumes that TeX magnification factor is 1000.
 *  The real TeX magnification factor is used to fine tune the synctex context
 *  scale in the synctex_dot_open function.
 *  IMPORTANT: We can say that the natural unit of .synctex files is SYNCTEX_UNIT_FACTOR sp.
 *  To retrieve the proper bp unit, we'll have to divide by 8.03.  To reduce
 *  rounding errors, we'll certainly have to add 0.5 for non negative integers
 *  and +/-0.5 for negative integers.  This trick is mainly to gain speed and
 *  size. A binary file would be more appropriate in that respect, but I guess
 *  that some clients like auctex would not like it very much.  we cannot use
 *  "<<13" instead of "/SYNCTEX_UNIT_FACTOR" because the integers are signed and we do not
 *  want the sign bit to be propagated.  The origin of the coordinates is at
 *  the top left corner of the page.  For pdf mode, it is straightforward, but
 *  for dvi mode, we'll have to record the 1in offset in both directions,
 *  eventually modified by the magnification.
 */
pub type synctex_recorder_t = Option<unsafe extern "C" fn(_: i32) -> ()>;
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
// Initialized in run_static_initializers
static mut synctex_ctxt: C2RustUnnamed = C2RustUnnamed {
    file: 0 as *const libc::c_void as *mut libc::c_void,
    root_name: 0 as *const i8 as *mut i8,
    count: 0,
    node: 0,
    recorder: None,
    tag: 0,
    line: 0,
    curh: 0,
    curv: 0,
    magnification: 0,
    unit: 0,
    total_length: 0,
    lastv: 0,
    form_depth: 0,
    synctex_tag_counter: 0,
    flags: _flags {
        content_ready_off_not_void_warn_output_p: [0; 1],
        c2rust_padding: [0; 3],
    },
};
unsafe extern "C" fn get_current_name() -> *mut i8 {
    /* This used to always make the pathname absolute but I'm getting rid of
     * that since it ends up adding dependencies on a bunch of functions I
     * don't want to have to deal with. */
    if name_of_input_file.is_null() {
        return xstrdup(b"\x00" as *const u8 as *const i8);
    }
    return xstrdup(name_of_input_file);
}
/* synctex.h

Copyright (c) 2008, 2009 jerome DOT laurens AT u-bourgogne DOT fr

This file is part of the SyncTeX package.

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE

Acknowledgments:
----------------
The author received useful remarks from the pdfTeX developers, especially Hahn The Thanh,
and significant help from XeTeX developer Jonathan Kew

Nota Bene:
----------
If you include or use a significant part of the synctex package into a software,
I would appreciate to be listed as contributor and see "SyncTeX" highlighted.

Version 1
Latest Revision: Wed Jul  1 08:17:50 UTC 2009

*/
/*  Send this message to init the synctex command value to the command line option.
 *  Sending this message too early will cause a bus error.  */
#[no_mangle]
pub unsafe extern "C" fn synctex_init_command() {
    /* In the web2c implementations this dealt with the -synctex command line
     * argument. */
    /* Reset state */
    synctex_ctxt.file = 0 as *mut libc::c_void;
    synctex_ctxt.root_name = 0 as *mut i8;
    synctex_ctxt.count = 0i32;
    synctex_ctxt.node = 0i32;
    synctex_ctxt.recorder = None;
    synctex_ctxt.tag = 0i32;
    synctex_ctxt.line = 0i32;
    synctex_ctxt.curh = 0i32;
    synctex_ctxt.curv = 0i32;
    synctex_ctxt.magnification = 0i32;
    synctex_ctxt.unit = 0i32;
    synctex_ctxt.total_length = 0i32;
    synctex_ctxt.lastv = -1i32;
    synctex_ctxt.form_depth = 0i32;
    synctex_ctxt.synctex_tag_counter = 0i32 as u32;
    synctex_ctxt.flags.set_content_ready(0i32 as u32);
    synctex_ctxt.flags.set_off(0i32 as u32);
    synctex_ctxt.flags.set_not_void(0i32 as u32);
    synctex_ctxt.flags.set_warn(0i32 as u32);
    synctex_ctxt.flags.set_output_p(0i32 as u32);
    if synctex_enabled != 0 {
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 = 1i32
    } else {
        (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 = 0i32
        /* \synctex=0 : don't record stuff */
    };
}
/*  Free all memory used, close the file if any,
 *  It is sent locally when there is a problem with synctex output.
 *  It is sent by pdftex when a fatal error occurred in pdftex.web. */
unsafe extern "C" fn synctexabort() {
    if !synctex_ctxt.file.is_null() {
        ttstub_output_close(synctex_ctxt.file);
        synctex_ctxt.file = 0 as *mut libc::c_void
    }
    synctex_ctxt.root_name = mfree(synctex_ctxt.root_name as *mut libc::c_void) as *mut i8;
    synctex_ctxt.flags.set_off(1i32 as u32);
    /* disable synctex */
}
static mut synctex_suffix: *const i8 = b".synctex\x00" as *const u8 as *const i8;
static mut synctex_suffix_gz: *const i8 = b".gz\x00" as *const u8 as *const i8;
/*  synctex_dot_open ensures that the foo.synctex file is open.
 *  In case of problem, it definitely disables synchronization.
 *  Now all the output synchronization info is gathered in only one file.
 *  It is possible to split this info into as many different output files as sheets
 *  plus 1 for the control but the overall benefits are not so clear.
 *  For example foo-i.synctex would contain input synchronization
 *  information for page i alone.
 */
unsafe extern "C" fn synctex_dot_open() -> rust_output_handle_t {
    let mut tmp: *mut i8 = 0 as *mut i8;
    let mut the_name: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
    {
        return 0 as *mut libc::c_void;
    }
    if !synctex_ctxt.file.is_null() {
        return synctex_ctxt.file;
    }
    tmp = gettexstring(job_name);
    len = strlen(tmp);
    if !(len <= 0i32 as u64) {
        the_name = xmalloc(
            len.wrapping_add(strlen(synctex_suffix))
                .wrapping_add(strlen(synctex_suffix_gz))
                .wrapping_add(1i32 as u64),
        ) as *mut i8;
        strcpy(the_name, tmp);
        strcat(the_name, synctex_suffix);
        strcat(the_name, synctex_suffix_gz);
        tmp = mfree(tmp as *mut libc::c_void) as *mut i8;
        synctex_ctxt.file = ttstub_output_open(the_name, 1i32);
        if !synctex_ctxt.file.is_null() {
            if !(synctex_record_preamble() != 0) {
                synctex_ctxt.magnification = 1000i32;
                synctex_ctxt.unit = 1i32;
                the_name = mfree(the_name as *mut libc::c_void) as *mut i8;
                if !synctex_ctxt.root_name.is_null() {
                    synctex_record_input(1i32, synctex_ctxt.root_name);
                    synctex_ctxt.root_name =
                        mfree(synctex_ctxt.root_name as *mut libc::c_void) as *mut i8
                }
                synctex_ctxt.count = 0i32;
                return synctex_ctxt.file;
            }
        }
    }
    /*printf("\nSyncTeX warning: no synchronization, problem with %s\n", the_name);*/
    free(tmp as *mut libc::c_void);
    free(the_name as *mut libc::c_void);
    synctexabort();
    return 0 as *mut libc::c_void;
}
/* *
 *  synctex_record_settings must be called very late,
 *  only once there is an opportunity to know whether
 *  in pdf or dvi mode.
 */
unsafe extern "C" fn synctex_prepare_content() -> *mut libc::c_void {
    if synctex_ctxt.flags.content_ready() != 0 {
        return synctex_ctxt.file;
    }
    if !synctex_dot_open().is_null()
        && 0i32 == synctex_record_settings()
        && 0i32 == synctex_record_content()
    {
        synctex_ctxt.flags.set_content_ready(1i32 as u32);
        return synctex_ctxt.file;
    }
    synctexabort();
    return 0 as *mut libc::c_void;
}
/*  Send this message when starting a new input.  */
/*  Each time TeX opens a file, it sends a synctexstartinput message and enters
 *  this function.  Here, a new synchronization tag is created and stored in
 *  the synctex_tag of the TeX current input context.  Each synchronized
 *  TeX node will record this tag instead of the file name.  synctexstartinput
 *  writes the mapping synctag <-> file name to the .synctex (or .synctex.gz) file.  A client
 *  will read the .synctex file and retrieve this mapping, it will be able to
 *  open the correct file just knowing its tag.  If the same file is read
 *  multiple times, it might be associated to different tags.  Synchronization
 *  controllers, either in viewers, editors or standalone should be prepared to
 *  handle this situation and take the appropriate action if they want to
 *  optimize memory.  No two different files will have the same positive tag.
 *  It is not advisable to definitely store the file names here.  If the file
 *  names ever have to be stored, it should definitely be done at the TeX level
 *  just like src-specials do, such that other components of the program can use
 *  it.  This function does not make any difference between the files, it
 *  treats the same way .tex, .aux, .sty ... files, even if many of them do not
 *  contain any material meant to be typeset.
 */
#[no_mangle]
pub unsafe extern "C" fn synctex_start_input() {
    if synctex_ctxt.flags.off() != 0 {
        return;
    }
    /*  synctex_tag_counter is a counter uniquely identifying the file actually
     *  open.  Each time tex opens a new file, synctexstartinput will increment this
     *  counter  */
    if !synctex_ctxt.synctex_tag_counter > 0i32 as u32 {
        synctex_ctxt.synctex_tag_counter = synctex_ctxt.synctex_tag_counter.wrapping_add(1)
    } else {
        /*  we have reached the limit, subsequent files will be softly ignored
         *  this makes a lot of files... even in 32 bits
         *  Maybe we will limit this to 16bits and
         *  use the 16 other bits to store the column number */
        synctex_ctxt.synctex_tag_counter = 0i32 as u32;
        /* was this, but this looks like a bug */
        /* cur_input.synctex_tag = 0; */
        return;
    } /*  -> *TeX.web  */
    cur_input.synctex_tag = synctex_ctxt.synctex_tag_counter as i32;
    if synctex_ctxt.synctex_tag_counter == 1i32 as u32 {
        /*  this is the first file TeX ever opens, in general \jobname.tex we
         *  do not know yet if synchronization will ever be enabled so we have
         *  to store the file name, because we will need it later.
         *  This is necessary because \jobname can be different */
        synctex_ctxt.root_name = get_current_name();
        if strlen(synctex_ctxt.root_name) == 0 {
            synctex_ctxt.root_name = xrealloc(
                synctex_ctxt.root_name as *mut libc::c_void,
                strlen(b"texput\x00" as *const u8 as *const i8).wrapping_add(1i32 as u64),
            ) as *mut i8;
            strcpy(
                synctex_ctxt.root_name,
                b"texput\x00" as *const u8 as *const i8,
            );
        }
        return;
    }
    if !synctex_ctxt.file.is_null() || !synctex_dot_open().is_null() {
        let mut tmp: *mut i8 = get_current_name();
        /* Always record the input, even if INTPAR(synctex) is 0 */
        synctex_record_input(cur_input.synctex_tag, tmp);
        free(tmp as *mut libc::c_void);
    };
}
/*  Send this message to clean memory, and close the file.  */
/*  All the synctex... functions below have the smallest set of parameters.  It
 *  appears to be either the address of a node, or nothing at all.  Using mem,
 *  which is the place where all the nodes are stored, one can retrieve every
 *  information about a node.  The other information is obtained through the
 *  global context variable.
 */
/*  Free all memory used and close the file,
 *  sent by close_files_and_terminate in tex.web.
 *  synctexterminate() is called when the TeX run terminates.
 */
#[no_mangle]
pub unsafe extern "C" fn synctex_terminate(mut log_opened: bool) {
    if !synctex_ctxt.file.is_null() {
        /* We keep the file even if no tex output is produced
         * (synctex_ctxt.flags.not_void == 0). I assume that this means that there
         * was an error and tectonic will not save anything anyway. */
        synctex_record_postamble();
        ttstub_output_close(synctex_ctxt.file);
        synctex_ctxt.file = 0 as *mut libc::c_void
    }
    synctexabort();
}
/*  Recording the "{..." line.  In *tex.web, use synctex_sheet(pdf_output) at
 *  the very beginning of the ship_out procedure.
*/
/*  Recording the "{..." line.  In *tex.web, use synctex_sheet(pdf_output) at
 *  the very beginning of the ship_out procedure.
 */
#[no_mangle]
pub unsafe extern "C" fn synctex_sheet(mut mag: i32) {
    if synctex_ctxt.flags.off() != 0 {
        if (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 != 0
            && synctex_ctxt.flags.warn() == 0
        {
            synctex_ctxt.flags.set_warn(1i32 as u32);
            ttstub_issue_warning(
                b"SyncTeX was disabled -- changing the value of \\synctex has no effect\x00"
                    as *const u8 as *const i8,
            );
        }
        return;
    }
    if !synctex_prepare_content().is_null() {
        /*  First possibility: the .synctex file is already open because SyncTeX was activated on the CLI
         *  or it was activated with the \synctex macro and the first page is already shipped out.
         *  Second possibility: tries to open the .synctex, useful if synchronization was enabled
         *  from the source file and not from the CLI. */
        if total_pages == 0i32 {
            /*  Now it is time to properly set up the scale factor. */
            if mag > 0i32 {
                synctex_ctxt.magnification = mag
            }
        }
        synctex_record_sheet(total_pages + 1i32);
    };
}
/*  Recording the "}..." line.  In *tex.web, use synctex_teehs at
 *  the very end of the ship_out procedure.
*/
/*  Recording the "}..." line.  In *tex.web, use synctex_teehs at
 *  the very end of the ship_out procedure.
 */
#[no_mangle]
pub unsafe extern "C" fn synctex_teehs() {
    if synctex_ctxt.flags.off() as i32 != 0 || synctex_ctxt.file.is_null() {
        return;
    } /* not total_pages+1*/
    synctex_record_teehs(total_pages);
}
/*  This message is sent when a vlist will be shipped out, more precisely at
 *  the beginning of the vlist_out procedure in *TeX.web.  It will be balanced
 *  by a synctex_tsilv, sent at the end of the vlist_out procedure.  p is the
 *  address of the vlist We assume that p is really a vlist node! */
/*  When an hlist ships out, it can contain many different kern/glue nodes with
 *  exactly the same sync tag and line.  To reduce the size of the .synctex
 *  file, we only display a kern node sync info when either the sync tag or the
 *  line changes.  Also, we try ro reduce the distance between the chosen nodes
 *  in order to improve accuracy.  It means that we display information for
 *  consecutive nodes, as far as possible.  This tricky part uses a "recorder",
 *  which is the address of the routine that knows how to write the
 *  synchronization info to the .synctex file.  It also uses criteria to detect
 *  a change in the context, this is the macro SYNCTEX_???_CONTEXT_DID_CHANGE. The
 *  SYNCTEX_IGNORE macro is used to detect unproperly initialized nodes.  See
 *  details in the implementation of the functions below.  */
/*  This message is sent when a vlist will be shipped out, more precisely at
 *  the beginning of the vlist_out procedure in *TeX.web.  It will be balanced
 *  by a synctex_tsilv, sent at the end of the vlist_out procedure.  p is the
 *  address of the vlist. We assume that p is really a vlist node! */
#[no_mangle]
pub unsafe extern "C" fn synctex_vlist(mut this_box: i32) {
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return;
    } /*  0 to reset  */
    synctex_ctxt.node = this_box; /*  reset  */
    synctex_ctxt.recorder = None;
    synctex_ctxt.tag = (*mem.offset((this_box + 8i32 - 1i32) as isize)).b32.s0;
    synctex_ctxt.line = (*mem.offset((this_box + 8i32 - 1i32) as isize)).b32.s1;
    synctex_ctxt.curh = cur_h + 4736287i32;
    synctex_ctxt.curv = cur_v + 4736287i32;
    synctex_record_node_vlist(this_box);
}
/*  Recording a "}" line ending a vbox: this message is sent whenever a vlist
 *  has been shipped out. It is used to close the vlist nesting level. It is
 *  sent at the end of each vlist_out procedure in *TeX.web to balance a former
 *  synctex_vlist sent at the beginning of that procedure.    */
/*  Recording a "f" line ending a vbox: this message is sent whenever a vlist
 *  has been shipped out. It is used to close the vlist nesting level. It is
 *  sent at the end of the vlist_out procedure in *TeX.web to balance a former
 *  synctex_vlist sent at the beginning of that procedure.    */
#[no_mangle]
pub unsafe extern "C" fn synctex_tsilv(mut this_box: i32) {
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return;
    }
    /*  Ignoring any pending info to be recorded  */
    synctex_ctxt.node = this_box; /*  0 to reset  */
    synctex_ctxt.tag = (*mem.offset((this_box + 8i32 - 1i32) as isize)).b32.s0;
    synctex_ctxt.line = (*mem.offset((this_box + 8i32 - 1i32) as isize)).b32.s1;
    synctex_ctxt.curh = cur_h + 4736287i32;
    synctex_ctxt.curv = cur_v + 4736287i32;
    synctex_ctxt.recorder = None;
    synctex_record_node_tsilv(this_box);
}
/*  This message is sent when a void vlist will be shipped out.
 *  There is no need to balance a void vlist.  */
/*  This message is sent when a void vlist will be shipped out.
 *  There is no need to balance a void vlist.  */
#[no_mangle]
pub unsafe extern "C" fn synctex_void_vlist(mut p: i32, mut this_box: i32) {
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return;
    } /*  reset  */
    synctex_ctxt.node = p; /*  reset  */
    synctex_ctxt.tag = (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s0;
    synctex_ctxt.line = (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s1;
    synctex_ctxt.curh = cur_h + 4736287i32;
    synctex_ctxt.curv = cur_v + 4736287i32;
    synctex_ctxt.recorder = None;
    synctex_record_node_void_vlist(p);
}
/*  Send this message when an hlist will be shipped out, more precisely at
 *  the beginning of the hlist_out procedure in *TeX.web.  It must be balanced
 *  by a synctex_tsilh, sent at the end of the hlist_out procedure.  p is the
 *  address of the hlist. */
/*  This message is sent when an hlist will be shipped out, more precisely at
 *  the beginning of the hlist_out procedure in *TeX.web.  It will be balanced
 *  by a synctex_tsilh, sent at the end of the hlist_out procedure.  p is the
 *  address of the hlist We assume that p is really an hlist node! */
#[no_mangle]
pub unsafe extern "C" fn synctex_hlist(mut this_box: i32) {
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return;
    } /*  0 to reset  */
    synctex_ctxt.node = this_box; /*  reset  */
    synctex_ctxt.tag = (*mem.offset((this_box + 8i32 - 1i32) as isize)).b32.s0;
    synctex_ctxt.line = (*mem.offset((this_box + 8i32 - 1i32) as isize)).b32.s1;
    synctex_ctxt.curh = cur_h + 4736287i32;
    synctex_ctxt.curv = cur_v + 4736287i32;
    synctex_ctxt.recorder = None;
    synctex_record_node_hlist(this_box);
}
/*  Send this message at the end of the various hlist_out procedure in *TeX.web
 *  to balance a former synctex_hlist.    */
/*  Recording a ")" line ending an hbox this message is sent whenever an hlist
 *  has been shipped out it is used to close the hlist nesting level. It is
 *  sent at the end of the hlist_out procedure in *TeX.web to balance a former
 *  synctex_hlist sent at the beginning of that procedure.    */
#[no_mangle]
pub unsafe extern "C" fn synctex_tsilh(mut this_box: i32) {
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return;
    }
    /*  Ignoring any pending info to be recorded  */
    synctex_ctxt.node = this_box; /*  0 to force next node to be recorded!  */
    synctex_ctxt.tag = (*mem.offset((this_box + 8i32 - 1i32) as isize)).b32.s0; /*  reset  */
    synctex_ctxt.line = (*mem.offset((this_box + 8i32 - 1i32) as isize)).b32.s1;
    synctex_ctxt.curh = cur_h + 4736287i32;
    synctex_ctxt.curv = cur_v + 4736287i32;
    synctex_ctxt.recorder = None;
    synctex_record_node_tsilh(this_box);
}
/*  This message is sent when a void hlist will be shipped out.
 *  There is no need to balance a void hlist.  */
/*  This message is sent when a void hlist will be shipped out.
 *  There is no need to balance a void hlist.  */
#[no_mangle]
pub unsafe extern "C" fn synctex_void_hlist(mut p: i32, mut this_box: i32) {
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return;
    }
    /*  the sync context has changed  */
    if synctex_ctxt.recorder.is_some() {
        /*  but was not yet recorded  */
        synctex_ctxt.recorder.expect("non-null function pointer")(synctex_ctxt.node);
        /*  0 to reset  */
    } /*  reset  */
    synctex_ctxt.node = p;
    synctex_ctxt.tag = (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s0;
    synctex_ctxt.line = (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s1;
    synctex_ctxt.curh = cur_h + 4736287i32;
    synctex_ctxt.curv = cur_v + 4736287i32;
    synctex_ctxt.recorder = None;
    synctex_record_node_void_hlist(p);
}
/*  Send this message whenever an inline math node will ship out. */
/*  This macro will detect a change in the synchronization context.  As long as
 *  the synchronization context remains the same, there is no need to write
 *  synchronization info: it would not help more.  The synchronization context
 *  has changed when either the line number or the file tag has changed.  */
/*  glue code, this message is sent whenever an inline math node will ship out
See: @ @<Output the non-|char_node| |p| for...  */
#[no_mangle]
pub unsafe extern "C" fn synctex_math(mut p: i32, mut this_box: i32) {
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return;
    }
    if synctex_ctxt.recorder.is_some()
        && (0i32 == synctex_ctxt.node
            || (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0 != synctex_ctxt.tag
            || (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1 != synctex_ctxt.line)
    {
        /*  the sync context did change  */
        synctex_ctxt.recorder.expect("non-null function pointer")(synctex_ctxt.node);
        /*  no need to record once more  */
    }
    synctex_ctxt.node = p;
    synctex_ctxt.tag = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0;
    synctex_ctxt.line = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1;
    synctex_ctxt.curh = cur_h + 4736287i32;
    synctex_ctxt.curv = cur_v + 4736287i32;
    synctex_ctxt.recorder = None;
    synctex_record_node_math(p);
    /*  always record synchronously  */
}
/*  Send this message whenever an horizontal rule or glue node will ship out. */
/*  this message is sent whenever an horizontal glue node or rule node ships out
See: move_past:...    */
#[no_mangle]
pub unsafe extern "C" fn synctex_horizontal_rule_or_glue(mut p: i32, mut this_box: i32) {
    match (*mem.offset(p as isize)).b16.s1 as i32 {
        2 => {
            if synctex_ctxt.flags.off() as i32 != 0
                || (*eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 83i32) as isize,
                ))
                .b32
                .s1 == 0
                || 0i32 >= (*mem.offset((p + 5i32 - 1i32) as isize)).b32.s0
                || 0i32 >= (*mem.offset((p + 5i32 - 1i32) as isize)).b32.s1
            {
                return;
            }
        }
        10 => {
            if synctex_ctxt.flags.off() as i32 != 0
                || (*eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 83i32) as isize,
                ))
                .b32
                .s1 == 0
                || 0i32 >= (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0
                || 0i32 >= (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1
            {
                return;
            }
        }
        11 => {
            if synctex_ctxt.flags.off() as i32 != 0
                || (*eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 83i32) as isize,
                ))
                .b32
                .s1 == 0
                || 0i32 >= (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0
                || 0i32 >= (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1
            {
                return;
            }
        }
        _ => {
            ttstub_issue_error(
                b"unknown node type %d in SyncTeX\x00" as *const u8 as *const i8,
                (*mem.offset(p as isize)).b16.s1 as i32,
            ); /*  always record synchronously: maybe some text is outside the box  */
        }
    } /*  always record synchronously: maybe some text is outside the box  */
    synctex_ctxt.node = p; /*  always record synchronously: maybe some text is outside the box  */
    synctex_ctxt.curh = cur_h + 4736287i32;
    synctex_ctxt.curv = cur_v + 4736287i32;
    synctex_ctxt.recorder = None;
    match (*mem.offset(p as isize)).b16.s1 as i32 {
        2 => {
            synctex_ctxt.tag = (*mem.offset((p + 5i32 - 1i32) as isize)).b32.s0;
            synctex_ctxt.line = (*mem.offset((p + 5i32 - 1i32) as isize)).b32.s1;
            synctex_record_node_rule(p);
        }
        10 => {
            synctex_ctxt.tag = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0;
            synctex_ctxt.line = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1;
            synctex_record_node_glue(p);
        }
        11 => {
            synctex_ctxt.tag = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0;
            synctex_ctxt.line = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1;
            synctex_record_node_kern(p);
        }
        _ => {
            ttstub_issue_error(
                b"unknown node type %d in SyncTeX\x00" as *const u8 as *const i8,
                (*mem.offset(p as isize)).b16.s1 as i32,
            );
        }
    };
}
/*  Send this message whenever a kern node will ship out. */
/*  this message is sent whenever a kern node ships out
See: @ @<Output the non-|char_node| |p| for...    */
#[no_mangle]
pub unsafe extern "C" fn synctex_kern(mut p: i32, mut this_box: i32) {
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || 0i32 >= (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0
        || 0i32 >= (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1
    {
        return;
    }
    if 0i32 == synctex_ctxt.node
        || (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0 != synctex_ctxt.tag
        || (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1 != synctex_ctxt.line
    {
        /*  the sync context has changed  */
        if synctex_ctxt.recorder.is_some() {
            /*  but was not yet recorded  */
            synctex_ctxt.recorder.expect("non-null function pointer")(synctex_ctxt.node);
        }
        if synctex_ctxt.node == this_box {
            /* first node in the list */
            synctex_ctxt.node = p;
            synctex_ctxt.tag = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0;
            synctex_ctxt.line = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1;
            synctex_ctxt.recorder =
                Some(synctex_record_node_kern as unsafe extern "C" fn(_: i32) -> ())
        } else {
            synctex_ctxt.node = p;
            synctex_ctxt.tag = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0;
            synctex_ctxt.line = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1;
            synctex_ctxt.recorder = None;
            /*  always record when the context has just changed
             *  and when not the first node  */
            synctex_record_node_kern(p);
        }
    } else {
        /*  just update the geometry and type (for future improvements)  */
        synctex_ctxt.node = p;
        synctex_ctxt.tag = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0;
        synctex_ctxt.line = (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1;
        synctex_ctxt.recorder = Some(synctex_record_node_kern as unsafe extern "C" fn(_: i32) -> ())
    };
}
/*  For debugging purpose only    */
/*  this message should be sent to record information
synchronously for the current location    */
#[no_mangle]
pub unsafe extern "C" fn synctex_current() {
    let mut len: i32 = 0; /* magic pt/in conversion */
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return;
    } /* XXX: should this be `+=`? */
    len = ttstub_fprintf(
        synctex_ctxt.file,
        b"x%i,%i:%i,%i\n\x00" as *const u8 as *const i8,
        synctex_ctxt.tag,
        synctex_ctxt.line,
        (cur_h + 4736287i32) / synctex_ctxt.unit,
        (cur_v + 4736287i32) / synctex_ctxt.unit,
    ); /* XXX: should this be `+=`? */
    synctex_ctxt.lastv = cur_v + 4736287i32;
    if len > 0i32 {
        synctex_ctxt.total_length += len
    } else {
        synctexabort();
    };
}
#[inline]
unsafe extern "C" fn synctex_record_settings() -> i32 {
    let mut len: i32 = 0;
    if synctex_ctxt.file.is_null() {
        return 0i32;
    }
    len = ttstub_fprintf(
        synctex_ctxt.file,
        b"Output:pdf\nMagnification:%i\nUnit:%i\nX Offset:0\nY Offset:0\n\x00" as *const u8
            as *const i8,
        synctex_ctxt.magnification,
        synctex_ctxt.unit,
    );
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        return 0i32;
    }
    synctexabort();
    return -1i32;
}
#[inline]
unsafe extern "C" fn synctex_record_preamble() -> i32 {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"SyncTeX Version:%i\n\x00" as *const u8 as *const i8,
        1i32,
    );
    if len > 0i32 {
        synctex_ctxt.total_length = len;
        return 0i32;
    }
    synctexabort();
    return -1i32;
}
#[inline]
unsafe extern "C" fn synctex_record_input(mut tag: i32, mut name: *mut i8) -> i32 {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"Input:%i:%s\n\x00" as *const u8 as *const i8,
        tag,
        name,
    );
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        return 0i32;
    }
    synctexabort();
    return -1i32;
}
#[inline]
unsafe extern "C" fn synctex_record_anchor() -> i32 {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"!%i\n\x00" as *const u8 as *const i8,
        synctex_ctxt.total_length,
    );
    if len > 0i32 {
        synctex_ctxt.total_length = len;
        synctex_ctxt.count += 1;
        return 0i32;
    }
    synctexabort();
    return -1i32;
}
#[inline]
unsafe extern "C" fn synctex_record_content() -> i32 {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"Content:\n\x00" as *const u8 as *const i8,
    );
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        return 0i32;
    }
    synctexabort();
    return -1i32;
}
#[inline]
unsafe extern "C" fn synctex_record_sheet(mut sheet: i32) -> i32 {
    if 0i32 == synctex_record_anchor() {
        let mut len: i32 = ttstub_fprintf(
            synctex_ctxt.file,
            b"{%i\n\x00" as *const u8 as *const i8,
            sheet,
        );
        if len > 0i32 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
            return 0i32;
        }
    }
    synctexabort();
    return -1i32;
}
/*  Recording a "}..." or a ">" line  */
#[inline]
unsafe extern "C" fn synctex_record_teehs(mut sheet: i32) -> i32 {
    if 0i32 == synctex_record_anchor() {
        let mut len: i32 = ttstub_fprintf(
            synctex_ctxt.file,
            b"}%i\n\x00" as *const u8 as *const i8,
            sheet,
        );
        if len > 0i32 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
            return 0i32;
        }
    }
    synctexabort();
    return -1i32;
}
/*  Recording the "<..." line.  In pdftex.web, use synctex_pdfxform(p) at
 *  the very beginning of the pdf_ship_out procedure.
 */
#[no_mangle]
pub unsafe extern "C" fn synctex_pdfxform(mut p: i32) {
    if synctex_ctxt.flags.off() != 0 {
        if (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 != 0
            && synctex_ctxt.flags.warn() == 0
        {
            synctex_ctxt.flags.set_warn(1i32 as u32);
            ttstub_issue_warning(
                b"SyncTeX was disabled - changing the value of \\synctex has no effect\x00"
                    as *const u8 as *const i8,
            );
        }
        return;
    }
    if !synctex_prepare_content().is_null() {
        synctex_record_pdfxform(p);
    };
}
/*  Recording the ">" line.  In pdftex.web, use synctex_mrofxfdp at
 *  the very end of the ship_out procedure.
 */
#[no_mangle]
pub unsafe extern "C" fn synctex_mrofxfdp() {
    if !synctex_ctxt.file.is_null() {
        synctex_record_mrofxfdp();
    };
}
#[no_mangle]
pub unsafe extern "C" fn synctex_pdfrefxform(mut objnum: i32) {
    if !synctex_ctxt.file.is_null() {
        synctex_record_node_pdfrefxform(objnum);
    };
}
/*  Recording a "<..." line  */
#[inline]
unsafe extern "C" fn synctex_record_pdfxform(mut form: i32) -> i32 {
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return 0i32;
    } else {
        let mut len: i32 = 0;
        /* XXX Tectonic: guessing that SYNCTEX_PDF_CUR_FORM = synctex_ctxt.form_depth here */
        synctex_ctxt.form_depth += 1;
        len = ttstub_fprintf(
            synctex_ctxt.file,
            b"<%i\n\x00" as *const u8 as *const i8,
            synctex_ctxt.form_depth,
        );
        if len > 0i32 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
            return 0i32;
        }
    }
    synctexabort();
    return -1i32;
}
/*  Recording a ">" line  */
#[inline]
unsafe extern "C" fn synctex_record_mrofxfdp() -> i32 {
    if 0i32 == synctex_record_anchor() {
        let mut len: i32 = 0;
        /* XXX Tectonic: mistake here in original source, no %d in format string */
        synctex_ctxt.form_depth -= 1;
        len = ttstub_fprintf(synctex_ctxt.file, b">\n\x00" as *const u8 as *const i8);
        if len > 0i32 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
            return 0i32;
        }
    }
    synctexabort();
    return -1i32;
}
/*  Recording a "f..." line  */
#[inline]
unsafe extern "C" fn synctex_record_node_pdfrefxform(mut objnum: i32) -> i32
/* UNUSED form JL */ {
    synctex_ctxt.curh = cur_h + 4736287i32;
    synctex_ctxt.curv = cur_v + 4736287i32;
    if synctex_ctxt.flags.off() as i32 != 0
        || (*eqtb.offset(
            (1i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 1i32
                + 15000i32
                + 12i32
                + 9000i32
                + 1i32
                + 1i32
                + 19i32
                + 256i32
                + 256i32
                + 13i32
                + 256i32
                + 4i32
                + 256i32
                + 1i32
                + 3i32 * 256i32
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + (0x10ffffi32 + 1i32)
                + 83i32) as isize,
        ))
        .b32
        .s1 == 0
        || synctex_ctxt.file.is_null()
    {
        return 0i32;
    } else {
        let mut len: i32 = 0i32;
        len = ttstub_fprintf(
            synctex_ctxt.file,
            b"f%i:%i,%i\n\x00" as *const u8 as *const i8,
            objnum,
            (cur_h + 4736287i32) / synctex_ctxt.unit,
            (cur_v + 4736287i32) / synctex_ctxt.unit,
        );
        synctex_ctxt.lastv = cur_v + 4736287i32;
        if len > 0i32 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
            return 0i32;
        }
    }
    synctexabort();
    return -1i32;
}
#[inline]
unsafe extern "C" fn synctex_record_node_void_vlist(mut p: i32) {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"v%i,%i:%i,%i:%i,%i,%i\n\x00" as *const u8 as *const i8,
        (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s0,
        (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s1,
        synctex_ctxt.curh / synctex_ctxt.unit,
        synctex_ctxt.curv / synctex_ctxt.unit,
        (*mem.offset((p + 1i32) as isize)).b32.s1 / synctex_ctxt.unit,
        (*mem.offset((p + 3i32) as isize)).b32.s1 / synctex_ctxt.unit,
        (*mem.offset((p + 2i32) as isize)).b32.s1 / synctex_ctxt.unit,
    );
    synctex_ctxt.lastv = cur_v + 4736287i32;
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        synctex_ctxt.count += 1
    } else {
        synctexabort();
    };
}
#[inline]
unsafe extern "C" fn synctex_record_node_vlist(mut p: i32) {
    let mut len: i32 = 0;
    synctex_ctxt.flags.set_not_void(1i32 as u32);
    len = ttstub_fprintf(
        synctex_ctxt.file,
        b"[%i,%i:%i,%i:%i,%i,%i\n\x00" as *const u8 as *const i8,
        (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s0,
        (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s1,
        synctex_ctxt.curh / synctex_ctxt.unit,
        synctex_ctxt.curv / synctex_ctxt.unit,
        (*mem.offset((p + 1i32) as isize)).b32.s1 / synctex_ctxt.unit,
        (*mem.offset((p + 3i32) as isize)).b32.s1 / synctex_ctxt.unit,
        (*mem.offset((p + 2i32) as isize)).b32.s1 / synctex_ctxt.unit,
    );
    synctex_ctxt.lastv = cur_v + 4736287i32;
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        synctex_ctxt.count += 1
    } else {
        synctexabort();
    };
}
#[inline]
unsafe extern "C" fn synctex_record_node_tsilv(mut p: i32) {
    let mut len: i32 = ttstub_fprintf(synctex_ctxt.file, b"]\n\x00" as *const u8 as *const i8);
    if len > 0i32 {
        synctex_ctxt.total_length += len
    /* is it correct that synctex_ctxt.count is not incremented here? */
    } else {
        synctexabort();
    };
}
#[inline]
unsafe extern "C" fn synctex_record_node_void_hlist(mut p: i32) {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"h%i,%i:%i,%i:%i,%i,%i\n\x00" as *const u8 as *const i8,
        (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s0,
        (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s1,
        synctex_ctxt.curh / synctex_ctxt.unit,
        synctex_ctxt.curv / synctex_ctxt.unit,
        (*mem.offset((p + 1i32) as isize)).b32.s1 / synctex_ctxt.unit,
        (*mem.offset((p + 3i32) as isize)).b32.s1 / synctex_ctxt.unit,
        (*mem.offset((p + 2i32) as isize)).b32.s1 / synctex_ctxt.unit,
    );
    synctex_ctxt.lastv = cur_v + 4736287i32;
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        synctex_ctxt.count += 1
    } else {
        synctexabort();
    };
}
#[inline]
unsafe extern "C" fn synctex_record_node_hlist(mut p: i32) {
    let mut len: i32 = 0;
    synctex_ctxt.flags.set_not_void(1i32 as u32);
    len = ttstub_fprintf(
        synctex_ctxt.file,
        b"(%i,%i:%i,%i:%i,%i,%i\n\x00" as *const u8 as *const i8,
        (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s0,
        (*mem.offset((p + 8i32 - 1i32) as isize)).b32.s1,
        synctex_ctxt.curh / synctex_ctxt.unit,
        synctex_ctxt.curv / synctex_ctxt.unit,
        (*mem.offset((p + 1i32) as isize)).b32.s1 / synctex_ctxt.unit,
        (*mem.offset((p + 3i32) as isize)).b32.s1 / synctex_ctxt.unit,
        (*mem.offset((p + 2i32) as isize)).b32.s1 / synctex_ctxt.unit,
    );
    synctex_ctxt.lastv = cur_v + 4736287i32;
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        synctex_ctxt.count += 1
    } else {
        synctexabort();
    };
}
#[inline]
unsafe extern "C" fn synctex_record_node_tsilh(mut p: i32) {
    let mut len: i32 = ttstub_fprintf(synctex_ctxt.file, b")\n\x00" as *const u8 as *const i8);
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        synctex_ctxt.count += 1
    } else {
        synctexabort();
    };
}
#[inline]
unsafe extern "C" fn synctex_record_count() -> i32 {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"Count:%i\n\x00" as *const u8 as *const i8,
        synctex_ctxt.count,
    );
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        return 0i32;
    }
    synctexabort();
    return -1i32;
}
#[inline]
unsafe extern "C" fn synctex_record_postamble() -> i32 {
    if 0i32 == synctex_record_anchor() {
        let mut len: i32 = ttstub_fprintf(
            synctex_ctxt.file,
            b"Postamble:\n\x00" as *const u8 as *const i8,
        );
        if len > 0i32 {
            synctex_ctxt.total_length += len;
            if synctex_record_count() == 0 && synctex_record_anchor() == 0 {
                len = ttstub_fprintf(
                    synctex_ctxt.file,
                    b"Post scriptum:\n\x00" as *const u8 as *const i8,
                );
                if len > 0i32 {
                    synctex_ctxt.total_length += len;
                    return 0i32;
                }
            }
        }
    }
    synctexabort();
    return -1i32;
}
#[inline]
unsafe extern "C" fn synctex_record_node_glue(mut p: i32) {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"g%i,%i:%i,%i\n\x00" as *const u8 as *const i8,
        (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0,
        (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1,
        synctex_ctxt.curh / synctex_ctxt.unit,
        synctex_ctxt.curv / synctex_ctxt.unit,
    );
    synctex_ctxt.lastv = cur_v + 4736287i32;
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        synctex_ctxt.count += 1
    } else {
        synctexabort();
    };
}
#[inline]
unsafe extern "C" fn synctex_record_node_kern(mut p: i32) {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"k%i,%i:%i,%i:%i\n\x00" as *const u8 as *const i8,
        (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0,
        (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1,
        synctex_ctxt.curh / synctex_ctxt.unit,
        synctex_ctxt.curv / synctex_ctxt.unit,
        (*mem.offset((p + 1i32) as isize)).b32.s1 / synctex_ctxt.unit,
    );
    synctex_ctxt.lastv = cur_v + 4736287i32;
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        synctex_ctxt.count += 1
    } else {
        synctexabort();
    };
}
#[inline]
unsafe extern "C" fn synctex_record_node_rule(mut p: i32) {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"r%i,%i:%i,%i:%i,%i,%i\n\x00" as *const u8 as *const i8,
        (*mem.offset((p + 5i32 - 1i32) as isize)).b32.s0,
        (*mem.offset((p + 5i32 - 1i32) as isize)).b32.s1,
        synctex_ctxt.curh / synctex_ctxt.unit,
        synctex_ctxt.curv / synctex_ctxt.unit,
        rule_wd / synctex_ctxt.unit,
        rule_ht / synctex_ctxt.unit,
        rule_dp / synctex_ctxt.unit,
    );
    synctex_ctxt.lastv = cur_v + 4736287i32;
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        synctex_ctxt.count += 1
    } else {
        synctexabort();
    };
}
unsafe extern "C" fn synctex_record_node_math(mut p: i32) {
    let mut len: i32 = ttstub_fprintf(
        synctex_ctxt.file,
        b"$%i,%i:%i,%i\n\x00" as *const u8 as *const i8,
        (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s0,
        (*mem.offset((p + 3i32 - 1i32) as isize)).b32.s1,
        synctex_ctxt.curh / synctex_ctxt.unit,
        synctex_ctxt.curv / synctex_ctxt.unit,
    );
    synctex_ctxt.lastv = cur_v + 4736287i32;
    if len > 0i32 {
        synctex_ctxt.total_length += len;
        synctex_ctxt.count += 1
    } else {
        synctexabort();
    };
}
unsafe extern "C" fn run_static_initializers() {
    synctex_ctxt = {
        let mut init = C2RustUnnamed {
            file: 0 as *mut libc::c_void,
            root_name: 0 as *mut i8,
            count: 0i32,
            node: 0i32,
            recorder: None,
            tag: 0i32,
            line: 0i32,
            curh: 0i32,
            curv: 0i32,
            magnification: 0i32,
            unit: 0i32,
            total_length: 0i32,
            lastv: -1i32,
            form_depth: 0i32,
            synctex_tag_counter: 0i32 as u32,
            flags: {
                let mut init = _flags {
                    content_ready_off_not_void_warn_output_p: [0; 1],
                    c2rust_padding: [0; 3],
                };
                init.set_content_ready(0i32 as u32);
                init.set_off(0i32 as u32);
                init.set_not_void(0i32 as u32);
                init.set_warn(0i32 as u32);
                init.set_output_p(0i32 as u32);
                init
            },
        };
        init
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
/*
License:
--------
Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE

Except as contained in this notice, the name of the copyright holder
shall not be used in advertising or otherwise to promote the sale,
use or other dealings in this Software without prior written
authorization from the copyright holder.

Important notice:
-----------------
This file is named "synctex.c", it may or may not have a header counterpart
depending on its use.  It aims to provide basic components useful for the
input/output synchronization technology for TeX.
The purpose of the implementation is threefold
- firstly, it defines a new input/output synchronization technology named
"synchronize texnology", "SyncTeX" or "synctex"
- secondly, it defines the naming convention and format of the auxiliary file
used by this technology
- thirdly, it defines the API of a controller and a controller, used in
particular by the pdfTeX and XeTeX programs to prepare synchronization.

All these are up to a great extent de facto definitions, which means that they
are partly defined by the implementation itself.

This technology was first designed for pdfTeX, an extension of TeX managing the
pdf output file format, but it can certainly be adapted to other programs built
from TeX as long as the extensions do not break too much the core design.
Moreover, the synchronize texnology only relies on code concept and not
implementation details, so it can be ported to other TeX systems.  In order to
support SyncTeX, one can start reading the dedicated section in synctex.ch,
sync-pdftex.ch and sync-xetex.ch. Actually, support is provided for TeX, e-TeX,
pdfTeX and XeTeX.

Other existing public synchronization technologies are defined by srcltx.sty -
also used by source specials - and pdfsync.sty.  Like them, the synchronize
texnology is meant to be shared by various text editors, viewers and TeX
engines.  A centralized reference and source of information is available in TeX-Live.

Versioning:
-----------
As synctex is embedded into different TeX implementation, there is an independent
versionning system.
For TeX implementations, the actual version is: 3
For .synctex file format, the actual version is SYNCTEX_VERSION below

Please, do not remove these explanations.

Acknowledgments:
----------------
The author received useful remarks from the pdfTeX developers, especially Hahn The Thanh,
and significant help from XeTeX developer Jonathan Kew

Nota Bene:
----------
If you include or use a significant part of the synctex package into a software,
I would appreciate to be listed as contributor and see "SyncTeX" highlighted.

History:
--------
Version 1.14
Fri Apr 15 19:10:57 UTC 2011
- taking output_directory into account
- Replaced FOPEN_WBIN_MODE by FOPEN_W_MODE when opening the text version of the .synctex file.
- Merging with LuaTeX's version of synctex.c

Version 3
- very minor design change to take luatex into account
- typo fixed
- some size_t replaced by int
- very minor code design change to remove wrong xetex specific warnings

Version 2
Fri Sep 19 14:55:31 UTC 2008
- support for file names containing spaces.
This is one thing that xetex and pdftex do not manage the same way.
When the input file name contains a space character ' ',
pdftex will automatically enclose this name between two quote characters '"',
making programs believe that these quotes are really part of the name.
xetex does nothing special.
For that reason, running the command line
xetex --synctex=-1 "my file.tex"
is producing the expected file named <my file.synctex>, (the '<' and '>' are not part of the name)
whereas running the command line
pdftex --synctex=-1 "my file.tex"
was producing the unexpected file named <"my file".synctex> where the two '"' characters were part of the name.
Of course, that was breaking the typesetting mechanism when pdftex was involved.
To solve this problem, we prefer to rely on the output_file_name instead of the jobname.
In the case when no output_file_name is available, we use jobname and test if the file name
starts and ends with a quote character. Every synctex output file is removed because we consider
TeX encontered a problem.
There is some conditional coding.

*/
/* deleted because unused in Tectonic:
    synctex_record_node_char (AKA synctex_node_recorder),
    synctex_record_node_unknown (AKA synctex_node_recorder),
*/
