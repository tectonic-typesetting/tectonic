
#ifndef _T1_LOAD_H_
#define _T1_LOAD_H_

#include "cff.h"
extern cff_font *t1_load_font (char **enc_vec, int mode, FILE *fp);
extern int   is_pfb (FILE *fp);
extern int   t1_get_fontname (FILE *fp, char *fontname);
extern const char *t1_get_standard_glyph (int code);

#endif /* _T1_LOAD_H_ */
