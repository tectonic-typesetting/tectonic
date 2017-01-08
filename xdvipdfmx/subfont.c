/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.
    
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

#include <errno.h>

#include "system.h"
#include "mem.h"
#include "error.h"
#include "dpxfile.h"

#include "subfont.h"

static int verbose = 0;
void
subfont_set_verbose (void)
{
  verbose++;
}

/* Don't forget fontmap reading now requires information
 * from SFD files. You must initialize at least sfd_file_
 * cache before starting loading of fontmaps.
 */

/* Subfont Definition File:
 *  struct sfd_file_ is for storing subfont identifiers
 *  contained in a SFD file and for mapping string pair
 *  <SFD_file, Subfont_id> to internal code mapping table
 *  ID which is index within an array of struct sfd_rec_.
 *  We store code mapping tables in different place than
 *  struct sfd_file_.
 */
struct sfd_file_
{
  char  *ident;  /* SFD file name */
  char **sub_id; /* Subfont IDs   */

  int   *rec_id; /* indices within struct sfd_rec_ array "sfd_record" */

  int    max_subfonts;
  int    num_subfonts;
};

/* Mapping table */
struct sfd_rec_
{
  /* unsigned char  misbit[32]; */
  unsigned short vector[256]; /* 0 for undefined */
};

static void
init_sfd_file_ (struct sfd_file_ *sfd)
{
  sfd->ident  = NULL;
  sfd->sub_id = NULL;
  sfd->rec_id = NULL;
  sfd->max_subfonts = sfd->num_subfonts = 0;
}

static void
clean_sfd_file_ (struct sfd_file_ *sfd)
{
  int  i;
  if (sfd->ident)
    RELEASE(sfd->ident);
  if (sfd->sub_id) {
    for (i = 0; i < sfd->num_subfonts; i++) {
      if (sfd->sub_id[i])
        RELEASE(sfd->sub_id[i]);
    }
    RELEASE(sfd->sub_id);
  }
  if (sfd->rec_id)
    RELEASE(sfd->rec_id);
  init_sfd_file_(sfd);
}

static struct sfd_file_ *sfd_files = NULL;
static int num_sfd_files = 0, max_sfd_files = 0;

static struct sfd_rec_ *sfd_record = NULL;
static int num_sfd_records = 0, max_sfd_records = 0;



/* Another buffer...
 * We want buffer size at least 7 x 256 + a
 * 4096 is usually enough.
 */
#define LINE_BUF_SIZE 4096
static char line_buf[LINE_BUF_SIZE];

/* Each lines describes character code mapping for each
 * subfonts. '#' is start of comment.
 * SFD file format uses a '\' before newline sequence
 * for line-continuation.
 */
static char *
readline (char *buf, int buf_len, FILE *fp)
{
  char  *r, *q, *p = buf;
  int    n = 0, c = 0;

  while (buf_len - n > 0 && (q = mfgets(p, buf_len - n, fp))) {
    c++;
    r = strchr(q, '#');
    /* Comment is converted to single wsp (followed by a newline). */
    if (r) {
      *r = ' ';
      *(r + 1) = '\0';
    }
    if (strlen(q) == 0)
      break; /* empty line */
    n += strlen(q);
    q += strlen(q) - 1;
    if (*q != '\\')
      break;
    else { /* line continued */
      n -= 1;
      p  = buf + n;
    }
  }
  if (n >= buf_len - 1) {
    WARN("Possible buffer overflow in reading SFD file (buffer full, size=%d bytes)",
         buf_len - 1);
  }

  return  (c > 0 ? buf : NULL);
}

#define clear_vector(v) if ((v)) { \
  int __i; \
  for (__i = 0; __i < 256; __i++) \
    (v)[__i] = 0; \
}

/* subfont_id ( integer ':' | integer '_' integer | integer )*
 *
 *  0x32: ==> Subsequent integers are place into slots starting at 0x32.
 *    0x32: 0xA1A1 0xA1A2 ... ==> 0x32 is mappned to 0xA1A1, 0x33 to 0xA1A2
 *  0xA1A1_0xA1A5 ==> Expanded to 0xA1A1 0xA1A2 ... 0xA1A5
 */

