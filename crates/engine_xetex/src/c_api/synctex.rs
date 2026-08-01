use crate::c_api::globals::Globals;
use bitflags::bitflags;
use std::cell::RefCell;
use std::io::Write;
use std::ptr;
use tectonic_bridge_core::OutputId;

pub type SynctexRecorder = extern "C" fn(i32);

thread_local! {
    static SYNCTEX: RefCell<SynctexCtx> = const { RefCell::new(SynctexCtx::new()) };
}

bitflags! {
    #[repr(transparent)]
    struct Flags: u8 {
        const CONTENT_READY = 0x1;
        /// Definitely turn off synctex, corresponds to cli option -synctex=0
        const OFF = 0x2;
        /// Whether it really contains synchronization material
        const NOT_VOID = 0x4;
        /// One shot warning flag
        const WARN = 0x8;
        /// Whether the output_directory is used
        const OUTPUT_P = 0x10;
    }
}

#[repr(C)]
pub struct SynctexCtx {
    /// the foo.synctex or foo.synctex.gz I/O identifier
    file: Option<OutputId>,
    /// in general jobname.tex
    root_name: *const libc::c_char,
    /*  next concern the last sync record encountered  */
    /// The number of interesting records in "foo.synctex"
    count: i32,
    /// the last synchronized node, must be set before the recorder
    node: i32,
    /// the recorder of the node above, the routine that knows how to record the node to the .synctex file
    recorder: Option<SynctexRecorder>,
    /// Current tag
    tag: i32,
    /// Current line
    line: i32,
    /// Current point h
    curh: i32,
    /// Current point v
    curv: i32,
    /// The magnification as given by \mag
    magnification: i32,
    /// The unit, defaults to 1, use 8192 to produce shorter but less accurate info
    unit: i32,
    /// The total length of the bytes written since the last check point
    total_length: i32,
    /// compression trick if |synctex_options & 4| > 0
    lastv: i32,
    /// PDF forms are an example of nested sheets
    form_depth: i32,
    /// Global tag counter, used to be a local static in synctex_start_input
    synctex_tag_counter: u32,
    /// Synctex flags that control behavior
    flags: Flags,
}

impl SynctexCtx {
    const fn new() -> SynctexCtx {
        SynctexCtx {
            file: None,
            root_name: ptr::null(),
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
            lastv: -1,
            form_depth: 0,
            synctex_tag_counter: 0,
            flags: Flags::empty(),
        }
    }

    pub fn with<T>(f: impl FnOnce(&mut SynctexCtx) -> T) -> T {
        SYNCTEX.with_borrow_mut(f)
    }
}

#[no_mangle]
pub extern "C" fn synctex_ctx() -> *mut SynctexCtx {
    SYNCTEX.with_borrow_mut(ptr::from_mut)
}

pub fn rs_synctex_record_anchor(globals: &mut Globals<'_, '_>) -> Option<()> {
    let output = globals.state.get_output(globals.synctex.file?);
    let to_write = format!("!{}\n", globals.synctex.total_length);
    if output.write_all(to_write.as_bytes()).is_ok() {
        /* XXX: should this be `+=`? */
        globals.synctex.total_length = to_write.len() as i32;
        globals.synctex.count += 1;
        return Some(());
    }
    None
}

pub fn rs_synctex_record_count(globals: &mut Globals<'_, '_>) -> Option<()> {
    let output = globals.state.get_output(globals.synctex.file?);
    let to_write = format!("Count:{}\n", globals.synctex.count);
    if output.write_all(to_write.as_bytes()).is_ok() {
        globals.synctex.total_length += to_write.len() as i32;
        return Some(());
    }
    None
}

pub fn rs_synctex_record_postamble(globals: &mut Globals<'_, '_>) -> Option<()> {
    rs_synctex_record_anchor(globals)?;
    let output = globals.state.get_output(globals.synctex.file?);
    if writeln!(output, "Postamble:").is_ok() {
        globals.synctex.total_length += "Postamble:\n".len() as i32;
        rs_synctex_record_count(globals)?;
        rs_synctex_record_anchor(globals)?;

        let output = globals.state.get_output(globals.synctex.file?);
        if writeln!(output, "Post scriptum:").is_ok() {
            globals.synctex.total_length += "Post scriptum:\n".len() as i32;
            return Some(());
        }
    }
    None
}

#[no_mangle]
pub extern "C" fn synctex_record_anchor() -> i32 {
    Globals::with(|globals| rs_synctex_record_anchor(globals).or_else(|| rs_synctex_abort(globals)))
        .map_or(-1, |_| 0)
}

#[no_mangle]
pub extern "C" fn synctex_record_count() -> i32 {
    Globals::with(|globals| rs_synctex_record_count(globals).or_else(|| rs_synctex_abort(globals)))
        .map_or(-1, |_| 0)
}

#[no_mangle]
pub extern "C" fn synctex_record_postamble() -> i32 {
    Globals::with(|globals| {
        rs_synctex_record_postamble(globals).or_else(|| rs_synctex_abort(globals))
    })
    .map_or(-1, |_| 0)
}

/// Free all memory used, close the file if any,
/// It is sent locally when there is a problem with synctex output.
/// It is sent by pdftex when a fatal error occurred in pdftex.web.
pub fn rs_synctex_abort(globals: &mut Globals<'_, '_>) -> Option<()> {
    if let Some(file) = globals.synctex.file.take() {
        globals.state.output_close(file);
    }
    unsafe { libc::free(globals.synctex.root_name.cast_mut().cast()) };
    globals.synctex.root_name = ptr::null();
    globals.synctex.flags |= Flags::OFF;
    None
}

#[no_mangle]
pub extern "C" fn synctexabort() {
    Globals::with(rs_synctex_abort);
}

/// Free all memory used and close the file,
///  sent by close_files_and_terminate in tex.web.
///  synctexterminate() is called when the TeX run terminates.
pub fn rs_synctex_terminate(globals: &mut Globals<'_, '_>, _: bool) {
    if globals.synctex.file.is_some() {
        /* We keep the file even if no tex output is produced
         * (synctex_ctx()->flags.not_void == 0). I assume that this means that there
         * was an error and tectonic will not save anything anyway. */
        rs_synctex_record_postamble(globals);
    }
    rs_synctex_abort(globals);
}

#[no_mangle]
pub extern "C" fn synctex_terminate(log_opened: bool) {
    Globals::with(|globals| rs_synctex_terminate(globals, log_opened))
}
