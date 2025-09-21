use crate::c_api::engine::{rs_open_log_file, InteractionMode, Selector};
use crate::c_api::globals::Globals;
use crate::c_api::output::rs_error_here_with_diagnostic;

pub fn rs_pre_error_message(globals: &mut Globals<'_, '_>) {
    if globals.engine.log_opened {
        globals.engine.selector = Selector::TermAndLog;
    } else {
        globals.engine.selector = Selector::TermOnly;
    }

    if globals.engine.job_name == 0 {
        rs_open_log_file(globals);
    }

    if globals.engine.interaction == InteractionMode::Batch {
        globals.engine.selector = match globals.engine.selector {
            Selector::TermAndLog => Selector::LogOnly,
            Selector::TermOnly => Selector::NoPrint,
            _ => panic!(),
        }
    }

    rs_error_here_with_diagnostic(globals, b"");
}

#[no_mangle]
extern "C" fn pre_error_message() {
    Globals::with(|globals| rs_pre_error_message(globals))
}