/* subfont_id is already consumed here. */
static int
read_sfd_record (struct sfd_rec_ *rec, const char *lbuf)
{
  const char *p = lbuf, *q;
  char  *r;
  int    repos  = 0;
  int    c,  v1 = 0, v2 = 0;
  int    curpos = 0;
  int    error  = 0;

#define IS_TOKSEP(c) ((c) == '\0' || isspace((unsigned char)(c)))
  for ( ; *p && isspace((unsigned char)*p); p++);
  while (!error && *p) {
    repos = 0; q = p;
    v1    = strtol(p, &r, 0);
    q = r;
    if (q == p ||
        (!IS_TOKSEP(*q) && *q != ':' && *q != '_')) {
      WARN("Unknown token in subfont mapping table: %c", *q);
      return  -1;
    }

    switch (*q) {
    case  ':':
      if (v1 < 0 || v1 > 0xff) {
        WARN("Invalud value for subfont table offset: %ld", v1);
        return  -1;
      }
      repos = 1;
      q++;
      break;
    case  '_':
      p  = q + 1;
      v2 = strtol(p, &r, 0);
      q = r;
      if (v1 < 0 || v1 > 0xffffL ||
          v2 < 0 || v2 > 0xffffL) {
        WARN("Invalid value in subfont mapping table: 0x%x_0x%x", v1, v2);
        return -1;
      } else if (q == p || !IS_TOKSEP(*q)) {
        WARN("Invalid char in subfont mapping table: %c", *q);
        return  -1;
      }
      break;
    default:
      if (v1 < 0 || v1 > 0xffffL) {
        WARN("Invalid character code in subfont mapping table: 0x%x", v1);
        return -1;
      }
      v2 = v1;
      break;
    }

    if (repos)
      curpos = v1;
    else {
      if (v2 < v1 || curpos + (v2 - v1) > 0xff) {
        WARN("Invalid range in subfont mapping: curpos=\"0x%02x\" range=\"0x%04x,0x%04x\"",
             curpos, v1, v2);
        return  -1;
      }
      for (c = v1; c <= v2; c++) {
        if (rec->vector[curpos] != 0) {
          WARN("Subfont mapping for slot=\"0x%02x\" already defined...", curpos);
          return  -1;
        }
        ASSERT( curpos >= 0 && curpos <= 255 );
        rec->vector[curpos++] = (unsigned short) c;
      }
    }
    for (p = q; *p && isspace((unsigned char)*p); p++);
  }

  return  error;
}

/* Scan for subfont IDs */
static int
scan_sfd_file (struct sfd_file_ *sfd, FILE *fp)
{
  char  *id;
  char  *q, *p;
  int    n, lpos = 0;

  ASSERT( sfd && fp );

  if (verbose > 3) {
    MESG("\nsubfont>> Scanning SFD file \"%s\"...\n", sfd->ident);
  }

  rewind(fp);
  sfd->max_subfonts = sfd->num_subfonts = 0;
  while ((p = readline(line_buf, LINE_BUF_SIZE, fp)) != NULL) {
    lpos++;
    for ( ; *p && isspace((unsigned char)*p); p++);
    if (*p == 0)
      continue; /* empty */

    /* Saw non-wsp here */
    for (n = 0, q = p; *p && !isspace((unsigned char)*p); p++, n++);
    id = NEW(n + 1, char);
    memcpy(id, q, n); id[n] = '\0';
    if (sfd->num_subfonts >= sfd->max_subfonts) {
      sfd->max_subfonts += 16;
      sfd->sub_id = RENEW(sfd->sub_id, sfd->max_subfonts, char *);
    }

    if (verbose > 3) {
      MESG("subfont>>   id=\"%s\" at line=\"%d\"\n", id, lpos);
    }
    sfd->sub_id[sfd->num_subfonts] = id;
    sfd->num_subfonts++;
  }

  sfd->rec_id = NEW(sfd->num_subfonts, int);
  for (n = 0; n < sfd->num_subfonts; n++) {
    sfd->rec_id[n] = -1; /* Not loaded yet. We do lazy loading of map definitions. */
  }

  if (verbose > 3) {
    MESG("subfont>> %d entries found in SFD file \"%s\".\n", sfd->num_subfonts, sfd->ident);
  }

  return  0;
}


