use bitflags::bitflags;
use std::cell::RefCell;
use std::ffi::CString;
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
    SYNCTEX.with_borrow_mut(|synctex| ptr::from_mut(synctex))
}
