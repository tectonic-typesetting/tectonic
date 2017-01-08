/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include <ctype.h>
#include <string.h>
/* floor and abs */
#include <math.h>

#include "system.h"
#include "mem.h"
#include "error.h"
#include "mfileio.h"
#include "dpxutil.h"
#include "pdflimits.h"
#include "pdfencrypt.h"
#include "pdfparse.h"

#ifdef HAVE_ZLIB
#include <zlib.h>
#endif /* HAVE_ZLIB */

#include "pdfobj.h"
#include "pdfdev.h"

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

  unsigned int   label;  /* Only used for indirect objects
			    all other "label" to zero */
  unsigned short generation;  /* Only used if "label" is used */
  unsigned refcount;  /* Number of links to this object */
  int      flags;
  void    *data;
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
  unsigned short length;
};

struct pdf_name
{
  char *name;
};

struct pdf_array
{
  unsigned int max;
  unsigned int size;
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
  unsigned int        stream_length;
  unsigned int        max_length;
  int32_t             _flags;
  struct decode_parms decodeparms;
};

struct pdf_indirect
{
  pdf_file      *pf;
  pdf_obj       *obj;             /* used when PF == NULL */
  unsigned       label;
  unsigned short generation;
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

static FILE *pdf_output_file = NULL;

static int pdf_output_file_position = 0;
static int pdf_output_line_position = 0;
static int compression_saved        = 0;

#define FORMAT_BUF_SIZE 4096
static char format_buffer[FORMAT_BUF_SIZE];

typedef struct xref_entry
{
  unsigned char  type;       /* object storage type              */
  unsigned int   field2;     /* offset in file or object stream  */
  unsigned short field3;     /* generation or index              */
  pdf_obj       *direct;     /* used for imported objects        */
  pdf_obj       *indirect;   /* used for imported objects        */
} xref_entry;

static xref_entry *output_xref;

static unsigned int pdf_max_ind_objects;
static unsigned int next_label;

static unsigned int startxref;

struct pdf_file
{
  FILE       *file;
  pdf_obj    *trailer;
  xref_entry *xref_table;
  pdf_obj    *catalog;
  int         num_obj;
  int         file_size;
  int         version;
};

static pdf_obj *output_stream;

#define OBJSTM_MAX_OBJS  200
/* the limit is only 100 for linearized PDF */

static int enc_mode;
static int doc_enc_mode;

static pdf_obj *trailer_dict;
static pdf_obj *xref_stream;

/* Internal static routines */

static int check_for_pdf_version (FILE *file);

static void pdf_flush_obj (pdf_obj *object, FILE *file);
static void pdf_label_obj (pdf_obj *object);
static void pdf_write_obj (pdf_obj *object, FILE *file);

static void  set_objstm_data (pdf_obj *objstm, int *data);
static int  *get_objstm_data (pdf_obj *objstm);
static void  release_objstm  (pdf_obj *objstm);

static void pdf_out_char (FILE *file, char c);
static void pdf_out      (FILE *file, const void *buffer, int length);

static pdf_obj *pdf_new_ref  (pdf_obj *object);
static void release_indirect (pdf_indirect *data);
static void write_indirect   (pdf_indirect *indirect, FILE *file);

static void release_boolean (pdf_obj *data);
static void write_boolean   (pdf_boolean *data, FILE *file);

static void write_null   (FILE *file);

static void release_number (pdf_number *number);
static void write_number   (pdf_number *number, FILE *file);

static void write_string   (pdf_string *str, FILE *file);
static void release_string (pdf_string *str);

static void write_name   (pdf_name *name, FILE *file);
static void release_name (pdf_name *name);

static void write_array   (pdf_array *array, FILE *file);
static void release_array (pdf_array *array);

static void write_dict   (pdf_dict *dict, FILE *file);
static void release_dict (pdf_dict *dict);

static void write_stream   (pdf_stream *stream, FILE *file);
static void release_stream (pdf_stream *stream);

static int  verbose = 0;
static char compression_level = 9;
static char compression_use_predictor = 1;

void
pdf_set_compression (int level)
{
#ifndef   HAVE_ZLIB
  ERROR("You don't have compression compiled in. Possibly libz wasn't found by configure.");
#else
#ifndef HAVE_ZLIB_COMPRESS2
  if (level != 0) 
    WARN("Unable to set compression level -- your zlib doesn't have compress2().");
#endif
  if (level >= 0 && level <= 9) 
    compression_level = level;
  else {
    ERROR("set_compression: invalid compression level: %d", level);
  }
#endif /* !HAVE_ZLIB */

  return;
}

void
pdf_set_use_predictor (int bval)
{
  compression_use_predictor = bval ? 1 : 0;
}

static unsigned pdf_version = PDF_VERSION_DEFAULT;

void
pdf_set_version (unsigned version)
{
  /* Don't forget to update CIDFont_stdcc_def[] in cid.c too! */
  if (version >= PDF_VERSION_MIN && version <= PDF_VERSION_MAX) {
    pdf_version = version;
  }
}

unsigned
pdf_get_version (void)
{
  return pdf_version;
}

int
pdf_obj_get_verbose(void)
{
  return verbose;
}

void
pdf_obj_set_verbose(void)
{
  verbose++;
}

static pdf_obj *current_objstm = NULL;
static int do_objstm;

static void
add_xref_entry (unsigned label, unsigned char type, unsigned int field2, unsigned short field3)
{
  if (label >= pdf_max_ind_objects) {
    pdf_max_ind_objects = (label/IND_OBJECTS_ALLOC_SIZE+1)*IND_OBJECTS_ALLOC_SIZE;
    output_xref = RENEW(output_xref, pdf_max_ind_objects, xref_entry);
  }

  output_xref[label].type   = type;
  output_xref[label].field2 = field2;
  output_xref[label].field3 = field3;
  output_xref[label].direct   = NULL;
  output_xref[label].indirect = NULL;
}

#define BINARY_MARKER "%\344\360\355\370\n"
void
pdf_out_init (const char *filename, int do_encryption, int enable_objstm)
{
  char v;

  output_xref = NULL;
  pdf_max_ind_objects = 0;
  add_xref_entry(0, 0, 0, 0xffff);
  next_label = 1;

  if (pdf_version >= 5) {
    if (enable_objstm) {
      xref_stream = pdf_new_stream(STREAM_COMPRESS);
      xref_stream->flags |= OBJ_NO_ENCRYPT;
      trailer_dict = pdf_stream_dict(xref_stream);
      pdf_add_dict(trailer_dict, pdf_new_name("Type"), pdf_new_name("XRef"));
      do_objstm = 1;
    } else {
      trailer_dict = pdf_new_dict();
      do_objstm = 0;
    }
  } else {
    xref_stream = NULL;
    trailer_dict = pdf_new_dict();
    do_objstm = 0;
  }

  output_stream = NULL;

  if (filename == NULL) { /* no filename: writing to stdout */
#ifdef WIN32
    setmode(fileno(stdout), _O_BINARY);
#endif
    pdf_output_file = stdout;
  } else {
    pdf_output_file = MFOPEN(filename, FOPEN_WBIN_MODE);
    if (!pdf_output_file) {
      if (strlen(filename) < 128)
        ERROR("Unable to open \"%s\".", filename);
      else
        ERROR("Unable to open file.");
    }
  }
  pdf_out(pdf_output_file, "%PDF-1.", strlen("%PDF-1."));
  v = '0' + pdf_version;
  pdf_out(pdf_output_file, &v, 1);
  pdf_out(pdf_output_file, "\n", 1);
  pdf_out(pdf_output_file, BINARY_MARKER, strlen(BINARY_MARKER));

  enc_mode = 0;
  doc_enc_mode = do_encryption;
}

static void
dump_xref_table (void)
{
  int length;
  unsigned int i;

  pdf_out(pdf_output_file, "xref\n", 5);

  length = sprintf(format_buffer, "%d %u\n", 0, next_label);
  pdf_out(pdf_output_file, format_buffer, length);

  /*
   * Every space counts.  The space after the 'f' and 'n' is * *essential*.
   * The PDF spec says the lines must be 20 characters long including the
   * end of line character.
   */
  for (i = 0; i < next_label; i++) {
    unsigned char type = output_xref[i].type;
    if (type > 1)
      ERROR("object type %hu not allowed in xref table", type);
    length = sprintf(format_buffer, "%010u %05hu %c \n",
		     output_xref[i].field2, output_xref[i].field3,
		     type ? 'n' : 'f');
    pdf_out(pdf_output_file, format_buffer, length);
  }
}

static void
dump_trailer_dict (void)
{
  pdf_out(pdf_output_file, "trailer\n", 8);
  enc_mode = 0;
  write_dict(trailer_dict->data, pdf_output_file);
  pdf_release_obj(trailer_dict);
  pdf_out_char(pdf_output_file, '\n');
}

/*
 * output a PDF 1.5 cross-reference stream;
 * contributed by Matthias Franz (March 21, 2007)
 */
static void
dump_xref_stream (void)
{
  unsigned int pos, i;
  unsigned poslen;
  unsigned char buf[7] = {0, 0, 0, 0, 0};

  pdf_obj *w;

  /* determine the necessary size of the offset field */
  pos = startxref; /* maximal offset value */
  poslen = 1;
  while (pos >>= 8)
    poslen++;

  w = pdf_new_array();
  pdf_add_array(w, pdf_new_number(1));      /* type                */
  pdf_add_array(w, pdf_new_number(poslen)); /* offset (big-endian) */
  pdf_add_array(w, pdf_new_number(2));      /* generation          */
  pdf_add_dict(trailer_dict, pdf_new_name("W"), w);

  /* We need the xref entry for the xref stream right now */
  add_xref_entry(next_label-1, 1, startxref, 0);

  for (i = 0; i < next_label; i++) {
    unsigned j;
    unsigned short f3;
    buf[0] = output_xref[i].type;
    pos = output_xref[i].field2;
    for (j = poslen; j--; ) {
      buf[1+j] = (unsigned char) pos;
      pos >>= 8;
    }
    f3 = output_xref[i].field3;
    buf[poslen+1] = (unsigned char) (f3 >> 8);
    buf[poslen+2] = (unsigned char) (f3);
    pdf_add_stream(xref_stream, &buf, poslen+3);
  }

  pdf_release_obj(xref_stream);
}

#if defined(LIBDPX)
long
pdf_output_stats (void)
{
  return pdf_output_file_position;
}
#endif /* LIBDPX */

void
pdf_out_flush (void)
{
  if (pdf_output_file) {
    int length;

    /* Flush current object stream */
    if (current_objstm) {
      release_objstm(current_objstm);
      current_objstm =NULL;
    }

    /*
     * Label xref stream - we need the number of correct objects
     * for the xref stream dictionary (= trailer).
     * Labelling it in pdf_out_init (with 1)  does not work (why?).
     */
    if (xref_stream)
      pdf_label_obj(xref_stream);

    /* Record where this xref is for trailer */
    startxref = pdf_output_file_position;

    pdf_add_dict(trailer_dict, pdf_new_name("Size"),
		 pdf_new_number(next_label));

    if (xref_stream)
      dump_xref_stream();
    else {
      dump_xref_table();
      dump_trailer_dict();
    }

    /* Done with xref table */
    RELEASE(output_xref);

    pdf_out(pdf_output_file, "startxref\n", 10);
    length = sprintf(format_buffer, "%u\n", startxref);
    pdf_out(pdf_output_file, format_buffer, length);
    pdf_out(pdf_output_file, "%%EOF\n", 6);

#if !defined(LIBDPX)
    MESG("\n");
#endif /* !LIBDPX */
    if (verbose) {
      if (compression_level > 0) {
	MESG("Compression saved %ld bytes%s\n", compression_saved,
	     pdf_version < 5 ? ". Try \"-V 5\" for better compression" : "");
      }
    }
#if !defined(LIBDPX)
    MESG("%ld bytes written", pdf_output_file_position);
#endif /* !LIBDPX */

    MFCLOSE(pdf_output_file);
  }
}

void
pdf_error_cleanup (void)
{
  /*
   * This routine is the cleanup required for an abnormal exit.
   * For now, simply close the file.
   */
  if (pdf_output_file)
    MFCLOSE(pdf_output_file);
}


void
pdf_set_root (pdf_obj *object)
{
  if (pdf_add_dict(trailer_dict, pdf_new_name("Root"), pdf_ref_obj(object))) {
    ERROR("Root object already set!");
  }
  /* Adobe Readers don't like a document catalog inside an encrypted
   * object stream, although the PDF v1.5 spec seems to allow this.
   * Note that we don't set OBJ_NO_ENCRYPT since the name dictionary in
   * a document catalog may contain strings, which should be encrypted.
   */
  if (doc_enc_mode)
    object->flags |= OBJ_NO_OBJSTM;
}

void
pdf_set_info (pdf_obj *object)
{
  if (pdf_add_dict(trailer_dict, pdf_new_name("Info"), pdf_ref_obj(object))) {
    ERROR ("Info object already set!");
  }
}

void
pdf_set_id (pdf_obj *id)
{
  if (pdf_add_dict(trailer_dict, pdf_new_name("ID"), id)) {
    ERROR ("ID already set!");
  }
}

void
pdf_set_encrypt (pdf_obj *encrypt)
{
  if (pdf_add_dict(trailer_dict, pdf_new_name("Encrypt"), pdf_ref_obj(encrypt))) {
    ERROR("Encrypt object already set!");
  }
  encrypt->flags |= OBJ_NO_ENCRYPT;
}

static
void pdf_out_char (FILE *file, char c)
{
  if (output_stream && file ==  pdf_output_file)
    pdf_add_stream(output_stream, &c, 1);
  else {
    fputc(c, file);
    /* Keep tallys for xref table *only* if writing a pdf file. */
    if (file == pdf_output_file) {
      pdf_output_file_position += 1;
      if (c == '\n')
        pdf_output_line_position  = 0;
      else
        pdf_output_line_position += 1;
    }
  }
}

static char xchar[] = "0123456789abcdef";

#define pdf_out_xchar(f,c) do {\
  pdf_out_char((f), xchar[((c) >> 4) & 0x0f]);\
  pdf_out_char((f), xchar[(c) & 0x0f]);\
} while (0)

