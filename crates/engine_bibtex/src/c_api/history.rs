use std::cell::Cell;

/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(C)]
pub enum History {
    Spotless = 0,
    WarningIssued = 1,
    ErrorIssued = 2,
    FatalError = 3,
    Aborted = 4,
}

thread_local! {
    static HISTORY: Cell<History> = Cell::new(History::Spotless);
    static ERR_COUNT: Cell<u32> = Cell::new(0);
}

pub(crate) fn reset() {
    HISTORY.with(|cell| cell.set(History::Spotless));
    ERR_COUNT.with(|cell| cell.set(0));
}

fn get_err() -> u32 {
    ERR_COUNT.with(|e| e.get())
}

fn set_err(f: impl FnOnce(u32) -> u32) {
    ERR_COUNT.with(|e| e.set(f(e.get())))
}

#[no_mangle]
pub extern "C" fn get_history() -> History {
    HISTORY.with(|h| h.get())
}

pub fn set_history(hist: History) {
    HISTORY.with(|h| h.set(hist))
}

#[no_mangle]
pub extern "C" fn mark_warning() {
    let history = get_history();
    if history == History::WarningIssued {
        set_err(|e| e + 1);
    } else if history == History::Spotless {
        set_history(History::WarningIssued);
        set_err(|_| 1);
    }
}

pub fn mark_error() {
    if get_history() < History::ErrorIssued {
        set_history(History::ErrorIssued);
        set_err(|_| 1);
    } else {
        set_err(|e| e + 1);
    }
}

pub fn mark_fatal() {
    set_history(History::FatalError);
}

#[no_mangle]
pub extern "C" fn err_count() -> u32 {
    get_err()
}
