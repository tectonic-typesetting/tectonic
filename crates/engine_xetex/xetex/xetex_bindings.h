#ifndef TECTONIC_ENGINE_XETEX_BINDGEN_H
#define TECTONIC_ENGINE_XETEX_BINDGEN_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A serial number describing the detailed binary layout of the TeX "format
 * files" used by this crate. This number will occasionally increment,
 * indicating that the format file structure has changed. There is no provision
 * for partial forwards or backwards compatibility: if the number changes, you
 * need to regenerate your format files. If you’re generating format files, you
 * should munge this serial number in the filename, or something along those
 * lines, to make sure that when the engine is updated you don’t attempt to
 * reuse old files.
 */
#define FORMAT_SERIAL 33

typedef int32_t Scaled;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern int tt_xetex_set_int_variable(const char *var_name, int value);

extern int tt_engine_xetex_main(ttbc_state_t *api,
                                const char *dump_name,
                                const char *input_file_name,
                                uint64_t build_date);

bool arith_error(void);

void set_arith_error(bool val);

Scaled tex_remainder(void);

void set_tex_remainder(Scaled val);

int32_t randoms(uintptr_t idx);

uint8_t j_random(void);

void set_j_random(uint8_t val);

int32_t tex_round(double r);

int32_t half(int32_t x);

Scaled mult_and_add(int32_t n, Scaled x, Scaled y, Scaled max_answer);

Scaled x_over_n(Scaled x, int32_t n);

Scaled xn_over_d(Scaled x, int32_t n, int32_t d);

Scaled round_xn_over_d(Scaled x, int32_t n, int32_t d);

int32_t make_frac(int32_t p, int32_t q);

int32_t take_frac(int32_t q, int32_t f);

int32_t ab_vs_cd(int32_t a, int32_t b, int32_t c, int32_t d);

void new_randoms(void);

void init_randoms(int32_t seed);

int32_t unif_rand(int32_t x);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* TECTONIC_ENGINE_XETEX_BINDGEN_H */
