// Copyright 2016-2022 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! Core APIs for bridging the C and Rust portions of Tectonic’s processing
//! backends.
//!
//! This crate is used by the Tectonic “engines”, which are predominantly C/C++
//! code derived from the original TeX codebase. It provides a framework so that
//! the C/C++ code can invoke the support services provided by Tectonic, such as
//! its pluggable I/O backends. The interfaces exposed to the C/C++ layers are
//! created by [cbindgen].
//!
//! [cbindgen]: https://github.com/eqrion/cbindgen
//!
//! If you change the interfaces here, rerun cbindgen as described in the README!
//!
//! In order to provide access to a C/C++ engine in Rust, you should create some
//! kind of interface that expects to be given a reference to a
//! [`CoreBridgeLauncher`] struct. You should use that struct's
//! `with_global_lock` method to obtain a [`CoreBridgeState`] reference, and
//! then pass that reference across the FFI layer. On the other side of the FFI
//! divide, your code *must* call the functions `ttbc_global_engine_enter()` and
//! `ttbc_global_engine_exit()` according to the pattern described in
//! `tectonic_bridge_core.h`. If an abort is detected, the callback function
//! must return `Err(EngineAbortedError::new_abort_indicator().into())`.
//! Unfortunately, this is the cleanest and most reliable API that we can
//! provide because our abort handling uses `setjmp`/`longjmp` and those can't
//! cross FFI boundaries.
//!
//! In order to use a C/C++ engine, you need to provide something that
//! implements the [`DriverHooks`] trait. The [`MinimalDriver`] struct provides
//! a minimal implementation that only requires you to provide an [`IoProvider`]
//! implementation.

use flate2::{read::GzDecoder, Compression, GzBuilder};
use md5::{Digest, Md5};
use std::{
    convert::TryInto,
    ffi::CStr,
    fmt::{Display, Error as FmtError, Formatter},
    io::{self, Read, SeekFrom, Write},
    path::PathBuf,
    ptr,
    result::Result as StdResult,
    slice,
    sync::Mutex,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{
    digest::DigestData, normalize_tex_path, InputFeatures, InputHandle, IoProvider, OpenResult,
    OutputHandle,
};
use tectonic_status_base::{tt_error, tt_warning, MessageKind, StatusBackend};

/// Possible failures for “system request” calls to the driver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemRequestError {
    /// The driver does not implement this system request.
    NotImplemented,

    /// The driver implements this system request but is not allowing it
    /// to be used in this circumstance.
    NotAllowed,

    /// The driver tried to execute this system request but it failed in some
    /// fashion. There's no facility for providing more detailed information
    /// because the calling C/C++ code can't do anything useful with the
    /// details.
    Failed,
}

impl Display for SystemRequestError {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        write!(
            f,
            "{}",
            match self {
                SystemRequestError::NotImplemented => "not implemented by this driver",
                SystemRequestError::NotAllowed => "not allowed by this driver",
                SystemRequestError::Failed => "execution of the request failed",
            }
        )
    }
}

/// The DriverHooks trait allows engines to interact with the higher-level code
/// that is driving the TeX processing.
///
/// Drivers mainly manage interactions between the engines and the outside
/// world. The primary way they do this is by exposing an [`IoProvider`]
/// implementation.
///
/// Drivers can also implement handlers for additional events that help it track
/// the input and output access patterns of the engines. The CLI program needs
/// these to intelligently decide when to rerun the TeX engine, to choose which
/// files to actually save to disk, and to emit Makefile rules describing the
/// dependency of the outputs on the inputs. The relevant trait methods have
/// default implementations that do nothing.
pub trait DriverHooks {
    /// Get the main I/O implementations of this driver.
    fn io(&mut self) -> &mut dyn IoProvider;

    /// This function is called when an output file is closed. The "digest"
    /// argument specifies the cryptographic digest of the data that were
    /// written. Note that this function takes ownership of the name and
    /// digest.
    fn event_output_closed(
        &mut self,
        _name: String,
        _digest: DigestData,
        _status: &mut dyn StatusBackend,
    ) {
    }

    /// This function is called when an input file is closed. The "digest"
    /// argument specifies the cryptographic digest of the data that were
    /// read, if available. This digest is not always available, if the engine
    /// used seeks while reading the file. Note that this function takes
    /// ownership of the name and digest.
    fn event_input_closed(
        &mut self,
        _name: String,
        _digest: Option<DigestData>,
        _status: &mut dyn StatusBackend,
    ) {
    }

    /// The engine is requesting a “shell escape” evaluation.
    ///
    /// If the driver wishes to implement this request, it should run the
    /// specified command using the OS’s default shell. Relevant files should be
    /// available in the command's working directory, and if the command creates
    /// any files, they should be incorporated into the I/O environment. The
    /// shell-escape environment should persist across multiple invocations of
    /// this system request, because some packages run a series of commands that
    /// assume such persistence. Also note that the command text has to be
    /// evaluated through a shell, not just with `exec()`, since shell features
    /// such as redirections might be used. This is therefore a wildly insecure
    /// feature.
    ///
    /// This function can only return a limited range of error values because
    /// the C/C++ engines can't do anything useful with them. Detailed error
    /// information should be logged or stored inside the hook function.
    fn sysrq_shell_escape(
        &mut self,
        _command: &str,
        _status: &mut dyn StatusBackend,
    ) -> StdResult<(), SystemRequestError> {
        Err(SystemRequestError::NotImplemented)
    }
}

