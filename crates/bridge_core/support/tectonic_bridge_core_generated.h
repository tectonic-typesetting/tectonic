#ifndef TECTONIC_BRIDGE_CORE_GENERATED_H
#define TECTONIC_BRIDGE_CORE_GENERATED_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct ttbc_input_handle_t ttbc_input_handle_t;
typedef struct ttbc_output_handle_t ttbc_output_handle_t;
typedef ttbc_input_handle_t *rust_input_handle_t;
typedef ttbc_output_handle_t *rust_output_handle_t;


/**
 * Different types of files that can be opened by TeX engines
 *
 * This enumeration is used to guess filename extensions to try when looking
 * for a file to open.
 *
 */
typedef enum {
  /**
   * An Adobe Font Metrics file.
   */
  TTBC_FILE_FORMAT_AFM = 4,
  /**
   * A BibTeX bibliography data file.
   */
  TTBC_FILE_FORMAT_BIB = 6,
  /**
   * A BibTeX style file.
   */
  TTBC_FILE_FORMAT_BST = 7,
  /**
   * A character map data file.
   */
  TTBC_FILE_FORMAT_CMAP = 45,
  /**
   * A configuration file.
   */
  TTBC_FILE_FORMAT_CNF = 8,
  /**
   * An encoding data file.
   */
  TTBC_FILE_FORMAT_ENC = 44,
  /**
   * A TeX "format" file.
   */
  TTBC_FILE_FORMAT_FORMAT = 10,
  /**
   * A font-map file.
   */
  TTBC_FILE_FORMAT_FONT_MAP = 11,
  /**
   * A miscellaneous font file.
   */
  TTBC_FILE_FORMAT_MISC_FONTS = 41,
  /**
   * An OFM font metrics file.
   */
  TTBC_FILE_FORMAT_OFM = 20,
  /**
   * An OpenType font file.
   */
  TTBC_FILE_FORMAT_OPEN_TYPE = 47,
  /**
   * An OVF file.
   */
  TTBC_FILE_FORMAT_OVF = 23,
  /**
   * An image file.
   */
  TTBC_FILE_FORMAT_PICT = 25,
  /**
   * A PK font file.
   */
  TTBC_FILE_FORMAT_PK = 1,
  /**
   * A general program data file.
   */
  TTBC_FILE_FORMAT_PROGRAM_DATA = 39,
  /**
   * An SFD file.
   */
  TTBC_FILE_FORMAT_SFD = 46,
  /**
   * The Tectonic primary input file.
   */
  TTBC_FILE_FORMAT_TECTONIC_PRIMARY = 59,
  /**
   * A TeX language file.
   */
  TTBC_FILE_FORMAT_TEX = 26,
  /**
   * A TeX PostScript header file.
   */
  TTBC_FILE_FORMAT_TEX_PS_HEADER = 30,
  /**
   * A TeX Font Metrics file.
   */
  TTBC_FILE_FORMAT_TFM = 3,
  /**
   * A TrueType font file.
   */
  TTBC_FILE_FORMAT_TRUE_TYPE = 36,
  /**
   * A Type1 font file.
   */
  TTBC_FILE_FORMAT_TYPE1 = 32,
  /**
   * A Virtual Font file.
   */
  TTBC_FILE_FORMAT_VF = 33,
} ttbc_file_format;

/**
 * The CoreBridgeState structure is a handle to Rust state that can be used by
 * C/C++ engine code to perform basic I/O functions.
 *
 * Code that invokes a Tectonic C/C++ engine should pass a pointer to one of
 * these state structures into the C/C++ layer. It is essential that lifetimes
 * be properly managed across the Rust/C boundary.
 */
typedef struct ttbc_state_t ttbc_state_t;

/**
 * A buffer for diagnostic messages. Rust code does not need to use this type.
 *
 * This type has to be public so that it can be exposed in the C/C++ headers,
 * but it doesn't provide any useful functionality on the Rust side.
 */
typedef struct ttbc_diagnostic_t ttbc_diagnostic_t;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const char *_ttbc_get_error_message(void);

/**
 * Issue a warning.
 *
 * # Safety
 *
 * This function is unsafe because it accepts a raw C string.
 */
void ttbc_issue_warning(ttbc_state_t *es, const char *text);

/**
 * Issue an error.
 *
 * # Safety
 *
 * This function is unsafe because it accepts a raw C string.
 */
void ttbc_issue_error(ttbc_state_t *es, const char *text);

/**
 * Calculate the MD5 digest of a Tectonic file.
 *
 * # Safety
 *
 * This function is unsafe because it dereferences raw pointers from C.
 */
int ttbc_get_file_md5(ttbc_state_t *es, const char *path, uint8_t *digest);

/**
 * Calculate the MD5 digest of a block of binary data.
 *
 * This actually doesn't rely on the state and isn't really I/O, but we also
 * have a get-file-MD5 routine so it's convenient to have this here.
 *
 * # Safety
 *
 * This function is unsafe because it dereferences raw pointers from C.
 */
