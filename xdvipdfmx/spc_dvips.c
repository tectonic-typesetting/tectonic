/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.
    
    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.
    
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    
    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <string.h>

#include "system.h"
#include "mem.h"
#include "error.h"

#include "dpxfile.h"

#include "dvi.h"
#include "dvicodes.h"

#include "pdfparse.h"

#include "pdfdoc.h"

#include "mpost.h"

#include "pdfximage.h"
#include "pdfdraw.h"
#include "pdfcolor.h"
#include "pdfdev.h"

#include "specials.h"
#include "spc_util.h"
#include "spc_dvips.h"

#include "mfileio.h"
#include "spc_xtx.h"
#include "epdf.h"

static int    block_pending = 0;
static double pending_x     = 0.0;
static double pending_y     = 0.0;
static int    position_set  = 0;

static char** ps_headers = 0;
static int num_ps_headers = 0;

static int
spc_handler_ps_header (struct spc_env *spe, struct spc_arg *args)
{
  char *ps_header, *pro;

  skip_white(&args->curptr, args->endptr);
  if (args->curptr + 1 >= args->endptr ||
      args->curptr[0] != '=') {
    spc_warn(spe, "No filename specified for PSfile special.");
    return  -1;
  }
  args->curptr++;

  pro = malloc(args->endptr - args->curptr + 1);
  strncpy(pro, args->curptr, args->endptr - args->curptr);
  pro[args->endptr - args->curptr] = 0;
  ps_header = kpse_find_file(pro, kpse_tex_ps_header_format, 0);
  if (!ps_header) {
    spc_warn(spe, "PS header %s not found.", pro);
    return -1;
  }
  free(pro);

  if (!(num_ps_headers & 0x0f))
    ps_headers = realloc(ps_headers, sizeof(char*) * (num_ps_headers + 16));
  ps_headers[num_ps_headers++] = ps_header;
  args->curptr = args->endptr;
  return 0;
}

static char *
parse_filename (const char **pp, const char *endptr)
{
  char  *r;
  const char *q = NULL, *p = *pp;
  char   qchar;
  int    n;

  if (!p || p >= endptr)
    return  NULL;
  else if (*p == '\"' || *p == '\'')
    qchar = *p++;
  else {
    qchar = ' ';
  }
  for (n = 0, q = p; p < endptr && *p != qchar; n++, p++);
  if (qchar != ' ') {
    if (*p != qchar)
      return  NULL;
    p++;
  }
  if (!q || n == 0)
    return  NULL;

#if  0
  {
    int  i;
    for (i = 0; i < n && isprint(q[i]); i++);
    if (i != n) {
      WARN("Non printable char in filename string...");
    }
  }
#endif

  r = NEW(n + 1, char);
  memcpy(r, q, n); r[n] = '\0';

  *pp = p;
  return  r;
}

/* =filename ... */
static int
spc_handler_ps_file (struct spc_env *spe, struct spc_arg *args)
{
  int            form_id;
  char          *filename;
  transform_info ti;
  load_options   options = {1, 0, NULL};

  ASSERT(spe && args);

  skip_white(&args->curptr, args->endptr);
  if (args->curptr + 1 >= args->endptr ||
      args->curptr[0] != '=') {
    spc_warn(spe, "No filename specified for PSfile special.");
    return  -1;
  }
  args->curptr++;

  filename = parse_filename(&args->curptr, args->endptr);
  if (!filename) {
    spc_warn(spe, "No filename specified for PSfile special.");
    return  -1;
  }

  transform_info_clear(&ti);
  if (spc_util_read_dimtrns(spe, &ti, args, 1) < 0) {
    RELEASE(filename);
    return  -1;
  }

  form_id = pdf_ximage_findresource(filename, options);
  if (form_id < 0) {
    spc_warn(spe, "Failed to read image file: %s", filename);
    RELEASE(filename);
    return  -1;
  }
  RELEASE(filename);

  pdf_dev_put_image(form_id, &ti, spe->x_user, spe->y_user);

  return  0;
}