/// This type provides a minimal [`DriverHooks`] implementation.
#[derive(Clone, Debug, Default)]
pub struct MinimalDriver<T: IoProvider>(T);

impl<T: IoProvider> MinimalDriver<T> {
    /// Create a new minimal driver.
    pub fn new(io: T) -> Self {
        MinimalDriver(io)
    }
}

impl<T: IoProvider> DriverHooks for MinimalDriver<T> {
    fn io(&mut self) -> &mut dyn IoProvider {
        &mut self.0
    }
}

// Function defined in the C support code:
extern "C" {
    fn _ttbc_get_error_message() -> *const libc::c_char;
}

lazy_static::lazy_static! {
    static ref ENGINE_LOCK: Mutex<u8> = Mutex::new(0u8);
}

/// An error type indicating the the FFI code aborted.
///
/// FFI bridge callbacks should return this type, which will then be filled in
/// with error text extracted from the global FFI bridge framework.
#[derive(Debug)]
pub struct EngineAbortedError {
    message: String,
}

impl EngineAbortedError {
    /// Create an error indicating that the FFI engine aborted.
    ///
    /// Upon exit, the global bridge FFI framework will report an
    /// error of this same type, but filled in with error text extracted
    /// from the global FFI bridge framework.
    pub fn new_abort_indicator() -> Self {
        EngineAbortedError {
            message: "[failed to extract detailed error message]".to_owned(),
        }
    }

    unsafe fn new_with_details() -> Self {
        let ptr = _ttbc_get_error_message();
        let message = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        EngineAbortedError { message }
    }
}

impl Display for EngineAbortedError {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for EngineAbortedError {}

/// A mechanism for launching bridged FFI code.
pub struct CoreBridgeLauncher<'a> {
    hooks: &'a mut dyn DriverHooks,
    status: &'a mut dyn StatusBackend,
    security: SecuritySettings,
    filesystem_emulation_settings: FsEmulationSettings,
}

impl<'a> CoreBridgeLauncher<'a> {
    /// Set up a new context for launching bridged FFI code.
    ///
    /// This function uses the default security stance, which disallows all
    /// known-insecure engine features. Use [`Self::new_with_security`] to
    /// provide your own security settings that can attempt to allow the use of
    /// such features.
    pub fn new(hooks: &'a mut dyn DriverHooks, status: &'a mut dyn StatusBackend) -> Self {
        Self::new_with_security(hooks, status, SecuritySettings::default())
    }

    /// Set up a new context for launching bridged FFI code.
    pub fn new_with_security(
        hooks: &'a mut dyn DriverHooks,
        status: &'a mut dyn StatusBackend,
        security: SecuritySettings,
    ) -> Self {
        CoreBridgeLauncher {
            hooks,
            status,
            security,
            filesystem_emulation_settings: FsEmulationSettings::default(),
        }
    }

    /// While absolute paths are useful (for SyncTeX and external tools that
    /// resolve paths to TeX sources), we can disable them for reproducibility.
    pub fn with_expose_absolute_paths(&mut self, expose_absolute_paths: bool) -> &mut Self {
        self.filesystem_emulation_settings.expose_absolute_paths = expose_absolute_paths;
        self
    }

    /// Ditto for file modification timestamps. In deterministic mode, we return
    /// the configured build time (i.e. `SOURCE_DATE_EPOCH`) instead of the
    /// modification timestamp reported by the IO subsystem.
    pub fn with_mtime_override(&mut self, mtime_override: Option<i64>) -> &mut Self {
        self.filesystem_emulation_settings.mtime_override = mtime_override;
        self
    }

    /// Invoke a function to launch a bridged FFI engine with a global mutex
    /// held.
    ///
    /// This function *must* be used when invoking C code that makes use of the
    /// global core bridge state functions. Unfortunately, because our error
    /// reporting is based on setjmp/longjmp and it is Undefined Behavior to
    /// setjmp/longjmp across FFI boundaries, we cannot provide a more foolproof
    /// API.
    ///
    /// The invoked code *must* call the functions `ttbc_global_engine_enter`
    /// and `ttbc_global_engine_exit` according to the pattern described in
    /// `tectonic_bridge_core.h`. If an abort is detected, the callback function
    /// should return `Err(EngineAbortedError::new_abort_indicator())`.
    pub fn with_global_lock<F, T>(&mut self, callback: F) -> Result<T>
    where
        F: FnOnce(&mut CoreBridgeState<'_>) -> Result<T>,
    {
        let _guard = ENGINE_LOCK.lock().unwrap();
        let mut state = CoreBridgeState::new(
            self.security.clone(),
            self.hooks,
            self.status,
            self.filesystem_emulation_settings.clone(),
        );
        let result = callback(&mut state);

        if let Err(ref e) = result {
            if e.downcast_ref::<EngineAbortedError>().is_some() {
                return Err(unsafe { EngineAbortedError::new_with_details() }.into());
            }
        }

        result
    }
}

/// The CoreBridgeState structure is a handle to Rust state that can be used by
/// C/C++ engine code to perform basic I/O functions.
///
/// Code that invokes a Tectonic C/C++ engine should pass a pointer to one of
/// these state structures into the C/C++ layer. It is essential that lifetimes
/// be properly managed across the Rust/C boundary.
pub struct CoreBridgeState<'a> {
    /// The security settings for this invocation
    security: SecuritySettings,

