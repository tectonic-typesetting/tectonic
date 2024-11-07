/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2020 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include <assert.h>
#include <ctype.h>
/* floor and abs */
#include <math.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfencrypt.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfparse.h"

#include "dpx-pdfobj.h"

#include "dpx-pdfdev.h"

#include "tectonic_bridge_flate.h"

#define STREAM_ALLOC_SIZE      4096u
#define ARRAY_ALLOC_SIZE       256
#define IND_OBJECTS_ALLOC_SIZE 512

#define OBJ_NO_OBJSTM   (1 << 0)
/* Objects with this flag will not be put into an object stream.
   For instance, all stream objects have this flag set.          */
#define OBJ_NO_ENCRYPT  (1 << 1)
/* Objects with this flag will not be encrypted.
   This implies OBJ_NO_OBJSTM if encryption is turned on.        */

/* Any of these types can be represented as follows */
struct pdf_obj
{
    int type;

    uint32_t label;  /* Only used for indirect objects
                              all other "label" to zero */
    uint16_t generation;  /* Only used if "label" is used */
    int refcount;  /* Number of links to this object */
    int32_t  flags;
    void    *data;
    /* Tectonic: not including PDFOBJ_DEBUG #ifdefs */
};

struct pdf_boolean
{
    char  value;
};

struct pdf_number
{
    double value;
};

struct pdf_string
{
    unsigned char *string;
    size_t length;
};

struct pdf_name
{
    char *name;
};

struct pdf_array
{
    size_t max;
    size_t size;
    struct pdf_obj **values;
};

struct pdf_dict
{
    struct pdf_obj  *key;
    struct pdf_obj  *value;
    struct pdf_dict *next;
};

/* DecodeParms for FlateDecode */
struct decode_parms {
    int     predictor;
    int     colors;
    int     bits_per_component;
    int32_t columns;
};

/* 2015/12/27 Added support for predictor functions
 *
 * There are yet no way to specify the use of predictor functions.
 * Using TIFF2 or PNG predictor usually gives better compression for images
 * but there is a case that compression speed becomes significantly slower.
 * Please use -C 0x20 option to disable the use of predictor functions.
 *
 * See, e.g., for a heuristic approach for selecting filters
 *   http://www.w3.org/TR/PNG-Encoders.html#E.Filter-selection
 */

struct pdf_stream
{
    struct pdf_obj     *dict;
    unsigned char      *stream;
    int                *objstm_data;    /* used for object streams */
    size_t        stream_length;
    size_t        max_length;
    int32_t             _flags;
    struct decode_parms decodeparms;
};

struct pdf_indirect
{
    pdf_file      *pf;
    pdf_obj       *obj;             /* used when PF == NULL */
    uint32_t label;
    uint16_t generation;
};

typedef void                pdf_null;
typedef struct pdf_boolean  pdf_boolean;
typedef struct pdf_number   pdf_number;
typedef struct pdf_string   pdf_string;
typedef struct pdf_name     pdf_name;
typedef struct pdf_array    pdf_array;
typedef struct pdf_dict     pdf_dict;
typedef struct pdf_stream   pdf_stream;
typedef struct pdf_indirect pdf_indirect;

typedef struct xref_entry
{
    unsigned char  type;       /* object storage type              */
    unsigned int   field2;     /* offset in file or object stream  */
    unsigned short field3;     /* generation or index              */
    pdf_obj       *direct;     /* used for imported objects        */
    pdf_obj       *indirect;   /* used for imported objects        */
} xref_entry;

struct pdf_file
{
    rust_input_handle_t handle;
    pdf_obj    *trailer;
    xref_entry *xref_table;
    pdf_obj    *catalog;
    int         num_obj;
    int         file_size;
    unsigned int version;
};

/* Tectonic: no error_out; it's for debugging and breaks I/O encapsulation */

#define OBJSTM_MAX_OBJS  200
/* the limit is only 100 for linearized PDF */

struct pdf_out {
  struct {
    int         enc_mode; /* boolean */
  } state;

  unsigned char id1[16];
  unsigned char id2[16];

  struct {
    int         major;
    int         minor;
  } version;

  struct {
    struct {
      int       level;
      int       use_predictor;
    } compression;

    int         enable_encrypt;
    int         use_objstm;
  } options;

  struct {
    rust_output_handle_t handle;
    size_t      file_position;
    int         line_position;
    size_t      compression_saved;
  } output;

  struct {
    uint32_t    next_label;
    uint32_t    max_ind_objects;
  } obj;

  pdf_sec      *sec_data;

  pdf_obj      *trailer;
  uint32_t      startxref;
  xref_entry *  xref_table;

  pdf_obj      *xref_stream;
  pdf_obj      *output_stream;
  pdf_obj      *current_objstm;
  /* The following flag bits are (8,338,607+1)/8 bytes data
   * each bit represenging if the object is freed.
   * Where the value 8,338,607 is taken from PDF ref. manual, v.1.7,
   * Appendix C, "Implementation Limits".
   */
  char         *free_list;
};

/* Underway to reform PDF related code... For a moment place pdf_out
 * object as a static variable. */
static pdf_out pout;

/* Tectonic: during the XeTeX pass, if a PDF is loaded as an image,
 * the current_output can be accessed without init_pdf_out_struct having
 * been called. So we add a flag to ensure that it's always initialized. */
static int tectonic_pout_initialized = 0;
static void init_pdf_out_struct (pdf_out *p);

static pdf_out *
current_output (void)
{
  if (!tectonic_pout_initialized)
    init_pdf_out_struct(&pout);

  return &pout;
}

static void
init_pdf_out_struct (pdf_out *p)
{
  assert(p);

  p->state.enc_mode = 0;

  memset(p->id1, 0, 16);
  memset(p->id2, 0, 16);

  p->version.major  = 1;
  p->version.minor  = PDF_VERSION_DEFAULT % 10; /* Tectonic: fix this setting */

  p->options.compression.level = 9;
  p->options.compression.use_predictor = 1;
  p->options.enable_encrypt    = 0;
  p->options.use_objstm        = 1;

  p->output.handle = NULL;
  p->output.file_position = 0;
  p->output.line_position = 0;
  p->output.compression_saved = 0;

  p->obj.next_label = 1;
  p->obj.max_ind_objects = 0;

  p->sec_data   = NULL;
  p->trailer    = NULL;
  p->startxref  = 0;
  p->xref_table = NULL;

  p->xref_stream    = NULL;
  p->output_stream  = NULL;
  p->current_objstm = NULL;

  p->free_list = NEW((PDF_NUM_INDIRECT_MAX+1)/8, char);
  memset(p->free_list, 0, (PDF_NUM_INDIRECT_MAX+1)/8);
  tectonic_pout_initialized = 1;
}

static void
clean_pdf_out_struct (pdf_out *p)
{
  if (p->free_list)
    free(p->free_list);
  memset(p, 0, sizeof(pdf_out));
}

/* Internal static routines */

static int check_for_pdf_version (rust_input_handle_t handle);

static void pdf_flush_obj (pdf_out *p, pdf_obj *object);
static void pdf_label_obj (pdf_out *p, pdf_obj *object);
static void pdf_write_obj (pdf_out *p, pdf_obj *object);

static void  set_objstm_data (pdf_obj *objstm, int *data);
static int  *get_objstm_data (pdf_obj *objstm);
static void  release_objstm  (pdf_obj *objstm);

static void pdf_out_char (pdf_out *p, char c);
static void pdf_out_str  (pdf_out *p, const void *buffer, size_t length);

static pdf_obj *pdf_new_ref  (pdf_out *p, pdf_obj *object);
static void release_indirect (pdf_indirect *data);
static void write_indirect   (pdf_out *p, pdf_indirect *indirect);

static void release_boolean (pdf_obj *data);
static void write_boolean   (pdf_out *p, pdf_boolean *data);

static void write_null   (pdf_out *p);

static void release_number (pdf_number *number);
static void write_number   (pdf_out *p, pdf_number *number);

static void write_string   (pdf_out *p, pdf_string *str);
static void release_string (pdf_string *str);

static void write_name   (pdf_out *p, pdf_name *name);
static void release_name (pdf_name *name);

static void write_array   (pdf_out *p, pdf_array *array);
static void release_array (pdf_array *array);

static void write_dict   (pdf_out *p, pdf_dict *dict);
static void release_dict (pdf_dict *dict);

static void write_stream   (pdf_out *p, pdf_stream *stream);
static void release_stream (pdf_stream *stream);

static void
pdf_out_set_compression (pdf_out *p, int level)
{
    assert(p);

    if (level >= 0 && level <= 9) {
        p->options.compression.level = level;
    } else {
        _tt_abort("set_compression: invalid compression level: %d", level);
    }
}

rust_output_handle_t
pdf_get_output_file(void)
{
    pdf_out *p = current_output();
    return p->output.handle;
}

static void
pdf_out_set_version (pdf_out *p, int ver_major, int ver_minor)
{
    int version;

    assert(p);

    version = ver_major * 10 + ver_minor;

    /* Don't forget to update CIDFont_stdcc_def[] in cid.c too! */
    if (version >= PDF_VERSION_MIN && version <= PDF_VERSION_MAX) {
        p->version.major = ver_major;
        p->version.minor = ver_minor;
    } else {
        dpx_warning("Unsupported PDF version %d.%d ... Ignoring.", ver_major, ver_minor);
    }
}

int
pdf_get_version (void)
{
    pdf_out *p = current_output();
    return p->version.major * 10 + p->version.minor;
}

int
pdf_get_version_major (void)
{
    pdf_out *p = current_output();
    return p->version.major;
}

int
pdf_get_version_minor (void)
{
    pdf_out *p = current_output();
    return p->version.minor;
}

int
pdf_check_version (int major, int minor)
{
    pdf_out *p = current_output();

    if (p->version.major > major)
        return 0;
    else if (p->version.major < major)
        return -1;
    else {
        return (p->version.minor >= minor) ? 0 : -1;
    }

    return -1;
}

static void
add_xref_entry (pdf_out *p, uint32_t label, uint8_t type, uint32_t field2, uint16_t field3)
{
    assert(p);

    if (label >= p->obj.max_ind_objects) {
        p->obj.max_ind_objects = (label/IND_OBJECTS_ALLOC_SIZE+1)*IND_OBJECTS_ALLOC_SIZE;
        p->xref_table = RENEW(p->xref_table, p->obj.max_ind_objects, xref_entry);
    }

    p->xref_table[label].type   = type;
    p->xref_table[label].field2 = field2;
    p->xref_table[label].field3 = field3;
    p->xref_table[label].direct   = NULL;
    p->xref_table[label].indirect = NULL;
}

#define BINARY_MARKER "%\344\360\355\370\n"
pdf_out *
pdf_out_init (const char *filename,
              const unsigned char *id1,
              const unsigned char *id2,
              int ver_major, int ver_minor, int compression_level,
              int enable_encrypt,
              int enable_objstm,
              int enable_predictor)
{
    pdf_out *p = current_output();
    char v;

    init_pdf_out_struct(p);

    pdf_out_set_version(p, ver_major, ver_minor);
    pdf_out_set_compression(p, compression_level);

    add_xref_entry(p, 0, 0, 0, 0xFFFF);

    /* This must be set before pdf_set_root() is called */
    p->options.enable_encrypt = enable_encrypt;
    if (pdf_check_version(1, 5) == 0) {
        if (enable_objstm) {
            p->xref_stream = pdf_new_stream(STREAM_COMPRESS);
            p->xref_stream->flags |= OBJ_NO_ENCRYPT;
            p->trailer = pdf_stream_dict(p->xref_stream);
            pdf_add_dict(p->trailer, pdf_new_name("Type"), pdf_new_name("XRef"));
            p->options.use_objstm = 1;
        } else {
            p->xref_stream = NULL;
            p->trailer = pdf_new_dict();
            p->options.use_objstm = 0;
        }
    } else {
        p->xref_stream = NULL;
        p->trailer = pdf_new_dict();
        p->options.use_objstm = 0;
    }

    p->output_stream = NULL;

    if (filename == NULL)
        _tt_abort("stdout PDF output not supported");

    p->output.handle = ttstub_output_open(filename, 0);
    if (!p->output.handle) {
        if (strlen(filename) < 128)
            _tt_abort("Unable to open \"%s\".", filename);
        else
            _tt_abort("Unable to open file.");
    }

    pdf_out_str(p, "%PDF-", strlen("%PDF-"));
    v = '0' + p->version.major;
    pdf_out_str(p, &v, 1);
    pdf_out_str(p, ".", 1);
    v = '0' + p->version.minor;
    pdf_out_str(p, &v, 1);
    pdf_out_str(p, "\n", 1);
    pdf_out_str(p, BINARY_MARKER, strlen(BINARY_MARKER));

    /* Set trailer ID and setup security handler */
    {
        pdf_obj *id_array;

        memcpy(p->id1, id1, 16);
        memcpy(p->id2, id2, 16);
        id_array = pdf_new_array();
        pdf_add_array(id_array, pdf_new_string(p->id1, 16));
        pdf_add_array(id_array, pdf_new_string(p->id2, 16));
        pdf_add_dict(p->trailer, pdf_new_name("ID"), id_array);
    }
    p->state.enc_mode = 0;
    p->options.compression.use_predictor = enable_predictor;

    return p;
}

void
pdf_out_set_encrypt (int keybits, int32_t permission,
                     const char *opasswd, const char *upasswd,
                     int use_aes, int encrypt_metadata)
{
    pdf_out *p = current_output();

    pdf_obj *encrypt, *extension, *catalog;

    p->sec_data = pdf_enc_init(p->id1, keybits, permission,
                                opasswd, upasswd, use_aes, encrypt_metadata);
    if (!p->sec_data) {
        p->options.enable_encrypt = 0;
        return;
    }

    encrypt = pdf_enc_get_encrypt_dict(p->sec_data);
    pdf_add_dict(p->trailer,
                pdf_new_name("Encrypt"), pdf_ref_obj(encrypt));
    encrypt->flags |= OBJ_NO_ENCRYPT;
    encrypt->flags |= OBJ_NO_OBJSTM;
    pdf_release_obj(encrypt);

    extension = pdf_enc_get_extension_dict(p->sec_data);
    if (extension) {
        catalog = pdf_doc_catalog();
        pdf_add_dict(catalog, pdf_new_name("Extensions"), extension);
    }
}


static void
dump_xref_table (pdf_out *p)
{
    int i, length;
    char buf[32];

    assert(p);

    pdf_out_str(p, "xref\n", 5);

    length = sprintf(buf, "%d %u\n", 0, p->obj.next_label);
    pdf_out_str(p, buf, length);

    /*
     * Every space counts.  The space after the 'f' and 'n' is * *essential*.
     * The PDF spec says the lines must be 20 characters long including the
     * end of line character.
     */
    for (i = 0; i < p->obj.next_label; i++) {
        uint8_t type = p->xref_table[i].type;
        if (type > 1)
            _tt_abort("object type %c not allowed in xref table", type);
        length = sprintf(buf, "%010u %05hu %c \n",
                         p->xref_table[i].field2, p->xref_table[i].field3,
                         type ? 'n' : 'f');
        pdf_out_str(p, buf, length);
    }
}

static void
dump_trailer (pdf_out *p)
{
    assert(p);

    pdf_out_str(p, "trailer\n", 8);
    p->state.enc_mode = false;
    write_dict(p, p->trailer->data);
    pdf_release_obj(p->trailer);
    p->trailer = NULL;
    pdf_out_char(p, '\n');
}