/* This isn't correct implementation but dvipdfm supports... */
static int
spc_handler_ps_plotfile (struct spc_env *spe, struct spc_arg *args)
{
  int            error = 0;
  int            form_id;
  char          *filename;
  transform_info p;
  load_options   options = {1, 0, NULL};

  ASSERT(spe && args);

  spc_warn(spe, "\"ps: plotfile\" found (not properly implemented)");

  skip_white(&args->curptr, args->endptr);
  filename = parse_filename(&args->curptr, args->endptr);
  if (!filename) {
    spc_warn(spe, "Expecting filename but not found...");
    return -1;
  }

  form_id = pdf_ximage_findresource(filename, options);
  if (form_id < 0) {
    spc_warn(spe, "Could not open PS file: %s", filename);
    error = -1;
  } else {
    transform_info_clear(&p);
    p.matrix.d = -1.0; /* xscale = 1.0, yscale = -1.0 */
#if 0
    /* I don't know how to treat this... */
    pdf_dev_put_image(form_id, &p,
		      block_pending ? pending_x : spe->x_user,
		      block_pending ? pending_y : spe->y_user);
#endif
    pdf_dev_put_image(form_id, &p, 0, 0);
  }
  RELEASE(filename);

  return  error;
}

static int
spc_handler_ps_literal (struct spc_env *spe, struct spc_arg *args)
{
  int     error = 0;
  int     st_depth, gs_depth;
  double  x_user, y_user;

  ASSERT(spe && args && args->curptr <= args->endptr);

  if (args->curptr + strlen(":[begin]") <= args->endptr &&
      !strncmp(args->curptr, ":[begin]", strlen(":[begin]"))) {
    block_pending++;
    position_set = 1;

    x_user = pending_x = spe->x_user;
    y_user = pending_y = spe->y_user;
    args->curptr += strlen(":[begin]");
  } else if (args->curptr + strlen(":[end]") <= args->endptr &&
	     !strncmp(args->curptr, ":[end]", strlen(":[end]"))) {
    if (block_pending <= 0) {
      spc_warn(spe, "No corresponding ::[begin] found.");
      return -1;
    }
    block_pending--;

    position_set = 0;

    x_user = pending_x;
    y_user = pending_y;
    args->curptr += strlen(":[end]");
  } else if (args->curptr < args->endptr &&
	     args->curptr[0] == ':') {
    x_user = position_set ? pending_x : spe->x_user;
    y_user = position_set ? pending_y : spe->y_user;
    args->curptr++;
  } else {
    position_set = 1;
    x_user = pending_x = spe->x_user;
    y_user = pending_y = spe->y_user;
  }

  skip_white(&args->curptr, args->endptr);
  if (args->curptr < args->endptr) {

    st_depth = mps_stack_depth();
    gs_depth = pdf_dev_current_depth();

    error = mps_exec_inline(&args->curptr,
			    args->endptr,
			    x_user, y_user);
    if (error) {
      spc_warn(spe, "Interpreting PS code failed!!! Output might be broken!!!");
      pdf_dev_grestore_to(gs_depth);
    } else if (st_depth != mps_stack_depth()) {
      spc_warn(spe, "Stack not empty after execution of inline PostScript code.");
      spc_warn(spe, ">> Your macro package makes some assumption on internal behaviour of DVI drivers.");
      spc_warn(spe, ">> It may not compatible with dvipdfmx.");
    }
  }

  return  error;
}

static char *global_defs = 0;
static char *page_defs = 0;
static char *temporary_defs = 0;
static char *distiller_template = 0;
static pdf_coord *put_stack;
static int put_stack_depth = -1;
static char *gs_in = 0;

#if 0
/* Not used */
static int
spc_handler_ps_tricks_gdef (struct spc_env *spe, struct spc_arg *args)
{
  FILE* fp;

  fp = fopen(global_defs, "ab");
  fwrite(args->curptr, 1, args->endptr - args->curptr, fp);
  fprintf(fp, "\n");
  fclose(fp);

  return 0;
}
#endif