    /// The filesystem emulation settings for this invocation.
    fs_emulation_settings: FsEmulationSettings,

    /// The driver hooks associated with this engine invocation.
    hooks: &'a mut dyn DriverHooks,

    /// The status-reporting backend associated with this engine invocation.
    status: &'a mut dyn StatusBackend,

    #[allow(clippy::vec_box)]
    input_handles: Vec<Box<InputHandle>>,

    #[allow(clippy::vec_box)]
    output_handles: Vec<Box<OutputHandle>>,

    /// A semi-hack to allow us to feed input file path information to SyncTeX.
    /// This field is updated every time a new input file is opened. The XeTeX
    /// engine queries it when opening new source input files to get the
    /// absolute filesystem path info that SyncTeX wants. This field might be
    /// None because we're still reading the primary input, or because the most
    /// recent input didn't have a filesystem path (it came from a bundle or
    /// memory or something else).
    latest_input_path: Option<PathBuf>,
}

impl<'a> CoreBridgeState<'a> {
    fn new(
        security: SecuritySettings,
        hooks: &'a mut dyn DriverHooks,
        status: &'a mut dyn StatusBackend,
        fs_emulation_settings: FsEmulationSettings,
    ) -> CoreBridgeState<'a> {
        CoreBridgeState {
            security,
            hooks,
            status,
            output_handles: Vec::new(),
            input_handles: Vec::new(),
            latest_input_path: None,
            fs_emulation_settings,
        }
    }

    fn input_open_name_format(
        &mut self,
        name: &str,
        format: FileFormat,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        let io = self.hooks.io();

        if let FileFormat::Format = format {
            match io.input_open_format(name, self.status) {
                OpenResult::NotAvailable => {}
                OpenResult::Err(e) => return OpenResult::Err(e),
                OpenResult::Ok(h) => return OpenResult::Ok((h, None)),
            }
        } else {
            match io.input_open_name_with_abspath(name, self.status) {
                OpenResult::NotAvailable => {}
                r => return r,
            }
        }

        // It wasn't available under the immediately-given name. Try adding
        // extensions. Note that we always add a new extension here, even if the
        // filename already has one. E.g., lipsum in TeXLive 2020 asks for
        // `lipsum.ltd.tex` under the name `lipsum.ltd`.

        for e in format.extensions() {
            let ext = format!("{name}.{e}");

            if let FileFormat::Format = format {
                match io.input_open_format(&ext, self.status) {
                    OpenResult::NotAvailable => {}
                    OpenResult::Err(e) => return OpenResult::Err(e),
                    OpenResult::Ok(h) => return OpenResult::Ok((h, None)),
                }
            } else {
                match io.input_open_name_with_abspath(&ext, self.status) {
                    OpenResult::NotAvailable => {}
                    r => return r,
                }
            }
        }

        OpenResult::NotAvailable
    }

    fn input_open_name_format_gz(
        &mut self,
        name: &str,
        format: FileFormat,
        is_gz: bool,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        let base = self.input_open_name_format(name, format);

        if !is_gz {
            return base;
        }

        match base {
            OpenResult::Ok((ih, path)) => {
                let origin = ih.origin();
                let dr = GzDecoder::new(ih.into_inner());

                OpenResult::Ok((InputHandle::new(name, dr, origin), path))
            }
            _ => base,
        }
    }

    fn get_file_md5(&mut self, name: &str, dest: &mut [u8]) -> bool {
        let name = normalize_tex_path(name);
        let mut hash = Md5::default();

        // We could try to be fancy and look up the file in our cache to see
        // if we've already computed is SHA256 ... and then lie and use a
        // truncated SHA256 digest as the MD5 ... but it seems like a better
        // idea to just go and read the file.

        let mut ih = match self.input_open_name_format(&name, FileFormat::Tex) {
            OpenResult::Ok((ih, _path)) => ih,
            OpenResult::NotAvailable => {
                // We could issue a warning here, but the standard LaTeX
                // "rerun check" implementations trigger it very often, which
                // gets annoying. So we'll let this particular failure mode be
                // silent.
                return true;
            }
            OpenResult::Err(e) => {
                tt_error!(self.status, "error trying to open file \"{}\" for MD5 calculation",
                          name; e);
                return true;
            }
        };

        // No canned way to stream the whole file into the digest, it seems.

        const BUF_SIZE: usize = 1024;
        let mut buf = [0u8; BUF_SIZE];
        let mut error_occurred = false;

        loop {
            let nread = match ih.read(&mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    tt_error!(self.status, "error reading file \"{}\" for MD5 calculation",
                              ih.name(); e.into());
                    error_occurred = true;
                    break;
                }
            };
            hash.update(&buf[..nread]);
        }

        // Clean up.

        let (name, digest_opt) = ih.into_name_digest();
        self.hooks.event_input_closed(name, digest_opt, self.status);

        if !error_occurred {
            let result = hash.finalize();
            dest.copy_from_slice(result.as_slice());
        }

        error_occurred
    }

    fn output_open(&mut self, name: &str, is_gz: bool) -> *mut OutputHandle {
        let io = self.hooks.io();
        let name = normalize_tex_path(name);

        let mut oh = match io.output_open_name(&name) {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return ptr::null_mut(),
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of output {} failed", name; e);
                return ptr::null_mut();
            }
        };

        if is_gz {
            let name = oh.name().to_owned();
            oh = OutputHandle::new(
                name,
                GzBuilder::new().write(oh.into_inner(), Compression::default()),
            );
        }

        self.output_handles.push(Box::new(oh));
        &mut **self.output_handles.last_mut().unwrap()
    }

    fn output_open_stdout(&mut self) -> *mut OutputHandle {
        let io = self.hooks.io();

        let oh = match io.output_open_stdout() {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return ptr::null_mut(),
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of stdout failed"; e);
                return ptr::null_mut();
            }
        };

        self.output_handles.push(Box::new(oh));
        &mut **self.output_handles.last_mut().unwrap()
    }

    fn output_write(&mut self, handle: *mut OutputHandle, buf: &[u8]) -> bool {
        let rhandle: &mut OutputHandle = unsafe { &mut *handle };
        let result = rhandle.write_all(buf);

        match result {
            Ok(_) => false,
            Err(e) => {
                tt_warning!(self.status, "write failed"; e.into());
                true
            }
        }
    }

    fn output_flush(&mut self, handle: *mut OutputHandle) -> bool {
        let rhandle: &mut OutputHandle = unsafe { &mut *handle };
        let result = rhandle.flush();

        match result {
            Ok(_) => false,
            Err(e) => {
                tt_warning!(self.status, "flush failed"; e.into());
                true
            }
        }
    }

    fn output_close(&mut self, handle: *mut OutputHandle) -> bool {
        let len = self.output_handles.len();
        let mut rv = false;

        for i in 0..len {
            let p: *const OutputHandle = &*self.output_handles[i];

            if p == handle {
                let mut oh = self.output_handles.swap_remove(i);
                if let Err(e) = oh.flush() {
                    tt_warning!(self.status, "error when closing output {}", oh.name(); e.into());
                    rv = true;
                }
                let (name, digest) = oh.into_name_digest();
                self.hooks.event_output_closed(name, digest, self.status);
                break;
            }
        }

        rv
    }

    fn input_open(&mut self, name: &str, format: FileFormat, is_gz: bool) -> *mut InputHandle {
        let name = normalize_tex_path(name);

        let (ih, path) = match self.input_open_name_format_gz(&name, format, is_gz) {
            OpenResult::Ok(tup) => tup,
            OpenResult::NotAvailable => {
                return ptr::null_mut();
            }
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of input {} failed", name; e);
                return ptr::null_mut();
            }
        };

        self.input_handles.push(Box::new(ih));
        self.latest_input_path = path;
        &mut **self.input_handles.last_mut().unwrap()
    }

    fn input_open_primary(&mut self) -> *mut InputHandle {
        let io = self.hooks.io();

        let (ih, path) = match io.input_open_primary_with_abspath(self.status) {
            OpenResult::Ok(tup) => tup,
            OpenResult::NotAvailable => {
                tt_error!(self.status, "primary input not available (?!)");
                return ptr::null_mut();
            }
            OpenResult::Err(e) => {
                tt_error!(self.status, "open of primary input failed"; e);
                return ptr::null_mut();
            }
        };

        self.input_handles.push(Box::new(ih));
        self.latest_input_path = path;
        &mut **self.input_handles.last_mut().unwrap()
    }

    fn input_get_size(&mut self, handle: *mut InputHandle) -> usize {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };

        match rhandle.get_size() {
            Ok(s) => s,
            Err(e) => {
                tt_warning!(self.status, "failed to get the size of an input"; e);
                0
            }
        }
    }

    fn input_get_mtime(&mut self, handle: *mut InputHandle) -> i64 {
        if let Some(mtime) = self.fs_emulation_settings.mtime_override {
            return mtime;
        }
        let rhandle: &mut InputHandle = unsafe { &mut *handle };

        let maybe_time = match rhandle.get_unix_mtime() {
            Ok(t) => t,
            Err(e) => {
                tt_warning!(self.status, "failed to get the modification time of an input"; e);
                Some(0)
            }
        };

        if let Some(t) = maybe_time {
            t
        } else {
            1 // Intentionally make this distinguishable from the error value 0
        }
    }

    fn input_seek(&mut self, handle: *mut InputHandle, pos: SeekFrom) -> Result<u64> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        rhandle.try_seek(pos)
    }

    fn input_read(&mut self, handle: *mut InputHandle, buf: &mut [u8]) -> Result<()> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        rhandle.read_exact(buf).map_err(Error::from)
    }

    fn input_getc(&mut self, handle: *mut InputHandle) -> Result<u8> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        rhandle.getc()
    }

    fn input_ungetc(&mut self, handle: *mut InputHandle, byte: u8) -> Result<()> {
        let rhandle: &mut InputHandle = unsafe { &mut *handle };
        rhandle.ungetc(byte)
    }

    fn input_close(&mut self, handle: *mut InputHandle) -> bool {
        let len = self.input_handles.len();

        for i in 0..len {
            let p: *const InputHandle = &*self.input_handles[i];

            if p == handle {
                let mut ih = self.input_handles.swap_remove(i);
                let mut rv = false;

                if let Err(e) = ih.scan_remainder() {
                    tt_warning!(self.status, "error closing out input {}", ih.name(); e);
                    rv = true;
                }

                let (name, digest_opt) = ih.into_name_digest();
                self.hooks.event_input_closed(name, digest_opt, self.status);
                return rv;
            }
        }

        // TODO: Handle the error better. This indicates a bug in the engine.
        tt_error!(
            self.status,
            "serious internal bug: unexpected handle in input close: {:?}",
            handle
        );

        true
    }

    fn shell_escape(&mut self, command: &str) -> bool {
        if self.security.allow_shell_escape() {
            match self.hooks.sysrq_shell_escape(command, self.status) {
                Ok(_) => false,

                Err(e) => {
                    tt_error!(
                        self.status,
                        "failed to execute the shell-escape command \"{}\": {}",
                        command,
                        e
                    );
                    true
                }
            }
        } else {
            tt_error!(
                self.status,
                "forbidden to execute shell-escape command \"{}\"",
                command
            );
            true
        }
    }
}