static
void pdf_out (FILE *file, const void *buffer, int length)
{
  if (output_stream && file ==  pdf_output_file)
    pdf_add_stream(output_stream, buffer, length);
  else {
    fwrite(buffer, 1, length, file);
    /* Keep tallys for xref table *only* if writing a pdf file */
    if (file == pdf_output_file) {
      pdf_output_file_position += length;
      pdf_output_line_position += length;
      /* "foo\nbar\n "... */
      if (length > 0 &&
	((const char *)buffer)[length-1] == '\n')
        pdf_output_line_position = 0;
    }
  }
}

/*  returns 1 if a white-space character is necessary to separate
    an object of type1 followed by an object of type2              */
static
int pdf_need_white (int type1, int type2)
{
  return !(type1 == PDF_STRING || type1 == PDF_ARRAY || type1 == PDF_DICT ||
	   type2 == PDF_STRING || type2 == PDF_NAME ||
	   type2 == PDF_ARRAY || type2 == PDF_DICT);
}

static
void pdf_out_white (FILE *file)
{
  if (file == pdf_output_file && pdf_output_line_position >= 80) {
    pdf_out_char(file, '\n');
  } else {
    pdf_out_char(file, ' ');
  }
}

#define TYPECHECK(o,t) if (!(o) || (o)->type != (t)) {\
  ERROR("typecheck: Invalid object type: %d %d (line %d)", (o) ? (o)->type : -1, (t), __LINE__);\
}

#define INVALIDOBJ(o)  ((o) == NULL || (o)->type <= 0 || (o)->type > PDF_UNDEFINED)

