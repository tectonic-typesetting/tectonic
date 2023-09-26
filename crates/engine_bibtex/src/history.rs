use std::cell::Cell;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum History {
    Spotless,
    WarningIssued(u32),
    ErrorIssued(u32),
    FatalError,
}

thread_local! {
    static HISTORY: Cell<History> = Cell::new(History::Spotless);
}

pub(crate) fn reset() {
    HISTORY.with(|cell| cell.set(History::Spotless));
}

pub(crate) fn get_history() -> History {
    HISTORY.with(|h| h.get())
}

pub(crate) fn set_history(hist: History) {
    HISTORY.with(|h| h.set(hist))
}

pub(crate) fn mark_warning() {
    match get_history() {
        History::WarningIssued(cur) => set_history(History::WarningIssued(cur + 1)),
        History::Spotless => set_history(History::WarningIssued(1)),
        _ => (),
    }
}

pub(crate) fn mark_error() {
    match get_history() {
        History::Spotless | History::WarningIssued(_) => set_history(History::ErrorIssued(1)),
        History::ErrorIssued(cur) => set_history(History::ErrorIssued(cur + 1)),
        _ => (),
    }
}

pub fn mark_fatal() {
    set_history(History::FatalError);
}