/// A type for storing settings about potentially insecure engine features.
///
/// This type encapsulates configuration about which potentially insecure engine
/// features are enabled. Methods that configure or instantiate engines require
/// values of this type, and values of this type can only be created through
/// centralized methods that respect standard environment variables, ensuring
/// that there is some level of uniform control over the activation of any
/// known-insecure features.
///
/// The purpose of this framework is to manage the use of engine features that
/// are known to create security risks with *untrusted* input, but that trusted
/// users may wish to use due to the extra functionalities they bring. (This is
/// why these are settings and not simply security flaws!) The primary example
/// of this is the TeX engine’s shell-escape feature.
///
/// Of course, this framework is only as good as our understanding of Tectonic’s
/// security profile. Future versions might disable or restrict different pieces
/// of functionality as new risks are discovered.
#[derive(Clone, Debug)]
pub struct SecuritySettings {
    /// While we might eventually gain finer-grained enable/disable settings,
    /// there should always be a hard "disable everything known to be risky"
    /// option that supersedes everything else.
    disable_insecures: bool,
}

/// Different high-level security stances that can be adopted when creating
/// [`SecuritySettings`].
#[derive(Clone, Debug, Default)]
pub enum SecurityStance {
    /// Ensure that all known-insecure features are disabled.
    ///
    /// Use this stance if you are processing untrusted input.
    #[default]
    DisableInsecures,