static int
spc_handler_ps_tricks_pdef (struct spc_env *spe, struct spc_arg *args)
{
  FILE* fp;
  pdf_tmatrix M, T = { 1, 0, 0, 1, 0, 0 };
  pdf_coord pt;

  pdf_dev_currentmatrix(&M);
  pdf_dev_get_fixed_point(&pt);
  T.e = pt.x;
  T.f = pt.y;
  pdf_concatmatrix(&M, &T);

  if (!page_defs)
    page_defs = dpx_create_temp_file();
  if (!page_defs) {
    WARN("Failed to create temporary input file for PSTricks image conversion.");
    return  -1;
  }

  fp = fopen(page_defs, "ab");
  fprintf(fp, "gsave initmatrix [%f %f %f %f %f %f] concat %f %f moveto\n", M.a, M.b, M.c, M.d, M.e, M.f, spe->x_user - pt.x, spe->y_user - pt.y);
  fwrite(args->curptr, 1, args->endptr - args->curptr, fp);
  fprintf(fp, "\ngrestore\n");
  fclose(fp);

  return 0;
}

static int
spc_handler_ps_tricks_tdef (struct spc_env *spe, struct spc_arg *args)
{
  FILE* fp;
  if (!temporary_defs)
    temporary_defs = dpx_create_temp_file();
  if (!temporary_defs) {
    WARN("Failed to create temporary input file for PSTricks image conversion.");
    return  -1;
  }

  fp = fopen(temporary_defs, "wb");
  fwrite(args->curptr, 1, args->endptr - args->curptr, fp);
  fprintf(fp, "\n");
  fclose(fp);

  return 0;
}

static int calculate_PS (char *string, int length, double *res1, double *res2, double *res3, double *res4, double *res5, double *res6);

static int
spc_handler_ps_tricks_bput (struct spc_env *spe, struct spc_arg *args, int must_def, int pre_def)
{
  char *PutBegin, *formula, *ncLabel;
  int label = 0;
  pdf_coord tr;
  pdf_tmatrix M, T = { 1, 0, 0, 1, 0, 0 };

  if (must_def != 0) {
    ncLabel = strstr(args->curptr, "LPut");
    if (ncLabel != 0 && ncLabel < args->endptr - 3)
      label = 1;
    ncLabel = strstr(args->curptr, "HPutPos");
    if (ncLabel != 0 && ncLabel < args->endptr - 6)
      label = 1;
  }

  if (pre_def == 0) {
    dpx_delete_temp_file(temporary_defs, true);
    temporary_defs = 0;
  }

  pdf_dev_currentmatrix(&M);
  formula = malloc(args->endptr - args->curptr + 120);
  if (label != 0) {
    sprintf(formula, "[%f %f %f %f %f %f] concat %f %f moveto\n", M.a, M.b, M.c, M.d, M.e, M.f, spe->x_user + get_origin(1), spe->y_user + get_origin(0));
  } else
    sprintf(formula, "[%f %f %f %f %f %f] concat %f %f moveto\n", M.a, M.b, M.c, M.d, M.e, M.f, spe->x_user, spe->y_user);
  strncat(formula, args->curptr, args->endptr - args->curptr);
  PutBegin = strstr(formula, "PutBegin");
  strcpy(PutBegin, "exch = =");
  *(PutBegin + 8) = 0;
  if (calculate_PS(formula, strlen(formula), &tr.x, &tr.y, 0, 0, 0, 0) == 0) {
    if (!(++put_stack_depth & 0x0f))
      put_stack = realloc(put_stack, (put_stack_depth + 16) * sizeof(pdf_coord));
    put_stack[put_stack_depth] = tr;
  }
  T.e = tr.x; T.f = tr.y;

  pdf_dev_concat(&T);

  if (must_def != 0) {
    FILE* fp;
    if (!temporary_defs)
      temporary_defs = dpx_create_temp_file();
    if (!temporary_defs) {
      WARN("Failed to create temporary input file for PSTricks image conversion.");
      return  -1;
    }

    fp  = fopen(temporary_defs, "ab");
    fprintf(fp, "gsave\n");
    if (label == 0)
      fprintf(fp, "[%f %f %f %f %f %f] concat %f %f moveto\n", M.a, M.b, M.c, M.d, M.e, M.f, spe->x_user, spe->y_user);
    fwrite(args->curptr, 1, args->endptr - args->curptr, fp);
    fprintf(fp, "\ngrestore\n");
    fclose(fp);
  }

  free(formula);
  return 0;
}

static int
spc_handler_ps_tricks_eput (struct spc_env *spe, struct spc_arg *args)
{
  pdf_coord tr = put_stack[put_stack_depth--];
  pdf_tmatrix M = { 1, 0, 0, 1, -tr.x, -tr.y };

  pdf_dev_concat(&M);

  return 0;
}

