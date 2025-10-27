use crate::c_api::engine::{rs_open_log_file, rs_show_context, History, InteractionMode, Selector};
use crate::c_api::globals::Globals;
use crate::c_api::output::{
    rs_capture_to_diagnostic, rs_error_here_with_diagnostic, rs_print_char, rs_print_nl_bytes,
};

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

// pub fn rs_error(globals: &mut Globals<'_, '_>) -> Box<dyn Fn()> {
//     if globals.engine.history < History::ErrorIssued {
//         globals.engine.history = History::ErrorIssued;
//     }
//
//     rs_print_char(globals, '.' as i32);
//     rs_show_context(globals);
//     if globals.engine.halt_on_error_p != 0 {
//         globals.engine.history = History::FatalError;
//         // Execute this outside the globals lock for now
//         return Box::new(|| {
//             post_error_message(0);
//             panic!("halted on potentially-recoverable error as specified");
//         });
//     }
//
//     /* This used to be where there was a bunch of code if "interaction ==
//      * error_stop_mode" that would let the use interactively try to solve the
//      * error. */
//
//     globals.engine.error_count += 1;
//     if globals.engine.error_count == 100 {
//         rs_print_nl_bytes(globals, b"(That makes 100 errors; please try again.)");
//         globals.engine.history = History::FatalError;
//         return Box::new(|| {
//             post_error_message(0);
//             panic!("halted after 100 potentially-recoverable errors");
//         })
//     }
//
//     if globals.engine.interaction != InteractionMode::Batch {
//         globals.engine.selector = match globals.engine.selector {
//             Selector::File(i) => Selector::File(i-1),
//             Selector::NoPrint => Selector::File(15),
//             Selector::TermOnly => Selector::NoPrint,
//             Selector::LogOnly => Selector::TermOnly,
//             Selector::TermAndLog => Selector::LogOnly,
//             Selector::Pseudo => Selector::TermAndLog,
//             Selector::NewString => Selector::Pseudo,
//         }
//     }
//
//     /*
//     if (use_err_help) {
//         print_ln();
//         give_err_help();
//     } else {
//         while (help_ptr > 0) {
//             help_ptr--;
//             print_nl_cstr(help_line[help_ptr]);
//         }
//     }
//
//     print_ln();
//     if (interaction() > BATCH_MODE)
//         set_selector(selector()+1);
//     print_ln();
//     */
//     Box::new(|| ())
// }
//
// #[no_mangle]
// extern "C" fn error() {
//     let out_of_lock = Globals::with(|globals| rs_error(globals));
//     out_of_lock();
// }
//
// #[no_mangle]
// extern "C" fn post_error_message(need_to_print_it: i32) {
//     let out_of_lock = Globals::with(|globals| {
//         rs_capture_to_diagnostic(globals, None);
//         if globals.engine.interaction == InteractionMode::ErrorStop {
//             globals.engine.interaction = InteractionMode::Scroll;
//         }
//
//         if need_to_print_it != 0 && globals.engine.log_opened {
//             return rs_error(globals);
//         }
//         globals.engine.history = History::FatalError;
//         Box::new(|| ())
//     });
//     out_of_lock();
//     unsafe { close_files_and_terminate() };
//     unsafe { tt_cleanup() };
//     Globals::with(|globals| {
//         globals
//             .out
//             .rust_stdout
//             .map(|stdout| globals.state.output_flush(stdout))
//     });
// }
//
// // TODO: Use the Rust versions directly once they're ported. These just rely indirectly on this
// //       function, making it easier to port piecemeal.
// extern "C" {
//     fn close_files_and_terminate();
//     fn tt_cleanup();
// }