    /// Request to allow the use of known-insecure features.
    ///
    /// Use this stance if you are processing trusted input *and* there is some
    /// user-level request to use such features. The request to allow insecure
    /// features might be overridden if the environment variable
    /// `TECTONIC_UNTRUSTED_MODE` is set.
    MaybeAllowInsecures,
}

impl SecuritySettings {
    /// Create a new security configuration.
    ///
    /// The *stance* argument specifies the high-level security stance. If your
    /// program will be run by a trusted user, they should be able to control
    /// the setting through a command-line argument or something comparable.
    /// Even if there is a request to enable known-insecure features, however,
    /// such a request might be overridden by other mechanisms. In particular,
    /// if the environment variable `TECTONIC_UNTRUSTED_MODE` is set to any
    /// value, insecure features will always be disabled regardless of the
    /// user-level setting. Other mechanisms for disable known-insecure features
    /// may be added in the future.
    pub fn new(stance: SecurityStance) -> Self {
        let disable_insecures = if std::env::var_os("TECTONIC_UNTRUSTED_MODE").is_some() {
            true
        } else {
            match stance {
                SecurityStance::DisableInsecures => true,
                SecurityStance::MaybeAllowInsecures => false,
            }
        };

        SecuritySettings { disable_insecures }
    }

    /// Query whether the shell-escape TeX engine feature is allowed to be used.
    pub fn allow_shell_escape(&self) -> bool {
        !self.disable_insecures
    }

    /// Query whether we're allowed to specify extra paths to read files from.
    pub fn allow_extra_search_paths(&self) -> bool {
        !self.disable_insecures
    }
}

impl Default for SecuritySettings {
    fn default() -> Self {
        SecuritySettings::new(SecurityStance::default())
    }
}

/// A type that stores configuration knobs related to filesystem emulation.
/// These options are not security-critical, but are relevant for
/// reproducible document builds. We default to an "accurate" view of the
/// underlying IO subsystem and have options that stub the respective IO
/// functions with fake / stable values.
#[derive(Clone, Debug)]
struct FsEmulationSettings {
    /// While absolute paths are useful (for SyncTeX and external tools that
    /// resolve paths to TeX sources), we can disable them for reproducibility.
    expose_absolute_paths: bool,

