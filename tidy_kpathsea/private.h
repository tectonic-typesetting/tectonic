#ifndef TIDY_KPATHSEA_PRIVATE_H
#define TIDY_KPATHSEA_PRIVATE_H

#define KPSE_COMPAT_API 1
#define ST_NLINK_TRICK
#define TRANSFORM(x) (x)
#define KPSE_DEBUG
#define XTALLOC(n, t) ((t *) xmalloc ((n) * sizeof (t)))
#define XTALLOC1(t) XTALLOC (1, t)

#define DEFAULT_TEXMFDBS "/nonesuch"
#define DEFAULT_WEB2C "/nonesuch"
#define DEFAULT_TEXINPUTS "/nonesuch"
#define DEFAULT_MFINPUTS "/nonesuch"
#define DEFAULT_MPINPUTS "/nonesuch"
#define DEFAULT_TEXFORMATS "/nonesuch"
#define DEFAULT_MFBASES "/nonesuch"
#define DEFAULT_MPMEMS "/nonesuch"
#define DEFAULT_TEXPOOL "/nonesuch"
#define DEFAULT_MFPOOL "/nonesuch"
#define DEFAULT_MPPOOL "/nonesuch"
#define DEFAULT_VFFONTS "/nonesuch"
#define DEFAULT_TFMFONTS "/nonesuch"
#define DEFAULT_PKFONTS "/nonesuch"
#define DEFAULT_GFFONTS "/nonesuch"
#define DEFAULT_GLYPHFONTS "/nonesuch"
#define DEFAULT_MISCFONTS "/nonesuch"
#define DEFAULT_TEXFONTMAPS "/nonesuch"
#define DEFAULT_BIBINPUTS "/nonesuch"
#define DEFAULT_BSTINPUTS "/nonesuch"
#define DEFAULT_MLBIBINPUTS "/nonesuch"
#define DEFAULT_MLBSTINPUTS "/nonesuch"
#define DEFAULT_RISINPUTS "/nonesuch"
#define DEFAULT_BLTXMLINPUTS "/nonesuch"
#define DEFAULT_MFTINPUTS "/nonesuch"
#define DEFAULT_TEXPSHEADERS "/nonesuch"
#define DEFAULT_T1FONTS "/nonesuch"
#define DEFAULT_AFMFONTS "/nonesuch"
#define DEFAULT_TTFONTS "/nonesuch"
#define DEFAULT_OPENTYPEFONTS "/nonesuch"
#define DEFAULT_T42FONTS "/nonesuch"
#define DEFAULT_LIGFONTS "/nonesuch"
#define DEFAULT_TEXCONFIG "/nonesuch"
#define DEFAULT_INDEXSTYLE "/nonesuch"
#define DEFAULT_ENCFONTS "/nonesuch"
#define DEFAULT_CMAPFONTS "/nonesuch"
#define DEFAULT_SFDFONTS "/nonesuch"
#define DEFAULT_FONTFEATURES "/nonesuch"
#define DEFAULT_FONTCIDMAPS "/nonesuch"
#define DEFAULT_PDFTEXCONFIG "/nonesuch"
#define DEFAULT_TRFONTS "/nonesuch"
#define DEFAULT_MPSUPPORT "/nonesuch"
#define DEFAULT_TEXDOCS "/nonesuch"
#define DEFAULT_TEXSOURCES "/nonesuch"
#define DEFAULT_WEBINPUTS "/nonesuch"
#define DEFAULT_CWEBINPUTS "/nonesuch"
#define DEFAULT_OFMFONTS "/nonesuch"
#define DEFAULT_OPLFONTS "/nonesuch"
#define DEFAULT_OVFFONTS "/nonesuch"
#define DEFAULT_OVPFONTS "/nonesuch"
#define DEFAULT_OTPINPUTS "/nonesuch"
#define DEFAULT_OCPINPUTS "/nonesuch"
#define DEFAULT_LUAINPUTS "/nonesuch"
#define DEFAULT_CLUAINPUTS "/nonesuch"
#define DEFAULT_TEXMFSCRIPTS "/nonesuch"
#define DEFAULT_TEXMFCNF "{$SELFAUTOLOC}"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct kpathsea_instance kpathsea_instance;
typedef kpathsea_instance *kpathsea;

