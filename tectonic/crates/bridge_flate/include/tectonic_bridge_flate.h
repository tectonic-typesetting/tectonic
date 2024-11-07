#ifndef TECTONIC_BRIDGE_FLATE_H
#define TECTONIC_BRIDGE_FLATE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Outcomes of (de)flate operations.
 */
typedef enum {
  /**
   * The operation succeeded.
   */
  FlateResult_Success = 0,
  /**
   * The operation succeeded and encountered the end of the input.
   */
  FlateResult_StreamEnd = 1,
  /**
   * The operation failed because a buffer was not big enough or full enough.
   */
  FlateResult_BufError = -1,
  /**
   * The operation failed due to an error other than the ones enumerated
   * here.
   */
  FlateResult_OtherError = -2,
} FlateResult;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Compress a block of data. This function maps fairly directly onto the
 * `Compress::compress` function provided by `flate2`.
 *
 * Returns nonzero on error.
 *
 * # Safety
 *
 * This is a C API function, so it is unsafe.
 */
FlateResult tectonic_flate_compress(uint8_t *output_ptr,
                                    uint64_t *output_len,
                                    const uint8_t *input_ptr,
                                    uint64_t input_len,
                                    uint32_t compression_level);

/**
 * Deompress a block of data. This function maps fairly directly onto the
 * `Decompress::decompress` function provided by `flate2`.
 *
 * Returns nonzero on error.
 *
 * # Safety
 *
 * This is a C API function, so it is unsafe.
 */
FlateResult tectonic_flate_decompress(uint8_t *output_ptr,
                                      uint64_t *output_len,
                                      const uint8_t *input_ptr,
                                      uint64_t input_len);

/**
 * Allocate a new DEFLATE decompressor.
 *
 * # Safety
 *
 * This is a C API function, so it is unsafe.
 */
void *tectonic_flate_new_decompressor(const uint8_t *input_ptr, uint64_t input_len);

/**
 * Decompress some DEFLATEd data.
 *
 * After calling this function, the `input_len` parameter is rewritten with the
 * total number of bytes of compressed data that have been read. The
 * `output_len` parameter is rewritten with the total number of bytes of
 * decompressed data that have been written.
 *
 * Returns nonzero on error.
 *
 * # Safety
 *
 * This is a C API function, so it is unsafe.
 */
int tectonic_flate_decompress_chunk(void *handle, uint8_t *output_ptr, uint64_t *output_len);

/**
 * Deallocate a DEFLATE decompressor.
 *
 * # Safety
 *
 * This is a C API function, so it is unsafe.
 */
void tectonic_flate_free_decompressor(void *handle);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* TECTONIC_BRIDGE_FLATE_H */