    /// Ditto for file modification timestamps. In deterministic mode, we return
    /// the configured build time (i.e. `SOURCE_DATE_EPOCH`) instead of the
    /// modification timestamp reported by the IO subsystem.
    mtime_override: Option<i64>,
}

impl Default for FsEmulationSettings {
    fn default() -> Self {
        Self {
            expose_absolute_paths: true,
            mtime_override: None,
        }
    }
}

// The entry points.

/// Issue a warning.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_issue_warning(es: &mut CoreBridgeState, text: *const libc::c_char) {
    let rtext = CStr::from_ptr(text);
    tt_warning!(es.status, "{}", rtext.to_string_lossy());
}

/// Issue an error.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_issue_error(es: &mut CoreBridgeState, text: *const libc::c_char) {
    let rtext = CStr::from_ptr(text);
    tt_error!(es.status, "{}", rtext.to_string_lossy());
}

/// Calculate the MD5 digest of a Tectonic file.
///
/// # Safety
///
/// This function is unsafe because it dereferences raw pointers from C.
#[no_mangle]
pub unsafe extern "C" fn ttbc_get_file_md5(
    es: &mut CoreBridgeState,
    path: *const libc::c_char,
    digest: *mut u8,
) -> libc::c_int {
    let rpath = CStr::from_ptr(path).to_string_lossy();
    let rdest = slice::from_raw_parts_mut(digest, 16);

    libc::c_int::from(es.get_file_md5(rpath.as_ref(), rdest))
}

/// Calculate the MD5 digest of a block of binary data.
///
/// This actually doesn't rely on the state and isn't really I/O, but we also
/// have a get-file-MD5 routine so it's convenient to have this here.
///
/// # Safety
///
/// This function is unsafe because it dereferences raw pointers from C.
#[no_mangle]
pub unsafe extern "C" fn ttbc_get_data_md5(
    data: *const u8,
    len: libc::size_t,
    digest: *mut u8,
) -> libc::c_int {
    let rdata = slice::from_raw_parts(data, len);
    let rdest = slice::from_raw_parts_mut(digest, 16);

    let mut hash = Md5::default();
    hash.update(rdata);
    let result = hash.finalize();
    rdest.copy_from_slice(result.as_slice());

    0
}

/// Open a Tectonic file for output.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_output_open(
    es: &mut CoreBridgeState,
    name: *const libc::c_char,
    is_gz: libc::c_int,
) -> *mut OutputHandle {
    let rname = CStr::from_ptr(name).to_string_lossy();
    let ris_gz = is_gz != 0;

    es.output_open(&rname, ris_gz)
}

/// Open the general user output stream as a Tectonic output file.
#[no_mangle]
pub extern "C" fn ttbc_output_open_stdout(es: &mut CoreBridgeState) -> *mut OutputHandle {
    es.output_open_stdout()
}

/// Write a single character to a Tectonic output file.
#[no_mangle]
pub extern "C" fn ttbc_output_putc(
    es: &mut CoreBridgeState,
    handle: *mut OutputHandle,
    c: libc::c_int,
) -> libc::c_int {
    let rc = c as u8;

    if es.output_write(handle, &[rc]) {
        libc::EOF
    } else {
        c
    }
}

/// Write data to a Tectonic output file.
///
/// # Safety
///
/// This function is unsafe because it dereferences raw C pointers.
#[no_mangle]
pub unsafe extern "C" fn ttbc_output_write(
    es: &mut CoreBridgeState,
    handle: *mut OutputHandle,
    data: *const u8,
    len: libc::size_t,
) -> libc::size_t {
    let rdata = slice::from_raw_parts(data, len);

    // NOTE: we use f.write_all() so partial writes are not gonna be a thing.

    if es.output_write(handle, rdata) {
        0
    } else {
        len
    }
}

/// Flush pending writes to a Tectonic output file.
#[no_mangle]
pub extern "C" fn ttbc_output_flush(
    es: &mut CoreBridgeState,
    handle: *mut OutputHandle,
) -> libc::c_int {
    libc::c_int::from(es.output_flush(handle))
}

/// Close a Tectonic output file.
#[no_mangle]
pub extern "C" fn ttbc_output_close(
    es: &mut CoreBridgeState,
    handle: *mut OutputHandle,
) -> libc::c_int {
    if handle.is_null() {
        return 0; // This is/was the behavior of close_file() in C.
    }

    libc::c_int::from(es.output_close(handle))
}

/// Open a Tectonic file for input.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_input_open(
    es: &mut CoreBridgeState,
    name: *const libc::c_char,
    format: FileFormat,
    is_gz: libc::c_int,
) -> *mut InputHandle {
    let rname = CStr::from_ptr(name).to_string_lossy();
    let ris_gz = is_gz != 0;
    es.input_open(&rname, format, ris_gz)
}

/// Open the "primary input" file.
#[no_mangle]
pub extern "C" fn ttbc_input_open_primary(es: &mut CoreBridgeState) -> *mut InputHandle {
    es.input_open_primary()
}