/* Rotation without gsave/grestore. */
static double* RAngles = 0;
static int RAngleCount = -1;

static int
spc_handler_ps_tricks_brotate (struct spc_env *spe, struct spc_arg *args)
{
  double value, RAngle = 0;
  char *cmd, *RotBegin;
  int i, l = args->endptr - args->curptr;

  static const char pre[] = "tx@Dict begin /RAngle { %f } def\n";
  static const char *post = "= end";

  if (!(++RAngleCount & 0x0f))
    RAngles = realloc(RAngles, (RAngleCount + 16) * sizeof(double));
  for (i = 0; i < RAngleCount; i++)
    RAngle += RAngles[i];
  cmd = calloc(l + strlen(pre) + strlen(post) + 12, 1);
  sprintf(cmd, pre, RAngle);
  strncat(cmd, args->curptr, l);
  RotBegin = strstr(cmd, "RotBegin");
  strcpy(RotBegin, post);
  if (calculate_PS(cmd, strlen(cmd), &value, 0, 0, 0, 0, 0) != 0)
    return -1;
  RAngles[RAngleCount] = value;

  return  spc_handler_xtx_do_transform (spe->x_user, spe->y_user,
      cos(value * M_PI / 180), sin(value * M_PI / 180),
      -sin(value * M_PI / 180), cos(value * M_PI / 180),
      0, 0);
}

static int
spc_handler_ps_tricks_erotate (struct spc_env *spe, struct spc_arg *args)
{
  double value = RAngles[RAngleCount--];

  return  spc_handler_xtx_do_transform (spe->x_user, spe->y_user,
      cos(value * M_PI / 180), -sin(value * M_PI / 180),
      sin(value * M_PI / 180), cos(value * M_PI / 180),
      0, 0);
}

static int
spc_handler_ps_tricks_transform (struct spc_env *spe, struct spc_arg *args)
{
  double d1, d2, d3, d4, d5, d6;
  char *cmd, *concat;
  int l = args->endptr - args->curptr;

  static const char *post = "concat matrix currentmatrix ==";

  cmd = calloc(l + 41, 1);
  strncpy(cmd, "matrix setmatrix ", 17);
  strncpy(cmd + 17, args->curptr, l);
  concat = strstr(cmd, "concat");
  if (concat != 0) {
    strcpy(concat, post);
    concat[strlen(post)] = 0;
    concat = strstr(cmd, "{");
    *concat = ' ';
    if (calculate_PS(cmd, strlen(cmd), &d1, &d2, &d3, &d4, &d5, &d6) != 0)
      return -1;
    if (spc_handler_xtx_gsave (0, 0) != 0)
      return -1;
    return spc_handler_xtx_do_transform (spe->x_user, spe->y_user, d1, d2, d3, d4, d5, d6);
  }
  return  spc_handler_xtx_grestore (0, 0);
}

static int
check_next_obj(const unsigned char * buffer)
{
  switch (buffer[0]) {
    case XXX1:
      if (buffer[1] < 5)
        return 0;
      buffer += 2;
      break;
    case XXX2:
      buffer += 3;
      break;
    case XXX3:
      buffer += 4;
      break;
    case XXX4:
      buffer += 5;
      break;
    default:
      return 0;
  }

  if (strncmp((const char*)buffer, "pst:", 4))
    return 0;
  return 1;
}

