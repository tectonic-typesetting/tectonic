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

#define NULL_CS 2228225

#define PRIM_SIZE 2100

#define UNDEFINED_CONTROL_SEQUENCE 2254339

#define FROZEN_NULL_FONT 2245338

#define DIMEN_VAL_LIMIT 128

#define TEXT_SIZE 0

#define SCRIPT_SIZE 256

#define SCRIPT_SCRIPT_SIZE 512

#define WHATSIT_NODE 8

#define NATIVE_WORD_NODE 40

#define EQTB_SIZE 8941458

#define ACTIVE_BASE 1

#define SINGLE_BASE 1114113

#define PRIM_EQTB_BASE 2243238

#define CAT_CODE_BASE 2256169

#define INT_BASE 7826729

#define INT_PARS 83

#define HASH_OFFSET 514

#define HASH_BASE 2228226

#define MAX_PRINT_LINE 79

#define BIGGEST_CHAR 65535

#define BIGGEST_USV 1114111

#define TOO_BIG_CHAR 65536

typedef int32_t StrNumber;

#if defined(WORDS_BIGENDIAN)
typedef struct {
  int32_t s1;
  int32_t s0;
} B32x2;
#endif

#if !defined(WORDS_BIGENDIAN)
typedef struct {
  int32_t s0;
  int32_t s1;
} B32x2;
#endif

#if defined(WORDS_BIGENDIAN)
typedef struct {
  uint16_t s3;
  uint16_t s2;
  uint16_t s1;
  uint16_t s0;
} B16x4;
#endif

#if !defined(WORDS_BIGENDIAN)
typedef struct {
  uint16_t s0;
  uint16_t s1;
  uint16_t s2;
  uint16_t s3;
} B16x4;
#endif

typedef union {
  B32x2 b32;
  B16x4 b16;
  double gr;
  void *ptr;
} MemoryWord;

typedef int32_t Scaled;

#define EMPTY_STRING (65536 + 1)

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern int tt_xetex_set_int_variable(const char *var_name, int value);

extern int tt_engine_xetex_main(ttbc_state_t *api,
                                const char *dump_name,
                                const char *input_file_name,
                                uint64_t build_date);

uint32_t selector(void);

void set_selector(uint32_t val);

int32_t tally(void);

void set_tally(int32_t val);

int32_t error_line(void);

void set_error_line(int32_t val);

int32_t trick_count(void);

void set_trick_count(int32_t val);

uint16_t trick_buf(uintptr_t idx);

void set_trick_buf(uintptr_t idx, uint16_t val);

int32_t eqtb_top(void);

void set_eqtb_top(int32_t val);

uintptr_t name_length(void);

const char *name_of_file(void);

void set_name_of_file(const char *val);

StrNumber cur_name(void);

void set_cur_name(StrNumber val);

StrNumber cur_area(void);

void set_cur_area(StrNumber val);

StrNumber cur_ext(void);

void set_cur_ext(StrNumber val);

StrNumber job_name(void);

void set_job_name(StrNumber val);

MemoryWord eqtb(uintptr_t idx);

void set_eqtb(uintptr_t idx, MemoryWord val);

MemoryWord *eqtb_ptr(uintptr_t idx);

void resize_eqtb(uintptr_t len);

void clear_eqtb(void);

MemoryWord mem(uintptr_t idx);

void set_mem(uintptr_t idx, MemoryWord val);

MemoryWord *mem_ptr(uintptr_t idx);

void resize_mem(uintptr_t len);

void clear_mem(void);

B32x2 prim(uintptr_t idx);

void set_prim(uintptr_t idx, B32x2 val);

B32x2 *prim_ptr(uintptr_t idx);

StrNumber maketexstring(const char *str);

char *gettexstring(StrNumber s);

void pack_file_name(StrNumber n, StrNumber a, StrNumber e);

void resize_hash(uintptr_t len);

B32x2 hash(uintptr_t idx);

void set_hash(uintptr_t idx, B32x2 val);

B32x2 *hash_ptr(uintptr_t idx);

void clear_hash(void);

int32_t hash_used(void);

void set_hash_used(int32_t val);

int32_t hash_extra(void);

void set_hash_extra(int32_t val);

int32_t hash_top(void);

void set_hash_top(int32_t val);

int32_t in_open(void);

void set_in_open(int32_t val);

StrNumber full_source_filename_stack(uintptr_t idx);

void set_full_source_filename_stack(uintptr_t idx, StrNumber val);

void clear_full_source_filename_stack(void);

int32_t line(void);

void set_line(int32_t val);

int32_t line_stack(uintptr_t idx);