/// Get the filesystem path of the most-recently-opened input file.
///
/// This function is needed by SyncTeX, because its output file should contain
/// absolute filesystem paths to the input source files. In principle this
/// functionality could be implemented in a few different ways, but the approach
/// used here is the most backward-compatible. This function will fill in the
/// caller's buffer with the filesystem path associated with the most
/// recently-opened input file, including a terminating NUL, if possible.
///
/// It returns 0 if no such path is known, -1 if the path cannot be expressed
/// UTF-8, -2 if the destination buffer is not big enough, or the number of
/// bytes written into the buffer (including a terminating NUL) otherwise.
///
/// # Safety
///
/// This function is unsafe because it dereferences raw C pointers.
#[no_mangle]
pub unsafe extern "C" fn ttbc_get_last_input_abspath(
    es: &mut CoreBridgeState,
    buffer: *mut u8,
    len: libc::size_t,
) -> libc::ssize_t {
    if !es.fs_emulation_settings.expose_absolute_paths {
        return 0;
    }
    match es.latest_input_path {
        None => 0,

        Some(ref p) => {
            // In principle we could try to handle the full fun of
            // cross-platform PathBuf/Unicode conversions, but synctex and
            // friends will be treating our data as a traditional C string in
            // the end. So play it safe and stick to UTF-8.
            let p = match p.to_str() {
                Some(s) => s.as_bytes(),
                None => return -1,
            };

            let n = p.len();
            if n + 1 > len {
                return -2;
            }

            std::ptr::copy(p.as_ptr(), buffer, n);
            *buffer.offset(n.try_into().unwrap()) = b'\0';
            (n + 1).try_into().unwrap()
        }
    }
}

/// Get the size of a Tectonic input file.
#[no_mangle]
pub extern "C" fn ttbc_input_get_size(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
) -> libc::size_t {
    es.input_get_size(handle)
}

/// Get the modification time of a Tectonic input file.
#[no_mangle]
pub extern "C" fn ttbc_input_get_mtime(es: &mut CoreBridgeState, handle: *mut InputHandle) -> i64 {
    es.input_get_mtime(handle)
}

/// Seek in a Tectonic input stream.
///
/// # Safety
///
/// This function is unsafe because it dereferences raw pointers from C.
#[no_mangle]
pub unsafe extern "C" fn ttbc_input_seek(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
    offset: libc::ssize_t,
    whence: libc::c_int,
    internal_error: *mut libc::c_int,
) -> libc::size_t {
    let rwhence = match whence {
        libc::SEEK_SET => SeekFrom::Start(offset as u64),
        libc::SEEK_CUR => SeekFrom::Current(offset as i64),
        libc::SEEK_END => SeekFrom::End(offset as i64),
        _ => {
            tt_error!(
                es.status,
                "serious internal bug: unexpected \"whence\" parameter to fseek() wrapper: {}",
                whence
            );
            *internal_error = 1;
            return 0;
        }
    };

    match es.input_seek(handle, rwhence) {
        Ok(pos) => pos as libc::size_t,
        Err(e) => {
            // TODO: Handle the error better. Report the error properly to the caller?
            tt_error!(es.status, "input seek failed"; e);
            0
        }
    }
}

/// Get a single character from a Tectonic input file.
#[no_mangle]
pub extern "C" fn ttbc_input_getc(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
) -> libc::c_int {
    // If we couldn't fill the whole (1-byte) buffer, that's boring old EOF.
    // No need to complain. Fun match statement here.

    match es.input_getc(handle) {
        Ok(b) => libc::c_int::from(b),
        Err(e) => {
            if let Some(ioe) = e.downcast_ref::<io::Error>() {
                if ioe.kind() == io::ErrorKind::UnexpectedEof {
                    return libc::EOF;
                }
            }

            tt_warning!(es.status, "getc failed"; e);
            -1
        }
    }
}

/// Put back a character that was obtained from a `getc` call.
#[no_mangle]
pub extern "C" fn ttbc_input_ungetc(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
    ch: libc::c_int,
) -> libc::c_int {
    match es.input_ungetc(handle, ch as u8) {
        Ok(_) => 0,
        Err(e) => {
            tt_warning!(es.status, "ungetc() failed"; e);
            -1
        }
    }
}

/// Read data from a Tectonic input handle
///
/// # Safety
///
/// This function is unsafe because it dereferences raw C pointers.
#[no_mangle]
pub unsafe extern "C" fn ttbc_input_read(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
    data: *mut u8,
    len: libc::size_t,
) -> libc::ssize_t {
    let rdata = slice::from_raw_parts_mut(data, len);

    match es.input_read(handle, rdata) {
        Ok(_) => len as isize,
        Err(e) => {
            tt_warning!(es.status, "{}-byte read failed", len; e);
            -1
        }
    }
}

/// Close a Tectonic input file.
#[no_mangle]
pub extern "C" fn ttbc_input_close(
    es: &mut CoreBridgeState,
    handle: *mut InputHandle,
) -> libc::c_int {
    if handle.is_null() {
        return 0; // This is/was the behavior of close_file() in C.
    }

    libc::c_int::from(es.input_close(handle))
}