/* str-list.h */

typedef struct
{
  unsigned length;
  string *list;
} str_list_type;

typedef struct
{
  unsigned length;
  const_string *list;
} cstr_list_type;

#define STR_LIST_LENGTH(l) ((l).length)
#define STR_LIST(l) ((l).list)
#define STR_LIST_ELT(l, n) STR_LIST (l)[n]
#define STR_LIST_LAST_ELT(l) STR_LIST_ELT (l, STR_LIST_LENGTH (l) - 1)

static inline str_list_type
str_list_init (void)
{
  str_list_type ret;

  STR_LIST_LENGTH (ret) = 0;
  STR_LIST (ret) = NULL;

  return ret;
}

static inline cstr_list_type
cstr_list_init (void)
{
  cstr_list_type ret;

  STR_LIST_LENGTH (ret) = 0;
  STR_LIST (ret) = NULL;

  return ret;
}

extern void str_list_add (str_list_type *l, string s);
extern void cstr_list_add (cstr_list_type *l, const_string s);

extern void str_list_concat (str_list_type * target, str_list_type more);
extern void str_list_free (str_list_type *l);
extern void str_list_concat_elements (str_list_type *target, str_list_type more);
extern void str_list_uniqify (str_list_type *l);

/* str-llist.h */

struct str_llist_elt
{
  string str;
  boolean moved;
  struct str_llist_elt *next;
};

typedef struct str_llist_elt str_llist_elt_type;
typedef struct str_llist_elt *str_llist_type;

#define STR_LLIST(sl) ((sl).str)
#define STR_LLIST_MOVED(sl) ((sl).moved)
#define STR_LLIST_NEXT(sl) ((sl).next)

extern void str_llist_add (str_llist_type *l, string e);
extern void str_llist_float (str_llist_type *l, str_llist_elt_type *e);

/* misc */

typedef struct
{
  const_string key;
  str_llist_type *value;
} cache_entry;

typedef struct {
  const_string var;
  boolean expanding;
} expansion_type;

typedef struct
{
  const_string type;            /* Human-readable description.  */
  string path;                  /* The search path to use.  */
  const_string raw_path;        /* Pre-$~ (but post-default) expansion.  */
  const_string path_source;     /* Where the path started from.  */
  const_string override_path;   /* From client environment variable.  */
  const_string client_path;     /* E.g., from dvips's config.ps.  */
  const_string cnf_path;        /* From texmf.cnf.  */
  const_string default_path;    /* If all else fails.  */
  const_string *suffix;         /* For kpse_find_file to check for/append.  */
  const_string *alt_suffix;     /* More suffixes to check for.  */
  boolean suffix_search_only;   /* Only search with a suffix?  */
  const_string program;         /* ``mktexpk'', etc.  */
  int argc;                     /* Count of standard arguments.  */
  const_string *argv;           /* Standard arguments to `program'.  */
  boolean program_enabled_p;    /* Invoke `program'?  */
  kpse_src_type program_enable_level; /* Who said to invoke `program'.  */
  boolean binmode;              /* Open files in binary mode?  */
} kpse_format_info_type;

/* db.h */

extern void kpathsea_init_db (kpathsea kpse);
extern str_list_type *kpathsea_db_search (kpathsea kpse, const_string name, const_string path_elt, boolean all);
extern str_list_type *kpathsea_db_search_list (kpathsea kpse, string* names, const_string  path_elt, boolean all);
extern void kpathsea_db_insert (kpathsea kpse, const_string fname);

/* debug.h */

#define KPATHSEA_DEBUG_P(bit) (kpse->debug & (1 << (bit)))
#define KPSE_DEBUG_SET(bit) kpathsea_debug |= 1 << (bit)
#define KPSE_DEBUG_P(bit) (kpathsea_debug & (1 << (bit)))

#define KPSE_DEBUG_STAT 0               /* stat calls */
#define KPSE_DEBUG_HASH 1               /* hash lookups */
#define KPSE_DEBUG_FOPEN 2              /* fopen/fclose calls */
#define KPSE_DEBUG_PATHS 3              /* search path initializations */
#define KPSE_DEBUG_EXPAND 4             /* path element expansion */
#define KPSE_DEBUG_SEARCH 5             /* searches */
#define KPSE_DEBUG_VARS 6               /* variable values */
#define KPSE_LAST_DEBUG KPSE_DEBUG_VARS