void set_line_stack(uintptr_t idx, int32_t val);

void clear_line_stack(void);

int32_t file_line_error_style_p(void);

void set_file_line_error_style_p(int32_t val);

ttbc_diagnostic_t *current_diagnostic(void);

int32_t term_offset(void);

void set_term_offset(int32_t val);

int32_t file_offset(void);

void set_file_offset(int32_t val);

Option_OutputId rust_stdout(void);

void set_rust_stdout(Option_OutputId val);

Option_OutputId log_file(void);

void set_log_file(Option_OutputId val);

Option_OutputId write_file(uintptr_t idx);

void set_write_file(uintptr_t idx, Option_OutputId val);

bool doing_special(void);

void set_doing_special(bool val);

uint8_t dig(uintptr_t idx);

void set_dig(uintptr_t idx, uint8_t val);

/**
 * A lower-level API to begin or end the capture of messages into the diagnostic
 * buffer. You can start capture by obtaining a diagnostic_t and passing it to
 * this function -- however, the other functions in this API generally do this
 * for you. Complete capture by passing NULL. Either way, if a capture is in
 * progress when this function is called, it will be completed and reported.
 */
void capture_to_diagnostic(ttbc_diagnostic_t *diagnostic);

void diagnostic_print_file_line(ttbc_diagnostic_t *diagnostic);

/**
 * Duplicate messages printed to log/terminal into a warning diagnostic buffer,
 * until a call capture_to_diagnostic(0). A standard usage of this is
 * ```c
 * ttbc_diagnostic_t *warning = diagnostic_begin_capture_warning_here();
 *
 * // ... XeTeX prints some errors using print_* functions ...
 *
 * capture_to_diagnostic(NULL);
 * ```
 *
 * The current file and line number information are prefixed to the captured
 * output.
 *
 * NOTE: the only reason there isn't also an _error_ version of this function is
 * that we haven't yet wired up anything that uses it.
 */
ttbc_diagnostic_t *diagnostic_begin_capture_warning_here(void);

/**
 * A replacement for xetex print_file_line+print_nl_ctr blocks. e.g. Replace
 *
 * ```c
 * if (file_line_error_style_p)
 *     print_file_line();
 * else
 *     print_nl_cstr("! ");
 * print_cstr("Cannot use ");
 * ```
 * with
 * ```c
 * ttbc_diagnostic_t *errmsg = error_here_with_diagnostic("Cannot use ");
 * ```
 *
 * This function calls `capture_to_diagnostic(errmsg)` to begin diagnostic
 * capture. You must call `capture_to_diagnostic(NULL)` to mark the capture as
 * complete.
 */
ttbc_diagnostic_t *error_here_with_diagnostic(const char *msg);

void warn_char(int c);

void print_ln(void);

void print_raw_char(uint16_t s, uint8_t offset);

void print_char(int32_t s);

void print_cstr(const char *str);

void print_nl_cstr(const char *str);

void print_esc_cstr(const char *str);

void print(StrNumber str);

void print_nl(StrNumber str);

void print_esc(StrNumber str);

void print_the_digs(uint8_t k);

void print_int(int32_t n);

void print_file_line(void);

void print_cs(int32_t p);

void sprint_cs(int32_t p);

void print_file_name(int32_t n, int32_t a, int32_t e);

void print_size(int32_t s);

void print_write_whatsit(const char *s, int32_t p);

void print_native_word(int32_t p);

void print_sa_num(int32_t q);

void print_two(int32_t n);

void print_hex(int32_t n);

void print_scaled(Scaled s);

void print_ucs_code(uint32_t n);

void print_current_string(void);

void print_roman_int(int32_t n);

void resize_str_pool(uintptr_t size);

void clear_str_pool(void);

uint16_t str_pool(uintptr_t idx);

uint16_t *str_pool_ptr(uintptr_t idx);

void set_str_pool(uintptr_t idx, uint16_t val);

uint32_t str_start(uintptr_t idx);

uint32_t *str_start_ptr(uintptr_t idx);

void resize_str_start(uintptr_t size);

void clear_str_start(void);

void set_str_start(uintptr_t idx, uint32_t val);

uintptr_t pool_ptr(void);

void set_pool_ptr(uintptr_t val);

uintptr_t str_ptr(void);

void set_str_ptr(uintptr_t val);

uintptr_t pool_size(void);

void set_pool_size(uintptr_t val);

uintptr_t max_strings(void);

void set_max_strings(uintptr_t val);

StrNumber make_string(void);

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

int32_t norm_rand(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* TECTONIC_ENGINE_XETEX_BINDGEN_H */