/* Open SFD file and gather subfont IDs. We do not read mapping tables
 * here but only read subfont IDs used in SFD file.
 */
static int
find_sfd_file (const char *sfd_name)
{
  int    id = -1;
  int    i, error = -1;

  /* Check if we already opened SFD file */
  for (i = 0; i < num_sfd_files; i++) {
    if (!strcmp(sfd_files[i].ident, sfd_name)) {
      id = i;
      break;
    }
  }

  if (id < 0) {
    struct sfd_file_ *sfd = NULL;
    FILE  *fp;

    if (num_sfd_files >= max_sfd_files) {
      max_sfd_files += 8;
      sfd_files = RENEW(sfd_files, max_sfd_files, struct sfd_file_);
    }
    sfd = &sfd_files[num_sfd_files];
    init_sfd_file_(sfd);
    sfd->ident = NEW(strlen(sfd_name) + 1, char);
    strcpy(sfd->ident, sfd_name);
    fp = DPXFOPEN(sfd->ident, DPX_RES_TYPE_SFD);
    if (!fp) {
      clean_sfd_file_(sfd);
      return  -1;
    }
    error = scan_sfd_file(sfd, fp);
    DPXFCLOSE(fp);
    if (!error)
      id = num_sfd_files++;
    else {
      WARN("Error occured while reading SFD file \"%s\"", sfd_name);
      clean_sfd_file_(sfd);
      id = -1;
    }
  }

  return  id;
}

char **
sfd_get_subfont_ids (const char *sfd_name, int *num_ids)
{
  int  sfd_id;

  if (!sfd_name)
    return  NULL;

  sfd_id = find_sfd_file(sfd_name);
  if (sfd_id < 0)
    return  NULL;

  if (num_ids)
    *num_ids = sfd_files[sfd_id].num_subfonts;
  return  sfd_files[sfd_id].sub_id;
}

/* Make sure that sfd_name does not have the extension '.sfd'.
 * Mapping tables are actually read here.
 */
int
sfd_load_record (const char *sfd_name, const char *subfont_id)
{
  int               rec_id = -1;
  struct sfd_file_ *sfd;
  FILE             *fp;
  int               sfd_id, i, error = 0;
  char             *p, *q;

  if (!sfd_name || !subfont_id)
    return  -1;

  sfd_id = find_sfd_file(sfd_name);
  if (sfd_id < 0)
    return  -1;

  sfd = &sfd_files[sfd_id];
  /* Check if we already loaded mapping table. */
  for (i = 0;
       i < sfd->num_subfonts && strcmp(sfd->sub_id[i], subfont_id); i++);
  if (i == sfd->num_subfonts) {
    WARN("Subfont id=\"%s\" not exist in SFD file \"%s\"...",
         subfont_id, sfd->ident);
    return  -1;
  } else if (sfd->rec_id[i] >= 0) {
    return  sfd->rec_id[i];
  }

  if (verbose > 3) {
    MESG("\nsubfont>> Loading SFD mapping table for <%s,%s>...",
         sfd->ident, subfont_id);
  }

  /* reopen */
  fp = DPXFOPEN(sfd->ident, DPX_RES_TYPE_SFD);
  if (!fp) {
    return  -1;
    /* ERROR("Could not open SFD file \"%s\"", sfd_name); */
  }

  /* Seek to record for 'sub_name'. */
  while ((p = readline(line_buf, LINE_BUF_SIZE, fp))) {
    for ( ; *p && isspace((unsigned char)*p); p++);
    if (*p == 0)
      continue; /* empty line */

    /* q = parse_ident(&p, p + strlen(p)); */
    for (q = p; *p && !isspace((unsigned char)*p); p++);
    *p = '\0'; p++;
    if (!strcmp(q, subfont_id)) {
      if (num_sfd_records >= max_sfd_records) {
        max_sfd_records += 16;
        sfd_record = RENEW(sfd_record, max_sfd_records, struct sfd_rec_);
      }
      clear_vector(sfd_record[num_sfd_records].vector);
      error = read_sfd_record(&sfd_record[num_sfd_records], p);
      if (error)
        WARN("Error occured while reading SFD file: file=\"%s\" subfont_id=\"%s\"",
             sfd->ident, subfont_id);
      else {
        rec_id = num_sfd_records++;
      }
    }
  }
  if (rec_id < 0) {
    WARN("Failed to load subfont mapping table for SFD=\"%s\" subfont_id=\"%s\"",
         sfd->ident, subfont_id);
  }
  sfd->rec_id[i] = rec_id;
  DPXFCLOSE(fp);

  if (verbose > 3) {
    int __i;
    if (rec_id >= 0) {
      MESG(" at id=\"%d\"", rec_id);
      MESG("\nsubfont>> Content of mapping table:");
      for (__i = 0; __i < 256; __i++) {
        if (__i % 16 == 0)
          MESG("\nsubfont>>  ");
        MESG(" %04x", sfd_record[rec_id].vector[__i]);
      }
    }
    MESG("\n");
  }

  return  rec_id;
}