/*
 * output a PDF 1.5 cross-reference stream;
 * contributed by Matthias Franz (March 21, 2007)
 */
static void
dump_xref_stream (pdf_out *p)
{
    uint32_t pos, i;
    uint32_t poslen;
    unsigned char buf[7] = {0, 0, 0, 0, 0};
    pdf_obj *w;

    assert(p);

    /* determine the necessary size of the offset field */
    pos = p->startxref; /* maximal offset value */
    poslen = 1;
    while (pos >>= 8)
        poslen++;

    w = pdf_new_array();
    pdf_add_array(w, pdf_new_number(1));      /* type                */
    pdf_add_array(w, pdf_new_number(poslen)); /* offset (big-endian) */
    pdf_add_array(w, pdf_new_number(2));      /* generation          */
    pdf_add_dict(p->trailer, pdf_new_name("W"), w);

    /* We need the xref entry for the xref stream right now */
    add_xref_entry(p, p->obj.next_label-1, 1, p->startxref, 0);

    for (i = 0; i < p->obj.next_label; i++) {
        size_t j;
        uint16_t f3;
        buf[0] = p->xref_table[i].type;
        pos = p->xref_table[i].field2;
        for (j = poslen; j--; ) {
            buf[1+j] = (unsigned char) pos;
            pos >>= 8;
        }
        f3 = p->xref_table[i].field3;
        buf[poslen+1] = (unsigned char) (f3 >> 8);
        buf[poslen+2] = (unsigned char) (f3);
        pdf_add_stream(p->xref_stream, &buf, poslen+3);
    }

    pdf_release_obj(p->xref_stream);
    p->xref_stream = NULL;
}

void
pdf_out_flush (void)
{
    pdf_out *p = current_output();
    char buf[16];

    if (p->output.handle) {
        int length;

        /* Flush current object stream */
        if (p->current_objstm) {
            release_objstm(p->current_objstm);
            p->current_objstm =NULL;
        }

        /*
         * Label xref stream - we need the number of correct objects
         * for the xref stream dictionary (= trailer).
         * Labelling it in pdf_out_init (with 1)  does not work (why?).
         */
        if (p->xref_stream)
            pdf_label_obj(p, p->xref_stream);

        /* Record where this xref is for trailer */
        p->startxref = p->output.file_position;

        pdf_add_dict(p->trailer, pdf_new_name("Size"),
                     pdf_new_number(p->obj.next_label));

        if (p->xref_stream)
            dump_xref_stream(p);
        else {
            dump_xref_table(p);
            dump_trailer(p);
        }

        /* Done with xref table */
        free(p->xref_table);
        p->xref_table = NULL;

        pdf_out_str(p, "startxref\n", 10);
        length = sprintf(buf, "%u\n", p->startxref);
        pdf_out_str(p, buf, length);
        pdf_out_str(p, "%%EOF\n", 6);

        if (dpx_conf.verbose_level > 0) {
            if (p->options.compression.level > 0) {
                dpx_message("Compression saved %"PRIuZ" bytes\n", p->output.compression_saved);
            }
        }

        dpx_message("%"PRIuZ" bytes written", p->output.file_position);

        ttstub_output_close(p->output.handle);
        p->output.handle = NULL;
        p->output.file_position = 0;
        p->output.line_position = 0;
    }

    if (p->sec_data)
        pdf_enc_close(&p->sec_data);

    clean_pdf_out_struct(p);
}

void
pdf_error_cleanup (void)
{
    pdf_out *p = current_output();

    /*
     * This routine is the cleanup required for an abnormal exit.
     * For now, simply close the file.
     */
    if (p->output.handle) {
        ttstub_output_close(p->output.handle);
        p->output.handle = NULL;
    }
}


void
pdf_set_root (pdf_obj *object)
{
    pdf_out *p = current_output();

    if (pdf_lookup_dict(p->trailer, "Root")) {
        _tt_abort("Root object already set!");
    }

    pdf_add_dict(p->trailer, pdf_new_name("Root"), pdf_ref_obj(object));

    /* Adobe Readers don't like a document catalog inside an encrypted
     * object stream, although the PDF v1.5 spec seems to allow this.
     * Note that we don't set OBJ_NO_ENCRYPT since the name dictionary in
     * a document catalog may contain strings, which should be encrypted.
     */
    if (p->options.enable_encrypt)
        object->flags |= OBJ_NO_OBJSTM;
}

void
pdf_set_info (pdf_obj *object)
{
    pdf_out *p = current_output();

    if (pdf_lookup_dict(p->trailer, "Info")) {
        _tt_abort("Info object already set!");
    }

    pdf_add_dict(p->trailer, pdf_new_name("Info"), pdf_ref_obj(object));
}

static void
pdf_out_char (pdf_out *p, char c)
{
    assert(p);

    if (p->output_stream)
        pdf_add_stream(p->output_stream, &c, 1);
    else {
        ttstub_output_putc(p->output.handle, c);
        p->output.file_position += 1;

        if (c == '\n')
            p->output.line_position  = 0;
        else
            p->output.line_position += 1;
    }
}

static char xchar[] = "0123456789abcdef";

static void
pdf_out_xchar(pdf_out *p, char c)
{
    assert(p);

    pdf_out_char(p, xchar[(c >> 4) & 0x0f]);
    pdf_out_char(p, xchar[c & 0x0f]);
}

static void
pdf_out_str(pdf_out *p, const void *buffer, size_t length)
{
    assert(p);

    if (p->output_stream)
        pdf_add_stream(p->output_stream, buffer, length);
    else {
        ttstub_output_write(p->output.handle, buffer, length);
        p->output.file_position += length;
        p->output.line_position += length;
        /* "foo\nbar\n "... */
        if (length > 0 &&
            ((const char *)buffer)[length-1] == '\n')
            p->output.line_position = 0;
    }
}

/*  returns 1 if a white-space character is necessary to separate
    an object of type1 followed by an object of type2              */
static int
pdf_need_white (int type1, int type2)
{
    return !(type1 == PDF_STRING || type1 == PDF_ARRAY || type1 == PDF_DICT ||
             type2 == PDF_STRING || type2 == PDF_NAME ||
             type2 == PDF_ARRAY || type2 == PDF_DICT);
}

static void
pdf_out_white (pdf_out *p)
{
    assert(p);

    if (p->output.line_position >= 80) {
        pdf_out_char(p, '\n');
    } else {
        pdf_out_char(p, ' ');
    }
}

#define TYPECHECK(o,t) if (!(o) || (o)->type != (t)) {                  \
        _tt_abort("typecheck: Invalid object type: %d %d (line %d)", (o) ? (o)->type : -1, (t), __LINE__); \
    }

#define INVALIDOBJ(o)  ((o) == NULL || (o)->type <= 0 || (o)->type > PDF_UNDEFINED)

static pdf_obj *
pdf_new_obj(int type)
{
    pdf_obj *result;

    if (type > PDF_UNDEFINED || type < 0)
        _tt_abort("Invalid object type: %d", type);

    result = NEW(1, pdf_obj);
    result->type  = type;
    result->data  = NULL;
    result->label      = 0;
    result->generation = 0;
    result->refcount   = 1;
    result->flags      = 0;

    return result;
}

int
pdf_obj_typeof (pdf_obj *object)
{
    if (INVALIDOBJ(object))
        return PDF_OBJ_INVALID;

    return object->type;
}

static void
pdf_label_obj (pdf_out *p, pdf_obj *object)
{
    assert(p);

    if (INVALIDOBJ(object))
        _tt_abort("pdf_label_obj(): passed invalid object.");

    /*
     * Don't change label on an already labeled object. Ignore such calls.
     */
    if (object->label == 0) {
        if (p->obj.next_label == PDF_NUM_INDIRECT_MAX) {
            _tt_abort("Number of indirect object has reached its maximum value!");
        }

        object->label      = p->obj.next_label++;
        object->generation = 0;
    }
}

/*
 * Transfer the label assigned to the object src to the object dst.
 * The object dst must not yet have been labeled.
 */
void
pdf_transfer_label (pdf_obj *dst, pdf_obj *src)
{
    assert(dst && !dst->label && src);

    dst->label      = src->label;
    dst->generation = src->generation;
    src->label      = 0;
    src->generation = 0;
}

/*
 * This doesn't really copy the object, but allows it to be used without
 * fear that somebody else will free it.
 */
pdf_obj *
pdf_link_obj (pdf_obj *object)
{
    if (INVALIDOBJ(object))
        _tt_abort("pdf_link_obj(): passed invalid object.");

    object->refcount += 1;

    return object;
}


pdf_obj *
pdf_ref_obj (pdf_obj *object)
{
    pdf_out *p = current_output();

    if (INVALIDOBJ(object))
        _tt_abort("pdf_ref_obj(): passed invalid object.");

    if (object->refcount == 0) {
        dpx_message("\nTrying to refer already released object!!!\n");
        _tt_abort("Cannot continue...");
    }

    if (PDF_OBJ_INDIRECTTYPE(object)) {
        return pdf_link_obj(object);
    } else {
        return pdf_new_ref(p, object);
    }
}

static void
release_indirect (pdf_indirect *data)
{
    free(data);
}

static void
write_indirect (pdf_out *p, pdf_indirect *indirect)
{
    int length;
    char buf[64];

    assert(p);
    assert(!indirect->pf);

    length = sprintf(buf, "%u %hu R", indirect->label, indirect->generation);
    pdf_out_str(p, buf, length);
}

/* The undefined object is used as a placeholder in pdfnames.c
 * for objects which are referenced before they are defined.
 */
pdf_obj *
pdf_new_undefined (void)
{
    pdf_obj *result;

    result = pdf_new_obj(PDF_UNDEFINED);
    result->data = NULL;

    return result;
}

pdf_obj *
pdf_new_null (void)
{
    pdf_obj *result;

    result = pdf_new_obj(PDF_NULL);
    result->data = NULL;

    return result;
}

static void
write_null (pdf_out *p)
{
    pdf_out_str(p, "null", 4);
}

pdf_obj *
pdf_new_boolean (char value)
{
    pdf_obj     *result;
    pdf_boolean *data;

    result = pdf_new_obj(PDF_BOOLEAN);
    data   = NEW(1, pdf_boolean);
    data->value  = value;
    result->data = data;

    return result;
}

static void
release_boolean (pdf_obj *data)
{
    free (data);
}

static void
write_boolean (pdf_out *p, pdf_boolean *data)
{
    if (data->value) {
        pdf_out_str(p, "true", 4);
    } else {
        pdf_out_str(p, "false", 5);
    }
}

char
pdf_boolean_value (pdf_obj *object)
{
    pdf_boolean *data;

    TYPECHECK(object, PDF_BOOLEAN);

    data = object->data;

    return data->value;
}

pdf_obj *
pdf_new_number (double value)
{
    pdf_obj    *result;
    pdf_number *data;

    result = pdf_new_obj(PDF_NUMBER);
    data   = NEW(1, pdf_number);
    data->value  = value;
    result->data = data;

    return result;
}

static void
release_number (pdf_number *data)
{
    free (data);
}

static void
write_number (pdf_out *p, pdf_number *number)
{
    int count;
    char buf[512];

    count = pdf_sprint_number(buf, number->value);

    pdf_out_str(p, buf, count);
}


void
pdf_set_number (pdf_obj *object, double value)
{
    pdf_number *data;

    TYPECHECK(object, PDF_NUMBER);

    data = object->data;
    data->value = value;
}

double
pdf_number_value (pdf_obj *object)
{
    pdf_number *data;

    TYPECHECK(object, PDF_NUMBER);

    data = object->data;

    return data->value;
}

pdf_obj *
pdf_new_string (const void *str, size_t length)
{
    pdf_obj    *result;
    pdf_string *data;

    assert(str);

    result = pdf_new_obj(PDF_STRING);
    data   = NEW(1, pdf_string);
    result->data = data;
    data->length = length;

    if (length) {
        data->string = NEW(length+1, unsigned char);
        memcpy(data->string, str, length);
        /* Shouldn't assume NULL terminated. */
        data->string[length] = '\0';
    } else {
        data->string = NULL;
    }

    return result;
}

void *
pdf_string_value (pdf_obj *object)
{
    pdf_string *data;

    TYPECHECK(object, PDF_STRING);

    data = object->data;

    return data->string;
}

unsigned int
pdf_string_length (pdf_obj *object)
{
    pdf_string *data;

    TYPECHECK(object, PDF_STRING);

    data = object->data;

    return (unsigned) data->length;
}

/*
 * This routine escapes non printable characters and control
 * characters in an output string.
 */
size_t
pdfobj_escape_str (char *buffer, size_t bufsize, const unsigned char *s, size_t len)
{
    size_t result = 0;
    size_t i;

    for (i = 0; i < len; i++) {
        unsigned char ch;

        ch = s[i];
        if (result > bufsize - 4)
            _tt_abort("pdfobj_escape_str: Buffer overflow");

        /*
         * We always write three octal digits. Optimization only gives few Kb
         * smaller size for most documents when zlib compressed.
         */
        if (ch < 32 || ch > 126) {
            buffer[result++] = '\\';
            result += sprintf(buffer+result, "%03o", ch);
        } else {
            switch (ch) {
            case '(':
                buffer[result++] = '\\';
                buffer[result++] = '(';
                break;
            case ')':
                buffer[result++] = '\\';
                buffer[result++] = ')';
                break;
            case '\\':
                buffer[result++] = '\\';
                buffer[result++] = '\\';
                break;
            default:
                buffer[result++] = ch;
                break;
            }
        }
    }

    return result;
}

static void
write_string (pdf_out *p, pdf_string *str)
{
    unsigned char *s = NULL;
    size_t  i, nescc = 0;
    size_t len = 0;

    assert(p);

    if (p->state.enc_mode) {
        pdf_encrypt_data(p->sec_data, str->string, str->length, &s, &len);
    } else {
        s = str->string;
        len = str->length;
    }

    /*
     * Count all ASCII non-printable characters.
     */
    for (i = 0; i < len; i++) {
        if (!isprint(s[i]))
            nescc++;
    }
    /*
     * If the string contains much escaped chars, then we write it as
     * ASCII hex string.
     */
    if (nescc > len / 3) {
        pdf_out_char(p, '<');
        for (i = 0; i < len; i++) {
            pdf_out_xchar(p, s[i]);
        }
        pdf_out_char(p, '>');
    } else {
        char *buf;
        size_t size, count;

        size = len * 2 + 3;
        buf = NEW(size, char);

        pdf_out_char(p, '(');
        /*
         * This section of code probably isn't speed critical.  Escaping the
         * characters in the string one at a time may seem slow, but it's
         * safe if the formatted string length exceeds FORMAT_BUF_SIZE.
         * Occasionally you see some long strings in PDF.  pdfobj_escape_str
         * is also used for strings of text with no kerning.  These must be
         * handled as quickly as possible since there are so many of them.
         */
        for (i = 0; i < len; i++) {
            count = pdfobj_escape_str(buf, size, &(s[i]), 1);
            pdf_out_str(p, buf, count);
        }
        pdf_out_char(p, ')');
        free(buf);
    }
    if (p->state.enc_mode && s)
        free(s);
}

static void
release_string (pdf_string *data)
{
    data->string = mfree(data->string);
    free(data);
}

