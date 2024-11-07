#ifndef XDVIPDFMX_BINDINGS_H
#define XDVIPDFMX_BINDINGS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const char *paperspec;
  unsigned char enable_compression;
  unsigned char deterministic_tags;
  uint64_t build_date;
} XdvipdfmxConfig;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern int tt_engine_xdvipdfmx_main(ttbc_state_t *api,
                                    const XdvipdfmxConfig *cfg,
                                    const char *dviname,
                                    const char *pdfname);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* XDVIPDFMX_BINDINGS_H */