#define DEBUGF_START() do { fputs ("kdebug:", stderr)
#define DEBUGF_END()        fflush (stderr); } while (0)

#define DEBUGF(str)                                                     \
  DEBUGF_START (); fputs (str, stderr); DEBUGF_END ()
#define DEBUGF1(str, e1)                                                \
  DEBUGF_START (); fprintf (stderr, str, e1); DEBUGF_END ()
#define DEBUGF2(str, e1, e2)                                            \
  DEBUGF_START (); fprintf (stderr, str, e1, e2); DEBUGF_END ()
#define DEBUGF3(str, e1, e2, e3)                                        \
  DEBUGF_START (); fprintf (stderr, str, e1, e2, e3); DEBUGF_END ()
#define DEBUGF4(str, e1, e2, e3, e4)                                    \
  DEBUGF_START (); fprintf (stderr, str, e1, e2, e3, e4); DEBUGF_END ()

#undef fopen
#define fopen kpse_fopen_trace
extern FILE *fopen (const char *filename, const char *mode);
#undef fclose
#define fclose kpse_fclose_trace
extern int fclose (FILE *);

/* lib.h */

#define LIB_START_FATAL() do { \
  fprintf (stderr, "%s: fatal: ", kpse->invocation_name);

#define LIB_FATAL(str)                                                  \
  LIB_START_FATAL (); fputs (str, stderr); END_FATAL ()
#define LIB_FATAL1(str, e1)                                             \
  LIB_START_FATAL (); fprintf (stderr, str, e1); END_FATAL ()
#define LIB_FATAL2(str, e1, e2)                                         \
  LIB_START_FATAL (); fprintf (stderr, str, e1, e2); END_FATAL ()

/* c-ctype.h */

#define ISALNUM(c) (isascii (c) && isalnum((unsigned char)c))
#define ISALPHA(c) (isascii (c) && isalpha((unsigned char)c))
#define ISASCII isascii
#define ISCNTRL(c) (isascii (c) && iscntrl((unsigned char)c))
#define ISDIGIT(c) (isascii (c) && isdigit ((unsigned char)c))
#define ISGRAPH(c) (isascii (c) && isgraph((unsigned char)c))
#define ISLOWER(c) (isascii (c) && islower((unsigned char)c))
#define ISPRINT(c) (isascii (c) && isprint((unsigned char)c))
#define ISPUNCT(c) (isascii (c) && ispunct((unsigned char)c))
#define ISSPACE(c) (isascii (c) && isspace((unsigned char)c))
#define ISUPPER(c) (isascii (c) && isupper((unsigned char)c))
#define ISXDIGIT(c) (isascii (c) && isxdigit((unsigned char)c))
#define TOASCII toascii
#define TOLOWER(c) (ISUPPER (c) ? tolower ((unsigned char)c) : (c))
#define TOUPPER(c) (ISLOWER (c) ? toupper ((unsigned char)c) : (c))

/* c-pathch.h */

#ifndef IS_DIR_SEP_CH
#define IS_DIR_SEP_CH(ch) IS_DIR_SEP(ch)
#endif
#ifndef IS_DEVICE_SEP /* No `devices' on, e.g., Unix.  */
#define IS_DEVICE_SEP(ch) 0
#endif
#ifndef NAME_BEGINS_WITH_DEVICE
#define NAME_BEGINS_WITH_DEVICE(name) 0
#endif
#ifndef IS_UNC_NAME /* Unc names are in practice found on Win32 only. */
#define IS_UNC_NAME(name) 0
#endif

#ifndef ENV_SEP
# define ENV_SEP ':'
# define ENV_SEP_STRING ":"
#endif /* not ENV_SEP */

#ifndef IS_ENV_SEP
#define IS_ENV_SEP(ch) ((ch) == ENV_SEP)
#endif

/* hack for kpathutil split */

#define kpathsea_absolute_p(pkse, fn, rok) kpse_absolute_p(fn, rok)