void
pdf_set_string (pdf_obj *object, unsigned char *str, size_t length)
{
    pdf_string *data;

    TYPECHECK(object, PDF_STRING);

    data = object->data;
    if (data->string != NULL) {
        free(data->string);
    }
    if (length != 0) {
        data->length = length;
        data->string = NEW(length + 1, unsigned char);
        memcpy(data->string, str, length);
        data->string[length] = '\0';
    } else {
        data->length = 0;
        data->string = NULL;
    }
}

/* Name does *not* include the /. */
pdf_obj *
pdf_new_name (const char *name)
{
    pdf_obj  *result;
    size_t length;
    pdf_name *data;

    result = pdf_new_obj(PDF_NAME);
    data   = NEW (1, pdf_name);
    result->data = data;
    length = strlen(name);
    if (length != 0) {
        data->name = NEW(length+1, char);
        memcpy(data->name, name, length);
        data->name[length] = '\0';
    } else {
        data->name = NULL;
    }

    return result;
}

static void
write_name (pdf_out *p, pdf_name *name)
{
    char *s;
    size_t i, length;

    assert(p);

    s      = name->name;
    length = name->name ? strlen(name->name) : 0;
    /*
     * From PDF Reference, 3rd ed., p.33:
     *
     *  Beginning with PDF 1.2, any character except null (character code 0)
     *  may be included in a name by writing its 2-digit hexadecimal code,
     *  preceded bythe number sign character (#); see implementation notes 3
     *  and 4 in Appendix H. This syntax is required in order to represent
     *  any of the delimiter or white-space characters or the number sign
     *  character itself; it is recommended but not required for characters
     *  whose codes are outside the range 33 (!) to 126 (~).
     */
#ifndef is_delim
    /* Avoid '{' and '}' for PostScript compatibility? */
#define is_delim(c) ((c) == '(' || (c) == ')' ||        \
                     (c) == '/' ||                      \
                     (c) == '<' || (c) == '>' ||        \
                     (c) == '[' || (c) == ']' ||        \
                     (c) == '{' || (c) == '}' ||        \
                     (c) == '%')
#endif
    pdf_out_char(p, '/');
    for (i = 0; i < length; i++) {
        if (s[i] < '!' || s[i] > '~' || s[i] == '#' || is_delim(s[i])) {
            /*     ^ "space" is here. */
            pdf_out_char(p, '#');
            pdf_out_xchar(p, s[i]);
        } else {
            pdf_out_char(p, s[i]);
        }
    }
}

static void
release_name (pdf_name *data)
{
    data->name = mfree(data->name);
    free(data);
}

char *
pdf_name_value (pdf_obj *object)
{
    pdf_name *data;

    TYPECHECK(object, PDF_NAME);

    data = object->data;

    return data->name;
}

/*
 * We do not have pdf_name_length() since '\0' is not allowed
 * in PDF name object.
 */

pdf_obj *
pdf_new_array (void)
{
    pdf_obj   *result;
    pdf_array *data;

    result = pdf_new_obj(PDF_ARRAY);
    data   = NEW(1, pdf_array);
    data->values = NULL;
    data->max    = 0;
    data->size   = 0;
    result->data = data;

    return result;
}

static void
write_array (pdf_out *p, pdf_array *array)
{
    assert(p);

    pdf_out_char(p, '[');
    if (array->size > 0) {
        size_t i;
        int type1 = PDF_UNDEFINED, type2;

        for (i = 0; i < array->size; i++) {
            if (array->values[i]) {
                type2 = array->values[i]->type;
                if (type1 != PDF_UNDEFINED && pdf_need_white(type1, type2))
                    pdf_out_white(p);
                type1 = type2;
                pdf_write_obj(p, array->values[i]);
            } else
                dpx_warning("PDF array element %zu undefined.", i);
        }
    }
    pdf_out_char(p, ']');
}

pdf_obj *
pdf_get_array (pdf_obj *array, int idx)
{
    pdf_obj   *result = NULL;
    pdf_array *data;

    TYPECHECK(array, PDF_ARRAY);

    data = array->data;
    if (idx < 0)
        result = data->values[idx + data->size];
    else if (idx < data->size) {
        result = data->values[idx];
    }

    return result;
}

unsigned
pdf_array_length (pdf_obj *array)
{
    pdf_array *data;

    TYPECHECK(array, PDF_ARRAY);

    data = (pdf_array *) array->data;

    return (unsigned) data->size;
}

static void
release_array (pdf_array *data)
{
    size_t i;

    if (data->values) {
        for (i = 0; i < data->size; i++) {
            pdf_release_obj(data->values[i]);
            data->values[i] = NULL;
        }
        data->values = mfree(data->values);
    }
    free(data);
}

/*
 * The name pdf_add_array is misleading. It behaves differently than
 * pdf_add_dict(). This should be pdf_push_array().
 */
void
pdf_add_array (pdf_obj *array, pdf_obj *object)
{
    pdf_array *data;

    TYPECHECK(array, PDF_ARRAY);

    data = array->data;
    if (data->size >= data->max) {
        data->max   += ARRAY_ALLOC_SIZE;
        data->values = RENEW(data->values, data->max, pdf_obj *);
    }
    data->values[data->size] = object;
    data->size++;

    return;
}


/* Prepend an object to an array */
static void
pdf_unshift_array (pdf_obj *array, pdf_obj *object)
{
    pdf_array *data;

    TYPECHECK(array, PDF_ARRAY);

    data = array->data;
    if (data->size >= data->max) {
        data->max   += ARRAY_ALLOC_SIZE;
        data->values = RENEW(data->values, data->max, pdf_obj *);
    }
    memmove(&data->values[1], data->values, data->size * sizeof(pdf_obj *));
    data->values[0] = object;
    data->size++;
}


static void
write_dict (pdf_out *p, pdf_dict *dict)
{
    pdf_out_str(p, "<<", 2);

    while (dict->key != NULL) {
        pdf_write_obj(p, dict->key);
        if (pdf_need_white(PDF_NAME, (dict->value)->type)) {
            pdf_out_white(p);
        }
        pdf_write_obj(p, dict->value);
        dict = dict->next;
    }
    pdf_out_str(p, ">>", 2);
}

pdf_obj *
pdf_new_dict (void)
{
    pdf_obj  *result;
    pdf_dict *data;

    result = pdf_new_obj(PDF_DICT);
    data   = NEW(1, pdf_dict);
    data->key    = NULL;
    data->value  = NULL;
    data->next   = NULL;
    result->data = data;

    return result;
}

static void
release_dict (pdf_dict *data)
{
    pdf_dict *next;

    while (data != NULL && data->key != NULL) {
        pdf_release_obj(data->key);
        pdf_release_obj(data->value);
        data->key   = NULL;
        data->value = NULL;
        next = data->next;
        free(data);
        data = next;
    }
    free(data);
}

/* Array is ended by a node with NULL this pointer */
/* pdf_add_dict returns 0 if the key is new and non-zero otherwise */
int
pdf_add_dict (pdf_obj *dict, pdf_obj *key, pdf_obj *value)
{
    pdf_dict *data, *new_node;

    TYPECHECK(dict, PDF_DICT);
    TYPECHECK(key,  PDF_NAME);

    /* It seems that NULL is sometimes used for null object... */
    if (value != NULL && INVALIDOBJ(value))
        _tt_abort("pdf_add_dict(): Passed invalid value");

    /* If this key already exists, simply replace the value */
    for (data = dict->data; data->key != NULL; data = data->next) {
        if (streq_ptr(pdf_name_value(key), pdf_name_value(data->key))) {
            /* Release the old value */
            pdf_release_obj(data->value);
            /* Release the new key (we don't need it) */
            pdf_release_obj(key);
            data->value = value;
            return 1;
        }
    }
    /*
     * We didn't find the key. We build a new "end" node and add
     * the new key just before the end
     */
    new_node = NEW (1, pdf_dict);
    new_node->key = NULL;
    new_node->value = NULL;
    new_node->next = NULL;
    data->next  = new_node;
    data->key   = key;
    data->value = value;
    return 0;
}


/* pdf_merge_dict makes a link for each item in dict2 before stealing it */
void
pdf_merge_dict (pdf_obj *dict1, pdf_obj *dict2)
{
    pdf_dict *data;

    TYPECHECK(dict1, PDF_DICT);
    TYPECHECK(dict2, PDF_DICT);

    data = dict2->data;
    while (data->key != NULL) {
        pdf_add_dict(dict1, pdf_link_obj(data->key), pdf_link_obj(data->value));
        data = data->next;
    }
}

int
pdf_foreach_dict (pdf_obj *dict,
                  int (*proc) (pdf_obj *, pdf_obj *, void *), void *pdata)
{
    int       error = 0;
    pdf_dict *data;

    assert(proc);

    TYPECHECK(dict, PDF_DICT);

    data = dict->data;
    while (!error &&
           data->key != NULL) {
        error = proc(data->key, data->value, pdata);
        data = data->next;
    }

    return error;
}

#define pdf_match_name(o,s) ((o) && (s) && streq_ptr(((pdf_name *)(o)->data)->name, (s)))
pdf_obj *
pdf_lookup_dict (pdf_obj *dict, const char *name)
{
    pdf_dict *data;

    assert(name);

    TYPECHECK(dict, PDF_DICT);

    data = dict->data;
    while (data->key != NULL) {
        if (streq_ptr(name, pdf_name_value(data->key))) {
            return data->value;
        }
        data = data->next;
    }

    return NULL;
}

/* Returns array of dictionary keys */
pdf_obj *
pdf_dict_keys (pdf_obj *dict)
{
    pdf_obj  *keys;
    pdf_dict *data;

    TYPECHECK(dict, PDF_DICT);

    keys = pdf_new_array();
    for (data = dict->data; (data &&
                             data->key != NULL); data = data->next) {
        /* We duplicate name object rather than linking keys.
         * If we forget to free keys, broken PDF is generated.
         */
        pdf_add_array(keys, pdf_new_name(pdf_name_value(data->key)));
    }

    return keys;
}

void
pdf_remove_dict (pdf_obj *dict, const char *name)
{
    pdf_dict *data, **data_p;

    TYPECHECK(dict, PDF_DICT);

    data   = dict->data;
    data_p = (pdf_dict **) (void *) &(dict->data);
    while (data->key != NULL) {
        if (pdf_match_name(data->key, name)) {
            pdf_release_obj(data->key);
            pdf_release_obj(data->value);
            *data_p = data->next;
            free(data);
            break;
        }
        data_p = &(data->next);
        data   = data->next;
    }
}

pdf_obj *
pdf_new_stream (int flags)
{
    pdf_obj    *result;
    pdf_stream *data;

    result = pdf_new_obj(PDF_STREAM);
    data   = NEW(1, pdf_stream);
    /*
     * Although we are using an arbitrary pdf_object here, it must have
     * type=PDF_DICT and cannot be an indirect reference.  This will be
     * checked by the output routine.
     */
    data->dict   = pdf_new_dict();
    data->_flags = flags;
    data->stream = NULL;
    data->stream_length = 0;
    data->max_length    = 0;
    data->objstm_data = NULL;

    data->decodeparms.predictor = 2;
    data->decodeparms.columns   = 0;
    data->decodeparms.bits_per_component = 0;
    data->decodeparms.colors    = 0;

    result->data = data;
    result->flags |= OBJ_NO_OBJSTM;

    return result;
}

void
pdf_stream_set_predictor (pdf_obj *stream,
                          int predictor, int32_t columns, int bpc, int colors)
{
    struct pdf_stream *data;

    if (pdf_obj_typeof(stream) != PDF_STREAM)
        return;
    else if (columns < 0 || bpc < 0 || colors < 0)
        return;

    data = (struct pdf_stream *) stream->data;
    data->decodeparms.predictor = predictor;
    data->decodeparms.columns   = columns;
    data->decodeparms.bits_per_component = bpc;
    data->decodeparms.colors    = colors;
    data->_flags |= STREAM_USE_PREDICTOR;
}

/* Adaptive PNG filter
 * We use the "minimum sum of absolute differences" heuristic approach
 * for finding the most optimal filter to be used.
 *
 * From http://www.libpng.org/pub/png/book/chapter09.html
 *
 *   For grayscale and truecolor images of 8 or more bits per sample, with or
 *   without alpha channels, dynamic filtering is almost always beneficial. The
 *   approach that has by now become standard is known as the minimum sum of
 *   absolute differences heuristic and was first proposed by Lee Daniel
 *   Crocker in February 1995.
 */
static unsigned char *
filter_PNG15_apply_filter (unsigned char *raster,
                           int32_t columns, int32_t rows,
                           int8_t bpc, int8_t colors, int32_t *length)
{
    unsigned char *dst;
    int      bits_per_pixel  = colors * bpc;
    int      bytes_per_pixel = (bits_per_pixel + 7) / 8;
    int32_t  rowbytes = columns * bytes_per_pixel;
    int32_t  i, j;

    assert(raster && length);

    /* Result */
    dst = NEW((rowbytes+1)*rows, unsigned char);
    *length = (rowbytes + 1) * rows;

    for (j = 0; j < rows; j++) {
        int type = 0;
        unsigned char *pp = dst + j * (rowbytes + 1);
        unsigned char *p  = raster + j * rowbytes;
        uint32_t sum[5]   = {0, 0, 0, 0, 0};
        /* First calculated sum of values to make a heuristic guess
         * of optimal predictor function.
         */
        for (i = 0; i < rowbytes; i++) {
            int left  = (i - bytes_per_pixel >= 0) ? p[i - bytes_per_pixel] : 0;
            int up    = (j > 0) ? *(p+i-rowbytes) : 0;
            int uplft = (j > 0) ?
                ((i - bytes_per_pixel >= 0) ?
                 *(p+i-rowbytes-bytes_per_pixel) : 0) : 0;
            /* Type 0 -- None */
            sum[0] += p[i];
            /* Type 1 -- Sub */
            sum[1] += abs((int) p[i] - left);
            /* Type 2 -- Up */
            sum[2] += abs((int) p[i] - up);
            /* Type 3 -- Average */
            {
                int tmp = floor((up + left) / 2);
                sum[3] += abs((int) p[i] - tmp);
            }
            /* Type 4 -- Peath */
            {
                int q = left + up - uplft;
                int qa = abs(q - left), qb = abs(q - up), qc = abs(q - uplft);
                if (qa <= qb && qa <= qc)
                    sum[4] += abs((int) p[i] - left);
                else if (qb <= qc)
                    sum[4] += abs((int) p[i] - up);
                else
                    sum[4] += abs((int) p[i] - uplft);
            }
        }
        {
            int min = sum[0], min_idx = 0;
            for (i = 0; i < 5; i++) {
                if (sum[i] < min) {
                    min = sum[i]; min_idx = i;
                }
            }
            type = min_idx;
        }
        /* Now we actually apply filter. */
        pp[0] = type;
        switch (type) {
        case 0:
            memcpy(pp+1, p, rowbytes);
            break;
        case 1:
            for (i = 0; i < rowbytes; i++) {
                int left = (i - bytes_per_pixel >= 0) ? p[i - bytes_per_pixel] : 0;
                pp[i+1] = p[i] - left;
            }
            break;
        case 2:
            for (i = 0; i < rowbytes; i++) {
                int up  = (j > 0) ? *(p+i - rowbytes) : 0;
                pp[i+1] = p[i] - up;
            }
            break;
        case 3:
        {
            for (i = 0; i < rowbytes; i++) {
                int up   = (j > 0) ? *(p+i-rowbytes) : 0;
                int left = (i - bytes_per_pixel >= 0) ? p[i - bytes_per_pixel] : 0;
                int tmp  = floor((up + left) / 2);
                pp[i+1]  = p[i] - tmp;
            }
        }
        break;
        case 4: /* Peath */
        {
            for (i = 0; i < rowbytes; i++) {
                int up   = (j > 0) ? *(p+i-rowbytes) : 0;
                int left = (i - bytes_per_pixel >= 0) ? p[i - bytes_per_pixel] : 0;
                int uplft = (j > 0) ?
                    ((i - bytes_per_pixel >= 0) ?
                     *(p+i-rowbytes-bytes_per_pixel) : 0) : 0;
                int q = left + up - uplft;
                int qa = abs(q - left), qb = abs(q - up), qc = abs(q - uplft);
                if (qa <= qb && qa <= qc)
                    pp[i+1] = p[i] - left;
                else if (qb <= qc)
                    pp[i+1] = p[i] - up;
                else
                    pp[i+1] = p[i] - uplft;
            }
        }
        break;
        }
    }

    return  dst;
}