/* Lookup mapping table */
unsigned short
lookup_sfd_record (int rec_id, unsigned char c)
{
  if (!sfd_record ||
       rec_id < 0 || rec_id >= num_sfd_records)
    ERROR("Invalid subfont_id: %d", rec_id);
  return sfd_record[rec_id].vector[c];
}

void
release_sfd_record (void)
{
  int  i;

  if (sfd_record) {
    RELEASE(sfd_record);
  }
  if (sfd_files) {
    for (i = 0; i < num_sfd_files; i++) {
      clean_sfd_file_(&sfd_files[i]);
    }
    RELEASE(sfd_files);
  }
  sfd_record = NULL;
  sfd_files  = NULL;
  num_sfd_records = max_sfd_records = 0;
  num_sfd_files = max_sfd_files = 0;
}


#if  DPXTEST
/* SFD file dumper */
#ifdef HAVE_ICONV
#include <iconv.h>
#else
typedef int iconv_t;
#endif
#include <string.h>

static void
dump_table (const char *sfd_name, const char *sub_name, iconv_t cd)
{
  int  rec_id, i;

  rec_id = sfd_load_record(sfd_name, sub_name);
  if (rec_id < 0) {
    WARN("Could not load SFD mapping for \"%s\"", sub_name);
    return;
  }
  fprintf(stdout, "  <subfont id=\"%s\">\n", sub_name);
  for (i = 0; i < 256; i++) {
    unsigned short c = lookup_sfd_record(rec_id, i);
    char    *p, inbuf[2];
    char    *q, outbuf[32];
    size_t   r, inbufleft = 2, outbufleft = 32;

    if (c == 0)
      fprintf(stdout, "    <!-- %02x: undefined -->", i);
    else {
      fprintf(stdout, "    <a bi=\"%02x\" bo=\"%02x %02x\"", i, (c >> 8) & 0xff, c & 0xff);
      if (cd != (iconv_t) -1) {
        p = inbuf; q = outbuf;
        inbuf[0] = (c >> 8) & 0xff;
        inbuf[1] = c & 0xff;
#ifdef HAVE_ICONV
        r = iconv(cd, &p, &inbufleft, &q, &outbufleft);
        if (r == -1) {
          if (verbose) {
            WARN("Conversion to Unicode failed for subfont-id=\"%s\" code=\"0x%02x\"",
                 sub_name, i);
            WARN(">> with: %s", strerror(errno));
          }
        } else {
          outbuf[32-outbufleft] = 0;
          fprintf(stdout, " uc=\"%s\"", outbuf);
        }
#endif /* HAVE_ICONV */
      }
      fprintf(stdout, " />");
    }
    fprintf(stdout, "\n");
  }
  fprintf(stdout, "  </subfont>\n");
  return;
}

