/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _PDFOBJ_H_
#define _PDFOBJ_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <sys/types.h>
#include <time.h>

#include "tectonic_bridge_core.h"


/* Here is the complete list of PDF object types */

#define PDF_BOOLEAN     1
#define PDF_NUMBER      2
#define PDF_STRING      3
#define PDF_NAME        4
#define PDF_ARRAY       5
#define PDF_DICT        6
#define PDF_STREAM      7
#define PDF_NULL        8
#define PDF_INDIRECT    9
#define PDF_UNDEFINED   10

#define PDF_OBJ_INVALID 0

#define STREAM_COMPRESS (1 << 0)
#define STREAM_USE_PREDICTOR   (1 << 1)

/* A deeper object hierarchy will be considered as (illegal) loop. */
#define PDF_OBJ_MAX_DEPTH  30

typedef struct pdf_obj  pdf_obj;
typedef struct pdf_file pdf_file;

/* External interface to pdf routines */
void     pdf_obj_reset_global_state (void);
void     pdf_error_cleanup   (void);

void     pdf_out_init      (const char *filename,
                                   bool enable_encrypt, bool enable_objstm,
                                   bool enable_predictor);
void     pdf_out_flush     (void);
void     pdf_set_version   (int version);
int pdf_get_version (void);
int pdf_get_version_major (void);
int pdf_get_version_minor (void);

void     pdf_release_obj (pdf_obj *object);
int      pdf_obj_typeof  (pdf_obj *object);

#define PDF_OBJ_NUMBERTYPE(o)   ((o) && pdf_obj_typeof((o)) == PDF_NUMBER)
#define PDF_OBJ_BOOLEANTYPE(o)  ((o) && pdf_obj_typeof((o)) == PDF_BOOLEAN)
#define PDF_OBJ_STRINGTYPE(o)   ((o) && pdf_obj_typeof((o)) == PDF_STRING)
#define PDF_OBJ_NAMETYPE(o)     ((o) && pdf_obj_typeof((o)) == PDF_NAME)
#define PDF_OBJ_ARRAYTYPE(o)    ((o) && pdf_obj_typeof((o)) == PDF_ARRAY)
#define PDF_OBJ_NULLTYPE(o)     ((o) && pdf_obj_typeof((o)) == PDF_NULL)
#define PDF_OBJ_DICTTYPE(o)     ((o) && pdf_obj_typeof((o)) == PDF_DICT)
#define PDF_OBJ_STREAMTYPE(o)   ((o) && pdf_obj_typeof((o)) == PDF_STREAM)
#define PDF_OBJ_INDIRECTTYPE(o) ((o) && pdf_obj_typeof((o)) == PDF_INDIRECT)
#define PDF_OBJ_UNDEFINED(o)    ((o) && pdf_obj_typeof((o)) == PDF_UNDEFINED)

#define PDF_OBJ_TYPEOF(o)       pdf_obj_typeof((o))

pdf_obj *pdf_ref_obj        (pdf_obj *object);
pdf_obj *pdf_link_obj       (pdf_obj *object);

void     pdf_transfer_label (pdf_obj *dst, pdf_obj *src);
pdf_obj *pdf_new_undefined  (void);

pdf_obj *pdf_new_null       (void);

pdf_obj *pdf_new_boolean    (char value);
char     pdf_boolean_value  (pdf_obj *object);

pdf_obj *pdf_new_number     (double value);
void     pdf_set_number     (pdf_obj *object, double value);
double   pdf_number_value   (pdf_obj *number);

pdf_obj  *pdf_new_string    (const void *str, size_t length);
void      pdf_set_string    (pdf_obj *object, unsigned char *str, size_t length);
void     *pdf_string_value  (pdf_obj *object);
unsigned int  pdf_string_length (pdf_obj *object);

/* Name does not include the / */
pdf_obj *pdf_new_name   (const char *name);
char    *pdf_name_value (pdf_obj *object);

pdf_obj *pdf_new_array     (void);
/* pdf_add_dict requires key but pdf_add_array does not.
 * pdf_add_array always append elements to array.
 * They should be pdf_put_array(array, idx, element) and
 * pdf_put_dict(dict, key, value)
 */
void     pdf_add_array     (pdf_obj *array, pdf_obj *object);
pdf_obj *pdf_get_array     (pdf_obj *array, int idx);
unsigned int pdf_array_length  (pdf_obj *array);

pdf_obj *pdf_new_dict    (void);
void     pdf_remove_dict (pdf_obj *dict,  const char *key);
void     pdf_merge_dict  (pdf_obj *dict1, pdf_obj *dict2);
pdf_obj *pdf_lookup_dict (pdf_obj *dict,  const char *key);
pdf_obj *pdf_dict_keys   (pdf_obj *dict);

/* pdf_add_dict() want pdf_obj as key, however, key must always be name
 * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
 * key. This strange difference seems come from pdfdoc that first allocate
 * name objects frequently used (maybe 1000 times) such as /Type and does
 * pdf_link_obj() it rather than allocate/free-ing them each time. But I
 * already removed that.
 */
int      pdf_add_dict     (pdf_obj *dict, pdf_obj *key,    pdf_obj *value);

/* Apply proc(key, value, pdata) for each key-value pairs in dict, stop if proc()
 * returned non-zero value (and that value is returned). PDF object is passed for
 * key to allow modification (fix) of key.
 */
int      pdf_foreach_dict (pdf_obj *dict,
                                  int (*proc) (pdf_obj *, pdf_obj *, void *),
                                  void *pdata);

pdf_obj    *pdf_new_stream        (int flags);
void        pdf_add_stream        (pdf_obj *stream,
                                          const void *stream_data_ptr,
                                          int stream_data_len);
int         pdf_concat_stream     (pdf_obj *dst, pdf_obj *src);
pdf_obj    *pdf_stream_dict       (pdf_obj *stream);
int         pdf_stream_length     (pdf_obj *stream);
const void *pdf_stream_dataptr    (pdf_obj *stream);
void        pdf_stream_set_predictor (pdf_obj *stream,
                                             int predictor, int32_t columns,
                                             int bpc, int colors);

/* Compare label of two indirect reference object.
 */
int         pdf_compare_reference (pdf_obj *ref1, pdf_obj *ref2);

/* The following routines are not appropriate for pdfobj.
 */

void      pdf_set_compression (int level);

void      pdf_set_info     (pdf_obj *obj);
void      pdf_set_root     (pdf_obj *obj);
void      pdf_set_id       (pdf_obj *id);
void      pdf_set_encrypt  (pdf_obj *encrypt);

void      pdf_files_init    (void);
void      pdf_files_close   (void);
int       check_for_pdf     (rust_input_handle_t handle);
pdf_file *pdf_open          (const char *ident, rust_input_handle_t handle);
void      pdf_close         (pdf_file *pf);
pdf_obj  *pdf_file_get_trailer (pdf_file *pf);
pdf_obj  *pdf_file_get_catalog (pdf_file *pf);
int  pdf_file_get_version (pdf_file *pf);

pdf_obj *pdf_deref_obj     (pdf_obj *object);
pdf_obj *pdf_import_object (pdf_obj *object);

size_t pdfobj_escape_str (char *buffer, size_t size, const unsigned char *s, size_t len);

pdf_obj *pdf_new_indirect  (pdf_file *pf, unsigned label, unsigned short generation);

int pdf_check_version (int major, int minor);

#endif  /* _PDFOBJ_H_ */