/// A buffer for diagnostic messages. Rust code does not need to use this type.
///
/// This type has to be public so that it can be exposed in the C/C++ headers,
/// but it doesn't provide any useful functionality on the Rust side.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    message: String,
    kind: MessageKind,
}

/// Create a new diagnostic that will be reported as a warning.
#[no_mangle]
pub extern "C" fn ttbc_diag_begin_warning() -> *mut Diagnostic {
    let warning = Box::new(Diagnostic {
        message: String::new(),
        kind: MessageKind::Warning,
    });
    Box::into_raw(warning)
}

/// Create a new diagnostic that will be reported as an error.
#[no_mangle]
pub extern "C" fn ttbc_diag_begin_error() -> *mut Diagnostic {
    let warning = Box::new(Diagnostic {
        message: String::new(),
        kind: MessageKind::Error,
    });
    Box::into_raw(warning)
}

/// Append text to a diagnostic.
///
/// # Safety
///
/// This function is unsafe because it accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_diag_append(diag: &mut Diagnostic, text: *const libc::c_char) {
    let rtext = CStr::from_ptr(text);
    diag.message.push_str(&rtext.to_string_lossy());
}

/// "Finish" a diagnostic: report it to the driver and free the diagnostic object.
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw Diagnostic pointer
#[no_mangle]
pub unsafe extern "C" fn ttbc_diag_finish(es: &mut CoreBridgeState, diag: *mut Diagnostic) {
    // By creating the box, we will free the diagnostic when this function exits.
    let rdiag = Box::from_raw(diag);
    es.status
        .report(rdiag.kind, format_args!("{}", rdiag.message), None);
}

/// Run a shell command
///
/// # Safety
///
/// This function is unsafe because it dereferences raw pointers from C and accepts a raw C string.
#[no_mangle]
pub unsafe extern "C" fn ttbc_shell_escape(
    es: &mut CoreBridgeState,
    cmd: *const u16,
    len: libc::size_t,
) -> libc::c_int {
    let rcmd = slice::from_raw_parts(cmd, len);
    let rcmd = match String::from_utf16(rcmd) {
        Ok(cmd) => cmd,
        Err(err) => {
            tt_error!(es.status, "command was not valid UTF-16"; err.into());
            return -1;
        }
    };

    libc::c_int::from(es.shell_escape(&rcmd))
}

/// Different types of files that can be opened by TeX engines
///
/// This enumeration is used to guess filename extensions to try when looking
/// for a file to open.
///
/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum FileFormat {
    /// An Adobe Font Metrics file.
    Afm = 4,

    /// A BibTeX bibliography data file.
    Bib = 6,

    /// A BibTeX style file.
    Bst = 7,

    /// A character map data file.
    Cmap = 45,

    /// A configuration file.
    Cnf = 8,

    /// An encoding data file.
    Enc = 44,

    /// A TeX "format" file.
    Format = 10,

    /// A font-map file.
    FontMap = 11,

    /// A miscellaneous font file.
    MiscFonts = 41,

    /// An OFM font metrics file.
    Ofm = 20,

    /// An OpenType font file.
    OpenType = 47,

    /// An OVF file.
    Ovf = 23,

    /// An image file.
    Pict = 25,

    /// A PK font file.
    Pk = 1,

    /// A general program data file.
    ProgramData = 39,

    /// An SFD file.
    Sfd = 46,

    /// The Tectonic primary input file.
    TectonicPrimary = 59,

    /// A TeX language file.
    Tex = 26,

    /// A TeX PostScript header file.
    TexPsHeader = 30,

    /// A TeX Font Metrics file.
    Tfm = 3,

    /// A TrueType font file.
    TrueType = 36,

    /// A Type1 font file.
    Type1 = 32,

    /// A Virtual Font file.
    Vf = 33,
}

impl FileFormat {
    fn extensions(&self) -> &[&str] {
        match *self {
            FileFormat::Afm => &["afm"],
            FileFormat::Bib => &["bib"],
            FileFormat::Bst => &["bst"],
            FileFormat::Cmap => &[],
            FileFormat::Cnf => &["cnf"],
            FileFormat::Enc => &["enc"],
            FileFormat::Format => &["fmt"],
            FileFormat::FontMap => &["map"],
            FileFormat::MiscFonts => &[],
            FileFormat::Ofm => &["ofm"],
            FileFormat::OpenType => &["otf", "OTF"],
            FileFormat::Ovf => &["ovf", "vf"],
            FileFormat::Pict => &["pdf", "jpg", "eps", "epsi"],
            FileFormat::Pk => &["pk"],
            FileFormat::ProgramData => &[],
            FileFormat::Sfd => &["sfd"],
            FileFormat::TectonicPrimary => &[],
            FileFormat::Tex => &["tex", "sty", "cls", "fd", "aux", "bbl", "def", "clo", "ldf"],
            FileFormat::TexPsHeader => &["pro"],
            FileFormat::Tfm => &["tfm"],
            FileFormat::TrueType => &["ttf", "ttc", "TTF", "TTC", "dfont"],
            FileFormat::Type1 => &["pfa", "pfb"],
            FileFormat::Vf => &["vf"],
        }
    }
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