#define subfontDefinition_DTD "\
<!ELEMENT subfontDefinition (subfont+)>\n\
<!ATTLIST subfontDefinition\n\
  id CDATA #REQUIRED\n\
  output-encoding CDATA #IMPLIED\n\
>\n\
<!ELEMENT subfont (a*)>\n\
<!ATTLIST subfont\n\
  id CDATA #REQUIRED\n\
>\n\
<!ELEMENT a EMPTY>\n\
<!ATTLIST a\n\
  bi NMTOKENS #REQUIRED\n\
  bo NMTOKENS #REQUIRED\n\
  uc CDATA #IMPLIED\n\
>\
"

void
test_subfont_help (void)
{
  fprintf(stdout, "usage: subfont [options] SFD_name\n");
  fprintf(stdout, "-e, --encoding string\n");
  fprintf(stdout, "  Target (output) encoding of SFD mapping is 'string'.\n");
  fprintf(stdout, "  It must be an encoding name recognized by iconv.\n");
  fprintf(stdout, "  With this option write Unicode character in auxiliary attribute 'uc'.\n");
  fprintf(stdout, "-s, --subfont-id string\n");
  fprintf(stdout, "  Load and dump mapping table only for subfont 'string'.\n");
}

int
test_subfont_main (int argc, char *argv[])
{
  char   *sfd_name = NULL, *sub_name = NULL;
  char   *from = NULL;
  int     i;
  iconv_t cd = (iconv_t) -1;

  for (;;) {
    int  c, optidx = 0;
    static struct option long_options[] = {
      {"encoding",   1, 0, 'e'}, /* for to-Unicode conversion */
      {"subfont-id", 1, 0, 's'},
      {"help",       0, 0, 'h'},
      {0, 0, 0, 0}
    };
    c = getopt_long(argc, argv, "e:s:h", long_options, &optidx);
    if (c == -1)
      break;

    switch (c) {
    case  'e':
      from = optarg;
      break;
    case  's':
      sub_name = optarg;
      break;
    case  'h':
      test_subfont_help();
      return  0;
      break;
    default:
      test_subfont_help();
      return  -1;
      break;
    }
  }
  if (optind < argc) {
    sfd_name = argv[optind++];
  }

  if (sfd_name == NULL) {
    WARN("No SFD file name specified.");
    test_subfont_help();
    return  -1;
  }
  if (!from)
    cd = (iconv_t) -1;
  else {
#ifdef HAVE_ICONV
    cd = iconv_open("utf-8", from);
    if (cd == (iconv_t) -1) {
      WARN("Can't open iconv conversion descriptor for %s --> utf-8", from);
      return  -1;
    }
#else
    WARN("Your system doesn't have iconv() in libc!");
#endif
  }

  fprintf(stdout, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
  fprintf(stdout, "<!DOCTYPE subfontDefinition [\n%s\n]>\n",
          subfontDefinition_DTD);
  fprintf(stdout, "<subfontDefinition id=\"%s\" output-encoding=\"%s\">\n",
          sfd_name, from ? from : "unknown");
  if (sub_name == NULL || !strcmp(sub_name, "all")) {
    char **sub_id;
    int    num_ids = 0;
    sub_id = sfd_get_subfont_ids(sfd_name, &num_ids);
    if (!sub_id)
      WARN("Could not open SFD file: %s", sfd_name);
    else {
      for (i = 0; i < num_ids; i++)
        dump_table(sfd_name, sub_id[i], cd);
    }
  } else {
    dump_table(sfd_name, sub_name, cd);
  }
  fprintf(stdout, "</subfontDefinition>\n");

#ifdef HAVE_ICONV
  if (cd != (iconv_t) -1)
    iconv_close(cd);
#endif

  return  0;
}

#endif  /* DPXTEST */
