#ifndef _T1_CSTR_H_
#define _T1_CSTR_H_

#include "cff_types.h"

typedef struct {
  int use_seac;
  double wx, wy;
  struct {
    double llx, lly, urx, ury;
  } bbox;
  struct {
    double asb, adx, ady;
    card8 bchar, achar;
  } seac;
} t1_ginfo;

extern int  t1char_get_metrics (card8 *src, int srclen,
				cff_index *subrs, t1_ginfo *ginfo);
extern int  t1char_convert_charstring (card8 *dst, int dstlen,
				       card8 *src, int srclen,
				       cff_index *subrs,
				       double default_width, double nominal_width,
				       t1_ginfo *ginfo);

#endif /* _T1_CSTR_H_ */
