/* Copyright 2017-2020 the Tectonic Project
   Licensed under the MIT License.
*/

/* TODO: we're migrating away from this setup. This file should go away. */

#include "tectonic_bridge_core.h"
#include "core-bindgen.h" /* declarations of these functions */
#include "xetex-xetexd.h" /* HISTORY_FATAL_ERROR */
#include "dpx-dvipdfmx.h" /* dpx_config */

int
tex_simple_main(ttbc_state_t *api, const char *dump_name, const char *input_file_name, time_t build_date)
{
    int rv;

    if (setjmp(*ttbc_global_engine_enter(api))) {
        ttbc_global_engine_exit();
        return HISTORY_FATAL_ERROR;
    }

    rv = tt_run_engine(dump_name, input_file_name, build_date);
    ttbc_global_engine_exit();
    return rv;
}


int
dvipdfmx_simple_main(ttbc_state_t *api, const XdvipdfmxConfig* config, const char *dviname, const char *pdfname, bool compress, bool deterministic_tags, time_t build_date)
{
    int rv;

    if (setjmp(*ttbc_global_engine_enter(api))) {
        ttbc_global_engine_exit();
        return 99;
    }

    dpx_config = config;
    rv = dvipdfmx_main(pdfname, dviname, NULL, 0, false, compress, deterministic_tags, false, 0, build_date);
    ttbc_global_engine_exit();
    return rv;
}