/* This modifies "raster" itself! */
static void
apply_filter_TIFF2_1_2_4 (unsigned char *raster,
                          int32_t width, int32_t height,
                          int8_t bpc, int8_t num_comp)
{
    int32_t   rowbytes = (bpc * num_comp * width + 7) / 8;
    uint8_t   mask     = (1 << bpc) - 1;
    uint16_t *prev;
    int32_t   i, j;

    assert(raster);
    assert(bpc > 0 && bpc <= 8);

    prev = NEW(num_comp, uint16_t);

    /* Generic routine for 1 to 16 bit.
     * It supports, e.g., 7 bpc images too.
     * Actually, it is not necessary to have 16 bit inbuf and outbuf
     * since we only need 1, 2, and 4 bit support here. 8 bit is enough.
     */
    for (j = 0; j < height; j++) {
        int32_t  k, l, inbits, outbits;
        uint16_t inbuf, outbuf;
        int      c;

        memset(prev, 0, sizeof(uint16_t) * num_comp);
        inbuf = outbuf = 0;
        inbits = outbits = 0;
        l = k = j * rowbytes;

        for (i = 0; i < width; i++) {
            for (c = 0; c < num_comp; c++) {
                uint8_t cur;
                int8_t  sub;

                if (inbits < bpc) { /* need more byte */
                    inbuf = (inbuf << 8) | raster[l];
                    l++;
                    inbits += 8;
                }

                cur     = (inbuf >> (inbits - bpc)) & mask;
                inbits -= bpc; /* consumed bpc bits */
                sub     = cur - prev[c];
                prev[c] = cur;

                if (sub < 0)
                    sub += (1 << bpc);

                /* Append newly filtered component value */
                outbuf   = (outbuf << bpc) | sub;
                outbits += bpc;

                /* flush */
                if (outbits >= 8) {
                    raster[k] = (outbuf >> (outbits - 8));
                    k++;
                    outbits  -= 8;
                }
            }
        }

        if (outbits > 0) {
            raster[k] = outbuf << (8 - outbits);
            k++;
        }
    }

    free(prev);
}


static unsigned char *
filter_TIFF2_apply_filter (unsigned char *raster,
                           int32_t columns, int32_t rows,
                           int8_t bpc, int8_t colors, int32_t *length)
{
    unsigned char *dst;
    uint16_t      *prev;
    int32_t        rowbytes = (bpc * colors * columns + 7) / 8;
    int32_t        i, j;

    assert(raster && length);

    dst = NEW(rowbytes*rows, unsigned char);
    memcpy(dst, raster, rowbytes*rows);
    *length = rowbytes * rows;

    switch (bpc) {
    case 1: case 2: case 4:
        apply_filter_TIFF2_1_2_4(dst, columns, rows, bpc, colors);
        break;

    case 8:
        prev = NEW(colors, uint16_t);
        for (j = 0; j < rows; j++) {
            memset(prev, 0, sizeof(uint16_t)*colors);
            for (i = 0; i < columns; i++) {
                int     c;
                int32_t pos = colors * (columns * j + i);
                for (c = 0; c < colors; c++) {
                    uint8_t cur = raster[pos+c];
                    int32_t sub = cur - prev[c];
                    prev[c]     = cur;
                    dst[pos+c]  = sub;
                }
            }
        }
        free(prev);
        break;

    case 16:
        prev = NEW(colors, uint16_t);
        for (j = 0; j < rows; j++) {
            memset(prev, 0, sizeof(uint16_t)*colors);
            for (i = 0; i < columns; i++) {
                int     c;
                int32_t pos = 2 * colors * (columns * j + i);
                for (c = 0; c < colors; c++) {
                    uint16_t cur   = ((uint8_t)raster[pos+2*c])*256 +
                        (uint8_t)raster[pos+2*c+1];
                    uint16_t sub   = cur - prev[c];
                    prev[c]        = cur;
                    dst[pos+2*c  ] = (sub >> 8) & 0xff;
                    dst[pos+2*c+1] = sub & 0xff;
                }
            }
        }
        free(prev);
        break;

    }

    return  dst;
}

static pdf_obj *
filter_create_predictor_dict (int predictor, int32_t columns,
                              int bpc, int colors)
{
    pdf_obj *parms;

    parms = pdf_new_dict();
    pdf_add_dict(parms, pdf_new_name("BitsPerComponent"), pdf_new_number(bpc));
    pdf_add_dict(parms, pdf_new_name("Colors"),  pdf_new_number(colors));
    pdf_add_dict(parms, pdf_new_name("Columns"), pdf_new_number(columns));
    pdf_add_dict(parms, pdf_new_name("Predictor"), pdf_new_number(predictor));

    return  parms;
}

static void
write_stream (pdf_out *p, pdf_stream *stream)
{
    unsigned char *filtered;
    size_t   filtered_length;
    size_t         buffer_length;
    uint64_t       buffer_length64;
    unsigned char *buffer;

    assert(p);

    /*
     * Always work from a copy of the stream. All filters read from
     * "filtered" and leave their result in "filtered".
     */
    filtered = NEW(stream->stream_length, unsigned char);
    memcpy(filtered, stream->stream, stream->stream_length);
    filtered_length = stream->stream_length;

    /* PDF/A requires Metadata to be not filtered. */
    {
        pdf_obj *type;
        type = pdf_lookup_dict(stream->dict, "Type");
        if (type && streq_ptr("Metadata", pdf_name_value(type))) {
            stream->_flags &= ~STREAM_COMPRESS;
        }
    }

    /* Apply compression filter if requested */
    if (stream->stream_length > 0 &&
        (stream->_flags & STREAM_COMPRESS) &&
        p->options.compression.level > 0) {
        pdf_obj *filters;

        /* First apply predictor filter if requested. */
        if ( p->options.compression.use_predictor &&
             (stream->_flags & STREAM_USE_PREDICTOR) &&
             !pdf_lookup_dict(stream->dict, "DecodeParms")) {
            int      bits_per_pixel  = stream->decodeparms.colors *
                stream->decodeparms.bits_per_component;
            int32_t  len  = (stream->decodeparms.columns * bits_per_pixel + 7) / 8;
            int32_t  rows = stream->stream_length / len;
            unsigned char *filtered2 = NULL;
            int32_t        length2 = stream->stream_length;
            pdf_obj       *parms;

            parms = filter_create_predictor_dict(stream->decodeparms.predictor,
                                                 stream->decodeparms.columns,
                                                 stream->decodeparms.bits_per_component,
                                                 stream->decodeparms.colors);

            switch (stream->decodeparms.predictor) {
            case 2: /* TIFF2 */
                filtered2 = filter_TIFF2_apply_filter(filtered,
                                                      stream->decodeparms.columns,
                                                      rows,
                                                      stream->decodeparms.bits_per_component,
                                                      stream->decodeparms.colors, &length2);
                break;
            case 15: /* PNG optimun */
                filtered2 = filter_PNG15_apply_filter(filtered,
                                                      stream->decodeparms.columns,
                                                      rows,
                                                      stream->decodeparms.bits_per_component,
                                                      stream->decodeparms.colors, &length2);
                break;
            default:
                dpx_warning("Unknown/unsupported Predictor function %d.",
                     stream->decodeparms.predictor);
                break;
            }
            if (parms && filtered2) {
                free(filtered);
                filtered = filtered2;
                filtered_length = length2;
                pdf_add_dict(stream->dict, pdf_new_name("DecodeParms"), parms);
            }
        }

        filters = pdf_lookup_dict(stream->dict, "Filter");

        buffer_length = filtered_length + filtered_length/1000 + 14;
        buffer = NEW(buffer_length, unsigned char);
        {
            pdf_obj *filter_name = pdf_new_name("FlateDecode");

            if (filters)
                /*
                 * FlateDecode is the first filter to be applied to the stream.
                 */
                pdf_unshift_array(filters, filter_name);
            else
                /*
                 * Adding the filter as a name instead of a one-element array
                 * is crucial because otherwise Adobe Reader cannot read the
                 * cross-reference stream any more, cf. the PDF v1.5 Errata.
                 */
                pdf_add_dict(stream->dict, pdf_new_name("Filter"), filter_name);
        }

        buffer_length64 = (uint64_t) buffer_length;

        if (tectonic_flate_compress(
                buffer,
                &buffer_length64,
                filtered,
                filtered_length,
                p->options.compression.level
            ) < 0) {
            _tt_abort("Zlib error");
        }

        buffer_length = (size_t) buffer_length64;

        free(filtered);
        p->output.compression_saved += filtered_length - buffer_length
            - (filters ? strlen("/FlateDecode "): strlen("/Filter/FlateDecode\n"));

        filtered        = buffer;
        filtered_length = buffer_length;
    }

    /* AES will change the size of data! */
    if (p->state.enc_mode) {
        unsigned char *cipher = NULL;
        size_t         cipher_len = 0;
        pdf_encrypt_data(p->sec_data, filtered, filtered_length, &cipher, &cipher_len);
        free(filtered);
        filtered        = cipher;
        filtered_length = cipher_len;
    }


    pdf_add_dict(stream->dict,
                 pdf_new_name("Length"), pdf_new_number(filtered_length));

    pdf_write_obj(p, stream->dict);

    pdf_out_str(p, "\nstream\n", 8);

    if (filtered_length > 0)
        pdf_out_str(p, filtered, filtered_length);
    free(filtered);

    /*
     * This stream length "object" gets reset every time write_stream is
     * called for the stream object.
     * If this stream gets written more than once with different
     * filters, this could be a problem.
     */

    pdf_out_str(p, "\n", 1);
    pdf_out_str(p, "endstream", 9);
}

static void
release_stream (pdf_stream *stream)
{
    pdf_release_obj(stream->dict);
    stream->dict = NULL;

    stream->stream = mfree(stream->stream);

    stream->objstm_data = mfree(stream->objstm_data);

    free(stream);
}

pdf_obj *
pdf_stream_dict (pdf_obj *stream)
{
    pdf_stream *data;

    TYPECHECK(stream, PDF_STREAM);

    data = stream->data;

    return data->dict;
}

const void *
pdf_stream_dataptr (pdf_obj *stream)
{
    pdf_stream *data;

    TYPECHECK(stream, PDF_STREAM);

    data = stream->data;

    return (const void *) data->stream;
}

int
pdf_stream_length (pdf_obj *stream)
{
    pdf_stream *data;

    TYPECHECK(stream, PDF_STREAM);

    data = stream->data;

    return (int) data->stream_length;
}

static void
set_objstm_data (pdf_obj *objstm, int *data) {
    TYPECHECK(objstm, PDF_STREAM);

    ((pdf_stream *) objstm->data)->objstm_data = data;
}

static int *
get_objstm_data (pdf_obj *objstm) {
    TYPECHECK(objstm, PDF_STREAM);

    return ((pdf_stream *) objstm->data)->objstm_data;
}

void
pdf_add_stream (pdf_obj *stream, const void *stream_data, int length)
{
    pdf_stream *data;

    TYPECHECK(stream, PDF_STREAM);

    if (length < 1)
        return;
    data = stream->data;
    if (data->stream_length + length > data->max_length) {
        data->max_length += length + STREAM_ALLOC_SIZE;
        data->stream      = RENEW(data->stream, data->max_length, unsigned char);
    }
    memcpy(data->stream + data->stream_length, stream_data, length);
    data->stream_length += length;
}

static int
filter_get_DecodeParms_FlateDecode (struct decode_parms *parms, pdf_obj *dict)
{
    pdf_obj *tmp;

    assert(dict && parms);
    assert(PDF_OBJ_DICTTYPE(dict));

    /* Fill with default values */
    parms->predictor = 1;
    parms->colors    = 1;
    parms->bits_per_component = 8;
    parms->columns   = 1;

    tmp = pdf_deref_obj(pdf_lookup_dict(dict, "Predictor"));
    if (tmp) {
        parms->predictor = pdf_number_value(tmp);
        pdf_release_obj(tmp);
    }
    tmp = pdf_deref_obj(pdf_lookup_dict(dict, "Colors"));
    if (tmp) {
        parms->colors = pdf_number_value(tmp);
        pdf_release_obj(tmp);
    }
    tmp = pdf_deref_obj(pdf_lookup_dict(dict, "BitsPerComponent"));
    if (tmp) {
        parms->bits_per_component = pdf_number_value(tmp);
        pdf_release_obj(tmp);
    }
    tmp = pdf_deref_obj(pdf_lookup_dict(dict, "Columns"));
    if (tmp) {
        parms->columns = pdf_number_value(tmp);
        pdf_release_obj(tmp);
    }
    if (parms->bits_per_component != 1 &&
        parms->bits_per_component != 2 &&
        parms->bits_per_component != 4 &&
        parms->bits_per_component != 8 &&
        parms->bits_per_component != 16) {
        dpx_warning("Invalid BPC value in DecodeParms: %d", parms->bits_per_component);
        return -1;
    } else if (parms->predictor <= 0 || parms->colors <= 0 ||
               parms->columns <= 0)
        return -1;
    return 0;
}

/* From Xpdf version 3.04
 * I'm not sure if I properly ported... Untested.
 */
static int
filter_row_TIFF2 (unsigned char *dst, const unsigned char *src,
                  struct decode_parms *parms)
{
    const unsigned char *p = src;
    unsigned char *col;
    /* bits_per_component < 8 here */
    int  mask = (1 << parms->bits_per_component) - 1;
    int  inbuf, outbuf; /* 2 bytes buffer */
    int  i, ci, j, k, inbits, outbits;

    col = NEW(parms->colors, unsigned char);
    memset(col, 0, parms->colors);
    inbuf = outbuf = 0; inbits = outbits = 0;
    j = k = 0;
    for (i = 0; i < parms->columns; i++) {
        /* expanding each color component into an 8-bits bytes array */
        for (ci = 0; ci < parms->colors; ci++) {
            if (inbits < parms->bits_per_component) {
                /* need more byte */
                inbuf   = (inbuf << 8) | p[j++];
                inbits += 8;
            }
            /* predict current color component */
            col[ci]  = (unsigned char) ((col[ci] +
                                         (inbuf >> (inbits - parms->bits_per_component))) & mask);
            inbits  -= parms->bits_per_component; /* consumed bpc bits */
            /* append newly predicted color component value */
            outbuf   = (outbuf << parms->bits_per_component) | col[ci];
            outbits += parms->bits_per_component;
            if (outbits >= 8) { /* flush */
                dst[k++] = (unsigned char) (outbuf >> (outbits - 8));
                outbits -= 8;
            }
        }
    }
    if (outbits > 0) {
        dst[k] = (unsigned char) (outbuf << (8 - outbits));
    }
    free(col);