static int
spc_handler_ps_tricks_parse_path (struct spc_env *spe, struct spc_arg *args)
{
  FILE* fp;
  int k;
  pdf_tmatrix M;
  char *gs_out;
  const char *clip;
  int error;

  if (!distiller_template)
    distiller_template = get_distiller_template();

  pdf_dev_currentmatrix(&M);
  if (!gs_in) {
    gs_in = dpx_create_temp_file();
    if (!gs_in) {
      WARN("Failed to create temporary input file for PSTricks image conversion.");
      return  -1;
    }
    fp = fopen(gs_in, "wb");
    for (k = 0; k < num_ps_headers; k++)
      fprintf(fp, "(%s) run\n", ps_headers[k]);
    fprintf(fp, "[%f %f %f %f %f %f] concat %f %f translate 0 0 moveto\n", M.a, M.b, M.c, M.d, M.e, M.f, spe->x_user, spe->y_user);
    fprintf(fp, "(%s) run\n", global_defs);
    if (page_defs != 0)
      fprintf(fp, "(%s) run\n", page_defs);

#if 0
    fprintf(fp, "/clip {stroke} def\n");
    fwrite(args->curptr, 1, args->endptr - args->curptr, fp);
#else
    clip = strstr(args->curptr, " clip");
    if (clip == 0 || clip > args->endptr - 5) {
      fprintf(fp, "tx@TextPathDict begin /stroke {} def\n");
      fwrite(args->curptr, 1, args->endptr - args->curptr, fp);
      fprintf(fp, "\nend\n");
      fclose(fp);
      return 0;
    } else {
      fwrite(args->curptr, 1, clip - args->curptr, fp);
      fprintf(fp, " stroke ");
      skip_white(&clip, args->endptr);
      parse_ident(&clip, args->endptr);
      fwrite(clip, 1, args->endptr - clip, fp);
    }
#endif
  } else {
    fp = fopen(gs_in, "ab");
    fprintf(fp, "flattenpath stroke\n");
  }
  fclose(fp);

  gs_out = dpx_create_temp_file();
  if (!gs_out) {
    WARN("Failed to create temporary output file for PSTricks image conversion.");
    RELEASE(gs_in);
    gs_in = 0;
    return  -1;
  }
#ifdef MIKTEX
  {
    char *p;
    for (p = (char *)gs_in; *p; p++) {
      if (*p == '\\') *p = '/';
    }
    for (p = (char *)gs_out; *p; p++) {
      if (*p == '\\') *p = '/';
    }
  }
#endif
/*
  Ghostscript 9.15 needs showpage
*/
  fp = fopen(gs_in, "ab");
  fprintf(fp, " showpage\n");
  fclose(fp);

  error = dpx_file_apply_filter(distiller_template, gs_in, gs_out,
                               (unsigned char) pdf_get_version());
  if (error) {
    WARN("Image format conversion for PSTricks failed.");
    RELEASE(gs_in);
    gs_in = 0;
    return error;
  }

  fp = fopen(gs_out, "rb");
   if (pdf_copy_clip(fp, 1, 0, 0) != 0) {
    spc_warn(spe, "Failed to parse the clipping path.");
    RELEASE(gs_in);
    gs_in = 0;
    RELEASE(gs_out);
    return -1;
  }
  fclose(fp);

  dpx_delete_temp_file(gs_out, true);
  dpx_delete_temp_file(gs_in, true);
  gs_in = 0;

  return 0;
}

static int
spc_handler_ps_tricks_render (struct spc_env *spe, struct spc_arg *args)
{
  FILE        *fp;
  int k;
  pdf_tmatrix M;
  load_options options = {1, 0, NULL};

  if (!distiller_template)
    distiller_template = get_distiller_template();

  pdf_dev_currentmatrix(&M);
  if (!gs_in) {
    gs_in = dpx_create_temp_file();
    if (!gs_in) {
      WARN("Failed to create temporary input file for PSTricks image conversion.");
      return  -1;
    }
    fp = fopen(gs_in, "wb");
    for (k = 0; k < num_ps_headers; k++)
      fprintf(fp, "(%s) run\n", ps_headers[k]);
    fprintf(fp, "[%f %f %f %f %f %f] concat %f %f translate 0 0 moveto\n", M.a, M.b, M.c, M.d, M.e, M.f, spe->x_user, spe->y_user);
    fprintf(fp, "(%s) run\n", global_defs);
    if (page_defs != 0)
      fprintf(fp, "(%s) run\n", page_defs);
  } else
    fp = fopen(gs_in, "ab");

  fprintf(fp, "\nsave\n");
  fwrite(args->curptr, 1, args->endptr - args->curptr, fp);
  fprintf(fp, "\ncount 1 sub {pop} repeat restore\n");

  if (check_next_obj((const unsigned char*)args->endptr)) {
    fclose(fp);
  } else {
    char *gs_out;
    int error, form_id;
    transform_info p;
    transform_info_clear(&p);
    pdf_invertmatrix(&M);
    p.matrix = M;

    fclose(fp);

    gs_out = dpx_create_temp_file();
    if (!gs_out) {
      WARN("Failed to create temporary output file for PSTricks image conversion.");
      RELEASE(gs_in);
      gs_in = 0;
      return  -1;
    }
#ifdef MIKTEX
    {
      char *p;
      for (p = (char *)gs_in; *p; p++) {
        if (*p == '\\') *p = '/';
      }
      for (p = (char *)gs_out; *p; p++) {
        if (*p == '\\') *p = '/';
      }
    }
#endif
/*
    Ghostscript 9.15 needs showpage
*/
    fp = fopen(gs_in, "ab");
    fprintf(fp, " showpage\n");
    fclose(fp);

    error = dpx_file_apply_filter(distiller_template, gs_in, gs_out,
                                 (unsigned char) pdf_get_version());
    if (error) {
      WARN("Image format conversion for PSTricks failed.");
      RELEASE(gs_in);
      gs_in = 0;
      return error;
    }

    form_id = pdf_ximage_findresource(gs_out, options);
    if (form_id < 0) {
      spc_warn(spe, "Failed to read converted PSTricks image file.");
      RELEASE(gs_in);
      gs_in = 0;
      RELEASE(gs_out);
      return  -1;
    }
    pdf_dev_put_image(form_id, &p, 0, 0);

    dpx_delete_temp_file(gs_out, true);
    dpx_delete_temp_file(gs_in, true);
    gs_in = 0;
  }

  return 0;
}