int ttbc_get_data_md5(const uint8_t *data, size_t len, uint8_t *digest);

/**
 * Open a Tectonic file for output.
 *
 * # Safety
 *
 * This function is unsafe because it accepts a raw C string.
 */
ttbc_output_handle_t *ttbc_output_open(ttbc_state_t *es, const char *name, int is_gz);

/**
 * Open the general user output stream as a Tectonic output file.
 */
ttbc_output_handle_t *ttbc_output_open_stdout(ttbc_state_t *es);

/**
 * Write a single character to a Tectonic output file.
 */
int ttbc_output_putc(ttbc_state_t *es, ttbc_output_handle_t *handle, int c);

/**
 * Write data to a Tectonic output file.
 *
 * # Safety
 *
 * This function is unsafe because it dereferences raw C pointers.
 */
size_t ttbc_output_write(ttbc_state_t *es,
                         ttbc_output_handle_t *handle,
                         const uint8_t *data,
                         size_t len);

/**
 * Flush pending writes to a Tectonic output file.
 */
int ttbc_output_flush(ttbc_state_t *es, ttbc_output_handle_t *handle);

/**
 * Close a Tectonic output file.
 */
int ttbc_output_close(ttbc_state_t *es, ttbc_output_handle_t *handle);

/**
 * Open a Tectonic file for input.
 *
 * # Safety
 *
 * This function is unsafe because it accepts a raw C string.
 */
ttbc_input_handle_t *ttbc_input_open(ttbc_state_t *es,
                                     const char *name,
                                     ttbc_file_format format,
                                     int is_gz);

/**
 * Open the "primary input" file.
 */
ttbc_input_handle_t *ttbc_input_open_primary(ttbc_state_t *es);

/**
 * Get the filesystem path of the most-recently-opened input file.
 *
 * This function is needed by SyncTeX, because its output file should contain
 * absolute filesystem paths to the input source files. In principle this
 * functionality could be implemented in a few different ways, but the approach
 * used here is the most backward-compatible. This function will fill in the
 * caller's buffer with the filesystem path associated with the most
 * recently-opened input file, including a terminating NUL, if possible.
 *
 * It returns 0 if no such path is known, -1 if the path cannot be expressed
 * UTF-8, -2 if the destination buffer is not big enough, or the number of
 * bytes written into the buffer (including a terminating NUL) otherwise.
 *
 * # Safety
 *
 * This function is unsafe because it dereferences raw C pointers.
 */
ssize_t ttbc_get_last_input_abspath(ttbc_state_t *es, uint8_t *buffer, size_t len);

/**
 * Get the size of a Tectonic input file.
 */
size_t ttbc_input_get_size(ttbc_state_t *es, ttbc_input_handle_t *handle);

/**
 * Get the modification time of a Tectonic input file.
 */
int64_t ttbc_input_get_mtime(ttbc_state_t *es, ttbc_input_handle_t *handle);

/**
 * Seek in a Tectonic input stream.
 *
 * # Safety
 *
 * This function is unsafe because it dereferences raw pointers from C.
 */
size_t ttbc_input_seek(ttbc_state_t *es,
                       ttbc_input_handle_t *handle,
                       ssize_t offset,
                       int whence,
                       int *internal_error);

/**
 * Get a single character from a Tectonic input file.
 */
int ttbc_input_getc(ttbc_state_t *es, ttbc_input_handle_t *handle);

/**
 * Put back a character that was obtained from a `getc` call.
 */
int ttbc_input_ungetc(ttbc_state_t *es, ttbc_input_handle_t *handle, int ch);

/**
 * Read data from a Tectonic input handle
 *
 * # Safety
 *
 * This function is unsafe because it dereferences raw C pointers.
 */
ssize_t ttbc_input_read(ttbc_state_t *es, ttbc_input_handle_t *handle, uint8_t *data, size_t len);

/**
 * Close a Tectonic input file.
 */
int ttbc_input_close(ttbc_state_t *es, ttbc_input_handle_t *handle);

/**
 * Create a new diagnostic that will be reported as a warning.
 */
ttbc_diagnostic_t *ttbc_diag_begin_warning(void);

/**
 * Create a new diagnostic that will be reported as an error.
 */
ttbc_diagnostic_t *ttbc_diag_begin_error(void);

/**
 * Append text to a diagnostic.
 *
 * # Safety
 *
 * This function is unsafe because it accepts a raw C string.
 */
void ttbc_diag_append(ttbc_diagnostic_t *diag, const char *text);

/**
 * "Finish" a diagnostic: report it to the driver and free the diagnostic object.
 */
void ttbc_diag_finish(ttbc_state_t *es, ttbc_diagnostic_t *diag);

/**
 * Run a shell command
 *
 * # Safety
 *
 * This function is unsafe because it dereferences raw pointers from C and accepts a raw C string.
 */
int ttbc_shell_escape(ttbc_state_t *es, const uint16_t *cmd, size_t len);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* TECTONIC_BRIDGE_CORE_GENERATED_H */