    return 0;
}

/* This routine is inefficient. Length is typically 4 for Xref streams.
 * Especially, calling pdf_add_stream() for each 4 bytes append is highly
 * inefficient.
 */
static pdf_obj *
filter_stream_decode_Predictor (const void *src, size_t srclen, struct decode_parms *parms)
{
    pdf_obj             *dst;
    const unsigned char *p = (const unsigned char *) src;
    const unsigned char *endptr = p + srclen;
    unsigned char *prev, *buf;
    int bits_per_pixel  = parms->colors * parms->bits_per_component;
    int bytes_per_pixel = (bits_per_pixel + 7) / 8;
    int length = (parms->columns * bits_per_pixel + 7) / 8;
    int i, error = 0;

    dst = pdf_new_stream(0);

    prev = NEW(length, unsigned char);
    buf  = NEW(length, unsigned char);

    memset(prev, 0, length);
    switch (parms->predictor) {
    case 1 : /* No prediction */
        pdf_add_stream(dst, src, srclen); /* Just copy */
        break;
    case 2: /* TIFF Predictor 2 */
    {
        if (parms->bits_per_component == 8) {
            while (p + length < endptr) {
                /* Same as PNG Sub */
                for (i = 0; i < length; i++) {
                    int pv = i - bytes_per_pixel >= 0 ? buf[i - bytes_per_pixel] : 0;
                    buf[i] = (unsigned char)(((int) p[i] + pv) & 0xff);
                }
                pdf_add_stream(dst, buf, length);
                p += length;
            }
        } else if (parms->bits_per_component == 16) {
            while (p + length < endptr) {
                for (i = 0; i < length; i += 2) {
                    int  b  = i - bytes_per_pixel;
                    char hi = b >= 0 ? buf[b] : 0;
                    char lo = b >= 0 ? buf[b + 1] : 0;
                    int  pv = (hi << 8) | lo;
                    int  cv = (p[i] << 8) | p[i + 1];
                    int  c  = pv + cv;
                    buf[i]     = (unsigned char) (c >> 8);
                    buf[i + 1] = (unsigned char) (c & 0xff);
                }
                pdf_add_stream(dst, buf, length);
                p += length;
            }
        } else { /* bits per component 1, 2, 4 */
            while (!error && p + length < endptr) {
                error = filter_row_TIFF2(buf, p, parms);
                if (!error) {
                    pdf_add_stream(dst, buf, length);
                    p += length;
                }
            }
        }
    }
    break;
    /* PNG predictors: first byte of each rows is predictor type */
    case 10: /* PNG None */
    case 11: /* PNG Sub on all rows */
    case 12: /* PNG UP on all rows */
    case 13: /* PNG Average on all rows */
    case 14: /* PNG Paeth on all rows */
    case 15: /* PNG optimun: prediction algorithm can change from line to line. */
    {
        int type = parms->predictor - 10;

        while (!error && p + length < endptr) {
            if (parms->predictor == 15)
                type = *p;
            else if (*p != type) {
                dpx_warning("Mismatched Predictor type in data stream.");
                error = -1;
            }
            p++;
            switch (type) {
            case 0: /* Do nothing just skip first byte */
                memcpy(buf, p, length);
                break;
            case 1:
                for (i = 0; i < length; i++) {
                    int pv = i - bytes_per_pixel >= 0 ? buf[i - bytes_per_pixel] : 0;
                    buf[i] = (unsigned char)(((int) p[i] + pv) & 0xff);
                }
                break;
            case 2:
                for (i = 0; i < length; i++) {
                    buf[i] = (unsigned char)(((int) p[i] + (int) prev[i]) & 0xff);
                }
                break;
            case 3:
                for (i = 0; i < length; i++) {
                    int up   = prev[i];
                    int left = i - bytes_per_pixel >= 0 ? buf[i - bytes_per_pixel] : 0;
                    int tmp  = floor((up + left) / 2);
                    buf[i] = (unsigned char)((p[i] + tmp) & 0xff);
                }
                break;
            case 4:
                for (i = 0; i < length; i++) {
                    int a = i - bytes_per_pixel >= 0 ? buf[i - bytes_per_pixel] : 0; /* left */
                    int b = prev[i]; /* above */
                    int c = i - bytes_per_pixel >= 0 ? prev[i - bytes_per_pixel] : 0; /* upper left */
                    int q = a + b - c;
                    int qa = q - a, qb = q - b, qc = q - c;
                    qa = qa < 0 ? -qa : qa;
                    qb = qb < 0 ? -qb : qb;
                    qc = qc < 0 ? -qc : qc;
                    if (qa <= qb && qa <= qc)
                        buf[i] = (unsigned char) (((int) p[i] + a) & 0xff);
                    else if (qb <= qc)
                        buf[i] = (unsigned char) (((int) p[i] + b) & 0xff);
                    else
                        buf[i] = (unsigned char) (((int) p[i] + c) & 0xff);
                }
                break;
            default:
                dpx_warning("Unknown PNG predictor type: %d", type);
                error = -1;
            }
            if (!error) {
                pdf_add_stream(dst, buf, length); /* highly inefficient */
                memcpy(prev, buf, length);
                p += length;
            }
        }
    }
    break;
    default:
        dpx_warning("Unknown Predictor type value :%d", parms->predictor);
        error = -1;
    }

    free(prev);
    free(buf);

    if (error) {
        pdf_release_obj(dst);
        dst = NULL;
    }

    return dst;
}

#define WBUF_SIZE 4096

static pdf_obj *
filter_stream_decode_FlateDecode(const void *data, size_t len, struct decode_parms *parms)
{
    pdf_obj *dst;
    pdf_obj *tmp;
    uint8_t  wbuf[WBUF_SIZE];
    void    *flate_handle;

    flate_handle = tectonic_flate_new_decompressor(data, (uint64_t) len);
    if (flate_handle == NULL) {
        dpx_warning("tectonic_flate_new_decompressor() failed");
        return NULL;
    }

    tmp = pdf_new_stream(0);

    for (;;) {
        uint64_t out_size = WBUF_SIZE;

        if (tectonic_flate_decompress_chunk(flate_handle, wbuf, &out_size)) {
            dpx_warning("tectonic_flate_decompress() failed");
            tectonic_flate_free_decompressor(flate_handle);
            pdf_release_obj(tmp);
            return NULL;
        }

        if (out_size == 0)
            break;

        pdf_add_stream(tmp, wbuf, out_size);
    }

    tectonic_flate_free_decompressor(flate_handle);

    if (parms) {
        dst = filter_stream_decode_Predictor(pdf_stream_dataptr(tmp), pdf_stream_length(tmp), parms);
    } else {
        dst = pdf_link_obj(tmp);
    }

    pdf_release_obj(tmp);
    return dst;
}

static pdf_obj *
filter_stream_decode_ASCIIHexDecode (const void *data, size_t len)
{
  pdf_obj       *dst;
  int            eod, error;
  const char    *p = (const char *) data;
  const char    *endptr = p + len;
  unsigned char *buf, ch;
  size_t         pos, n;

  buf = NEW((len+1)/2, unsigned char);
  skip_white(&p, endptr);
  ch = 0; n = 0; pos = 0; eod = 0; error = 0;
  while (p < endptr && !error && !eod) {
    char c1, val;
    c1 = p[0];
    if (c1 >= 'A' && c1 <= 'F') {
      val = c1 - 'A' + 10;
    } else if (c1 >= 'a' && c1 <= 'f') {
      val = c1 - 'a' + 10;
    } else if (c1 >= '0' && c1 <= '9') {
      val = c1 - '0';
    } else if (c1 == '>') {
      val = 0;
      eod = 1;
      if ((pos % 2) == 0)
        break;
    } else {
      error = -1;
      break;
    }
    if (pos % 2) {
      buf[n] = ch + val;
      n++;
      ch = 0;
    } else {
      ch = val << 4;
    }
    pos++; p++;
    skip_white(&p, endptr);
  }
  if (error || !eod) {
    dpx_warning("Invalid ASCIIHex data seen: %s", error ? "Invalid character" : "No EOD marker");
    dst = NULL;
  } else {
    dst = pdf_new_stream(0);
    pdf_add_stream(dst, buf, n);
  }
  free(buf);

  return dst;
}

/* Percent sign is not start of comment here.
 * We need this for reading Ascii85 encoded data.
 */
#define is_space(c) ((c) == ' '  || (c) == '\t' || (c) == '\f' || \
                    (c) == '\r' || (c) == '\n' || (c) == '\0')
static void
skip_white_a85 (const char **p, const char *endptr)
{
  while (*p < endptr && (is_space(**p))) {
    (*p)++;
  }
}

static pdf_obj *
filter_stream_decode_ASCII85Decode (const void *data, size_t len)
{
  pdf_obj       *dst = NULL; /* Tectonic: avoid uninitialized warning */
  int            eod, error;
  const char    *p = (const char *) data;
  const char    *endptr = p + len;
  unsigned char *buf;
  size_t         n;

  buf = NEW(((len+4)/5)*4, unsigned char);
  skip_white_a85(&p, endptr);
  n = 0; eod = 0; error = 0;
  while (p < endptr && !error && !eod) {
    char q[5] = {'u', 'u', 'u', 'u', 'u'};
    int  m;
    char ch;

    ch = p[0];
    p++;
    skip_white_a85(&p, endptr);
    if (ch == 'z') {
      memset(buf+n, 0, 4);
      n += 4;
      continue;
    } else if (ch == '~') {
      if (p < endptr && p[0] == '>') {
        eod = 1;
        p++;
      } else {
        error = -1;
      }
      break;
    }
    q[0] = ch;
    for (m = 1; m < 5 && p < endptr; m++) {
      ch = p[0];
      p++;
      skip_white_a85(&p, endptr);
      if (ch == '~') {
        if (p < endptr && p[0] == '>') {
          eod = 1;
          p++;
        } else {
          error = -1;
        }
        break;
      } else if (ch < '!' || ch > 'u') {
        error = -1;
        break;
      } else {
        q[m] = ch;
      }
    }
    if (!error) {
      uint32_t val = 0;
      int      i;
      if (m <= 1) {
        error = -1;
        break;
      }
      val = 85*85*85*(q[0] - '!') + 85*85*(q[1] - '!')
              + 85*(q[2] - '!') + (q[3] - '!');
      /* Check overflow */
      if (val > UINT32_MAX / 85) {
        error = -1;
        break;
      } else {
        val = 85 * val;
        if (val > UINT32_MAX - (q[4] - '!')) {
          error = -1;
          break;
        }
        val += (q[4] - '!');
      }
      if (!error) {
        for (i = 3; i >= 0; i--) {
          buf[n + i] = val & 0xff;
          val /= 256;
        }
        n += m - 1;
      }
    }
  }

  if (error) {
    dpx_warning("Error in reading ASCII85 data.");
  } else if (!eod) {
    dpx_warning("Error in reading ASCII85 data: No EOD");
    dst = NULL;
  } else {
    dst = pdf_new_stream(0);
    pdf_add_stream(dst, buf, n);
  }
  free(buf);

  return dst;
}

static pdf_obj *
filter_stream_decode (const char *filter_name, pdf_obj *src, pdf_obj *parm)
{
    pdf_obj    *dec;
    const char *stream_data;
    size_t stream_length;

    if (!filter_name)
        return pdf_link_obj(src);

    stream_data   = pdf_stream_dataptr(src);
    stream_length = pdf_stream_length(src);

  if (!strcmp(filter_name, "ASCIIHexDecode")) {
    dec = filter_stream_decode_ASCIIHexDecode(stream_data, stream_length);
  } else if (!strcmp(filter_name, "ASCII85Decode")) {
    dec = filter_stream_decode_ASCII85Decode(stream_data, stream_length);
  } else if (!strcmp(filter_name, "FlateDecode")) {
    struct decode_parms decode_parm;
    if (parm)
      filter_get_DecodeParms_FlateDecode(&decode_parm, parm);
    dec = filter_stream_decode_FlateDecode(stream_data, stream_length, parm ? &decode_parm : NULL);
  } else {
    dpx_warning("DecodeFilter \"%s\" not supported.", filter_name);
    dec = NULL;
  }

  return dec;
}

int
pdf_concat_stream (pdf_obj *dst, pdf_obj *src)
{
  pdf_obj *filtered;
  pdf_obj *stream_dict;
  pdf_obj *filter, *parms;
  int      error = 0;

  if (!PDF_OBJ_STREAMTYPE(dst) || !PDF_OBJ_STREAMTYPE(src)) {
    dpx_warning("Passed invalid type in pdf_concat_stream().");
    return -1;
  }

  stream_dict = pdf_stream_dict(src);

  filter = pdf_lookup_dict(stream_dict, "Filter");
  if (!filter) {
    pdf_add_stream(dst, pdf_stream_dataptr(src), pdf_stream_length(src));
    return 0;
  }
  if (pdf_lookup_dict(stream_dict, "DecodeParms")) {
    /* Dictionary or array */
    parms = pdf_deref_obj(pdf_lookup_dict(stream_dict, "DecodeParms"));
    if (!parms) {
      dpx_warning("Failed to deref DeocdeParms...");
      return -1;
    } else if (!PDF_OBJ_ARRAYTYPE(parms) && !PDF_OBJ_DICTTYPE(parms)) {
      dpx_warning("PDF dict or array expected for DecodeParms...");
      pdf_release_obj(parms);
      return -1;
    }
  } else {
    parms = NULL;
  }
  if (PDF_OBJ_ARRAYTYPE(filter)) {
    int      i, num;
    pdf_obj *prev = NULL;

    num = pdf_array_length(filter);
    if (parms) {
      if (!PDF_OBJ_ARRAYTYPE(parms) || pdf_array_length(parms) != num) {
        dpx_warning("Invalid DecodeParam object found.");
        pdf_release_obj(parms);
        return -1;
      }
    }
    if (num == 0) {
      filtered = pdf_link_obj(src);
    } else {
      filtered = NULL;
      prev = pdf_link_obj(src);
      for (i = 0; i < num && prev != NULL; i++) {
        pdf_obj *tmp1, *tmp2;

        tmp1 = pdf_deref_obj(pdf_get_array(filter, i));
        if (parms) {
          tmp2 = pdf_deref_obj(pdf_get_array(parms, i));
        } else {
          tmp2 = NULL;
        }
        if (PDF_OBJ_NAMETYPE(tmp1)) {
          filtered = filter_stream_decode(pdf_name_value(tmp1), prev, tmp2);
        } else if (PDF_OBJ_NULLTYPE(tmp1)) {
          filtered = pdf_link_obj(prev);
        } else {
          dpx_warning("Unexpected object found for /Filter...");
          filtered = NULL;
        }
        if (prev)
          pdf_release_obj(prev);
        if (tmp1)
          pdf_release_obj(tmp1);
        if (tmp2)
          pdf_release_obj(tmp2);
        prev = filtered;
      }
    }
  } else if (PDF_OBJ_NAMETYPE(filter)) {
    filtered = filter_stream_decode(pdf_name_value(filter), src, parms);
  } else {
    dpx_warning("Invalid value for /Filter found.");
    filtered = NULL;
  }
  if (parms)
    pdf_release_obj(parms);
  if (filtered) {
    pdf_add_stream(dst, pdf_stream_dataptr(filtered), pdf_stream_length(filtered));
    pdf_release_obj(filtered);
    error = 0;
  } else {
    error = -1;
  }

  return error;
}