/* hash.h */

typedef struct hash_element_struct
{
  const_string key;
  const_string value;
  struct hash_element_struct *next;
} hash_element_type;

typedef struct
{
  hash_element_type **buckets;
  unsigned size;
} hash_table_type;

extern hash_table_type hash_create (unsigned size);
extern void hash_insert (hash_table_type *table, const_string key, const_string value);
extern void hash_insert_normalized (hash_table_type *table, const_string key, const_string value);
extern void hash_remove (hash_table_type *table,  const_string key, const_string value);
extern const_string *hash_lookup (hash_table_type table, const_string key);
extern void hash_print (hash_table_type table, boolean summary_only);

/* pathsearch.h */

extern string kpathsea_path_element (kpathsea kpse, const_string path);
extern string kpathsea_filename_component (kpathsea kpse, const_string path);
extern unsigned kpathsea_normalize_path (kpathsea kpse, string elt);
extern str_llist_type *kpathsea_element_dirs (kpathsea kpse, string elt);
extern string *kpathsea_path_search_list_generic (kpathsea kpse, const_string path, string* names, boolean must_exist, boolean all);

/* kpathsea struct */

typedef struct kpathsea_instance {
    /* from cnf.c */
    p_record_input record_input;        /* for --recorder */
    p_record_output record_output;      /* for --recorder */
    hash_table_type cnf_hash;           /* used by read_all_cnf */
    boolean doing_cnf_init;             /* for kpse_cnf_get */
    /* from db.c */
    hash_table_type db;                 /* The hash table for all ls-R's */
    hash_table_type alias_db;           /* The hash table for the aliases */
    str_list_type db_dir_list;          /* list of ls-R's */
    /* from debug.c */
    unsigned debug;                     /* for --kpathsea-debug */
    /* from dir.c */
    hash_table_type link_table;         /* a hash of links-per-dir */
    /* from elt-dir.c */
    cache_entry *the_cache;
    unsigned cache_length;
    /* from fontmap.c */
    hash_table_type map;                /* the font mapping hash */
    const_string map_path;              /* path for kpse_fontmap_format */
    /* from hash.c */
    /* Print the hash values as integers if this is nonzero.  */
    boolean debug_hash_lookup_int;
    /* from path-elt.c */
    string elt;                         /* static buffer for return value */
    unsigned elt_alloc;
    const_string path;                  /* path we're currently working on */
    /* from pathsearch.c */
    boolean followup_search;
    FILE *log_file;
    boolean log_opened;                 /* Need to open the log file? */
    /* from progname.c */
    string invocation_name;
    string invocation_short_name;
    string program_name;                /* pretended name */
    int ll_verbose;                     /* for symlinks (conditional) */
    /* from tex-file.c */
    /* If non-NULL, try looking for this if can't find the real font.  */
    const_string fallback_font;
    /* If non-NULL, default list of fallback resolutions comes from this
       instead of the compile-time value.  Set by dvipsk for the R config
       cmd.  *SIZES environment variables override/use as default.  */
    const_string fallback_resolutions_string;
    /* If non-NULL, check these if can't find (within a few percent of) the
       given resolution.  List must end with a zero element.  */
    unsigned *fallback_resolutions;
    kpse_format_info_type format_info[kpse_last_format];
    /* from tex-make.c */
    /* We never throw away stdout, since that is supposed to be the filename
       found, if all is successful.  This variable controls whether stderr
       is thrown away.  */
    boolean make_tex_discard_errors;
    FILE *missfont;
    /* from variable.c  */
    expansion_type *expansions; /* sole variable of this type */
    unsigned expansion_len ;
    /* from xputenv.c */
    /* These record the strings we've set and have to keep around.
       This function can be called many times during a run, and this
       allows us to reclaim memory we allocated.  */
    char **saved_env;           /* keep track of changed items */
    int saved_count;
} kpathsea_instance;

extern kpathsea_instance kpse_def_inst;
extern kpathsea kpse_def;

#define kpathsea_debug kpse_def_inst.debug

/* cnf.h */

extern const_string kpathsea_cnf_get (kpathsea kpse, const_string name);
extern const_string kpse_cnf_get (const_string var);

/* default.h */