static pdf_obj *
pdf_new_obj(int type)
{
  pdf_obj *result;

  if (type > PDF_UNDEFINED || type < 0)
    ERROR("Invalid object type: %d", type);

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
pdf_label_obj (pdf_obj *object)
{
  if (INVALIDOBJ(object))
    ERROR("pdf_label_obj(): passed invalid object.");

  /*
   * Don't change label on an already labeled object. Ignore such calls.
   */
  if (object->label == 0) {
    object->label      = next_label++;
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
  ASSERT(dst && !dst->label && src);

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
    ERROR("pdf_link_obj(): passed invalid object.");

  object->refcount += 1;

  return object;
}


pdf_obj *
pdf_ref_obj (pdf_obj *object)
{
  if (INVALIDOBJ(object))
    ERROR("pdf_ref_obj(): passed invalid object.");
  
  if (object->refcount == 0) {
    MESG("\nTrying to refer already released object!!!\n");
    pdf_write_obj(object, stderr);
    ERROR("Cannot continue...");
  }

  if (PDF_OBJ_INDIRECTTYPE(object)) {
    return pdf_link_obj(object);
  } else {
    return pdf_new_ref(object);
  }
}

static void
release_indirect (pdf_indirect *data)
{
  RELEASE(data);
}

static void
write_indirect (pdf_indirect *indirect, FILE *file)
{
  int length;

  ASSERT(!indirect->pf);

  length = sprintf(format_buffer, "%u %hu R", indirect->label, indirect->generation);
  pdf_out(file, format_buffer, length);
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
write_null (FILE *file)
{
  pdf_out(file, "null", 4);
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
  RELEASE (data);
}

static void
write_boolean (pdf_boolean *data, FILE *file)
{
  if (data->value) {
    pdf_out(file, "true", 4);
  } else {
    pdf_out(file, "false", 5);
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
  RELEASE (data);
}

static void
write_number (pdf_number *number, FILE *file)
{
  int count;

  count = pdf_sprint_number(format_buffer, number->value);

  pdf_out(file, format_buffer, count);
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
pdf_new_string (const void *str, unsigned length)
{
  pdf_obj    *result;
  pdf_string *data;

  ASSERT(str);

  result = pdf_new_obj(PDF_STRING);
  data   = NEW(1, pdf_string);
  result->data = data;
  data->length = length;

  if (length) {
    data->string = NEW(length+1, unsigned char);
    memcpy(data->string, str, length);
    /* Shouldn't assume NULL terminated. */
    data->string[length] = '\0';
  } else
    data->string = NULL;

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

unsigned
pdf_string_length (pdf_obj *object)
{
  pdf_string *data;

  TYPECHECK(object, PDF_STRING);

  data = object->data;

  return (unsigned) (data->length);
}

/*
 * This routine escapes non printable characters and control
 * characters in an output string.
 */
int
pdfobj_escape_str (char *buffer, int bufsize, const unsigned char *s, int len)
{
  int result = 0;
  int i;

  for (i = 0; i < len; i++) {
    unsigned char ch;

    ch = s[i];
    if (result > bufsize - 4)
      ERROR("pdfobj_escape_str: Buffer overflow");

    /*
     * We always write three octal digits. Optimization only gives few Kb
     * smaller size for most documents when zlib compressed.
     */
    if (ch < 32 || ch > 126) {
      buffer[result++] = '\\';
#if 0
      if (i < len - 1 && !isdigit(s[i+1]))
	result += sprintf(buffer+result, "%o", ch);
      else
	result += sprintf(buffer+result, "%03o", ch);
#endif
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
write_string (pdf_string *str, FILE *file)
{
  unsigned char *s = NULL;
  char wbuf[FORMAT_BUF_SIZE]; /* Shouldn't use format_buffer[]. */
  int  nescc = 0, i, count;
  size_t len = 0;

  if (enc_mode) {
    pdf_encrypt_data(str->string, str->length, &s, &len);
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
    pdf_out_char(file, '<');
    for (i = 0; i < len; i++) {
      pdf_out_xchar(file, s[i]);
    }
    pdf_out_char(file, '>');
  } else {
    pdf_out_char(file, '(');
    /*
     * This section of code probably isn't speed critical.  Escaping the
     * characters in the string one at a time may seem slow, but it's
     * safe if the formatted string length exceeds FORMAT_BUF_SIZE.
     * Occasionally you see some long strings in PDF.  pdfobj_escape_str
     * is also used for strings of text with no kerning.  These must be
     * handled as quickly as possible since there are so many of them.
     */ 
    for (i = 0; i < len; i++) {
      count = pdfobj_escape_str(wbuf, FORMAT_BUF_SIZE, &(s[i]), 1);
      pdf_out(file, wbuf, count);
    }
    pdf_out_char(file, ')');
  }
  if (enc_mode && s)
    RELEASE(s);
}

static void
release_string (pdf_string *data)
{
  if (data->string != NULL) {
    RELEASE(data->string);
    data->string = NULL;
  }
  RELEASE(data);
}

void
pdf_set_string (pdf_obj *object, unsigned char *str, unsigned length)
{
  pdf_string *data;

  TYPECHECK(object, PDF_STRING);

  data = object->data;
  if (data->string != 0) {
    RELEASE(data->string);
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
  unsigned  length;
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
write_name (pdf_name *name, FILE *file)
{
  char *s;
  int i, length;

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
#define is_delim(c) ((c) == '(' || (c) == ')' || \
                     (c) == '/' || \
                     (c) == '<' || (c) == '>' || \
                     (c) == '[' || (c) == ']' || \
                     (c) == '{' || (c) == '}' || \
                     (c) == '%')
#endif
  pdf_out_char(file, '/');
  for (i = 0; i < length; i++) {
    if (s[i] < '!' || s[i] > '~' || s[i] == '#' || is_delim(s[i])) {
      /*     ^ "space" is here. */
      pdf_out_char (file, '#');
      pdf_out_xchar(file, s[i]);
    } else {
      pdf_out_char (file, s[i]);
    }
  }
}

static void
release_name (pdf_name *data)
{
  if (data->name != NULL) {
    RELEASE(data->name);
    data->name = NULL;
  }
  RELEASE(data);
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
write_array (pdf_array *array, FILE *file)
{
  pdf_out_char(file, '[');
  if (array->size > 0) {
    unsigned int i;
    int type1 = PDF_UNDEFINED, type2;
    
    for (i = 0; i < array->size; i++) {
      if (array->values[i]) {
	type2 = array->values[i]->type;
	if (type1 != PDF_UNDEFINED && pdf_need_white(type1, type2))
	  pdf_out_white(file);
	type1 = type2;
	pdf_write_obj(array->values[i], file);
      } else
	WARN("PDF array element #ld undefined.", i);
    }
  }
  pdf_out_char(file, ']');
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

unsigned int
pdf_array_length (pdf_obj *array)
{
  pdf_array *data;

  TYPECHECK(array, PDF_ARRAY);

  data = (pdf_array *) array->data;

  return (unsigned int) data->size;
}

static void
release_array (pdf_array *data)
{
  unsigned int i;

  if (data->values) {
    for (i = 0; i < data->size; i++) {
      pdf_release_obj(data->values[i]);
      data->values[i] = NULL;
    }
    RELEASE(data->values);
    data->values = NULL;
  }
  RELEASE(data);
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

#if 0
void
pdf_put_array (pdf_obj *array, unsigned idx, pdf_obj *object)
{
  pdf_array *data;
  int        i;

  TYPECHECK(array, PDF_ARRAY);

  data = array->data;
  if (idx + 1 > data->max) {
    data->max   += ARRAY_ALLOC_SIZE;
    data->values = RENEW(data->values, data->max, pdf_obj *);
  }
  /*
   * Rangecheck error in PostScript interpreters if
   * idx > data->size - 1. But pdf_new_array() doesn't set
   * array size, pdf_add_array() dynamically increases size
   * of array. This might confusing...
   */
  if (idx + 1 > data->size) {
    for (i = data->size; i < idx; i++)
      data->values[i] = pdf_new_null(); /* release_array() won't work without this */
    data->values[idx] = object;
    data->size = idx + 1;
  } else {
    if (data->values[idx])
      pdf_release_obj(data->values[idx]);
    data->values[idx] = object;
  }
}

/* Easily leaks memory... */
pdf_obj *
pdf_shift_array (pdf_obj *array)
{
  pdf_obj   *result = NULL;
  pdf_array *data;

  TYPECHECK(array, PDF_ARRAY);

  data = array->data;
  if (data->size > 0) {
    int i;

    result = data->values[0];
    for (i = 1; i < data->size; i++)
      data->values[i-1] = data->values[i];
    data->size--;
  }

  return result;
}
#endif

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

#if 0
pdf_obj *
pdf_pop_array (pdf_obj *array)
{
  pdf_obj   *result;
  pdf_array *data;

  TYPECHECK(array, PDF_ARRAY);

  data = array->data;
  if (data->size > 0) {
    result = data->values[data->size - 1];
    data->size--;
  } else {
    result = NULL;
  }

  return result;
}
#endif

static void
write_dict (pdf_dict *dict, FILE *file)
{
#if 0
  pdf_out (file, "<<\n", 3); /* dropping \n saves few kb. */
#else
  pdf_out (file, "<<", 2);
#endif
  while (dict->key != NULL) {
    pdf_write_obj(dict->key, file);
    if (pdf_need_white(PDF_NAME, (dict->value)->type)) {
      pdf_out_white(file);
    }
    pdf_write_obj(dict->value, file);
#if 0
    pdf_out_char (file, '\n'); /* removing this saves few kb. */
#endif
    dict = dict->next;
  }
  pdf_out (file, ">>", 2);
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
    RELEASE(data);
    data = next;
  }
  RELEASE(data);
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
    ERROR("pdf_add_dict(): Passed invalid value");

  /* If this key already exists, simply replace the value */
  for (data = dict->data; data->key != NULL; data = data->next) {
    if (!strcmp(pdf_name_value(key), pdf_name_value(data->key))) {
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

#if 0
void
pdf_put_dict (pdf_obj *dict, const char *key, pdf_obj *value)
{
  pdf_dict *data;

  TYPECHECK(dict, PDF_DICT);

  if (!key) {
    ERROR("pdf_put_dict(): Passed invalid key.");
  }
  /* It seems that NULL is sometimes used for null object... */
  if (value != NULL && INVALIDOBJ(value)) {
    ERROR("pdf_add_dict(): Passed invalid value.");
  }

  data = dict->data;

  while (data->key != NULL) {
    if (!strcmp(key, pdf_name_value(data->key))) {
      pdf_release_obj(data->value);
      data->value = value;
      break;
    }
    data = data->next;
  }

  /*
   * If we didn't find the key, build a new "end" node and add
   * the new key just before the end
   */
  if (data->key == NULL) {
    pdf_dict *new_node;

    new_node = NEW (1, pdf_dict);
    new_node->key   = NULL;
    new_node->value = NULL;
    new_node->next  = NULL;
    data->next  = new_node;
    data->key   = pdf_new_name(key);
    data->value = value;
  }
}
#endif

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

  ASSERT(proc);

  TYPECHECK(dict, PDF_DICT);

  data = dict->data;
  while (!error &&
	 data->key != NULL) {
    error = proc(data->key, data->value, pdata);
    data = data->next;
  }

  return error;
}

#define pdf_match_name(o,s) ((o) && (s) && !strcmp(((pdf_name *)(o)->data)->name, (s)))
pdf_obj *
pdf_lookup_dict (pdf_obj *dict, const char *name)
{
  pdf_dict *data;

  ASSERT(name);

  TYPECHECK(dict, PDF_DICT);

  data = dict->data;
  while (data->key != NULL) {
    if (!strcmp(name, pdf_name_value(data->key))) {
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
      RELEASE(data);
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

  ASSERT(raster && length);

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

/* TIFF predictor filter support
 *
 * Many PDF viewers seems to have broken TIFF 2 predictor support?
 * Ony GhostScript and MuPDF render 4bpc grayscale image with TIFF 2 predictor
 * filter applied correctly.
 *
 *  Acrobat Reader DC  2015.007.20033  NG
 *  Adobe Acrobat X    10.1.13         NG
 *  Foxit Reader       4.1.5.425       NG
 *  GhostScript        9.16            OK
 *  SumatraPDF(MuPDF)  v3.0            OK
 *  Evince(poppler)    2.32.0.145      NG (1bit and 4bit broken)
 */

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

  ASSERT(raster);
  ASSERT( bpc > 0 && bpc <= 8 );

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

    memset(prev, 0, sizeof(uint16_t)*num_comp);
    inbuf = outbuf = 0; inbits = outbits = 0;
    l = k = j * rowbytes;
    for (i = 0; i < width; i++) {
      for (c = 0; c < num_comp; c++) {
        uint8_t cur;
        int8_t  sub;
        if (inbits < bpc) { /* need more byte */
          inbuf   = (inbuf << 8) | raster[l]; l++;
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
          raster[k] = (outbuf >> (outbits - 8)); k++;
          outbits  -= 8;
        }
      }
    }
    if (outbits > 0)
      raster[k] = (outbuf << (8 - outbits)); k++;
  }
  RELEASE(prev);
}

unsigned char *
filter_TIFF2_apply_filter (unsigned char *raster,
                           int32_t columns, int32_t rows,
                           int8_t bpc, int8_t colors, int32_t *length)
{
  unsigned char *dst;
  uint16_t      *prev;
  int32_t        rowbytes = (bpc * colors * columns + 7) / 8;
  int32_t        i, j;

  ASSERT(raster && length);

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
    RELEASE(prev);
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
    RELEASE(prev);
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
write_stream (pdf_stream *stream, FILE *file)
{
  unsigned char *filtered;
  unsigned int   filtered_length;
#ifdef HAVE_ZLIB
  uLong          buffer_length;
#else
  unsigned int   buffer_length;
#endif
  unsigned char *buffer;

  /*
   * Always work from a copy of the stream. All filters read from
   * "filtered" and leave their result in "filtered".
   */
#if 0
  filtered = NEW(stream->stream_length + 1, unsigned char);
#endif
  filtered = NEW(stream->stream_length, unsigned char);
  memcpy(filtered, stream->stream, stream->stream_length);
  filtered_length = stream->stream_length;

  /* PDF/A requires Metadata to be not filtered. */
  {
    pdf_obj *type;
    type = pdf_lookup_dict(stream->dict, "Type");
    if (type && !strcmp("Metadata", pdf_name_value(type))) {
      stream->_flags &= ~STREAM_COMPRESS;
    }
  }

#ifdef HAVE_ZLIB
  /* Apply compression filter if requested */
  if (stream->stream_length > 0 &&
      (stream->_flags & STREAM_COMPRESS) &&
      compression_level > 0) {
    pdf_obj *filters;

    /* First apply predictor filter if requested. */
    if ( compression_use_predictor &&
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
        WARN("Unknown/unsupported Predictor function %d.",
             stream->decodeparms.predictor);
        break;
      }
      if (parms && filtered2) {
        RELEASE(filtered);
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
#ifdef HAVE_ZLIB_COMPRESS2    
    if (compress2(buffer, &buffer_length, filtered,
		  filtered_length, compression_level)) {
      ERROR("Zlib error");
    }
#else 
    if (compress(buffer, &buffer_length, filtered,
		 filtered_length)) {
      ERROR ("Zlib error");
    }
#endif /* HAVE_ZLIB_COMPRESS2 */
    RELEASE(filtered);
    compression_saved += filtered_length - buffer_length
      - (filters ? strlen("/FlateDecode "): strlen("/Filter/FlateDecode\n"));

    filtered        = buffer;
    filtered_length = buffer_length;
  }
#endif /* HAVE_ZLIB */

  /* AES will change the size of data! */
  if (enc_mode) {
    unsigned char *cipher = NULL;
    size_t         cipher_len = 0;
    pdf_encrypt_data(filtered, filtered_length, &cipher, &cipher_len);
    RELEASE(filtered);
    filtered        = cipher;
    filtered_length = cipher_len;
  }

  #if 0
  /*
   * An optional end-of-line marker preceding the "endstream" is
   * not part of stream data. See, PDF Reference 4th ed., p. 38.
   */
  /* Add a '\n' if the last character wasn't one */
  if (filtered_length > 0 &&
      filtered[filtered_length-1] != '\n') {
    filtered[filtered_length] = '\n';
    filtered_length++;
  }
#endif

  pdf_add_dict(stream->dict,
	       pdf_new_name("Length"), pdf_new_number(filtered_length));

  pdf_write_obj(stream->dict, file);

  pdf_out(file, "\nstream\n", 8);

  if (filtered_length > 0)
    pdf_out(file, filtered, filtered_length);
  RELEASE(filtered);

  /*
   * This stream length "object" gets reset every time write_stream is
   * called for the stream object.
   * If this stream gets written more than once with different
   * filters, this could be a problem.
   */

  pdf_out(file, "\n", 1);
  pdf_out(file, "endstream", 9);
}

static void
release_stream (pdf_stream *stream)
{
  pdf_release_obj(stream->dict);
  stream->dict = NULL;

  if (stream->stream) {
    RELEASE(stream->stream);
    stream->stream = NULL;
  }

  if (stream->objstm_data) {
    RELEASE(stream->objstm_data);
    stream->objstm_data = NULL;
  }

  RELEASE(stream);
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

#if HAVE_ZLIB
#define WBUF_SIZE 4096
int
pdf_add_stream_flate (pdf_obj *dst, const void *data, int len)
{
  z_stream z;
  Bytef    wbuf[WBUF_SIZE];

  z.zalloc = Z_NULL; z.zfree = Z_NULL; z.opaque = Z_NULL;

  z.next_in  = (z_const Bytef *) data; z.avail_in  = len;
  z.next_out = (Bytef *) wbuf; z.avail_out = WBUF_SIZE;

  if (inflateInit(&z) != Z_OK) {
    WARN("inflateInit() failed.");
    return -1;
  }

  for (;;) {
    int status;
    status = inflate(&z, Z_NO_FLUSH);
    if (status == Z_STREAM_END)
      break;
    else if (status != Z_OK) {
      WARN("inflate() failed. Broken PDF file?");
      inflateEnd(&z);
      return -1;
    }

    if (z.avail_out == 0) {
      pdf_add_stream(dst, wbuf, WBUF_SIZE);
      z.next_out  = wbuf;
      z.avail_out = WBUF_SIZE;
    }
  }

  if (WBUF_SIZE - z.avail_out > 0)
    pdf_add_stream(dst, wbuf, WBUF_SIZE - z.avail_out);

  return (inflateEnd(&z) == Z_OK ? 0 : -1);
}

static int
get_decode_parms (struct decode_parms *parms, pdf_obj *dict)
{
  pdf_obj *tmp;

  ASSERT(dict && parms);
  ASSERT(PDF_OBJ_DICTTYPE(dict));

  /* Fill with default values */
  parms->predictor = 1;
  parms->colors    = 1;
  parms->bits_per_component = 8;
  parms->columns   = 1;

  tmp = pdf_deref_obj(pdf_lookup_dict(dict, "Predictor"));
  if (tmp)
    parms->predictor = pdf_number_value(tmp);
  tmp = pdf_deref_obj(pdf_lookup_dict(dict, "Colors"));
  if (tmp)
    parms->colors = pdf_number_value(tmp);
  tmp = pdf_deref_obj(pdf_lookup_dict(dict, "BitsPerComponent"));
  if (tmp)
    parms->bits_per_component = pdf_number_value(tmp);
  tmp = pdf_deref_obj(pdf_lookup_dict(dict, "Columns"));
  if (tmp)
    parms->columns = pdf_number_value(tmp);

  if (parms->bits_per_component != 1 &&
      parms->bits_per_component != 2 &&
      parms->bits_per_component != 4 &&
      parms->bits_per_component != 8 &&
      parms->bits_per_component != 16) {
      WARN("Invalid BPC value in DecodeParms: %d", parms->bits_per_component);
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
  RELEASE(col);

  return 0;
}

/* This routine is inefficient. Length is typically 4 for Xref streams.
 * Especially, calling pdf_add_stream() for each 4 bytes append is highly
 * inefficient.
 */
static int
filter_decoded (pdf_obj *dst, const void *src, int srclen,
                struct decode_parms *parms)
{
  const unsigned char *p = (const unsigned char *) src;
  const unsigned char *endptr = p + srclen;
  unsigned char *prev, *buf;
  int bits_per_pixel  = parms->colors * parms->bits_per_component;
  int bytes_per_pixel = (bits_per_pixel + 7) / 8;
  int length = (parms->columns * bits_per_pixel + 7) / 8;
  int i, error = 0;

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
          WARN("Mismatched Predictor type in data stream.");
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
          WARN("Unknown PNG predictor type: %d", type);
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
    WARN("Unknown Predictor type value :%d", parms->predictor);
    error = -1;
  }

  RELEASE(prev);
  RELEASE(buf);

  return error;
}

static int
pdf_add_stream_flate_filtered (pdf_obj *dst, const void *data, int len, struct decode_parms *parms)
{
  pdf_obj *tmp;
  z_stream z;
  Bytef    wbuf[WBUF_SIZE];
  int      error;

  z.zalloc = Z_NULL; z.zfree = Z_NULL; z.opaque = Z_NULL;

  z.next_in  = (z_const Bytef *) data; z.avail_in  = len;
  z.next_out = (Bytef *) wbuf; z.avail_out = WBUF_SIZE;

  if (inflateInit(&z) != Z_OK) {
    WARN("inflateInit() failed.");
    return -1;
  }

  tmp = pdf_new_stream(0);
  for (;;) {
    int status;
    status = inflate(&z, Z_NO_FLUSH);
    if (status == Z_STREAM_END)
      break;
    else if (status != Z_OK) {
      WARN("inflate() failed. Broken PDF file?");
      inflateEnd(&z);
      return -1;
    }

    if (z.avail_out == 0) {
      pdf_add_stream(tmp, wbuf, WBUF_SIZE);
      z.next_out  = wbuf;
      z.avail_out = WBUF_SIZE;
    }
  }

  if (WBUF_SIZE - z.avail_out > 0)
    pdf_add_stream(tmp, wbuf, WBUF_SIZE - z.avail_out);

  error = filter_decoded(dst, pdf_stream_dataptr(tmp), pdf_stream_length(tmp), parms);
  pdf_release_obj(tmp);

  return ((!error && inflateEnd(&z) == Z_OK) ? 0 : -1);
}
#endif

int
pdf_concat_stream (pdf_obj *dst, pdf_obj *src)
{
  const char *stream_data;
  int         stream_length;
  pdf_obj    *stream_dict;
  pdf_obj    *filter;
  int         error = 0;

  if (!PDF_OBJ_STREAMTYPE(dst) || !PDF_OBJ_STREAMTYPE(src))
    ERROR("Invalid type.");

  stream_data   = pdf_stream_dataptr(src);
  stream_length = pdf_stream_length (src);
  stream_dict   = pdf_stream_dict   (src);

  filter = pdf_lookup_dict(stream_dict, "Filter");
  if (!filter)
    pdf_add_stream(dst, stream_data, stream_length);
#if HAVE_ZLIB
  else {
    struct decode_parms parms;
    int    have_parms = 0;

    if (pdf_lookup_dict(stream_dict, "DecodeParms")) {
      pdf_obj *tmp;

      /* Dictionary or array */
      tmp = pdf_deref_obj(pdf_lookup_dict(stream_dict, "DecodeParms"));
      if (PDF_OBJ_ARRAYTYPE(tmp)) {
        if (pdf_array_length(tmp) > 1) {
          WARN("Unexpected size for DecodeParms array.");
          return -1;
        }
        tmp = pdf_deref_obj(pdf_get_array(tmp, 0));
      }
      if (!PDF_OBJ_DICTTYPE(tmp)) {
        WARN("PDF dict expected for DecodeParms...");
        return -1;
      }
      error = get_decode_parms(&parms, tmp);
      if (error)
        ERROR("Invalid value(s) in DecodeParms dictionary.");
      have_parms = 1;
    }
    if (PDF_OBJ_ARRAYTYPE(filter)) {
      if (pdf_array_length(filter) > 1) {
        WARN("Multiple DecodeFilter not supported.");
        return -1;
      }
      filter = pdf_get_array(filter, 0);
    }
    if (PDF_OBJ_NAMETYPE(filter)) {
      char  *filter_name = pdf_name_value(filter);
      if (filter_name && !strcmp(filter_name, "FlateDecode")) {
        if (have_parms)
          error = pdf_add_stream_flate_filtered(dst, stream_data, stream_length, &parms);
        else
          error = pdf_add_stream_flate(dst, stream_data, stream_length);
      } else {
        WARN("DecodeFilter \"%s\" not supported.", filter_name);
        error = -1;
      }
    } else
      ERROR("Broken PDF file?");
#endif /* HAVE_ZLIB */
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

#if 0
void
pdf_stream_set_flags (pdf_obj *stream, int flags)
{
  pdf_stream *data;

  TYPECHECK(stream, PDF_STREAM);

  data = stream->data;
  data->_flags = flags;
}

int
pdf_stream_get_flags (pdf_obj *stream)
{
  pdf_stream *data;

  TYPECHECK(stream, PDF_STREAM);

  data = stream->data;

  return data->_flags;
}
#endif

static void
pdf_write_obj (pdf_obj *object, FILE *file)
{
  if (object == NULL) {
    write_null(file);
    return;
  }

  if (INVALIDOBJ(object) || PDF_OBJ_UNDEFINED(object))
    ERROR("pdf_write_obj: Invalid object, type = %d\n", object->type);

  if (file == stderr)
    fprintf(stderr, "{%d}", object->refcount);

  switch (object->type) {
  case PDF_BOOLEAN:
    write_boolean(object->data, file);
    break;
  case PDF_NUMBER:
    write_number (object->data, file);
    break;
  case PDF_STRING:
    write_string (object->data, file);
    break;
  case PDF_NAME:
    write_name(object->data, file);
    break;
  case PDF_ARRAY:
    write_array(object->data, file);
    break;
  case PDF_DICT:
    write_dict (object->data, file);
    break;
  case PDF_STREAM:
    write_stream(object->data, file);
    break;
  case PDF_NULL:
    write_null(file);
    break;
  case PDF_INDIRECT:
    write_indirect(object->data, file);
    break;
  }
}

/* Write the object to the file */ 
static void
pdf_flush_obj (pdf_obj *object, FILE *file)
{
  int length;

  /*
   * Record file position
   */
  add_xref_entry(object->label, 1,
		 pdf_output_file_position, object->generation);
  length = sprintf(format_buffer, "%u %hu obj\n", object->label, object->generation);
  enc_mode = doc_enc_mode && !(object->flags & OBJ_NO_ENCRYPT);
  pdf_enc_set_label(object->label);
  pdf_enc_set_generation(object->generation);
  pdf_out(file, format_buffer, length);
  pdf_write_obj(object, file);
  pdf_out(file, "\nendobj\n", 8);
}

static int
pdf_add_objstm (pdf_obj *objstm, pdf_obj *object)
{
  int *data, pos;

  TYPECHECK(objstm, PDF_STREAM);

  data = get_objstm_data(objstm);
  pos = ++data[0];

  data[2*pos]   = object->label;
  data[2*pos+1] = pdf_stream_length(objstm);

  add_xref_entry(object->label, 2, objstm->label, pos-1);
 
  /* redirect output into objstm */
  output_stream = objstm;
  enc_mode = 0;
  pdf_write_obj(object, pdf_output_file);
  pdf_out_char(pdf_output_file, '\n');
  output_stream = NULL;

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
  unsigned int old_length;
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
      int length = sprintf(format_buffer, "%d ", *(val++));
      pdf_add_stream(objstm, format_buffer, length);
    }
  }

  dict = pdf_stream_dict(objstm);
  pdf_add_dict(dict, pdf_new_name("Type"), pdf_new_name("ObjStm"));
  pdf_add_dict(dict, pdf_new_name("N"), pdf_new_number(pos));
  pdf_add_dict(dict, pdf_new_name("First"), pdf_new_number(stream->stream_length));
  
  pdf_add_stream(objstm, old_buf, old_length);
  RELEASE(old_buf);
  pdf_release_obj(objstm);
}

void
pdf_release_obj (pdf_obj *object)
{
  if (object == NULL)
    return;
  if (INVALIDOBJ(object) || object->refcount <= 0) {
    MESG("\npdf_release_obj: object=%p, type=%d, refcount=%d\n",
	 object, object->type, object->refcount);
    pdf_write_obj(object, stderr);
    ERROR("pdf_release_obj:  Called with invalid object.");
  }
  object->refcount -= 1;
  if (object->refcount == 0) {
    /*
     * Nothing is using this object so it's okay to remove it.
     * Nonzero "label" means object needs to be written before it's destroyed.
     */
    if (object->label && pdf_output_file != NULL) {
      if (!do_objstm || object->flags & OBJ_NO_OBJSTM
	  || (doc_enc_mode && object->flags & OBJ_NO_ENCRYPT)
	  || object->generation)
	pdf_flush_obj(object, pdf_output_file);
      else {
        if (!current_objstm) {
	  int *data = NEW(2*OBJSTM_MAX_OBJS+2, int);
	  data[0] = data[1] = 0;
	  current_objstm = pdf_new_stream(STREAM_COMPRESS);
	  set_objstm_data(current_objstm, data);
	  pdf_label_obj(current_objstm);
	}
	if (pdf_add_objstm(current_objstm, object) == OBJSTM_MAX_OBJS) {
	  release_objstm(current_objstm);
	  current_objstm = NULL;
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
    RELEASE(object);
  }
}

static int
backup_line (FILE *pdf_input_file)
{
  int ch = -1;

  /*
   * Note: this code should work even if \r\n is eol. It could fail on a
   * machine where \n is eol and there is a \r in the stream --- Highly
   * unlikely in the last few bytes where this is likely to be used.
   */
  if (tell_position(pdf_input_file) > 1)
    do {
      seek_relative (pdf_input_file, -2);
    } while (tell_position(pdf_input_file) > 0 &&
	     (ch = fgetc(pdf_input_file)) >= 0 &&
	     (ch != '\n' && ch != '\r' ));
  if (ch < 0) {
    return 0;
  }

  return 1;
}

static int
find_xref (FILE *pdf_input_file)
{
  int   xref_pos = 0;
  int   len, tries = 10;

  do {
    int currentpos;

    if (!backup_line(pdf_input_file)) {
      tries = 0;
      break;
    }
    currentpos = tell_position(pdf_input_file);
    fread(work_buffer, sizeof(char), strlen("startxref"), pdf_input_file);
    seek_absolute(pdf_input_file, currentpos);
    tries--;
  } while (tries > 0 &&
	   strncmp(work_buffer, "startxref", strlen("startxref")));
  if (tries <= 0)
    return 0;

  /* Skip rest of this line */
  mfgets(work_buffer, WORK_BUFFER_SIZE, pdf_input_file);
  /* Next line of input file should contain actual xref location */
  len = mfreadln(work_buffer, WORK_BUFFER_SIZE, pdf_input_file);
  if (len <= 0)
   WARN("Reading xref location data failed... Not a PDF file?");
  else {
    const char *start, *end;
    char *number;

    start = work_buffer;
    end   = start + len;
    skip_white(&start, end);
    number   = parse_number(&start, end);
    xref_pos = (int) atof(number);
    RELEASE(number);
  }

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
  /*
   * Fill work_buffer and hope trailer fits. This should
   * be made a bit more robust sometime.
   */
  if (fread(work_buffer, sizeof(char),
	    WORK_BUFFER_SIZE, pf->file) == 0 ||
      strncmp(work_buffer, "trailer", strlen("trailer"))) {
    WARN("No trailer.  Are you sure this is a PDF file?");
    WARN("buffer:\n->%s<-\n", work_buffer);
    result = NULL;
  } else {
    const char *p = work_buffer + strlen("trailer");
    skip_white(&p, work_buffer + WORK_BUFFER_SIZE);
    result = parse_pdf_dict(&p, work_buffer + WORK_BUFFER_SIZE, pf);
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
next_object_offset (pdf_file *pf, unsigned int obj_num)
{
  int  next = pf->file_size;  /* Worst case */
  int  i, curr;

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

#define checklabel(pf, n, g) ((n) > 0 && (n) < (pf)->num_obj && ( \
  ((pf)->xref_table[(n)].type == 1 && (pf)->xref_table[(n)].field3 == (g)) || \
  ((pf)->xref_table[(n)].type == 2 && !(g))))

pdf_obj *
pdf_new_indirect (pdf_file *pf, unsigned obj_num, unsigned short obj_gen)
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
pdf_read_object (unsigned int obj_num, unsigned short obj_gen,
		pdf_file *pf, int offset, int limit)
{
  int      length;
  char    *buffer;
  const char *p, *endptr;
  pdf_obj *result;

  length = limit - offset;

  if (length <= 0)
    return NULL;

  buffer = NEW(length + 1, char);

  seek_absolute(pf->file, offset);
  fread(buffer, sizeof(char), length, pf->file);

  p      = buffer;
  endptr = p + length;

  /* Check for obj_num and obj_gen */
  {
    const char   *q = p; /* <== p */
    char         *sp;
    unsigned int  n, g;

    skip_white(&q, endptr);
    sp = parse_unsigned(&q, endptr);
    if (!sp) {
      RELEASE(buffer);
      return NULL;
    }
    n = strtoul(sp, NULL, 10);
    RELEASE(sp);

    skip_white(&q, endptr);
    sp = parse_unsigned(&q, endptr);
    if (!sp) {
      RELEASE(buffer);
      return NULL;
    }
    g = strtoul(sp, NULL, 10);
    RELEASE(sp);

    if (obj_num && (n != obj_num || g != obj_gen)) {
      RELEASE(buffer);
      return NULL;
    }

    p = q; /* ==> p */
  }


  skip_white(&p, endptr);
  if (memcmp(p, "obj", strlen("obj"))) {
    WARN("Didn't find \"obj\".");
    RELEASE(buffer);
    return NULL;
  }
  p += strlen("obj");

  result = parse_pdf_object(&p, endptr, pf);

  skip_white(&p, endptr);
  if (memcmp(p, "endobj", strlen("endobj"))) {
    WARN("Didn't find \"endobj\".");
    if (result)
      pdf_release_obj(result);
    result = NULL;
  }
  RELEASE(buffer);

  return result;
}

static pdf_obj *
read_objstm (pdf_file *pf, unsigned int num)
{
  unsigned int offset = pf->xref_table[num].field2;
  unsigned short gen = pf->xref_table[num].field3;
  int limit = next_object_offset(pf, num), n, first, *header = NULL;
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
  RELEASE(data);
  
  return pf->xref_table[num].direct = objstm;

 error:
  WARN("Cannot parse object stream.");
  if (data)
    RELEASE(data);
  if (objstm)
    pdf_release_obj(objstm);
  return NULL;
}

/* Label without corresponding object definition are replaced by the
 * null object, as required by the PDF spec. This is important to parse
 * several cross-reference sections.
 */ 
static pdf_obj *
pdf_get_object (pdf_file *pf, unsigned int obj_num, unsigned short obj_gen)
{
  pdf_obj *result;

  if (!checklabel(pf, obj_num, obj_gen)) {
    WARN("Trying to read nonexistent or deleted object: %lu %u",
         obj_num, obj_gen);
    return pdf_new_null();
  }

  if ((result = pf->xref_table[obj_num].direct)) {
    return pdf_link_obj(result);
  }

  if (pf->xref_table[obj_num].type == 1) {
    /* type == 1 */
    unsigned int offset;
    int limit;
    offset = pf->xref_table[obj_num].field2;
    limit  = next_object_offset(pf, obj_num);
    result = pdf_read_object(obj_num, obj_gen, pf, offset, limit);
  } else {
    /* type == 2 */
    unsigned int   objstm_num = pf->xref_table[obj_num].field2;
    unsigned short index = pf->xref_table[obj_num].field3;
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
  WARN("Could not read object from object stream.");
  return pdf_new_null();
}

#define OBJ_FILE(o) (((pdf_indirect *)((o)->data))->pf)
#define OBJ_OBJ(o)  (((pdf_indirect *)((o)->data))->obj)
#define OBJ_NUM(o)  (((pdf_indirect *)((o)->data))->label)
#define OBJ_GEN(o)  (((pdf_indirect *)((o)->data))->generation)

static pdf_obj *
pdf_new_ref (pdf_obj *object)
{
  pdf_obj *result;

  if (object->label == 0) {
    pdf_label_obj(object);
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
      unsigned int   obj_num = OBJ_NUM(obj);
      unsigned short obj_gen = OBJ_GEN(obj);
      pdf_release_obj(obj);
      obj = pdf_get_object(pf, obj_num, obj_gen);
    } else {
      pdf_obj *next_obj = OBJ_OBJ(obj);
      if (!next_obj) {
        ERROR("Undefined object reference"); 
      }
      pdf_release_obj(obj);
      obj = pdf_link_obj(next_obj);
    }
  }

  if (!count)
    ERROR("Loop in object hierarchy detected. Broken PDF file?");

  if (PDF_OBJ_NULLTYPE(obj)) {
    pdf_release_obj(obj);
    return NULL;
  } else
    return obj;
}

static void
extend_xref (pdf_file *pf, int new_size) 
{
  unsigned int i;

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
parse_xref_table (pdf_file *pf, int xref_pos)
{
  FILE       *pdf_input_file = pf->file;
  const char *p, *endptr;
  char        buf[256]; /* See, PDF ref. v.1.7, p.91 for "255+1" here. */
  int         len;

  /*
   * This routine reads one xref segment. It may be called multiple times
   * on the same file.  xref tables sometimes come in pieces.
   */
  seek_absolute(pf->file, xref_pos);
  len = mfreadln(buf, 255, pdf_input_file);
  /* We should have already checked that "startxref" section exists.
   * So, EOF here (len = -1) is impossible.  We don't treat too long line
   * case seriously.
   */
  if (len < 0) {
    WARN("Something went wrong while reading xref table...giving up.");
    return -1;
  }
  p      = buf;
  endptr = buf + len;
  /* No skip_white() here. There should not be any white-spaces here. */
  if (memcmp(p, "xref", strlen("xref"))) {
    /* Might be an xref stream and not an xref table */
    return 0;
  }
  p += strlen("xref");
  skip_white(&p, endptr);
  if (p != endptr) {
    WARN("Garbage after \"xref\" keyword found.");
    return -1;
  }

  /* Next line in file has first item and size of table */
  for (;;) {
    char         flag;
    unsigned int current_pos;
    int          i;
    uint32_t     first, size, offset, obj_gen;

    current_pos = tell_position(pdf_input_file);
    len = mfreadln(buf, 255, pdf_input_file);
    if (len == 0) /* empty line... just skip. */
      continue;
    else if (len < 0) {
      WARN("Reading a line failed in xref table.");
      return -1;
    }

    p      = buf;
    endptr = buf + len;
    skip_white(&p, endptr);
    if (p == endptr) { /* Only white-spaces and/or comment found. */
      continue;
    } else if (!strncmp(p, "trailer", strlen ("trailer"))) {
      /*
       * Backup... This is ugly, but it seems like the safest thing to
       * do.  It is possible the trailer dictionary starts on the same
       * logical line as the word trailer.  In that case, the mfgets
       * call might have started to read the trailer dictionary and
       * parse_trailer would fail.
       */
      current_pos += p - buf; /* Jump to the beginning of "trailer" keyword. */
      seek_absolute(pdf_input_file, current_pos);
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
        WARN("An unsigned integer expected but could not find. (xref)");
        return -1;
      }
      first = atoi(q);
      RELEASE(q);
      skip_white(&p, endptr);

      /* Nnumber of objects in this xref subsection. */
      q = parse_unsigned(&p, endptr);
      if (!q) {
        WARN("An unsigned integer expected but could not find. (xref)");
        return -1;
      }
      size = atoi(q);
      RELEASE(q);
      skip_white(&p, endptr);

      /* Check for unrecognized tokens */
      if (p != endptr) {
        WARN("Unexpected token found in xref table.");
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
      len = mfreadln(buf, 255, pdf_input_file);
      if (len == 0) /* empty line...just skip. */
        continue;
      else if (len < 0) {
        WARN("Something went wrong while reading xref subsection...");
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
          WARN("An unsigned integer expected but could not find. (xref)");
          return -1;
        } else if (strlen(q) != 10) { /* exactly 10 digits */
          WARN(("Offset must be a 10 digits number. (xref)"));
          RELEASE(q);
          return -1;
        }
        /* FIXME: Possible overflow here. Consider using strtoll(). */
        offset = atoi(q);
        RELEASE(q);
        skip_white(&p, endptr);

        /* Generation number -- 5 digits (0 padded) */
        q = parse_unsigned(&p, endptr);
        if (!q) {
          WARN("An unsigned integer expected but could not find. (xref)");
          return -1;
        } else if (strlen(q) != 5) { /* exactly 5 digits */
          WARN(("Expecting a 5 digits number. (xref)"));
          RELEASE(q);
          return -1;
        }
        obj_gen = atoi(q);
        RELEASE(q);
        skip_white(&p, endptr);
      }
      if (p == endptr) {
        WARN("Unexpected EOL reached while reading a xref subsection entry.");
        return -1;
      }

      /* Flag -- a char */
      flag = *p; p++;
      skip_white(&p, endptr);
      if (p < endptr) {
        WARN("Garbage in xref subsection entry found...");
        return -1;
      } else if (((flag != 'n' && flag != 'f') ||
                  (flag == 'n' &&
                  (offset >= pf->file_size || (offset > 0 && offset < 4))))) {
        WARN("Invalid xref table entry [%lu]. PDF file is corrupt...", i);
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

static unsigned int
parse_xrefstm_field (const char **p, int length, unsigned int def)
{
  unsigned int val = 0;

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
		      const char **p, int *length,
		      int *W, int wsum,
		      int first, int size) {
  xref_entry *e;

  if ((*length -= wsum*size) < 0)
    return -1;

  if (pf->num_obj < first+size)
    extend_xref(pf, first+size);  /* TODO: change! why? */

  e = pf->xref_table + first;
  while (size--) {
    unsigned char  type;
    unsigned int   field2;
    unsigned short field3;

    type = (unsigned char) parse_xrefstm_field(p, W[0], 1);
    if (type > 2)
      WARN("Unknown cross-reference stream entry type.");
#if 0
    /* Not sure */
    else if (!W[1] || (type != 1 && !W[2]))
      return -1;
#endif

    field2 = (unsigned int)  parse_xrefstm_field(p, W[1], 0);
    field3 = (unsigned short) parse_xrefstm_field(p, W[2], 0);

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
parse_xref_stream (pdf_file *pf, int xref_pos, pdf_obj **trailer)
{
  pdf_obj *xrefstm, *size_obj, *W_obj, *index_obj;
  unsigned int size;
  int length;
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
  size = (unsigned int) pdf_number_value(size_obj);

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
    unsigned int index_len;
    if (!PDF_OBJ_ARRAYTYPE(index_obj) ||
	((index_len = pdf_array_length(index_obj)) % 2 ))
      goto error;

    i = 0;
    while (i < index_len) {
      pdf_obj *first = pdf_get_array(index_obj, i++);
      size_obj  = pdf_get_array(index_obj, i++);
      if (!PDF_OBJ_NUMBERTYPE(first) ||
	  !PDF_OBJ_NUMBERTYPE(size_obj) ||
	  parse_xrefstm_subsec(pf, &p, &length, W, wsum,
			       (int) pdf_number_value(first),
			       (int) pdf_number_value(size_obj)))
	goto error;
    }
  } else if (parse_xrefstm_subsec(pf, &p, &length, W, wsum, 0, size))
      goto error;

  if (length)
    WARN("Garbage in xref stream.");

  pdf_release_obj(xrefstm);

  return 1;

 error:
  WARN("Cannot parse cross-reference stream.");
  if (xrefstm)
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
  int      xref_pos;

  if (!(xref_pos = find_xref(pf->file)))
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
	  WARN("Skipping hybrid reference section.");
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
	xref_pos = (int) pdf_number_value(prev);
      else
	goto error;
    } else
      xref_pos = 0;

    pdf_release_obj(trailer);
  }

#if 0
  if (!pdf_lookup_dict(main_trailer, "Root")) {
      WARN("Trailer doesn't have catalog. Is this a correct PDF file?");
      goto error;
    }
#endif

  return main_trailer;

 error:
  WARN("Error while parsing PDF file.");
  if (trailer)
    pdf_release_obj(trailer);
  if (main_trailer)
    pdf_release_obj(main_trailer);
  return NULL;
}

static struct ht_table *pdf_files = NULL;

static pdf_file *
pdf_file_new (FILE *file)
{
  pdf_file *pf;
  ASSERT(file);
  pf = NEW(1, pdf_file);
  pf->file    = file;
  pf->trailer = NULL;
  pf->xref_table = NULL;
  pf->catalog = NULL;
  pf->num_obj = 0;
  pf->version = 0;

  seek_end(file);
  pf->file_size = tell_position(file);

  return pf;
}

static void
pdf_file_free (pdf_file *pf)
{
  unsigned int i;

  if (!pf) {
    return;
  }

  for (i = 0; i < pf->num_obj; i++) {
    if (pf->xref_table[i].direct)
      pdf_release_obj(pf->xref_table[i].direct);
    if (pf->xref_table[i].indirect)
      pdf_release_obj(pf->xref_table[i].indirect);
  }

  RELEASE(pf->xref_table);
  if (pf->trailer)
    pdf_release_obj(pf->trailer);
  if (pf->catalog)
    pdf_release_obj(pf->catalog);

  RELEASE(pf);  
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
  ASSERT(pf);
  return pf->version;
}

pdf_obj *
pdf_file_get_trailer (pdf_file *pf)
{
  ASSERT(pf);
  return pdf_link_obj(pf->trailer);
}

pdf_obj *
pdf_file_get_catalog (pdf_file *pf)
{
  ASSERT(pf);
  return pf->catalog;
}

pdf_file *
pdf_open (const char *ident, FILE *file)
{
  pdf_file *pf = NULL;

  ASSERT(pdf_files);

  if (ident)
    pf = (pdf_file *) ht_lookup_table(pdf_files, ident, strlen(ident));

  if (pf) {
    pf->file = file;
  } else {
    pdf_obj *new_version;
    int version = check_for_pdf_version(file);

    if (version < 1 || version > pdf_version) {
      WARN("pdf_open: Not a PDF 1.[1-%u] file.", pdf_version);
/*
  Try to embed the PDF image, even if the PDF version is newer than
  the setting.
      return NULL;
*/
    }

    pf = pdf_file_new(file);
    pf->version = version;

    if (!(pf->trailer = read_xref(pf)))
      goto error;

    if (pdf_lookup_dict(pf->trailer, "Encrypt")) {
      WARN("PDF document is encrypted.");
      goto error;
    }

    pf->catalog = pdf_deref_obj(pdf_lookup_dict(pf->trailer, "Root"));
    if (!PDF_OBJ_DICTTYPE(pf->catalog)) {
      WARN("Cannot read PDF document catalog. Broken PDF file?");
      goto error;
    }

    new_version = pdf_deref_obj(pdf_lookup_dict(pf->catalog, "Version"));
    if (new_version) {
      unsigned int minor;

      if (!PDF_OBJ_NAMETYPE(new_version) ||
	  sscanf(pdf_name_value(new_version), "1.%u", &minor) != 1) {
	pdf_release_obj(new_version);
	WARN("Illegal Version entry in document catalog. Broken PDF file?");
	goto error;
      }

      if (pf->version < minor)
	pf->version = minor;

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
    pf->file = NULL;
}

void
pdf_files_close (void)
{
  ASSERT(pdf_files);
  ht_clear_table(pdf_files);
  RELEASE(pdf_files);
}

static int
check_for_pdf_version (FILE *file) 
{
  unsigned int minor;

  rewind(file);

  return (ungetc(fgetc(file), file) == '%' &&
	  fscanf(file, "%%PDF-1.%u", &minor) == 1) ? minor : -1;
}

int
check_for_pdf (FILE *file) 
{
  int version = check_for_pdf_version(file);

  if (version < 0)  /* not a PDF file */
    return 0;

  if (version <= pdf_version)
    return 1;

  WARN("Version of PDF file (1.%d) is newer than version limit specification.",
       version);
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

static pdf_obj loop_marker = { PDF_OBJ_INVALID, 0, 0, 0, 0, NULL };

static pdf_obj *
pdf_import_indirect (pdf_obj *object)
{
  pdf_file *pf = OBJ_FILE(object);
  unsigned int obj_num = OBJ_NUM(object);
  unsigned short obj_gen = OBJ_GEN(object);

  pdf_obj *ref;

  ASSERT(pf);

  if (!checklabel(pf, obj_num, obj_gen)) {
    WARN("Can't resolve object: %lu %u", obj_num, obj_gen);
    return pdf_new_null();
  }

  if ((ref = pf->xref_table[obj_num].indirect)) {
    if (ref == &loop_marker)
      ERROR("Loop in object hierarchy detected. Broken PDF file?");
    return  pdf_link_obj(ref);
  } else {
    pdf_obj *obj, *tmp;

    obj = pdf_get_object(pf, obj_num, obj_gen);
    if (!obj) {
      WARN("Could not read object: %lu %u", obj_num, obj_gen);
      return NULL;
    }

    /* We mark the reference to be able to detect loops */
    pf->xref_table[obj_num].indirect = &loop_marker;

    tmp = pdf_import_object(obj);
    
    pf->xref_table[obj_num].indirect = ref = pdf_ref_obj(tmp);
    
    pdf_release_obj(tmp);
    pdf_release_obj(obj);
    
    return  pdf_link_obj(ref);
  }
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
  int       i;

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

  ASSERT(PDF_OBJ_INDIRECTTYPE(ref1) && PDF_OBJ_INDIRECTTYPE(ref2));

  data1 = (pdf_indirect *) ref1->data;
  data2 = (pdf_indirect *) ref2->data;

  return data1->pf != data2->pf || data1->label != data2->label
    || data1->generation != data2->generation;
}