static pdf_obj *
pdf_stream_uncompress (pdf_obj *src) {
    pdf_obj *dst = pdf_new_stream(0);

    TYPECHECK(src, PDF_STREAM);

    pdf_merge_dict(pdf_stream_dict(dst), pdf_stream_dict(src));
    pdf_remove_dict(pdf_stream_dict(dst), "Length");
    pdf_concat_stream(dst, src);

    return dst;
}


static void
pdf_write_obj (pdf_out *p, pdf_obj *object)
{
    assert(p);

    if (object == NULL) {
        write_null(p);
        return;
    }

    if (INVALIDOBJ(object) || PDF_OBJ_UNDEFINED(object))
        _tt_abort("pdf_write_obj: Invalid object, type = %d\n", object->type);

    switch (object->type) {
    case PDF_BOOLEAN:
        write_boolean(p, object->data);
        break;
    case PDF_NUMBER:
        write_number(p, object->data);
        break;
    case PDF_STRING:
        write_string(p, object->data);
        break;
    case PDF_NAME:
        write_name(p, object->data);
        break;
    case PDF_ARRAY:
        write_array(p, object->data);
        break;
    case PDF_DICT:
        write_dict(p, object->data);
        break;
    case PDF_STREAM:
        write_stream(p, object->data);
        break;
    case PDF_NULL:
        write_null(p);
        break;
    case PDF_INDIRECT:
        write_indirect(p, object->data);
        break;
    }
}

/* Write the object to the file */
static void
pdf_flush_obj (pdf_out *p, pdf_obj *object)
{
    size_t length;
    char buf[64];

    /*
     * Record file position
     */
    add_xref_entry(p, object->label, 1,
                   p->output.file_position, object->generation);
    length = sprintf(buf, "%u %hu obj\n", object->label, object->generation);
    p->state.enc_mode = p->options.enable_encrypt && !(object->flags & OBJ_NO_ENCRYPT) ? 1 : 0;
    if (p->state.enc_mode) {
        pdf_enc_set_label(p->sec_data, object->label);
        pdf_enc_set_generation(p->sec_data, object->generation);
    }
    pdf_out_str(p, buf, length);
    pdf_write_obj(p, object);
    pdf_out_str(p, "\nendobj\n", 8);
}

static int
pdf_add_objstm (pdf_out *p, pdf_obj *objstm, pdf_obj *object)
{
    int *data, pos;

    TYPECHECK(objstm, PDF_STREAM);
    assert(p);

    data = get_objstm_data(objstm);
    pos = ++data[0];

    data[2*pos]   = object->label;
    data[2*pos+1] = pdf_stream_length(objstm);

    add_xref_entry(p, object->label, 2, objstm->label, pos-1);

    /* redirect output into objstm */
    p->output_stream = objstm;
    p->state.enc_mode = false;
    pdf_write_obj(p, object);
    pdf_out_char(p, '\n');
    p->output_stream = NULL;

    return pos;
}

static void
release_objstm (pdf_obj *objstm)
{
    int *data = get_objstm_data(objstm);
    int pos = data[0];
    pdf_obj *dict;
    pdf_stream *stream;
    unsigned char *old_buf;
    size_t old_length;
    stream = (pdf_stream *) objstm->data;

    /* Precede stream data by offset table */
    old_buf = stream->stream;
    old_length = stream->stream_length;
    /* Reserve 22 bytes for each entry (two 10 digit numbers plus two spaces) */
    stream->stream = NEW(old_length + 22*pos, unsigned char);
    stream->stream_length = 0;

    {
        int i = 2*pos, *val = data+2;
        while (i--) {
            char buf[32];
            size_t length = sprintf(buf, "%d ", *(val++));
            pdf_add_stream(objstm, buf, length);
        }
    }

    dict = pdf_stream_dict(objstm);
    pdf_add_dict(dict, pdf_new_name("Type"), pdf_new_name("ObjStm"));
    pdf_add_dict(dict, pdf_new_name("N"), pdf_new_number(pos));
    pdf_add_dict(dict, pdf_new_name("First"), pdf_new_number(stream->stream_length));

    pdf_add_stream(objstm, old_buf, old_length);
    free(old_buf);
    pdf_release_obj(objstm);
}

#define is_free(b,c) (((b)[(c)/8]) & (1 << (7-((c)%8))))

void
pdf_release_obj (pdf_obj *object)
{
    pdf_out *p = current_output();

    if (object == NULL)
        return;
    if (INVALIDOBJ(object) || object->refcount <= 0) {
        dpx_message("\npdf_release_obj: object=%p, type=%d, refcount=%d\n",
             object, object->type, object->refcount);
        _tt_abort("pdf_release_obj:  Called with invalid object.");
    }

    object->refcount -= 1;

    if (object->refcount == 0) {
        /*
         * Nothing is using this object so it's okay to remove it.
         * Nonzero "label" means object needs to be written before it's destroyed.
         */
        if (object->label) {
            p->free_list[object->label/8] |= (1 << (7 - (object->label % 8)));

            if (p->output.handle != NULL) {
                if (!p->options.use_objstm || object->flags & OBJ_NO_OBJSTM
                    || (p->options.enable_encrypt && object->flags & OBJ_NO_ENCRYPT)
                    || object->generation)
                    pdf_flush_obj(p, object);
                else {
                    if (!p->current_objstm) {
                        int *data = NEW(2*OBJSTM_MAX_OBJS+2, int);
                        data[0] = data[1] = 0;
                        p->current_objstm = pdf_new_stream(STREAM_COMPRESS);
                        set_objstm_data(p->current_objstm, data);
                        pdf_label_obj(p, p->current_objstm);
                    }
                    if (pdf_add_objstm(p, p->current_objstm, object) == OBJSTM_MAX_OBJS) {
                        release_objstm(p->current_objstm);
                        p->current_objstm = NULL;
                    }
                }
            }
        }
        switch (object->type) {
        case PDF_BOOLEAN:
            release_boolean(object->data);
            break;
        case PDF_NULL:
            break;
        case PDF_NUMBER:
            release_number(object->data);
            break;
        case PDF_STRING:
            release_string(object->data);
            break;
        case PDF_NAME:
            release_name(object->data);
            break;
        case PDF_ARRAY:
            release_array(object->data);
            break;
        case PDF_DICT:
            release_dict(object->data);
            break;
        case PDF_STREAM:
            release_stream(object->data);
            break;
        case PDF_INDIRECT:
            release_indirect(object->data);
            break;
        }
        /* This might help detect freeing already freed objects */
        object->type = -1;
        object->data = NULL;
        free(object);
    }
}


/* PDF reading starts around here */

/* As each lines may contain null-characters, so outptr here is NOT
 * null-terminated string. Returns -1 for when EOF is already reached, and -2
 * if buffer has no enough space.
 */
static int
tt_mfreadln (char *buf, int size, rust_input_handle_t handle)
{
    int c;
    int len = 0;

    while ((c = ttstub_input_getc(handle)) != EOF && c != '\n' && c != '\r') {
        if (len >= size)
            return -2;
        buf[len++] = (char) c;
    }

    if (c == EOF && len == 0)
        return -1;

    if (c == '\r' && (c = ttstub_input_getc(handle)) >= 0 && (c != '\n'))
        ttstub_input_ungetc(handle, c);

    return len;
}

/* Reading external PDF files */

static int
backup_line (rust_input_handle_t handle)
{
    int ch = -1;

    /* Note: this code should work even if \r\n is eol. It could fail on a
     * machine where \n is eol and there is a \r in the stream --- Highly
     * unlikely in the last few bytes where this is likely to be used.
     */

    if (ttstub_input_seek(handle, 0, SEEK_CUR) > 1) {
        do
            ttstub_input_seek(handle, -2, SEEK_CUR);
        while (ttstub_input_seek(handle, 0, SEEK_CUR) > 0 &&
               (ch = ttstub_input_getc(handle)) >= 0 &&
               (ch != '\n' && ch != '\r' ));
    }

    if (ch < 0)
        return 0;
    return 1;
}

static int
find_xref (rust_input_handle_t handle, int file_size)
{
    size_t xref_pos = 0;
    int len, tries = 10;
    const char *start, *end;
    char *number;

    do {
        size_t currentpos;
        int n;

        if (!backup_line(handle)) {
            tries = 0;
            break;
        }

        currentpos = ttstub_input_seek(handle, 0, SEEK_CUR);
        n = MIN(strlen("startxref"), file_size - currentpos);
        ttstub_input_read(handle, work_buffer, n);
        ttstub_input_seek(handle, currentpos, SEEK_SET);
        tries--;
    } while (tries > 0 && !strstartswith(work_buffer, "startxref"));

    if (tries <= 0)
        return 0;

    /* Skip rest of this line */
    tt_mfgets(work_buffer, WORK_BUFFER_SIZE, handle);
    /* Next line of input file should contain actual xref location */
    len = tt_mfreadln(work_buffer, WORK_BUFFER_SIZE, handle);

    if (len <= 0) {
        dpx_warning("Reading xref location data failed... Not a PDF file?");
        return 0;
    }

    start = work_buffer;
    end   = start + len;
    skip_white(&start, end);
    number   = parse_number(&start, end);
    xref_pos = (int) atof(number);
    free(number);
    return xref_pos;
}

/*
 * This routine must be called with the file pointer located
 * at the start of the trailer.
 */
static pdf_obj *
parse_trailer (pdf_file *pf)
{
    pdf_obj *result;
    int cur_pos, nmax, nread;
    /*
     * Fill work_buffer and hope trailer fits. This should
     * be made a bit more robust sometime.
     */

    cur_pos = ttstub_input_seek(pf->handle, 0, SEEK_CUR);
    nmax = MIN(pf->file_size - cur_pos, WORK_BUFFER_SIZE);
    nread = ttstub_input_read(pf->handle, work_buffer, nmax);

    if (nread == 0 || !strstartswith(work_buffer, "trailer")) {
        dpx_warning("No trailer.  Are you sure this is a PDF file?");
        dpx_warning("buffer:\n->%s<-\n", work_buffer);
        result = NULL;
    } else {
        const char *p = work_buffer + strlen("trailer");
        skip_white(&p, work_buffer + nread);
        result = parse_pdf_dict(&p, work_buffer + nread, pf);
    }

    return result;
}

/*
 * This routine tries to estimate an upper bound for character position
 * of the end of the object, so it knows how big the buffer must be.
 * The parsing routines require that the entire object be read into
 * memory. It would be a major pain to rewrite them.  The worst case
 * is that an object before an xref table will grab the whole table
 * :-(
 */
static int
next_object_offset (pdf_file *pf, uint32_t obj_num)
{
    uint32_t next = pf->file_size;  /* Worst case */
    size_t i;
    uint32_t curr;

    curr = pf->xref_table[obj_num].field2;
    /* Check all other type 1 objects to find next one */
    for (i = 0; i < pf->num_obj; i++) {
        if (pf->xref_table[i].type == 1 &&
            pf->xref_table[i].field2 > curr &&
            pf->xref_table[i].field2 < next)
            next = pf->xref_table[i].field2;
    }

    return  next;
}

#define checklabel(pf, n, g) ((n) > 0 && (n) < (pf)->num_obj && (       \
                                  ((pf)->xref_table[(n)].type == 1 && (pf)->xref_table[(n)].field3 == (g)) || \
                                  ((pf)->xref_table[(n)].type == 2 && !(g))))

pdf_obj *
pdf_new_indirect (pdf_file *pf, uint32_t obj_num, uint16_t obj_gen)
{
    pdf_obj      *result;
    pdf_indirect *indirect;

    indirect = NEW(1, pdf_indirect);
    indirect->pf         = pf;
    indirect->obj        = NULL;
    indirect->label      = obj_num;
    indirect->generation = obj_gen;

    result   = pdf_new_obj(PDF_INDIRECT);
    result->data = indirect;

    return result;
}

static pdf_obj *
pdf_read_object (uint32_t obj_num, uint16_t obj_gen,
                 pdf_file *pf, size_t offset, size_t limit)
{
    pdf_obj *result = NULL;
    size_t   length;
    char    *buffer;
    const char *p, *endptr;

    length = limit - offset;

    if (length <= 0)
        return NULL;

    buffer = NEW(length + 1, char);

    ttstub_input_seek(pf->handle, offset, SEEK_SET);
    ttstub_input_read(pf->handle, buffer, length);

    p = buffer;
    endptr = p + length;

    /* Check for obj_num and obj_gen */
    {
        const char   *q = p; /* <== p */
        char         *sp;
        uint32_t  n, g;

        skip_white(&q, endptr);
        sp = parse_unsigned(&q, endptr);
        if (!sp) {
            free(buffer);
            return NULL;
        }
        n = strtoul(sp, NULL, 10);
        free(sp);

        skip_white(&q, endptr);
        sp = parse_unsigned(&q, endptr);
        if (!sp) {
            free(buffer);
            return NULL;
        }
        g = strtoul(sp, NULL, 10);
        free(sp);

        if (obj_num && (n != obj_num || g != obj_gen)) {
            free(buffer);
            return NULL;
        }

        p = q; /* ==> p */
    }


    skip_white(&p, endptr);
    if (memcmp(p, "obj", strlen("obj"))) {
        dpx_warning("Didn't find \"obj\".");
        free(buffer);
        return NULL;
    }
    p += strlen("obj");

    result = parse_pdf_object(&p, endptr, pf);

    skip_white(&p, endptr);
    if (memcmp(p, "endobj", strlen("endobj"))) {
        dpx_warning("Didn't find \"endobj\".");
        pdf_release_obj(result);
        result = NULL;
    }
    free(buffer);

    return result;
}

static pdf_obj *
read_objstm (pdf_file *pf, uint32_t num)
{
    size_t offset = pf->xref_table[num].field2;
    uint16_t gen = pf->xref_table[num].field3;
    size_t limit = next_object_offset(pf, num);
    int n, first, *header = NULL;
    char *data = NULL, *q;
    const char *p, *endptr;
    int i;
    pdf_obj *objstm, *dict, *type, *n_obj, *first_obj;

    objstm = pdf_read_object(num, gen, pf, offset, limit);

    if (!PDF_OBJ_STREAMTYPE(objstm))
        goto error;

    {
        pdf_obj *tmp = pdf_stream_uncompress(objstm);
        if (!tmp)
            goto error;
        pdf_release_obj(objstm);
        objstm = tmp;
    }

    dict = pdf_stream_dict(objstm);

    type = pdf_lookup_dict(dict, "Type");
    if (!PDF_OBJ_NAMETYPE(type) ||
        strcmp(pdf_name_value(type), "ObjStm"))
        goto error;

    n_obj = pdf_lookup_dict(dict, "N");
    if (!PDF_OBJ_NUMBERTYPE(n_obj))
        goto error;
    n = (int) pdf_number_value(n_obj);

    first_obj = pdf_lookup_dict(dict, "First");
    if (!PDF_OBJ_NUMBERTYPE(first_obj))
        goto error;
    first = (int) pdf_number_value(first_obj);
    /* reject object streams without object data */
    if (first >= pdf_stream_length(objstm))
        goto error;

    header = NEW(2*(n+1), int);
    set_objstm_data(objstm, header);
    *(header++) = n;
    *(header++) = first;

    /* avoid parsing beyond offset table */
    data = NEW(first + 1, char);
    memcpy(data, pdf_stream_dataptr(objstm), first);
    data[first] = 0;

    p      = data;
    endptr = p + first;
    i = 2*n;
    while (i--) {
        *(header++) = strtoul(p, &q, 10);
        if (q == p)
            goto error;
        p = q;
    }

    /* Any garbage after last entry? */
    skip_white(&p, endptr);
    if (p != endptr)
        goto error;
    free(data);

    return pf->xref_table[num].direct = objstm;

error:
    dpx_warning("Cannot parse object stream.");
    free(data);
    pdf_release_obj(objstm);
    return NULL;
}

