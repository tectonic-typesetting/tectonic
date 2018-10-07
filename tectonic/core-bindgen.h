#include <stdint.h>
#include <stdlib.h>

#define DEFAULT_MAX_TEX_PASSES 6

#define FORMAT_SERIAL 28

#define MAX_HTTP_ATTEMPTS 4

#define N_BYTES 32

/**
 * During the execution of a C/C++ engine, an ExecutionState structure holds
 * all of the state relevant on the *Rust* side of things: I/O, essentially.
 * The methods on ExecutionState pretty much all work to implement for the
 * "bridge" API (C/C++ => Rust) defined below.
 */
typedef struct ExecutionState ExecutionState;

/**
 * Input handles are basically Read objects with a few extras. We don't
 * require the standard io::Seek because we need to provide a dummy
 * implementation for GZip streams, which we wouldn't be allowed to do
 * because both the trait and the target struct are outside of our crate.
 * An important role for the InputHandle struct is computing a cryptographic
 * digest of the input file. The driver uses this information in order to
 * figure out if the TeX engine needs rerunning. TeX makes our life more
 * difficult, though, since it has somewhat funky file access patterns. LaTeX
 * file opens work by opening a file and immediately closing it, which tests
 * whether the file exists, and then by opening it again for real. Under the
 * hood, XeTeX reads a couple of bytes from each file upon open to sniff its
 * encoding. So we can't just stream data from `read()` calls into the SHA2
 * computer, since we end up seeking and reading redundant data.
 * The current system maintains some internal state that, so far, helps us Do
 * The Right Thing given all this. If there's a seek on the file, we give up
 * on our digest computation. But if there's a seek back to the file
 * beginning, we are open to the possibility of restarting the computation.
 * But if nothing is ever read from the file, we once again give up on the
 * computation. The `ExecutionState` code then has further pieces that track
 * access to nonexistent files, which we treat as being equivalent to an
 * existing empty file for these purposes.
 */
typedef struct InputHandle InputHandle;

typedef struct OutputHandle OutputHandle;

typedef struct {
  ExecutionState *context;
  void (*issue_warning)(ExecutionState *ctx, const char *text);
  void (*issue_error)(ExecutionState *ctx, const char *text);
  int32_t (*get_file_md5)(ExecutionState *ctx, const char *path, char *digest);
  int32_t (*get_data_md5)(ExecutionState *ctx, const char *data, uintptr_t len, char *digest);
  OutputHandle *(*output_open)(ExecutionState *ctx, const char *path, int32_t is_gz);
  OutputHandle *(*output_open_stdout)(ExecutionState *ctx);
  int32_t (*output_putc)(ExecutionState *ctx, OutputHandle *handle, int32_t ch);
  uintptr_t (*output_write)(ExecutionState *ctx, OutputHandle *handle, const char*, uintptr_t);
  int32_t (*output_flush)(ExecutionState *ctx, OutputHandle *handle);
  int32_t (*output_close)(ExecutionState *ctx, OutputHandle *handle);
  InputHandle *(*input_open)(ExecutionState *ctx, const char *path, int32_t format, int32_t is_gz);
  InputHandle *(*input_open_primary)(ExecutionState *ctx);
  uintptr_t (*input_get_size)(ExecutionState *ctx, InputHandle *handle);
  uintptr_t (*input_seek)(ExecutionState *ctx, InputHandle *handle, intptr_t offset, int32_t whence, int32_t *internal_error);
  intptr_t (*input_read)(ExecutionState *ctx, InputHandle *handle, char *data, uintptr_t len);
  int32_t (*input_getc)(ExecutionState *ctx, InputHandle *handle);
  int32_t (*input_ungetc)(ExecutionState *ctx, InputHandle *handle, int32_t ch);
  int32_t (*input_close)(ExecutionState *ctx, InputHandle *handle);
} TectonicBridgeApi;

#define APP_INFO (app_dirs){ .name = u8"Tectonic", .author = u8"TectonicProject" }

extern int bibtex_simple_main(TectonicBridgeApi *api, const char *aux_file_name);

extern int dvipdfmx_simple_main(TectonicBridgeApi *api,
                                const char *dviname,
                                const char *pdfname,
                                bool enable_compression,
                                bool deterministic_tags);

extern int tex_simple_main(TectonicBridgeApi *api,
                           const char *dump_name,
                           const char *input_file_name);

extern const char *tt_get_error_message(void);

extern int tt_xetex_set_int_variable(const char *var_name, int value);