extern string kpathsea_expand_default (kpathsea kpse, const_string path, const_string dflt);

/* expand.h */

extern string kpathsea_expand (kpathsea kpse, const_string s);

extern string kpathsea_brace_expand (kpathsea kpse, const_string path);
extern string kpathsea_path_expand (kpathsea kpse, const_string path);
extern string kpse_brace_expand (const_string path);
extern string kpse_path_expand (const_string path);

/* fn.h */

typedef struct
{
  string str;
  unsigned allocated;
  unsigned length; /* includes the terminating null byte, if any */
} fn_type;

#define FN_STRING(fn) ((fn).str)
#define FN_ALLOCATED(fn) ((fn).allocated)
#define FN_LENGTH(fn) ((fn).length)

extern fn_type fn_init (void);
extern fn_type fn_copy0 (const_string s,  unsigned len);
extern void fn_free (fn_type *f);
extern void fn_1grow (fn_type *f, char c);
extern void fn_grow (fn_type *f, const_string source, unsigned length);
extern void fn_str_grow (fn_type *f, const_string s);
extern void fn_shrink_to (fn_type *f, unsigned loc);

/* fontmap.h */

extern const_string *kpathsea_fontmap_lookup (kpathsea kpse, const_string key);

/* pathsearch.h */

extern string kpathsea_path_search (kpathsea kpse, const_string path, const_string name, boolean must_exist);
extern string *kpathsea_all_path_search (kpathsea kpse, const_string path, const_string name);
extern string kpse_path_search (const_string path, const_string name, boolean must_exist);
extern string *kpse_all_path_search (const_string path, const_string name);

/* proginit.h */

extern void kpathsea_init_prog (kpathsea kpse, const_string prefix, unsigned dpi,
					const_string mode, const_string fallback);

extern void kpse_init_prog (const_string prefix,  unsigned dpi,  const_string mode,
				    const_string fallback);

/* absolute.h */

extern boolean kpathsea_absolute_p (kpathsea kpse, const_string filename, boolean relative_ok);

/* progname.h */

extern void kpathsea_set_program_name (kpathsea kpse, const_string argv0, const_string progname);
extern string kpathsea_selfdir (kpathsea kpse, const_string argv0);
extern string kpse_selfdir (const_string argv0);

/* readable.h */

extern string kpathsea_readable_file (kpathsea kpse, string name);

/* tex-file.h */

extern void kpathsea_set_program_enabled (kpathsea kpse, kpse_file_format_type fmt, boolean value, kpse_src_type level);
extern void kpathsea_maketex_option (kpathsea kpse, const_string fmtname, boolean value);
extern string kpathsea_find_file (kpathsea kpse, const_string name, kpse_file_format_type format,  boolean must_exist);
extern boolean kpathsea_in_name_ok (kpathsea kpse, const_string fname);
extern boolean kpathsea_out_name_ok (kpathsea kpse, const_string fname);
extern void kpathsea_reset_program_name (kpathsea kpse, const_string progname);
extern void kpathsea_init_fallback_resolutions (kpathsea kpse, string envvar);
extern void kpse_init_fallback_resolutions (string envvar);
extern void kpathsea_set_suffixes (kpathsea kpse, kpse_file_format_type format, boolean alternate, ...);
extern void kpse_set_suffixes (kpse_file_format_type format, boolean alternate, ...);
extern const_string kpathsea_init_format (kpathsea kpse,
    kpse_file_format_type format);
extern const_string kpathsea_init_format_return_varlist (kpathsea kpse,
  kpse_file_format_type format);
extern const_string kpse_init_format (kpse_file_format_type);
extern string *kpathsea_find_file_generic (kpathsea kpse,
     const_string name, kpse_file_format_type format, boolean must_exist,
     boolean all);
extern string *kpse_find_file_generic (const_string name, kpse_file_format_type format,
      boolean must_exist, boolean all);
extern boolean kpathsea_in_name_ok_silent (kpathsea kpse, const_string fname);
extern boolean kpathsea_out_name_ok_silent (kpathsea kpse, const_string fname);
extern FILE *kpathsea_open_file (kpathsea kpse, const_string name,
                                         kpse_file_format_type format);

/* tex-glyph.h */