/* Label without corresponding object definition are replaced by the
 * null object, as required by the PDF spec. This is important to parse
 * several cross-reference sections.
 */
static pdf_obj *
pdf_get_object (pdf_file *pf, uint32_t obj_num, uint16_t obj_gen)
{
    pdf_obj *result;

    if (!checklabel(pf, obj_num, obj_gen)) {
        dpx_warning("Trying to read nonexistent or deleted object: %u %hu",
             obj_num, obj_gen);
        return pdf_new_null();
    }

    if ((result = pf->xref_table[obj_num].direct)) {
        return pdf_link_obj(result);
    }

    if (pf->xref_table[obj_num].type == 1) {
        /* type == 1 */
        uint32_t offset;
        size_t limit;
        offset = pf->xref_table[obj_num].field2;
        limit  = next_object_offset(pf, obj_num);
        result = pdf_read_object(obj_num, obj_gen, pf, offset, limit);
    } else {
        /* type == 2 */
        uint32_t objstm_num = pf->xref_table[obj_num].field2;
        uint16_t index = pf->xref_table[obj_num].field3;
        pdf_obj *objstm;
        int  *data, n, first, length;
        const char *p, *q;

        if (objstm_num >= pf->num_obj ||
            pf->xref_table[objstm_num].type != 1 ||
            !((objstm = pf->xref_table[objstm_num].direct) ||
              (objstm = read_objstm(pf, objstm_num))))
            goto error;

        data = get_objstm_data(objstm);
        n = *(data++);
        first = *(data++);

        if (index >= n || data[2*index] != obj_num)
            goto error;

        length = pdf_stream_length(objstm);
        p = (const char *) pdf_stream_dataptr(objstm) + first + data[2*index+1];
        q = p + (index == n-1 ? length : first+data[2*index+3]);
        result = parse_pdf_object(&p, q, pf);
        if (!result)
            goto error;
    }

    /* Make sure the caller doesn't free this object */
    pf->xref_table[obj_num].direct = pdf_link_obj(result);

    return result;

error:
    dpx_warning("Could not read object from object stream.");
    return pdf_new_null();
}

#define OBJ_FILE(o) (((pdf_indirect *)((o)->data))->pf)
#define OBJ_OBJ(o)  (((pdf_indirect *)((o)->data))->obj)
#define OBJ_NUM(o)  (((pdf_indirect *)((o)->data))->label)
#define OBJ_GEN(o)  (((pdf_indirect *)((o)->data))->generation)

static pdf_obj *
pdf_new_ref (pdf_out *p, pdf_obj *object)
{
    pdf_obj *result;

    assert(p);

    if (object->label == 0) {
        pdf_label_obj(p, object);
    }
    result = pdf_new_indirect(NULL, object->label, object->generation);
    OBJ_OBJ(result) = object;
    return result;
}

/* pdf_deref_obj always returns a link instead of the original   */
/* It never return the null object, but the NULL pointer instead */
pdf_obj *
pdf_deref_obj (pdf_obj *obj)
{
    int count = PDF_OBJ_MAX_DEPTH;

    if (obj)
        obj = pdf_link_obj(obj);

    while (PDF_OBJ_INDIRECTTYPE(obj) && --count) {
        pdf_file *pf = OBJ_FILE(obj);
        if (pf) {
            uint32_t obj_num = OBJ_NUM(obj);
            uint16_t obj_gen = OBJ_GEN(obj);
            pdf_release_obj(obj);
            obj = pdf_get_object(pf, obj_num, obj_gen);
        } else {
            pdf_out *p = current_output();
            pdf_indirect *data = obj->data;

            if ((p->free_list[data->label/8] & (1 << (7-((data->label) % 8))))) {
                pdf_release_obj(obj);
                return NULL;
            } else {
                pdf_obj *next_obj = OBJ_OBJ(obj);
                if (!next_obj) {
                    _tt_abort("Undefined object reference");
                }
                pdf_release_obj(obj);
                obj = pdf_link_obj(next_obj);
            }
        }
    }

    if (!count)
        _tt_abort("Loop in object hierarchy detected. Broken PDF file?");

    if (PDF_OBJ_NULLTYPE(obj)) {
        pdf_release_obj(obj);
        return NULL;
    } else
        return obj;
}

static void
extend_xref (pdf_file *pf, size_t new_size)
{
    size_t i;

    pf->xref_table = RENEW(pf->xref_table, new_size, xref_entry);
    for (i = pf->num_obj; i < new_size; i++) {
        pf->xref_table[i].direct   = NULL;
        pf->xref_table[i].indirect = NULL;
        pf->xref_table[i].type     = 0;
        pf->xref_table[i].field3 = 0;
        pf->xref_table[i].field2 = 0L;
    }
    pf->num_obj = new_size;
}

/* Returns < 0 for error, 1 for success, and 0 when xref stream found. */
static int
parse_xref_table (pdf_file *pf, size_t xref_pos)
{
    const char *p, *endptr;
    char buf[256]; /* See, PDF ref. v.1.7, p.91 for "255+1" here. */
    int len;

    /*
     * This routine reads one xref segment. It may be called multiple times
     * on the same file.  xref tables sometimes come in pieces.
     */
    ttstub_input_seek(pf->handle, xref_pos, SEEK_SET);
    len = tt_mfreadln(buf, 255, pf->handle);

    /* We should have already checked that "startxref" section exists. So, EOF
     * here (len = -1) is impossible. We don't treat too long line case
     * seriously.
     */
    if (len < 0) {
        dpx_warning("Something went wrong while reading xref table...giving up.");
        return -1;
    }

    p = buf;
    endptr = buf + len;
    /* No skip_white() here. There should not be any white-spaces here. */
    if (memcmp(p, "xref", strlen("xref"))) {
        /* Might be an xref stream and not an xref table */
        return 0;
    }
    p += strlen("xref");
    skip_white(&p, endptr);
    if (p != endptr) {
        dpx_warning("Garbage after \"xref\" keyword found.");
        return -1;
    }

    /* Next line in file has first item and size of table */
    for (;;) {
        char         flag;
        size_t current_pos, size, offset;
        int          i;
        uint32_t     first, obj_gen;

        current_pos = ttstub_input_seek(pf->handle, 0, SEEK_CUR);
        len = tt_mfreadln(buf, 255, pf->handle);
        if (len == 0) /* empty line... just skip. */
            continue;
        else if (len < 0) {
            dpx_warning("Reading a line failed in xref table.");
            return -1;
        }

        p      = buf;
        endptr = buf + len;
        skip_white(&p, endptr);
        if (p == endptr) /* Only white-spaces and/or comment found. */
            continue;

        if (strstartswith(p, "trailer")) {
            /* Backup... This is ugly, but it seems like the safest thing to
             * do. It is possible the trailer dictionary starts on the same
             * logical line as the word trailer. In that case, the mfgets call
             * might have started to read the trailer dictionary and
             * parse_trailer would fail.
             */
            current_pos += p - buf; /* Jump to the beginning of "trailer" keyword. */
            ttstub_input_seek(pf->handle, current_pos, SEEK_SET);
            break;
        }

        /* Line containing something other than white-space characters found.
         *
         * Start reading xref subsection
         *
         * This section just reads two nusigned integers, namely, the object number
         * of first object and the size of the xref subsection. PDF reference says
         * that only "a space" is allowed between those two numbers but we allow
         * more white-space characters.
         */
        {
            char *q;

            /* Object number of the first object whithin this xref subsection. */
            q = parse_unsigned(&p, endptr);
            if (!q) {
                dpx_warning("An unsigned integer expected but could not find. (xref)");
                return -1;
            }
            first = atoi(q);
            free(q);
            skip_white(&p, endptr);

            /* Nnumber of objects in this xref subsection. */
            q = parse_unsigned(&p, endptr);
            if (!q) {
                dpx_warning("An unsigned integer expected but could not find. (xref)");
                return -1;
            }
            size = atoi(q);
            free(q);
            skip_white(&p, endptr);

            /* Check for unrecognized tokens */
            if (p != endptr) {
                dpx_warning("Unexpected token found in xref table.");
                return -1;
            }
        }

        /* The first line of a xref subsection OK. */
        if (pf->num_obj < first + size) {
            extend_xref(pf, first + size);
        }

        /* Start parsing xref subsection body... */
        for (i = first; i < first + size; ) {
            /* PDF spec. requires each xref subsection lines being exactly 20 bytes
             * long [including end-of-line marker(s)], offset 10 decimal digits,
             * generation number being 5 decimal digits, and each entries delimitted
             * by "a single space". However, we don't srtictly follow this rule:
             * More than one "white-spaces" allowed, can be ended with a comment,
             * and so on.
             */
            len = tt_mfreadln(buf, 255, pf->handle);
            if (len == 0) /* empty line...just skip. */
                continue;
            else if (len < 0) {
                dpx_warning("Something went wrong while reading xref subsection...");
                return -1;
            }
            p      = buf;
            endptr = buf + len;
            skip_white(&p, endptr);
            if (p == endptr) /* Only white-spaces and/or comment. */
                continue;

            /*
             * Don't overwrite positions that have already been set by a
             * modified xref table.  We are working our way backwards
             * through the reference table, so we only set "position"
             * if it hasn't been set yet.
             */
            offset = 0UL; obj_gen = 0; flag = 0;
            {
                char *q;

                /* Offset value -- 10 digits (0 padded) */
                q = parse_unsigned(&p, endptr);
                if (!q) {
                    dpx_warning("An unsigned integer expected but could not find. (xref)");
                    return -1;
                } else if (strlen(q) != 10) { /* exactly 10 digits */
                    dpx_warning(("Offset must be a 10 digits number. (xref)"));
                    free(q);
                    return -1;
                }
                /* FIXME: Possible overflow here. Consider using strtoll(). */
                offset = atoi(q);
                free(q);
                skip_white(&p, endptr);

                /* Generation number -- 5 digits (0 padded) */
                q = parse_unsigned(&p, endptr);
                if (!q) {
                    dpx_warning("An unsigned integer expected but could not find. (xref)");
                    return -1;
                } else if (strlen(q) != 5) { /* exactly 5 digits */
                    dpx_warning(("Expecting a 5 digits number. (xref)"));
                    free(q);
                    return -1;
                }
                obj_gen = atoi(q);
                free(q);
                skip_white(&p, endptr);
            }
            if (p == endptr) {
                dpx_warning("Unexpected EOL reached while reading a xref subsection entry.");
                return -1;
            }

            /* Flag -- a char */
            flag = *p; p++;
            skip_white(&p, endptr);
            if (p < endptr) {
                dpx_warning("Garbage in xref subsection entry found...");
                return -1;
            } else if (((flag != 'n' && flag != 'f') ||
                        (flag == 'n' &&
                         (offset >= pf->file_size || (offset > 0 && offset < 4))))) {
                dpx_warning("Invalid xref table entry [%u]. PDF file is corrupt...", i);
                return -1;
            }

            /* Everything seems to be OK. */
            if (!pf->xref_table[i].field2) {
                pf->xref_table[i].type   = (flag == 'n');
                pf->xref_table[i].field2 = offset;
                pf->xref_table[i].field3 = obj_gen;
            }
            i++;
        }
    }

    return  1;
}

static uint32_t
parse_xrefstm_field (const char **p, size_t length, uint32_t def)
{
    uint32_t val = 0;

    if (!length)
        return def;

    while (length--) {
        val <<= 8;
        val |= (unsigned char) *((*p)++);
    }

    return val;
}

static int
parse_xrefstm_subsec (pdf_file *pf,
                      const char **p, size_t *length,
                      int *W, int wsum,
                      int first, int size)
{
    xref_entry *e;

    /* Tectonic: impossible size_t comparison against 0
     * if ((*length -= wsum*size) < 0)
     *     return -1;
    */
    *length -= wsum*size;

    if (pf->num_obj < first+size)
        extend_xref(pf, first+size);  /* TODO: change! why? */

    e = pf->xref_table + first;
    while (size--) {
        uint8_t  type;
        uint32_t   field2;
        uint16_t field3;

        type = (unsigned char) parse_xrefstm_field(p, W[0], 1);
        if (type > 2)
            dpx_warning("Unknown cross-reference stream entry type.");

        field2 = (uint32_t)  parse_xrefstm_field(p, W[1], 0);
        field3 = (uint16_t) parse_xrefstm_field(p, W[2], 0);

        if (!e->field2) {
            e->type   = type;
            e->field2 = field2;
            e->field3 = field3;
        }
        e++;
    }

    return 0;
}

static int
parse_xref_stream (pdf_file *pf, size_t xref_pos, pdf_obj **trailer)
{
    pdf_obj *xrefstm, *size_obj, *W_obj, *index_obj;
    uint32_t size;
    size_t length;
    int W[3], i, wsum = 0;
    const char *p;

    xrefstm = pdf_read_object(0, 0, pf, xref_pos, pf->file_size);
    if (!PDF_OBJ_STREAMTYPE(xrefstm))
        goto error;

    {
        pdf_obj *tmp = pdf_stream_uncompress(xrefstm);
        if (!tmp)
            goto error;
        pdf_release_obj(xrefstm);
        xrefstm = tmp;
    }

    *trailer = pdf_link_obj(pdf_stream_dict(xrefstm));

    size_obj = pdf_lookup_dict(*trailer, "Size");
    if (!PDF_OBJ_NUMBERTYPE(size_obj))
        goto error;
    size = (uint32_t) pdf_number_value(size_obj);

    length = pdf_stream_length(xrefstm);

    W_obj = pdf_lookup_dict(*trailer, "W");
    if (!PDF_OBJ_ARRAYTYPE(W_obj) || pdf_array_length(W_obj) != 3)
        goto error;

    for (i = 0; i < 3; i++) {
        pdf_obj *tmp = pdf_get_array(W_obj, i);
        if (!PDF_OBJ_NUMBERTYPE(tmp))
            goto error;
        wsum += (W[i] = (int) pdf_number_value(tmp));
    }

    p = pdf_stream_dataptr(xrefstm);

    index_obj = pdf_lookup_dict(*trailer, "Index");
    if (index_obj) {
        size_t index_len;
        if (!PDF_OBJ_ARRAYTYPE(index_obj) ||
            ((index_len = pdf_array_length(index_obj)) % 2 ))
            goto error;

        i = 0;
        while (i < index_len) {
            pdf_obj *first = pdf_get_array(index_obj, i++);
            size_obj  = pdf_get_array(index_obj, i++);
            if (!PDF_OBJ_NUMBERTYPE(first) || !PDF_OBJ_NUMBERTYPE(size_obj) ||
                parse_xrefstm_subsec(pf, &p, &length, W, wsum,
                                     (int) pdf_number_value(first), (int) pdf_number_value(size_obj)))
                goto error;
        }
    } else if (parse_xrefstm_subsec(pf, &p, &length, W, wsum, 0, size))
        goto error;

    if (length)
        dpx_warning("Garbage in xref stream.");

    pdf_release_obj(xrefstm);

    return 1;

error:
    dpx_warning("Cannot parse cross-reference stream.");
    pdf_release_obj(xrefstm);
    if (*trailer) {
        pdf_release_obj(*trailer);
        *trailer = NULL;
    }
    return 0;
}

