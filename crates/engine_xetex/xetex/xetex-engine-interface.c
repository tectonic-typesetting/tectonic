/* engine-interface.c: programmatic interface to control the engine behavior
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex-ext.h"

#include <string.h>

int tt_xetex_set_int_variable (const char *var_name, int value);
int tt_xetex_set_string_variable (const char *var_name, const char *value);
int tt_engine_xetex_main(
    ttbc_state_t *api,
    const char *dump_name,
    const char *input_file_name,
    uint64_t build_date
);

/* These functions aren't used within the C/C++ library, but are called
 * by the Rust code to configure the XeTeX engine before launching it. */

int
tt_xetex_set_int_variable (const char *var_name, int value)
{
    if (streq_ptr(var_name, "halt_on_error_p"))
        halt_on_error_p = value;
    else if (streq_ptr(var_name, "in_initex_mode"))
        in_initex_mode = (value != 0);
    else if (streq_ptr(var_name, "synctex_enabled"))
        synctex_enabled = (value != 0);
    else if (streq_ptr(var_name, "semantic_pagination_enabled"))
        semantic_pagination_enabled = (value != 0);
    else if (streq_ptr(var_name, "shell_escape_enabled"))
        shell_escape_enabled = (value != 0);
    else
        return 1; /* Uh oh: unrecognized variable */

    return 0; /* success */
}


int
tt_xetex_set_string_variable (const char *var_name, const char *value)
{
    /* Currently unused; see Git history for how we used to set output_comment */
    return 1;
}

int
tt_engine_xetex_main(
    ttbc_state_t *api,
    const char *dump_name,
    const char *input_file_name,
    uint64_t build_date
) {
    int rv;

    if (setjmp(*ttbc_global_engine_enter(api))) {
        ttbc_global_engine_exit();
        return HISTORY_FATAL_ERROR;
    }

    /* See ttstub_input_get_mtime() in tectonic_bridge_core about bridging time_t
    * over FFI. */
    rv = tt_run_engine(dump_name, input_file_name, (time_t) build_date);
    ttbc_global_engine_exit();
    return rv;
}


/* "What is happening here?", you might ask. Good question!
 *
 * My first attempt (PKGW, 2023 Sep) to build a static version of Tectonic
 * for the aarch64 platform using Tectonic's cross-compilation framework
 * ran into the following error when trying to link the final executable:
 *
 *   /home/rust/sysroot-aarch64/usr/lib/libc.a(sigsetjmp.lo): in function `sigsetjmp':
 *   /home/buildozer/aports/main/musl/src/v1.2.3/src/signal/aarch64/sigsetjmp.s:7:(.text+0x0):
 *     relocation truncated to fit: R_AARCH64_CONDBR19 against symbol `setjmp'
 *     defined in .text section in /home/rust/sysroot-aarch64/usr/lib/libc.a(setjmp.lo)
 *
 * So, musl libc's implementation of sigsetjmp() invokes setjmp() in some
 * hand-written assembly. It appears that for whatever reason, when the linker
 * is trying to build the Tectonic executable on aarch64, it ends up wanting to
 * locate setjmp() and sigsetjmp() far away from each other in the final file,
 * and the particular branch instruction used in sigsetjmp() can only specify a
 * relatively small offset that cannot capture the location of setjmp(). The musl
 * developers tentatively agree that this seems to be a bug in musl's
 * implementation.
 *
 * I had the idea that maybe if I referenced both functions in my code, that
 * would encourage the linker to place them closer to each other. And, guess
 * what, it seems to work! Bananas!
 *
 * This hardly costs us anything so we don't bother to try to #ifdef it for the
 * specific circumstances given above, but Windows doesn't provide sigsetjmp.
 */
#ifndef _WIN32
void  __terrible_aarch64_musl_linker_hack_never_call_me(void);

void
__terrible_aarch64_musl_linker_hack_never_call_me(void)
{
    jmp_buf buf1;
    sigjmp_buf buf2;

    setjmp(buf1);
    sigsetjmp(buf2, 0);
}
#endif