typedef enum {
  render	= 1 << 0,
  global_def	= 1 << 1,
  page_def	= 1 << 2,
  new_temp	= 1 << 3,
  add_temp	= 1 << 4,
  begin_put	= 1 << 5,
  end_put	= 1 << 6,
  begin_rotate	= 1 << 7,
  end_rotate	= 1 << 8,
  parse		= 1 << 9,
  req_ref	= 1 << 10,
  transform	= 1 << 11
} Operation;

/*	ToDo: all the substring search must be centralized so that	*
 *	keys can be read from external configuration.			*/
struct pstricks_key_ {
  const char * key;
  Operation exec;
} pstricks_key[] = {
  /* The first 5 are hard-coded here. */
  {"LPut",	add_temp | req_ref},
  {"HPutPos",	add_temp | req_ref},
  {"PutBegin",	begin_put},
  {"RotBegin",	begin_rotate},
  {"clip",	parse},
  /* The rest can be read from an external source. */
  {"NewNode",	page_def | req_ref},
  {"InitNC",	render | new_temp},
  {"/Glbx",	add_temp},
  {"NewtonSolving",	add_temp},
  {"tx@LightThreeDDict",	page_def},
  {"PutEnd",	end_put},
  {"RotEnd",	end_rotate},
  {"mtrxc",	parse},
  {"stroke",	render},
  {"fill",	render},
  {"Fill",	render},
  {" Glbx", req_ref},
  {"TextPathShow", parse},
  {"/rotAngle", page_def},
  {"NAngle", req_ref},
  {"TMatrix", transform}
};

static int
spc_handler_ps_trickscmd (struct spc_env *spe, struct spc_arg *args)
{
  char *test_string;
  int k, error = 0, f_exec = 0;

  /* Hack time! */
  /* The problem is that while any macros in pstricks.tex
   * can be overridden by the codes in pstricks.con, you cannot
   * modify the pst@Verb specials generated by other PSTricks
   * packages.  So pstricks generate specials won't signal what
   * to expect for you.
   */
  test_string = malloc(args->endptr - args->curptr + 1);
  strncpy(test_string, args->curptr, args->endptr - args->curptr);
  test_string[args->endptr - args->curptr] = 0;
  for (k = 0; k < sizeof(pstricks_key) / sizeof(pstricks_key[0]); k++) {
    if (strstr(test_string, pstricks_key[k].key) != 0)
      f_exec |= pstricks_key[k].exec;
  }
  free(test_string);

  if (f_exec & new_temp)
    error |= spc_handler_ps_tricks_tdef(spe, args);
  if (f_exec & render)
    error |= spc_handler_ps_tricks_render(spe, args);
  if (f_exec & parse)
    error |= spc_handler_ps_tricks_parse_path(spe, args);
  if (f_exec & begin_put)
    error |= spc_handler_ps_tricks_bput(spe, args, (f_exec & add_temp), (f_exec & req_ref));
  if (f_exec & end_put)
    error |= spc_handler_ps_tricks_eput(spe, args);
  if (f_exec & begin_rotate)
    error |= spc_handler_ps_tricks_brotate(spe, args);
  if (f_exec & end_rotate)
    error |= spc_handler_ps_tricks_erotate(spe, args);
  if (f_exec & transform)
    error |= spc_handler_ps_tricks_transform(spe, args);
  if (f_exec & page_def)
    error |= spc_handler_ps_tricks_pdef (spe, args);
  if (f_exec == 0)
    error |= spc_handler_ps_tricks_pdef (spe, args);

  args->curptr = args->endptr;
  return error;
}