typedef enum
{
  kpse_glyph_source_normal,  /* the searched-for font: already existed */
  kpse_glyph_source_alias,   /* : was an alias for an existing file */
  kpse_glyph_source_maketex, /* : was created on the fly */
  kpse_glyph_source_fallback /* : wasn't found, but the fallback font was */
} kpse_glyph_source_type;

typedef struct
{
  const_string name;            /* font name found */
  unsigned dpi;                 /* size found, for glyphs */
  kpse_file_format_type format; /* glyph format found */
  kpse_glyph_source_type source;        /* where we found it */
} kpse_glyph_file_type;

#define KPSE_GLYPH_FILE_NAME(f) ((f).name)
#define KPSE_GLYPH_FILE_DPI(f) ((f).dpi)
#define KPSE_GLYPH_FILE_FORMAT(f) ((f).format)
#define KPSE_GLYPH_FILE_SOURCE(f) ((f).source)


extern string kpathsea_find_glyph (kpathsea kpse,
                                  const_string font_name, unsigned dpi,
                                  kpse_file_format_type format,
                                  kpse_glyph_file_type *glyph_file);

#define KPSE_BITMAP_TOLERANCE(r) ((r) / 500.0 + 1)

extern boolean kpathsea_bitmap_tolerance (kpathsea kpse,
                                  double dpi1, double dpi2);

extern string kpse_find_glyph (const_string font_name, unsigned dpi,
                                  kpse_file_format_type format,
                                  kpse_glyph_file_type *glyph_file);

#define kpse_find_pk(font_name, dpi, glyph_file) \
  kpse_find_glyph (font_name, dpi, kpse_pk_format, glyph_file)
#define kpse_find_gf(font_name, dpi, glyph_file) \
  kpse_find_glyph (font_name, dpi, kpse_gf_format, glyph_file)

extern boolean kpse_bitmap_tolerance (double dpi1, double dpi2);

/* tex-hush.h */

extern boolean kpathsea_tex_hush (kpathsea kpse, const_string what);
extern boolean kpse_tex_hush (const_string what);

/* tex-make.h */

extern string kpathsea_make_tex (kpathsea kpse,
                                 kpse_file_format_type format,
                                 const_string base_file);
extern string kpse_make_tex (kpse_file_format_type format,
                             const_string base_file);

/* tilde.h */

extern string kpathsea_tilde_expand (kpathsea kpse, string filename);

/* variable.h */

extern string kpathsea_var_value (kpathsea kpse, const_string var);
extern string kpathsea_var_expand (kpathsea kpse, const_string src);
extern string kpse_var_expand (const_string src);

/* version.h */

extern const char *kpathsea_bug_address;

/* xopendir.h */

extern DIR *xopendir (const_string dirname);
extern void xclosedir (DIR *);

/* xstat.h */

#define SAME_FILE_P(s1, s2) ((s1).st_ino == (s2).st_ino && (s1).st_dev == (s2).st_dev)

extern struct stat xstat (const_string path);
extern struct stat xlstat (const_string path);

/* internal utilities */

extern unsigned atou (const_string);
extern string xdirname (const_string name);
extern boolean same_file_p (const_string filename1, const_string filename2);
extern string remove_suffix (const_string name);
extern string make_suffix (const_string s,  const_string suffix);
extern string make_prefix (string stem_prefix, string name);
extern void kpathsea_xputenv (kpathsea kpse, const_string var, const_string value);
extern void kpathsea_xputenv_int (kpathsea kpse, const_string var, int value);
extern void xputenv_int (const_string var, int value);
extern boolean kpathsea_dir_p (kpathsea kpse, string fn);
extern boolean dir_p (string fn);
extern int dir_links (const_string fn, long nlinks);
extern int kpathsea_dir_links (kpathsea kpse, const_string fn, long nlinks);
extern void xfseek (FILE *fp, long offset, int wherefrom, const_string filename);
extern void xfseeko (FILE *fp, off_t offset, int wherefrom, const_string filename);
extern long xftell (FILE *fp, const_string filename);
extern off_t xftello (FILE *fp, const_string filename);

#ifdef __cplusplus
}
#endif

#endif /* TIDY_KPATHSEA_PRIVATE_H */
