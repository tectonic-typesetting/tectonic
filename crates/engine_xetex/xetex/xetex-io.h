/* tectonic/xetex-io.h: XeTeX-specific low-level I/O routines
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_XETEX_IO_H
#define TECTONIC_XETEX_IO_H

#include "tectonic_bridge_core.h"


typedef struct {
    rust_input_handle_t handle;
    long savedChar;
    short skipNextLF;
    short encodingMode;
    void *conversionData;
} UFILE;


BEGIN_EXTERN_C

extern char *name_of_input_file;
extern const uint32_t offsetsFromUTF8[6];
extern const uint8_t bytesFromUTF8[256];
extern const uint8_t firstByteMark[7];

rust_input_handle_t tt_xetex_open_input(int filefmt);
void set_input_file_encoding(UFILE *f, int32_t mode, int32_t encodingData);
void u_close(UFILE *f);
int u_open_in(UFILE **f, int32_t filefmt, const char* fopen_mode, int32_t mode, int32_t encodingData);
int get_uni_c(UFILE* f);
int input_line(UFILE* f);
void make_utf16_name(void);

END_EXTERN_C

#endif /* not TECTONIC_XETEX_IO_H */