static int
spc_handler_ps_tricksobj (struct spc_env *spe, struct spc_arg *args)
{
  int error = spc_handler_ps_tricks_render(spe, args);
  args->curptr = args->endptr;
  return error;
}

static int
spc_handler_ps_default (struct spc_env *spe, struct spc_arg *args)
{
  int  error;
  int  st_depth, gs_depth;

  ASSERT(spe && args);

  pdf_dev_gsave();

  st_depth = mps_stack_depth();
  gs_depth = pdf_dev_current_depth();

  {
    pdf_tmatrix M;
    M.a = M.d = 1.0; M.b = M.c = 0.0; M.e = spe->x_user; M.f = spe->y_user;
    pdf_dev_concat(&M);
  error = mps_exec_inline(&args->curptr,
			  args->endptr,
			  spe->x_user, spe->y_user);
    M.e = -spe->x_user; M.f = -spe->y_user;
    pdf_dev_concat(&M);
  }
  if (error)
    spc_warn(spe, "Interpreting PS code failed!!! Output might be broken!!!");
  else {
    if (st_depth != mps_stack_depth()) {
      spc_warn(spe, "Stack not empty after execution of inline PostScript code.");
      spc_warn(spe, ">> Your macro package makes some assumption on internal behaviour of DVI drivers.");
      spc_warn(spe, ">> It may not compatible with dvipdfmx.");
    }
  }

  pdf_dev_grestore_to(gs_depth);
  pdf_dev_grestore();

  return  error;
}

static struct spc_handler dvips_handlers[] = {
  {"header",        spc_handler_ps_header},
  {"PSfile",        spc_handler_ps_file},
  {"psfile",        spc_handler_ps_file},
  {"ps: plotfile ", spc_handler_ps_plotfile},
  {"PS: plotfile ", spc_handler_ps_plotfile},
  {"PS:",           spc_handler_ps_literal},
  {"ps:",           spc_handler_ps_literal},
  {"PST:",          spc_handler_ps_trickscmd},
  {"pst:",          spc_handler_ps_tricksobj},
  {"\" ",           spc_handler_ps_default}
};

int
spc_dvips_at_begin_document (void)
{
  FILE* fp;

  /* This, together with \pscharpath support code, must be moved to xtex.pro header. */
  global_defs = dpx_create_temp_file();
  if (!global_defs) {
    WARN("Failed to create temporary input file for PSTricks image conversion.");
    return  -1;
  }

  fp = fopen(global_defs, "wb");
  fprintf(fp, "tx@Dict begin /STV {} def end\n");
  fclose(fp);

  return  0;
}

int
spc_dvips_at_end_document (void)
{
  if (ps_headers) {
    while (num_ps_headers > 0)
      RELEASE(ps_headers[--num_ps_headers]);
    free(ps_headers);
    ps_headers = NULL;
  }
  dpx_delete_temp_file(global_defs, true);
  dpx_delete_temp_file(page_defs, true);

  return  0;
}

int
spc_dvips_at_begin_page (void)
{
  if (page_defs) {
    dpx_delete_temp_file(page_defs, true);
    page_defs = 0;
  }

  put_stack_depth = -1;

  return  0;
}

int
spc_dvips_at_end_page (void)
{
  mps_eop_cleanup();
  if (temporary_defs) {
    dpx_delete_temp_file(temporary_defs, true);
    temporary_defs = 0;
  }
  return  0;
}