/* TODO: parse Version entry */
static pdf_obj *
read_xref (pdf_file *pf)
{
    pdf_obj *trailer = NULL, *main_trailer = NULL;
    size_t      xref_pos;

    if (!(xref_pos = find_xref(pf->handle, pf->file_size)))
        goto error;

    while (xref_pos) {
        pdf_obj *prev;

        int res = parse_xref_table(pf, xref_pos);
        if (res > 0) {
            /* cross-reference table */
            pdf_obj *xrefstm;

            if (!(trailer = parse_trailer(pf)))
                goto error;

            if (!main_trailer)
                main_trailer = pdf_link_obj(trailer);

            if ((xrefstm = pdf_lookup_dict(trailer, "XRefStm"))) {
                pdf_obj *new_trailer = NULL;
                if (PDF_OBJ_NUMBERTYPE(xrefstm) &&
                    parse_xref_stream(pf, (int) pdf_number_value(xrefstm),
                                      &new_trailer))
                    pdf_release_obj(new_trailer);
                else
                    dpx_warning("Skipping hybrid reference section.");
                /* Many PDF 1.5 xref streams use DecodeParms, which we cannot
                   parse. This way we can use at least xref tables in hybrid
                   documents. Or should we better stop parsing the file?
                */
            }

        } else if (!res && parse_xref_stream(pf, xref_pos, &trailer)) {
            /* cross-reference stream */
            if (!main_trailer)
                main_trailer = pdf_link_obj(trailer);
        } else
            goto error;

        if ((prev = pdf_lookup_dict(trailer, "Prev"))) {
            if (PDF_OBJ_NUMBERTYPE(prev))
                xref_pos = (size_t) pdf_number_value(prev);
            else
                goto error;
        } else
            xref_pos = 0;

        pdf_release_obj(trailer);
    }

    return main_trailer;

error:
    dpx_warning("Error while parsing PDF file.");
    pdf_release_obj(trailer);
    pdf_release_obj(main_trailer);
    return NULL;
}

static struct ht_table *pdf_files = NULL;

static pdf_file *
pdf_file_new (rust_input_handle_t handle)
{
    pdf_file *pf;

    assert(handle);

    pf = NEW(1, pdf_file);
    pf->handle = handle;
    pf->trailer = NULL;
    pf->xref_table = NULL;
    pf->catalog = NULL;
    pf->num_obj = 0;
    pf->version = 0;
    pf->file_size = ttstub_input_get_size(handle);

    ttstub_input_seek(handle, 0, SEEK_END);

    return pf;
}

static void
pdf_file_free (pdf_file *pf)
{
    size_t i;

    if (!pf) {
        return;
    }

    for (i = 0; i < pf->num_obj; i++) {
        pdf_release_obj(pf->xref_table[i].direct);
        pdf_release_obj(pf->xref_table[i].indirect);
    }

    free(pf->xref_table);
    pdf_release_obj(pf->trailer);
    pdf_release_obj(pf->catalog);

    free(pf);
}

void
pdf_files_init (void)
{
    pdf_files = NEW(1, struct ht_table);
    ht_init_table(pdf_files, (void (*)(void *)) pdf_file_free);
}

int
pdf_file_get_version (pdf_file *pf)
{
    assert(pf);
    return pf->version;
}

pdf_obj *
pdf_file_get_trailer (pdf_file *pf)
{
    assert(pf);
    return pdf_link_obj(pf->trailer);
}

/* FIXME:
 * pdf_file_get_trailer() does pdf_link_obj() but
 * pdf_file_get_catalog() does not. Why?
 */
pdf_obj *
pdf_file_get_catalog (pdf_file *pf)
{
    assert(pf);
    return pf->catalog;
}

pdf_file *
pdf_open (const char *ident, rust_input_handle_t handle)
{
    pdf_out *p = current_output();
    pdf_file *pf = NULL;

    assert(pdf_files);

    if (ident)
        pf = (pdf_file *) ht_lookup_table(pdf_files, ident, strlen(ident));

    if (pf) {
        pf->handle = handle;
    } else {
        pdf_obj *new_version;
        int version = check_for_pdf_version(handle);

        if (!dpx_conf.is_xbb) {
            int ver_major, ver_minor;

            ver_major = version / 10;
            ver_minor = version % 10;

            if (version < 10)
                dpx_warning("Unrecognized PDF version specified for input PDF file: %d.%d",
                    ver_major, ver_minor);
            else if (pdf_check_version(ver_major,  ver_minor) < 0) {
                dpx_warning("Trying to include PDF file with version (%d.%d), which is " \
                    "newer than current output PDF setting (%d.%d).",
                    ver_major, ver_minor, p->version.major, p->version.minor);
            }
        }

        pf = pdf_file_new(handle);
        pf->version = version;

        if (!(pf->trailer = read_xref(pf)))
            goto error;

        if (pdf_lookup_dict(pf->trailer, "Encrypt")) {
            dpx_warning("PDF document is encrypted.");
            goto error;
        }

        pf->catalog = pdf_deref_obj(pdf_lookup_dict(pf->trailer, "Root"));
        if (!PDF_OBJ_DICTTYPE(pf->catalog)) {
            dpx_warning("Cannot read PDF document catalog. Broken PDF file?");
            goto error;
        }

        new_version = pdf_deref_obj(pdf_lookup_dict(pf->catalog, "Version"));
        if (new_version) {
            unsigned int major, minor;

            if (!PDF_OBJ_NAMETYPE(new_version) ||
                sscanf(pdf_name_value(new_version), "%u.%u", &major, &minor) != 2) {
                pdf_release_obj(new_version);
                dpx_warning("Illegal Version entry in document catalog. Broken PDF file?");
                goto error;
            }

            if (pf->version < major*10+minor)
                pf->version = major*10+minor;

            pdf_release_obj(new_version);
        }

        if (ident)
            ht_append_table(pdf_files, ident, strlen(ident), pf);
    }

    return pf;

error:
    pdf_file_free(pf);
    return NULL;
}

void
pdf_close (pdf_file *pf)
{
    if (pf)
        pf->handle = NULL;
}

void
pdf_files_close (void)
{
    assert(pdf_files);
    ht_clear_table(pdf_files);
    free(pdf_files);
}

static int
check_for_pdf_version (rust_input_handle_t handle)
{
    char buffer[10] = "\0\0\0\0\0\0\0\0\0";
    unsigned int major, minor;

    ttstub_input_seek(handle, 0, SEEK_SET);
    if (ttstub_input_read(handle, buffer, sizeof(buffer) - 1) != sizeof(buffer) - 1)
        return -1;

    if (sscanf(buffer, "%%PDF-%u.%u", &major, &minor) != 2)
        return -1;

    return major*10+minor;
}

int
check_for_pdf (rust_input_handle_t handle)
{
    int version;

    version = check_for_pdf_version(handle);
    if (version < 0)  /* not a PDF file */
        return 0;

    if (version <= pdf_get_version())
        return 1;

    return 1;
}

static inline int
import_dict (pdf_obj *key, pdf_obj *value, void *pdata)
{
    pdf_obj *copy;
    pdf_obj *tmp;

    copy = (pdf_obj *) pdata;

    tmp  = pdf_import_object(value);
    if (!tmp) {
        return -1;
    }
    pdf_add_dict(copy, pdf_link_obj(key), tmp);

    return 0;
}

static pdf_obj *
pdf_import_indirect (pdf_obj *object)
{
    pdf_out *p = current_output();
    pdf_file *pf = OBJ_FILE(object);
    uint32_t obj_num = OBJ_NUM(object);
    uint16_t obj_gen = OBJ_GEN(object);
    pdf_obj *ref;

    assert(pf);

    if (!checklabel(pf, obj_num, obj_gen)) {
        dpx_warning("Can't resolve object: %u %u", obj_num, obj_gen);
        return pdf_new_null();
    }

    ref = pf->xref_table[obj_num].indirect;
    if (!ref) {
        pdf_obj *obj, *reserved, *imported;

        obj = pdf_get_object(pf, obj_num, obj_gen);
        if (!obj) {
            dpx_warning("Could not read object: %u %u", obj_num, obj_gen);
            return NULL;
        }

        /* Fix for circular reference issue
        *
        * Older version of dvipdfmx disallowed the following case of
        * circular reference:
        *   obj #1 --> << /Kids [2 0 R] >>
        *   obj #2 --> << /Parents [1 0 R] >>
        * The problem is in that dvipdfmx gives new labels to objects after they
        * are completely read.
        */
        reserved = pdf_new_null(); /* for reservation of label */
        pf->xref_table[obj_num].indirect = ref = pdf_new_ref(p, reserved);
        imported = pdf_import_object(obj);
        if (imported) {
            if (imported->label) {
                dpx_warning("Imported object already has a label: obj_id=%u", imported->label);
            }
            OBJ_OBJ(ref) = imported;
            imported->label = reserved->label;
            imported->generation = reserved->generation;
            reserved->label = 0;
            reserved->generation = 0;
            pdf_release_obj(imported);
        }

        pdf_release_obj(reserved);
        pdf_release_obj(obj);
    }

    return pdf_link_obj(ref);
}

/*
 * pdf_import_object recursively copies the object and those
 * referenced by it and changes the indirect references so that
 * they refer to the current output file. New indirect references
 * are remembered, which avoids duplicating objects when they
 * are imported several times.
 */
pdf_obj *
pdf_import_object (pdf_obj *object)
{
    pdf_obj  *imported;
    pdf_obj  *tmp;
    size_t i;

    switch (pdf_obj_typeof(object)) {

    case PDF_INDIRECT:
        if (OBJ_FILE(object)) {
            imported = pdf_import_indirect(object);
        } else {
            imported = pdf_link_obj(object);
        }
        break;

    case PDF_STREAM:
    {
        pdf_obj *stream_dict;

        tmp = pdf_import_object(pdf_stream_dict(object));
        if (!tmp)
            return NULL;

        imported    = pdf_new_stream(0);
        stream_dict = pdf_stream_dict(imported);
        pdf_merge_dict(stream_dict, tmp);
        pdf_release_obj(tmp);
        pdf_add_stream(imported,
                       pdf_stream_dataptr(object),
                       pdf_stream_length(object));
    }
    break;

    case PDF_DICT:

        imported = pdf_new_dict();
        if (pdf_foreach_dict(object, import_dict, imported) < 0) {
            pdf_release_obj(imported);
            return NULL;
        }

        break;

    case PDF_ARRAY:

        imported = pdf_new_array();
        for (i = 0; i < pdf_array_length(object); i++) {
            tmp = pdf_import_object(pdf_get_array(object, i));
            if (!tmp) {
                pdf_release_obj(imported);
                return NULL;
            }
            pdf_add_array(imported, tmp);
        }
        break;

    default:
        imported = pdf_link_obj(object);
    }

    return imported;
}


/* returns 0 if indirect references point to the same object */
int
pdf_compare_reference (pdf_obj *ref1, pdf_obj *ref2)
{
    pdf_indirect *data1, *data2;

    assert(PDF_OBJ_INDIRECTTYPE(ref1) && PDF_OBJ_INDIRECTTYPE(ref2));

    data1 = (pdf_indirect *) ref1->data;
    data2 = (pdf_indirect *) ref2->data;

    return data1->pf != data2->pf || data1->label != data2->label
        || data1->generation != data2->generation;
}

int
pdf_compare_object (pdf_obj *obj1, pdf_obj *obj2)
{
    int r = 0;

    if (!obj1 && !obj2) {
        return 0;
    } else if (!obj1 && obj2) {
        return 1;
    } else if (obj1 && !obj2) {
        return 1;
    } else if (pdf_obj_typeof(obj1) != pdf_obj_typeof(obj2)) {
        return 1;
    }

    switch (pdf_obj_typeof(obj1)) {
    case PDF_BOOLEAN:
        r = pdf_boolean_value(obj1) - pdf_boolean_value(obj2);
        break;
    case PDF_NUMBER:
        if (pdf_number_value(obj1) < pdf_number_value(obj2)) {
            r = -1;
        } else if (pdf_number_value(obj1) > pdf_number_value(obj2)) {
            r = 1;
        } else {
            r = 0;
        }
        break;
    case PDF_STRING:
        if (pdf_string_length(obj1) < pdf_string_length(obj2)) {
            r = -1;
        } else if (pdf_string_length(obj1) > pdf_string_length(obj2)) {
            r = 1;
        } else {
            r = memcmp(pdf_string_value(obj1), pdf_string_value(obj2), pdf_string_length(obj1));
        }
        break;
    case PDF_NAME:
        r = strcmp(pdf_name_value(obj1), pdf_name_value(obj2));
        break;
    case PDF_NULL:
        /* Always same */
        r = 0;
        break;
    case PDF_INDIRECT:
        r = pdf_compare_reference(obj1, obj2);
        break;
    case PDF_ARRAY:
        if (pdf_array_length(obj1) < pdf_array_length(obj2)) {
            r = -1;
        } else if (pdf_array_length(obj1) > pdf_array_length(obj2)) {
            r = 1;
        } else {
            int i;
            for (i = 0; r == 0 && i < pdf_array_length(obj1); i++) {
                pdf_obj *v1, *v2;
                v1 = pdf_get_array(obj1, i);
                v2 = pdf_get_array(obj2, i);
                r  = pdf_compare_object(v1, v2);
            }
        }
        break;
    case PDF_DICT:
        {
            pdf_obj *keys1, *keys2;
            keys1 = pdf_dict_keys(obj1);
            keys2 = pdf_dict_keys(obj2);
            r = pdf_compare_object(keys1, keys2);
            if (r == 0) {
                int i;
                for (i = 0; r == 0 && i < pdf_array_length(keys1); i++) {
                    pdf_obj *key, *v1, *v2;
                    key = pdf_get_array(keys1, i);
                    v1  = pdf_lookup_dict(obj1, pdf_name_value(key));
                    v2  = pdf_lookup_dict(obj2, pdf_name_value(key));
                    r   = pdf_compare_object(v1, v2);
                }
            }
            pdf_release_obj(keys1);
            pdf_release_obj(keys2);
        }
        break;
    case PDF_STREAM:
        /* Not seriously testing... */
        r = pdf_compare_object(pdf_stream_dict(obj1), pdf_stream_dict(obj2));
        if (r == 0) {
            size_t len1, len2;
            len1 = pdf_stream_length(obj1);
            len2 = pdf_stream_length(obj2);
            if (len1 < len2) {
                r = -1;
            } else if (len1 > len2) {
                r = 1;
            } else {
                r = 0;
            }
        }
        break;
    default:
        r = 1;
    }

    return r;
}

/* Tectonic new API: */
void
pdf_obj_reset_global_state(void)
{
    pdf_out *p = current_output();

    p->output.handle = NULL;
    p->output.file_position = 0;
    p->output.line_position = 0;
    p->output.compression_saved = 0;

    tectonic_pout_initialized = 0;
}