int
spc_dvips_check_special (const char *buf, int len)
{
  const char *p, *endptr;
  int   i;

  p      = buf;
  endptr = p + len;

  skip_white(&p, endptr);
  if (p >= endptr)
    return  0;

  len = (int) (endptr - p);
  for (i = 0;
       i < sizeof(dvips_handlers)/sizeof(struct spc_handler); i++) {
    if (len >= strlen(dvips_handlers[i].key) &&
        !memcmp(p, dvips_handlers[i].key,
                strlen(dvips_handlers[i].key))) {
      return  1;
    }
  }

  return  0;
}

int 
spc_dvips_setup_handler (struct spc_handler *handle,
			 struct spc_env *spe, struct spc_arg *args)
{
  const char *key;
  int   i, keylen;

  ASSERT(handle && spe && args);

  skip_white(&args->curptr, args->endptr);

  key = args->curptr;
  while (args->curptr < args->endptr &&
	 isalpha((unsigned char)args->curptr[0])) {
    args->curptr++;
  }
  /* Test for "ps:". The "ps::" special is subsumed under this case.  */
  if (args->curptr < args->endptr &&
      args->curptr[0] == ':') {
    args->curptr++;
    if (args->curptr+strlen(" plotfile ") <= args->endptr &&
	!strncmp(args->curptr, " plotfile ", strlen(" plotfile "))) {
      args->curptr += strlen(" plotfile ");
      }
  } else if (args->curptr+1 < args->endptr &&
             args->curptr[0] == '"' && args->curptr[1] == ' ') {
    args->curptr += 2;
  }

  keylen = (int) (args->curptr - key);
  if (keylen < 1) {
    spc_warn(spe, "Not ps: special???");
    return  -1;
  }

  for (i = 0;
       i < sizeof(dvips_handlers) / sizeof(struct spc_handler); i++) {
    if (keylen == strlen(dvips_handlers[i].key) &&
	!strncmp(key, dvips_handlers[i].key, keylen)) {

      skip_white(&args->curptr, args->endptr);

      args->command = dvips_handlers[i].key;

      handle->key  = "ps:";
      handle->exec = dvips_handlers[i].exec;

      return  0;
    }
  }

  return  -1;
}

#ifdef __EMX__
#define GS_CALCULATOR "gsos2 -q -dNOPAUSE -dBATCH -sDEVICE=nullpage -f "
#elif defined(WIN32)
#define GS_CALCULATOR "rungs -q -dNOPAUSE -dBATCH -sDEVICE=nullpage -f "
#else
#define GS_CALCULATOR "gs -q -dNOPAUSE -dBATCH -sDEVICE=nullpage -f "
#endif

static
int calculate_PS (char *string, int length, double *res1, double *res2, double *res3, double *res4, double *res5, double *res6) {
  char *formula, *cmd;
  FILE *fp, *coord;
  int k;

  if (res1 == 0 && res2 == 0)
    return -1;
  formula = dpx_create_temp_file();
  if (!formula) {
    WARN("Failed to create temporary input file for PSTricks image conversion.");
    return  -1;
  }

  fp = fopen(formula, "wb");
  for (k = 0; k < num_ps_headers; k++)
    fprintf(fp, "(%s) run\n", ps_headers[k]);
  fprintf(fp, "0 0 moveto\n");
  fprintf(fp, "(%s) run\n", global_defs);
  if (page_defs != 0)
    fprintf(fp, "(%s) run\n", page_defs);
  if (temporary_defs)
    fprintf(fp, "(%s) run\n", temporary_defs);
  fwrite(string, 1, length, fp);
  fclose(fp);
#ifdef MIKTEX
  {
    char *p;
    for (p = formula; *p; p++)
      if (*p == '\\')
        *p = '/';
  }
#endif
  k = strlen(GS_CALCULATOR) + strlen(formula) + 1;
  cmd = NEW(k, char);
  strcpy(cmd, GS_CALCULATOR);
  strcat(cmd, formula);

  coord = popen(cmd, "r");
  if (coord) {
    if (res1 == 0)
      fscanf(coord, " %lf ", res2);
    else if (res2 == 0)
      fscanf(coord, " %lf ", res1);
    else if (res3 == 0)
      fscanf(coord, " %lf %lf ", res1, res2);
    else
      fscanf(coord, " [%lf %lf %lf %lf %lf %lf] ", res1, res2, res3, res4, res5, res6);
  } else
    return -1;

  pclose(coord);
  RELEASE(cmd);
  dpx_delete_temp_file(formula, true);
  return 0;
}
