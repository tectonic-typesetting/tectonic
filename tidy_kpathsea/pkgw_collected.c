/* Collected kpathsea files in the tidied workalike version.

   Copyright 1993, 1994, 1995, 2008, 2009, 2010, 2011 Karl Berry.
   Copyright 1997, 2002, 2005 Olaf Weber.

   This library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   This library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this library; if not, see <http://www.gnu.org/licenses/>.  */

#include <tidy_kpathsea/public.h>
#include <tidy_kpathsea/private.h>

/* cnf.c */

/* By using our own hash table, instead of the environment, we
   complicate variable expansion (because we have to look in two
   places), but we don't bang so much on the system.  DOS and System V
   have very limited environment space.  Also, this way
   `kpse_init_format' can distinguish between values originating from
   the cnf file and ones from environment variables, which can be useful
   for users trying to figure out what's going on.  */

#define CNF_HASH_SIZE 751
#define CNF_NAME "texmf.cnf"


/* Do a single line in a cnf file: if it's blank or a comment or
   erroneous, skip it.  Otherwise, parse
     <variable>[.<program>] [=] <value>
   Do this even if the <variable> is already set in the environment,
   since the envvalue might contain a trailing :, in which case we'll be
   looking for the cnf value.

   We return NULL if ok, an error string otherwise.  */

static string
do_line (kpathsea kpse, string line)
{
  unsigned len;
  string start;
  string value, var;
  string prog = NULL;

  /* Skip leading whitespace.  */
  while (*line && ISSPACE (*line))
    line++;

  /* More to do only if we have non-comment material left.  */
  if (*line == 0 || *line == '%' || *line == '#')
    return NULL;

  /* Remove trailing comment: a % or # preceded by whitespace.  Also
     remove any whitespace before that.  For example, the value for
       foo = a#b  %something
     is a#b.  */
  value = line + strlen (line) - 1; /* start at end of line */
  while (value > line) {
    if (*value == '%' || *value == '#') {
      value--;                      /* move before comment char */
      while (ISSPACE (*value))
        *value-- = 0;               /* wipe out as much preceding whitespace
      continue;                        (and comment) as we find */
    }
    value--;                        /* move before the new null byte */
  }

  /* The variable name is everything up to the next space or = or `.'.  */
  start = line;
  while (*line && !ISSPACE (*line) && *line != '=' && *line != '.')
    line++;

  /* `line' is now one character past the end of the variable name.  */
  len = line - start;
  if (len == 0) {
    return ("No cnf variable name");
  }

  var = (string) xmalloc (len + 1);
  strncpy (var, start, len);
  var[len] = 0;

  /* If the variable is qualified with a program name, find out which. */
  while (*line && ISSPACE (*line))
    line++;
  if (*line == '.') {
    /* Skip spaces, then everything up to the next space or =.  */
    line++;
    while (ISSPACE (*line))
      line++;
    start = line;
    while (!ISSPACE (*line) && *line != '=')
      line++;

    /* It's annoying to repeat all this, but making a tokenizing
       subroutine would be just as long and annoying.  */
    len = line - start;
    prog = (string) xmalloc (len + 1);
    strncpy (prog, start, len);
    prog[len] = 0;
  }

  /* Skip whitespace, an optional =, more whitespace.  */
  while (*line && ISSPACE (*line))
    line++;
  if (*line == '=') {
    line++;
    while (*line && ISSPACE (*line))
      line++;
  }

  /* The value is whatever remains.  Remove trailing whitespace.  */
  start = line;
  len = strlen (start);
  while (len > 0 && ISSPACE (start[len - 1]))
    len--;
  if (len == 0) {
    return ("No cnf value");
  }

  value = (string) xmalloc (len + 1);
  strncpy (value, start, len);
  value[len] = 0;

  /* Suppose we want to write a single texmf.cnf that can be used under
     both NT and Unix.  This is feasible except for the path separators
     : on Unix, ; on NT.  We can't switch NT to allowing :'s, since :
     is the drive separator.  So we switch Unix to allowing ;'s.  On the
     other hand, we don't want to change IS_ENV_SEP and all the rest.

     So, simply translate all ;'s in the path
     values to :'s if we are a Unix binary.  (Fortunately we don't use ;
     in other kinds of texmf.cnf values.)  */

  if (IS_ENV_SEP(':')) {
      string loc;
      for (loc = value; *loc; loc++) {
          if (*loc == ';')
              *loc = ':';
      }
  }

  /* We want TEXINPUTS.prog to override plain TEXINPUTS.  The simplest
     way is to put both in the hash table (so we don't have to write
     hash_delete and hash_replace, and keep track of values' sources),
     and then look up the .prog version first in `kpse_cnf_get'.  */
  if (prog) {
    string lhs = concat3 (var, ".", prog);
    free (var);
    free (prog);
    var = lhs;
  }
  /* last-ditch debug */
  /* fprintf (stderr, "kpse/cnf.c hash_insert(%s,%s)\n", var, value); */
  hash_insert (&(kpse->cnf_hash), var, value);

  /* We should check that anything remaining is preceded by a comment
     character, but we don't.  Sorry.  */
  return NULL;
}

/* Read all the configuration files in the path.  */

static void
read_all_cnf (kpathsea kpse)
{
  kpse->cnf_hash = hash_create (CNF_HASH_SIZE);
}

/* Read the cnf files on the first call.  Return the first value in the
   returned list -- this will be from the last-read cnf file.  */

const_string
kpathsea_cnf_get (kpathsea kpse, const_string name)
{
  string ctry;
  const_string ret, *ret_list;

  /* When we expand the compile-time value for DEFAULT_TEXMFCNF,
     we end up needing the value for TETEXDIR and other variables,
     so kpse_var_expand ends up calling us again.  No good.  Except this
     code is not sufficient, somehow the ls-R path needs to be
     computed when initializing the cnf path.  Better to ensure that the
     compile-time path does not contain variable references.  */
  if (kpse->doing_cnf_init)
    return NULL;

  if (kpse->cnf_hash.size == 0) {
    /* Read configuration files and initialize databases.  */
    kpse->doing_cnf_init = true;
    read_all_cnf (kpse);
    kpse->doing_cnf_init = false;

    /* Since `kpse_init_db' recursively calls us, we must call it from
       outside a `kpse_path_element' loop (namely, the one in
       `read_all_cnf' above): `kpse_path_element' is not reentrant.  */
    kpathsea_init_db (kpse);
  }

  /* First look up NAME.`kpse->program_name', then NAME.  */
  assert (kpse->program_name);
  ctry = concat3 (name, ".", kpse->program_name);
  ret_list = hash_lookup (kpse->cnf_hash, ctry);
  free (ctry);
  if (ret_list) {
    ret = *ret_list;
    free (ret_list);
  } else {
    ret_list = hash_lookup (kpse->cnf_hash, name);
    if (ret_list) {
      ret = *ret_list;
      free (ret_list);
    } else {
      ret = NULL;
    }
  }

  return ret;
}

#if defined(KPSE_COMPAT_API)
const_string
kpse_cnf_get (const_string name)
{
    return kpathsea_cnf_get(kpse_def, name);
}
#endif

/* debug.c */

#ifdef KPSE_DEBUG /* whole file */

/* If the real definitions of fopen or fclose are macros, we lose -- the
   #undef won't restore them. */

FILE *
fopen (const char *filename,  const char *mode)
{
#undef fopen
  FILE *ret = fopen (filename, mode);
#if defined (KPSE_COMPAT_API)
  kpathsea kpse = kpse_def;
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_FOPEN))
    DEBUGF3 ("fopen(%s, %s) => 0x%lx\n", filename, mode, (unsigned long) ret);
#endif
  return ret;
}

int
fclose (FILE * f)
{
#undef fclose
  int ret = fclose (f);
#if defined (KPSE_COMPAT_API)
  kpathsea kpse = kpse_def;
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_FOPEN))
    DEBUGF2 ("fclose(0x%lx) => %d\n", (unsigned long) f, ret);
#endif
  return ret;
}

#endif /* KPSE DEBUG */

/* dir.c */

/* Return true if FN is a directory or a symlink to a directory,
   false if not. */

boolean
kpathsea_dir_p (kpathsea kpse, string fn)
{
  /* FIXME : using the stat() replacement in gnuw32,
         we could avoid this win32 specific code. However,
         I wonder if it would be as fast as this one is ?
  */
  struct stat stats;
  return stat (fn, &stats) == 0 && S_ISDIR (stats.st_mode);
}

#if defined(KPSE_COMPAT_API)
boolean
dir_p (string fn)
{
    return kpathsea_dir_p (kpse_def, fn);
}
#endif


/*
  Return -1 if FN isn't a directory, else its number of links.
  Duplicate the call to stat; no need to incur overhead of a function
  call for that little bit of cleanliness.

  The process is a bit different under Win32 : the first call
  memoizes the nlinks value, the following ones retrieve it.
*/
int
kpathsea_dir_links (kpathsea kpse, const_string fn, long nlinks)
{
  const_string *hash_ret;

  if (kpse->link_table.size == 0)
    kpse->link_table = hash_create (457);

#ifdef KPSE_DEBUG
  /* This is annoying, but since we're storing integers as pointers, we
     can't print them as strings.  */
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_HASH))
    kpse->debug_hash_lookup_int = true;
#endif

  hash_ret = hash_lookup (kpse->link_table, fn);

#ifdef KPSE_DEBUG
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_HASH))
    kpse->debug_hash_lookup_int = false;
#endif

  /* Have to cast the int we need to/from the const_string that the hash
     table stores for values. Let's hope an int fits in a pointer.  */
  if (hash_ret) {
      nlinks = (long) *hash_ret;
  } else {
      struct stat stats;
      if (stat (fn, &stats) == 0 && S_ISDIR (stats.st_mode))
        nlinks = stats.st_nlink;
      else
        nlinks = -1;
      /* It's up to us to copy the value.  */
      hash_insert(&(kpse->link_table), xstrdup(fn), (const_string)nlinks);

#ifdef KPSE_DEBUG
      if (KPATHSEA_DEBUG_P (KPSE_DEBUG_STAT))
        DEBUGF2 ("dir_links(%s) => %ld\n", fn, nlinks);
#endif
  }

  /* In any case, return nlinks
     (either 0, the value inserted or the value retrieved. */
  return nlinks;
}

#if defined (KPSE_COMPAT_API)
int
dir_links (const_string fn, long nlinks)
{
    return kpathsea_dir_links (kpse_def, fn, nlinks);
}
#endif

/* file-p.c */

/* Test whether FILENAME1 and FILENAME2 are actually the same file.  If
   stat fails on either of the names, we return false, without error.  */

boolean
same_file_p (const_string filename1,  const_string filename2)
{
    struct stat sb1, sb2;
    /* These are put in variables only so the results can be inspected
       under gdb.  */
    int r1 = stat (filename1, &sb1);
    int r2 = stat (filename2, &sb2);

    return r1 == 0 && r2 == 0 ? SAME_FILE_P (sb1, sb2) : false;
}

/* fn.c */

/* /usr/local/lib/texmf/fonts/public/cm/pk/ljfour/cmr10.300pk is 58
   chars, so ASCII `K' seems a good choice. */
#define CHUNK_SIZE 75


fn_type
fn_init (void)
{
  fn_type ret;

  FN_ALLOCATED (ret) = FN_LENGTH (ret) = 0;
  FN_STRING (ret) = NULL;

  return ret;
}


fn_type
fn_copy0 (const_string s,  unsigned len)
{
  fn_type ret;

  FN_ALLOCATED (ret) = CHUNK_SIZE > len ? CHUNK_SIZE : len + 1;
  FN_STRING (ret) = (string)xmalloc (FN_ALLOCATED (ret));

  strncpy (FN_STRING (ret), s, len);
  FN_STRING (ret)[len] = 0;
  FN_LENGTH (ret) = len + 1;

  return ret;
}

/* Don't think we ever try to free something that might usefully be
   empty, so give fatal error if nothing allocated.  */

void
fn_free (fn_type *f)
{
  assert (FN_STRING (*f) != NULL);
  free (FN_STRING (*f));
  FN_STRING (*f) = NULL;
  FN_ALLOCATED (*f) = 0;
  FN_LENGTH (*f) = 0;
}

/* An arithmetic increase seems more reasonable than geometric.  We
   don't increase the length member since it may be more convenient for
   the caller to add than subtract when appending the stuff that will
   presumably follow.  */

static void
grow (fn_type *f,  unsigned len)
{
  while (FN_LENGTH (*f) + len > FN_ALLOCATED (*f))
    {
      FN_ALLOCATED (*f) += CHUNK_SIZE;
      XRETALLOC (FN_STRING (*f), FN_ALLOCATED (*f), char);
    }
}


void
fn_1grow (fn_type *f,  char c)
{
  grow (f, 1);
  FN_STRING (*f)[FN_LENGTH (*f)] = c;
  FN_LENGTH (*f)++;
}


void
fn_grow (fn_type *f,  const_string source,  unsigned len)
{
  grow (f, len);
  strncpy (FN_STRING (*f) + FN_LENGTH (*f), source, len);
  FN_LENGTH (*f) += len;
}


void
fn_str_grow (fn_type *f,  const_string s)
{
  unsigned more_len = strlen (s);
  grow (f, more_len);
  strcat (FN_STRING (*f), s);
  FN_LENGTH (*f) += more_len;
}


void
fn_shrink_to (fn_type *f,  unsigned loc)
{
  assert (FN_LENGTH (*f) > loc);
  FN_STRING (*f)[loc] = 0;
  FN_LENGTH (*f) = loc + 1;
}

/* fontmap.c */

/* We have one and only one fontmap, so may as well make it static
   instead of passing it around.  */

#ifndef MAP_NAME
#define MAP_NAME "texfonts.map"
#endif
#ifndef MAP_HASH_SIZE
#define MAP_HASH_SIZE 4001
#endif


/* Return next whitespace-delimited token in STR or NULL if none.  */

static string
token (const_string str)
{
  unsigned len;
  const_string start;
  string ret;

  while (*str && ISSPACE (*str))
    str++;

  start = str;
  while (*str && !ISSPACE (*str))
    str++;

  len = str - start;
  ret = (string)xmalloc (len + 1);
  strncpy (ret, start, len);
  ret[len] = 0;

  return ret;
}

/* Open and read the mapping file MAP_FILENAME, putting its entries into
   MAP. Comments begin with % and continue to the end of the line.  Each
   line of the file defines an entry: the first word is the real
   filename (e.g., `ptmr'), the second word is the alias (e.g.,
   `Times-Roman'), and any subsequent words are ignored.  .tfm is added
   if either the filename or the alias have no extension.  This is the
   same order as in Dvips' psfonts.map.  Perhaps someday the programs
   will both read the same file.  */

static void
map_file_parse (kpathsea kpse, const_string map_filename)
{
  char *orig_l;
  unsigned map_lineno = 0;
  FILE *f = xfopen (map_filename, FOPEN_R_MODE);

  if (kpse->record_input)
    kpse->record_input (map_filename);

  while ((orig_l = read_line (f)) != NULL) {
    string filename;
    string l = orig_l; /* save for free() */
    string comment_loc = strrchr (l, '%');
    if (!comment_loc) {
      comment_loc = strstr (l, "@c");
    }

    /* Ignore anything after a % or @c.  */
    if (comment_loc)
      *comment_loc = 0;

    map_lineno++;

    /* Skip leading whitespace so we can use strlen below.  Can't use
       strtok since this routine is recursive.  */
    while (*l && ISSPACE (*l))
      l++;

    /* If we don't have any filename, that's ok, the line is blank.  */
    filename = token (l);
    if (filename) {
      string alias = token (l + strlen (filename));

      if (STREQ (filename, "include")) {
        if (alias == NULL) {
  WARNING2 ("kpathsea: %s:%u: Filename argument for include directive missing",
                    map_filename, map_lineno);
        } else {
          string include_fname = kpathsea_path_search (kpse,
                                   kpse->map_path, alias, false);
          if (include_fname) {
            map_file_parse (kpse, include_fname);
            if (include_fname != alias)
              free (include_fname);
          } else {
            WARNING3 ("kpathsea: %s:%u: Can't find fontname include file `%s'",
                      map_filename, map_lineno, alias);
          }
          free (alias);
          free (filename);
        }

      /* But if we have a filename and no alias, something's wrong.  */
      } else if (alias == NULL) {
        WARNING3 ("kpathsea: %s:%u: Fontname alias missing for filename `%s'",
                  map_filename, map_lineno, filename);
        free (filename);

      } else {
        /* We've got everything.  Insert the new entry.  They were
           already dynamically allocated by token(), so don't bother
           with xstrdup.  */
          hash_insert_normalized (&(kpse->map), alias, filename);
      }
    }

    free (orig_l);
  }

  xfclose (f, map_filename);
}

/* Parse the file MAP_NAME in each of the directories in PATH and
   return the resulting structure.  Entries in earlier files override
   later files.  */

static void
read_all_maps (kpathsea kpse)
{
  string *filenames;

  kpse->map_path = kpathsea_init_format (kpse, kpse_fontmap_format);
  filenames = kpathsea_all_path_search (kpse, kpse->map_path, MAP_NAME);

  kpse->map = hash_create (MAP_HASH_SIZE);

  while (*filenames) {
    map_file_parse (kpse, *filenames);
    filenames++;
  }
}

/* Look up KEY in texfonts.map's; if it's not found, remove any suffix
   from KEY and try again.  Create the map if necessary.  */

const_string *
kpathsea_fontmap_lookup (kpathsea kpse, const_string key)
{
  const_string *ret;
  const_string suffix = find_suffix (key);

  if (kpse->map.size == 0) {
    read_all_maps (kpse);
  }

  ret = hash_lookup (kpse->map, key);
  if (!ret) {
    /* OK, the original KEY didn't work.  Let's check for the KEY without
       an extension -- perhaps they gave foobar.tfm, but the mapping only
       defines `foobar'.  */
    if (suffix) {
      string base_key = remove_suffix (key);
      ret = hash_lookup (kpse->map, base_key);
      free (base_key);
    }
  }

  /* Append any original suffix.  */
  if (ret && suffix) {
    const_string *elt;
    for (elt = ret; *elt; elt++) {
      *elt = extend_filename (*elt, suffix);
    }
  }

  return ret;
}

/* hash.c */

/* The hash function.  We go for simplicity here.  */

/* All our hash tables are related to filenames.  */

static unsigned
hash (hash_table_type table,  const_string key)
{
  unsigned n = 0;

  /* Our keys aren't often anagrams of each other, so no point in
     weighting the characters.  */
  while (*key != 0)
#if defined (WIN32) && defined (KPSE_COMPAT_API)
    if (IS_KANJI(key)) {
      n = (n + n + (unsigned)(*key++)) % table.size;
      n = (n + n + (unsigned)(*key++)) % table.size;
    } else
#endif
    n = (n + n + TRANSFORM (*key++)) % table.size;

  return n;
}

/* Identical has function as above, but does not normalize keys. */
static unsigned
hash_normalized (hash_table_type table,  const_string key)
{
  unsigned n = 0;

  /* Our keys aren't often anagrams of each other, so no point in
     weighting the characters.  */
  while (*key != 0)
    n = (n + n + (*key++)) % table.size;

  return n;
}

hash_table_type
hash_create (unsigned size)
{
  /* The was "static ..." since Oct3, 1997 to work around a gcc
     optimizer bug for Alpha. That particular optimization bug
     should be gone by now (Mar4, 2009).
  */
  hash_table_type ret;
  unsigned b;
  ret.buckets = XTALLOC (size, hash_element_type *);
  ret.size = size;

  /* calloc's zeroes aren't necessarily NULL, so be safe.  */
  for (b = 0; b <ret.size; b++)
    ret.buckets[b] = NULL;

  return ret;
}

/* Whether or not KEY is already in TABLE, insert it and VALUE.  Do not
   duplicate the strings, in case they're being purposefully shared.  */

void
hash_insert (hash_table_type *table,
             const_string key,
             const_string value)
{
  unsigned n = hash (*table, key);
  hash_element_type *new_elt = XTALLOC1 (hash_element_type);

  new_elt->key = key;
  new_elt->value = value;
  new_elt->next = NULL;

  /* Insert the new element at the end of the list.  */
  if (!table->buckets[n])
    /* first element in bucket is a special case.  */
    table->buckets[n] = new_elt;
  else
    {
      hash_element_type *loc = table->buckets[n];
      while (loc->next)         /* Find the last element.  */
        loc = loc->next;
      loc->next = new_elt;      /* Insert the new one after.  */
    }
}

/* Same as above, for normalized keys. */
void
hash_insert_normalized (hash_table_type *table,
                        const_string key,
                        const_string value)
{
  unsigned n = hash_normalized (*table, key);
  hash_element_type *new_elt = XTALLOC1 (hash_element_type);

  new_elt->key = key;
  new_elt->value = value;
  new_elt->next = NULL;

  /* Insert the new element at the end of the list.  */
  if (!table->buckets[n])
    /* first element in bucket is a special case.  */
    table->buckets[n] = new_elt;
  else
    {
      hash_element_type *loc = table->buckets[n];
      while (loc->next)         /* Find the last element.  */
        loc = loc->next;
      loc->next = new_elt;      /* Insert the new one after.  */
    }
}

/* Remove a (KEY, VALUE) pair.  */

void
hash_remove (hash_table_type *table,  const_string key,
             const_string value)
{
  hash_element_type *p;
  hash_element_type *q;
  unsigned n = hash (*table, key);

  /* Find pair.  */
  for (q = NULL, p = table->buckets[n]; p != NULL; q = p, p = p->next)
    if (FILESTRCASEEQ (key, p->key) && STREQ (value, p->value))
      break;
  if (p) {
    /* We found something, remove it from the chain.  */
    if (q) q->next = p->next; else table->buckets[n] = p->next;
    /* We cannot dispose of the contents.  */
    free (p);
  }
}

/* Look up KEY in TABLE, and return NULL-terminated list of all matching
   values (not copies), in insertion order.  If none, return NULL.  */

const_string *
hash_lookup (hash_table_type table,  const_string key)
{
  hash_element_type *p;
  cstr_list_type ret;
  unsigned n = hash (table, key);
  ret = cstr_list_init ();

  /* Look at everything in this bucket.  */
  for (p = table.buckets[n]; p != NULL; p = p->next)
    if (FILESTRCASEEQ (key, p->key))
      cstr_list_add (&ret, p->value);

  /* If we found anything, mark end of list with null.  */
  if (STR_LIST (ret))
    cstr_list_add (&ret, NULL);

#ifdef KPSE_DEBUG
#if defined (KPSE_COMPAT_API)
  {
  kpathsea kpse = kpse_def;
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_HASH))
    {
      DEBUGF1 ("hash_lookup(%s) =>", key);
      if (!STR_LIST (ret))
        fputs (" (nil)\n", stderr);
      else
        {
          const_string *r;
          for (r = STR_LIST (ret); *r; r++)
            {
              putc (' ', stderr);
              if (kpse->debug_hash_lookup_int)
                fprintf (stderr, "%ld", (long) *r);
              else
                fputs (*r, stderr);
            }
          putc ('\n', stderr);
        }
      fflush (stderr);
    }
  }
#endif
#endif

  return STR_LIST (ret);
}

#ifdef KPSE_DEBUG
/* We only print nonempty buckets, to decrease output volume.  */

void
hash_print (hash_table_type table,  boolean summary_only)
{
  unsigned b;
  unsigned total_elements = 0, total_buckets = 0;

  for (b = 0; b < table.size; b++) {
    hash_element_type *bucket = table.buckets[b];

    if (bucket) {
      unsigned len = 1;
      hash_element_type *tb;

      total_buckets++;
      if (!summary_only) fprintf (stderr, "%4d ", b);

      for (tb = bucket->next; tb != NULL; tb = tb->next)
        len++;
      if (!summary_only) fprintf (stderr, ":%-5d", len);
      total_elements += len;

      if (!summary_only) {
        for (tb = bucket; tb != NULL; tb = tb->next)
          fprintf (stderr, " %s=>%s", tb->key, tb->value);
        putc ('\n', stderr);
      }
    }
  }

  fprintf (stderr,
          "%u buckets, %u nonempty (%u%%); %u entries, average chain %.1f.\n",
          table.size,
          total_buckets,
          100 * total_buckets / table.size,
          total_elements,
          total_buckets ? total_elements / (double) total_buckets : 0.0);
}
#endif

/* kdefault.c */

/* Check for leading colon first, then trailing, then doubled, since
   that is fastest.  Usually it will be leading or trailing.  */

string
kpathsea_expand_default (kpathsea kpse, const_string path,
                         const_string fallback)
{
  unsigned path_length;
  string expansion;
  (void)kpse; /* currenty not used */

  /* The default path better not be null.  */
  assert (fallback);

  if (path == NULL)
    expansion = xstrdup (fallback);

  /* Solitary or leading :?  */
  else if (IS_ENV_SEP (*path))
    {
      expansion = path[1] == 0 ? xstrdup (fallback) : concat (fallback, path);
    }

  /* Sorry about the assignment in the middle of the expression, but
     conventions were made to be flouted and all that.  I don't see the
     point of calling strlen twice or complicating the logic just to
     avoid the assignment (especially now that I've pointed it out at
     such great length).  */
  else if (path[(path_length = strlen (path)) - 1] == ENV_SEP)
    expansion = concat (path, fallback);

  /* OK, not leading or trailing.  Check for doubled.  */
  else
    {
      const_string loc;

      for (loc = path; *loc; loc++)
        if (IS_ENV_SEP (loc[0]) && IS_ENV_SEP (loc[1]))
          break;
      if (*loc)
        { /* We have a doubled colon.  */
          expansion = (string)xmalloc (path_length + strlen(fallback) + 1);

          /* Copy stuff up to and including the first colon.  */
          strncpy (expansion, path, loc - path + 1);
          expansion[loc - path + 1] = 0;

          /* Copy in FALLBACK, and then the rest of PATH.  */
          strcat (expansion, fallback);
          strcat (expansion, loc + 1);
        }
      else
        { /* No doubled colon. */
          expansion = xstrdup(path);
        }
    }

  return expansion;
}

/* kpathsea.c */

kpathsea
kpathsea_new (void)
{
    kpathsea ret;
    ret = xcalloc(1, sizeof(kpathsea_instance));
    return ret;
}

/* Sadly, quite a lot of the freeing is not safe:
   it seems there are literals used all over. */
void
kpathsea_finish (kpathsea kpse)
{
    if (kpse==NULL)
        return;
    if (kpse == kpse_def)
        return;
    free (kpse);
}

kpathsea_instance kpse_def_inst;
kpathsea kpse_def = &kpse_def_inst;

/* accessors to make it so we don't have to make the kpse instance struct public */

string
kpse_pkgw_get_definst_program_name (void)
{
    return kpse_def_inst.program_name;
}

string
kpse_pkgw_get_definst_invocation_name (void)
{
    return kpse_def_inst.invocation_name;
}

void
kpse_pkgw_set_definst_record_input (p_record_input val)
{
    kpse_def_inst.record_input = val;
}

void
kpse_pkgw_set_definst_record_output (p_record_output val)
{
    kpse_def_inst.record_output = val;
}

void
kpse_pkgw_set_definst_make_tex_discard_errors (boolean val)
{
    kpse_def_inst.make_tex_discard_errors = val;
}

/* make-suffix.c */

string
make_suffix (const_string s,  const_string suffix)
{
  string new_s;
  const_string dot_pos = strrchr (s, '.');
  const_string p;

  if (dot_pos)
    for (p = dot_pos + 1; *p; p++) {
      if (IS_DIR_SEP (*p)) {
        dot_pos = NULL;
        break;
      }
    }

  if (dot_pos == NULL)
    new_s = concat3 (s, ".", suffix);
  else
    {
      unsigned past_dot_index = dot_pos + 1 - s;

      new_s = (string)xmalloc (past_dot_index + strlen (suffix) + 1);
      strncpy (new_s, s, past_dot_index);
      strcpy (new_s + past_dot_index, suffix);
    }

  return new_s;
}

/* path-elt.c */

static string
element (kpathsea kpse, const_string passed_path,  boolean env_p)
{
  const_string p;
  string ret;
  int brace_level;
  unsigned len;

  if (passed_path)
    kpse->path = passed_path;
  /* Check if called with NULL, and no previous path (perhaps we reached
     the end).  */
  else if (!kpse->path)
    return NULL;

  /* OK, we have a non-null `path' if we get here.  */
  assert (kpse->path);
  p = kpse->path;

  /* Find the next colon not enclosed by braces (or the end of the path).  */
  brace_level = 0;
  while (*p != 0  && !(brace_level == 0
                       && (env_p ? IS_ENV_SEP (*p) : IS_DIR_SEP (*p)))) {
    if (*p == '{') ++brace_level;
    else if (*p == '}') --brace_level;
    p++;
  }

  /* Return the substring starting at `path'.  */
  len = p - kpse->path;

  /* Make sure we have enough space (including the null byte).  */
  if (len + 1 > kpse->elt_alloc)
    {
      kpse->elt_alloc = len + 1;
      kpse->elt = (string)xrealloc (kpse->elt, kpse->elt_alloc);
    }

  strncpy (kpse->elt, kpse->path, len);
  kpse->elt[len] = 0;
  ret = kpse->elt;

  /* If we are at the end, return NULL next time.  */
  if (kpse->path[len] == 0)
    kpse->path = NULL;
  else
    kpse->path += len + 1;

  return ret;
}

string
kpathsea_path_element (kpathsea kpse, const_string p)
{
    return element (kpse, p, true);
}

string
kpathsea_filename_component (kpathsea kpse, const_string p)
{
    return element (kpse, p, false);
}

/* proginit.c */

/* These initializations were common to all the drivers modified for
   kpathsea, so a single routine seemed in order.  Kind of a bollixed-up
   mess, but still better than repeating the code.  */

void
kpathsea_init_prog (kpathsea kpse, const_string prefix,  unsigned dpi,
                    const_string mode, const_string fallback)
{
  string font_var = concat (prefix, "FONTS");
  string header_var = concat (prefix, "HEADERS");
  string makepk_var = concat (prefix, "MAKEPK");
  string size_var = concat (prefix, "SIZES");

  /* Do both `pk_format' and `any_glyph_format' for the sake of xdvi; in
     general, mktexpk might apply to either, and the program will ask
     for the one it wants.  */

  /* Might have a program-specific name for mktexpk itself.  */
  if (getenv (makepk_var)) {
  /* If we did, we want to enable the program, I think.  */
    kpathsea_set_program_enabled (kpse, kpse_pk_format, 1, kpse_src_env);
    kpathsea_set_program_enabled (kpse, kpse_any_glyph_format, 1,kpse_src_env);

    kpse->format_info[kpse_pk_format].program
      = kpse->format_info[kpse_any_glyph_format].program
      = getenv (makepk_var);
  }

  /* A couple font paths have traditionally had application-specific
     environment variables to override all else; namely, XDVIFONTS and
     DVIPSHEADERS.  So set those if we have them.  */
  kpse->format_info[kpse_pk_format].override_path
    = kpse->format_info[kpse_gf_format].override_path
    = kpse->format_info[kpse_any_glyph_format].override_path
    = kpse->format_info[kpse_tfm_format].override_path
    = getenv (font_var);

  kpse->format_info[kpse_tex_ps_header_format].override_path
    = getenv (header_var);

  kpathsea_init_fallback_resolutions (kpse, size_var);
  kpathsea_xputenv_int (kpse, "MAKETEX_BASE_DPI", dpi);
  kpse->fallback_font = fallback;

  /* Ugliness.  See comments in kpse_make_tex in kpathsea/tex-make.c.  */
  kpathsea_xputenv (kpse, "MAKETEX_MODE", mode ? mode : DIR_SEP_STRING);

  free (font_var);
  free (header_var);
  free (makepk_var);
  free (size_var);
}

#if defined (KPSE_COMPAT_API)
void
kpse_init_prog (const_string prefix,  unsigned dpi,
                const_string mode, const_string fallback)
{
  kpathsea_init_prog(kpse_def,prefix,dpi,mode,fallback);
}
#endif

/* readable.c */

/* If access can read FN, run stat (assigning to stat buffer ST) and
   check that fn is not a directory.  Don't check for just being a
   regular file, as it is potentially useful to read fifo's or some
   kinds of devices.  */

#define READABLE(fn, st) \
  (access (fn, R_OK) == 0 && stat (fn, &(st)) == 0 && !S_ISDIR (st.st_mode))

/* POSIX invented the brain-damage of not necessarily truncating
   filename components; the system's behavior is defined by the value of
   the symbol _POSIX_NO_TRUNC, but you can't change it dynamically!  */

string
kpathsea_readable_file (kpathsea kpse, string name)
{
  struct stat st;

  kpathsea_normalize_path (kpse, name);
  if (READABLE (name, st)) {
      return name;
#ifdef ENAMETOOLONG
  } else if (errno == ENAMETOOLONG) {
      /* Truncate any too-long components in NAME.  */
      unsigned c_len = 0;        /* Length of current component.  */
      char *s = name;            /* Position in current component.  */
      char *t = name;            /* End of allowed length.  */

      for (; *s; s++) {
          if (c_len <= NAME_MAX)
              t = s;
          if (IS_DIR_SEP (*s) || IS_DEVICE_SEP (*s)) {
              if (c_len > NAME_MAX) {
                  /* Truncate if past the max for a component.  */
                  memmove (t, s, strlen (s) + 1);
                  s = t;
              }
              /* At a directory delimiter, reset component length.  */
              c_len = 0;
          } else
              c_len++;
      }
      if (c_len > NAME_MAX)
          /* Truncate if past the max for last component.  */
          *t = 0;

      /* Perhaps some other error will occur with the truncated name, so
         let's call access again.  */
      if (READABLE (name, st)) /* Success.  */
          return name;
#endif /* ENAMETOOLONG */
  } else { /* Some other error.  */
      if (errno == EACCES) { /* Maybe warn them if permissions are bad.  */
          if (!kpathsea_tex_hush (kpse, "readable")) {
              perror (name);
          }
      }
  }
  return NULL;
}

#if defined (KPSE_COMPAT_API)
string
kpse_readable_file (string name)
{
    return kpathsea_readable_file (kpse_def, name);
}
#endif

/* rm-suffix.c */

string
remove_suffix (const_string s)
{
  string ret;
  const_string suffix = find_suffix (s);

  if (suffix)
    {
      /* Back up to before the dot.  */
      suffix--;
      ret = (string) xmalloc (suffix - s + 1);
      strncpy (ret, s, suffix - s);
      ret[suffix - s] = 0;
    }
  else
    ret = xstrdup (s);

  return ret;
}

/* str-list.c */

/* See the .h file for comments.  */

void
str_list_add (str_list_type *l,  string s)
{
  STR_LIST_LENGTH (*l)++;
  XRETALLOC (STR_LIST (*l), STR_LIST_LENGTH (*l), string);
  STR_LIST_LAST_ELT (*l) = s;
}

void
cstr_list_add (cstr_list_type *l,  const_string s)
{
  STR_LIST_LENGTH (*l)++;
  XRETALLOC (STR_LIST (*l), STR_LIST_LENGTH (*l), const_string);
  STR_LIST_LAST_ELT (*l) = s;
}


/* May as well save some reallocations and do everything in a chunk
   instead of calling str_list_add on each element.  */

void
str_list_concat (str_list_type *target,  str_list_type more)
{
  unsigned e;
  unsigned prev_len = STR_LIST_LENGTH (*target);

  STR_LIST_LENGTH (*target) += STR_LIST_LENGTH (more);
  XRETALLOC (STR_LIST (*target), STR_LIST_LENGTH (*target), string);

  for (e = 0; e < STR_LIST_LENGTH (more); e++)
    STR_LIST_ELT (*target, prev_len + e) = STR_LIST_ELT (more, e);
}


/* Concatenate the elements of more to each element of target.  This
   _must_ be done with the first index varying fastest. */
/* Note that we free the old elements of target as well. */

void
str_list_concat_elements (str_list_type *target,  str_list_type more)
{
    if (STR_LIST_LENGTH(more) == 0) {
        return;
    } else if (STR_LIST_LENGTH(*target) == 0) {
        unsigned int i;
        STR_LIST_LENGTH(*target) = STR_LIST_LENGTH(more);
        STR_LIST(*target) =
                (string*)xmalloc(STR_LIST_LENGTH(more)*sizeof(char*));
        for (i=0;i!=STR_LIST_LENGTH(more);++i) {
            STR_LIST_ELT(*target,i)=xstrdup(STR_LIST_ELT(more,i));
        }
        return;
    } else {
        unsigned new_len;
        char ** new_list;
        unsigned int i,j;
        new_list = (string*)xmalloc(STR_LIST_LENGTH (*target)
                                    * STR_LIST_LENGTH (more) * sizeof(char*));

        new_len = 0;
        for (j = 0; j != STR_LIST_LENGTH(more); ++j) {
            for (i = 0; i != STR_LIST_LENGTH(*target); ++i) {
                new_list[new_len] = concat(STR_LIST_ELT(*target,i),
                                           STR_LIST_ELT(more,j));
                ++new_len;
            }
        }
        for (i = 0; i != STR_LIST_LENGTH(*target); ++i)
            free(STR_LIST_ELT(*target, i));
        free(STR_LIST(*target));
        STR_LIST_LENGTH(*target) = new_len;
        STR_LIST(*target) = new_list;
    }
}


/* Free the list (but not the elements within it).  */

void
str_list_free (str_list_type *l)
{
  if (STR_LIST (*l))
    {
      free (STR_LIST (*l));
      STR_LIST (*l) = NULL;
    }
}



/* Remove duplicate elements from L, freeing their space.  Since our
   lists are so short, we do a maximally inefficient bubble search.  */

void
str_list_uniqify (str_list_type *l)
{
  unsigned e;
  str_list_type ret = str_list_init ();

  for (e = 0; e < STR_LIST_LENGTH (*l); e++) {
    string elt1 = STR_LIST_ELT (*l, e);
    unsigned f;
    for (f = e + 1; f < STR_LIST_LENGTH (*l); f++) {
      string elt2 = STR_LIST_ELT (*l, f);
      /* I don't think our list should ever contain NULL's, but if
         it does, let it stay and don't bother collapsing multiple
         NULL's into one.  */
      if (FILESTRCASEEQ (elt1, elt2)) {
        break;
      }
    }

    if (f == STR_LIST_LENGTH (*l)) {
      str_list_add (&ret, elt1); /* not found */
    } else {
      free (elt1);  /* duplicate, forget this one */
    }
  }

  /* Replace the passed list with what we constructed.  */
  *l = ret;
}

/* str-llist.c */

/* Add the new string STR to the end of the list L.  */

void
str_llist_add (str_llist_type *l,  string str)
{
  str_llist_elt_type *e;
  str_llist_elt_type *new_elt = XTALLOC1 (str_llist_elt_type);

  /* The new element will be at the end of the list.  */
  STR_LLIST (*new_elt) = str;
  STR_LLIST_MOVED (*new_elt) = false;
  STR_LLIST_NEXT (*new_elt) = NULL;

  /* Find the current end of the list.  */
  for (e = *l; e && STR_LLIST_NEXT (*e); e = STR_LLIST_NEXT (*e))
    ;

  if (!e)
    *l = new_elt;
  else
    STR_LLIST_NEXT (*e) = new_elt;
}

/* Move an element towards the top. The idea is that when a file is
   found in a given directory, later files will likely be in that same
   directory, and looking for the file in all the directories in between
   is thus a waste.  */

void
str_llist_float (str_llist_type *l,  str_llist_elt_type *mover)
{
  str_llist_elt_type *last_moved, *unmoved;

  /* If we've already moved this element, never mind.  */
  if (STR_LLIST_MOVED (*mover))
    return;

  /* Find the first unmoved element (to insert before).  We're
     guaranteed this will terminate, since MOVER itself is currently
     unmoved, and it must be in L (by hypothesis).  */
  for (last_moved = NULL, unmoved = *l; STR_LLIST_MOVED (*unmoved);
       last_moved = unmoved, unmoved = STR_LLIST_NEXT (*unmoved))
    ;

  /* If we are the first unmoved element, nothing to relink.  */
  if (unmoved != mover)
    { /* Remember `mover's current successor, so we can relink `mover's
         predecessor to it.  */
      str_llist_elt_type *before_mover;
      str_llist_elt_type *after_mover = STR_LLIST_NEXT (*mover);

      /* Find `mover's predecessor.  */
      for (before_mover = unmoved; STR_LLIST_NEXT (*before_mover) != mover;
           before_mover = STR_LLIST_NEXT (*before_mover))
        ;

      /* `before_mover' now links to `after_mover'.  */
      STR_LLIST_NEXT (*before_mover) = after_mover;

      /* Insert `mover' before `unmoved' and after `last_moved' (or at
         the head of the list).  */
      STR_LLIST_NEXT (*mover) = unmoved;
      if (!last_moved)
        *l = mover;
      else
        STR_LLIST_NEXT (*last_moved) = mover;
    }

  /* We've moved it.  */
  STR_LLIST_MOVED (*mover) = true;
}

/* tex-hush.c */

boolean
kpathsea_tex_hush (kpathsea kpse, const_string what)
{
  string h;
  string hush = kpathsea_var_value (kpse, "TEX_HUSH");
  if (hush) {
    if (STREQ (hush, "all"))
        return true;
    if (STREQ (hush, "none"))
        return false;
    for (h = kpathsea_path_element (kpse, hush); h;
         h = kpathsea_path_element (kpse, NULL)) {
      /* Don't do anything special with empty elements.  */
      if (STREQ (h, what))
        return true;
    }
  }

  return false;
}

#if defined (KPSE_COMPAT_API)
boolean
kpse_tex_hush (const_string what)
{
    return kpathsea_tex_hush (kpse_def, what);
}
#endif

/* tex-make.c, edited to never do anything */

string
kpathsea_make_tex (kpathsea kpse, kpse_file_format_type format,
                   const_string base)
{
  return NULL;
}

#if defined (KPSE_COMPAT_API)
string
kpse_make_tex (kpse_file_format_type format,  const_string base)
{
  return kpathsea_make_tex (kpse_def, format, base);
}
#endif

/* tilde.c */

#undef USE_GETPWNAM
#include <pwd.h>
#define USE_GETPWNAM 1
#define HOMEVAR "HOME"

/* If NAME has a leading ~ or ~user, Unix-style, expand it to the user's
   home directory, and return a new malloced string.  If no ~, or no
   <pwd.h>, just return NAME.  */

string
kpathsea_tilde_expand (kpathsea kpse, string name)
{
  string expansion;
  const_string home;
  const_string prefix;

  (void)kpse; /* currenty not used */
  assert (name);

  /* If there is a leading "!!", set prefix to "!!", otherwise use
     the empty string.  After this, we can test whether a prefix was
     found by checking *prefix, and it is safe to unconditionally
     prepend it. */
  if (name[0] == '!' && name[1] == '!') {
    name += 2;
    prefix = "!!";
  } else {
    prefix = "";
  }

  /* If no leading tilde, do nothing, and return the original string.  */
  if (*name != '~'
     ) {
    if (*prefix)
      name -= 2;
    expansion = name;

  } else {
    /* If a bare tilde, return the home directory or `.'; if just `~user',
       return that user's home directory or `.'.  Very unlikely that the
       directory name will do anyone any good, but ...  */
    unsigned c;

    /* If `~user' or `~user/', look up user in the passwd database.  */
    if (name[1] && !IS_DIR_SEP (name[1])) {
      struct passwd *p;
      string user;
      c = 2;
      while (!IS_DIR_SEP (name[c]) && name[c] != 0) {  /* find user name */
        c++;
      }

      user = (string) xmalloc (c);
      strncpy (user, name + 1, c - 1);
      user[c - 1] = 0;

      /* We only need the cast here for (deficient) systems
         which do not declare `getpwnam' in <pwd.h>.  */
      p = (struct passwd *) getpwnam (user);
      free (user);

      /* If no such user, just use `.'.  */
      home = p ? p->pw_dir : ".";
    } else
    {
      c = 1;
      home = getenv (HOMEVAR);
      if (!home)
        home = ".";
    }

    /* handle leading // */
    if (IS_DIR_SEP (*home) && IS_DIR_SEP (home[1]))
      home++;

    /* If HOME ends in /, omit the / in ~/ or ~user/.  */
    if (name[c]) {
      if (IS_DIR_SEP (home[strlen (home) - 1]))
        c++;
    }

    expansion = concat3 (prefix, home, name + c);
  }

  /* We may return the same thing as the original, and then we might not
     be returning a malloc-ed string.  Callers beware.  Sorry.  */
  return expansion;
}

/* variable.c */

/* Here's the simple one, when a program just wants a value.  */

string
kpathsea_var_value (kpathsea kpse, const_string var)
{
  string vtry, ret;
  const_string value;

  assert (kpse->program_name);

  /* First look for VAR.progname. */
  vtry = concat3 (var, ".", kpse->program_name);
  value = getenv (vtry);
  free (vtry);

  if (!value || !*value) {
    /* Now look for VAR_progname. */
    vtry = concat3 (var, "_", kpse->program_name);
    value = getenv (vtry);
    free (vtry);
  }

  /* Just plain VAR.  */
  if (!value || !*value)
    value = getenv (var);

  /* Not in the environment; check a config file.  */
  if (!value || !*value)
      value = kpathsea_cnf_get (kpse, var);

  /* We have a value; do variable and tilde expansion.  We want to use ~
     in the cnf files, to adapt nicely to Windows and to avoid extra /'s
     (see tilde.c), but we also want kpsewhich -var-value=foo to not
     have any literal ~ characters, so our shell scripts don't have to
     worry about doing the ~ expansion.  */
  ret = value ? kpathsea_expand (kpse, value) : NULL;

#ifdef KPSE_DEBUG
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_VARS))
    DEBUGF2("variable: %s = %s\n", var, ret ? ret : "(nil)");
#endif

  return ret;
}

#if defined (KPSE_COMPAT_API)
string
kpse_var_value (const_string var)
{
    return kpathsea_var_value (kpse_def,var);
}
#endif


/* We have to keep track of variables being expanded, otherwise
   constructs like TEXINPUTS = $TEXINPUTS result in an infinite loop.
   (Or indirectly recursive variables, etc.)  Our simple solution is to
   add to a list each time an expansion is started, and check the list
   before expanding.  */

static void
expanding (kpathsea kpse, const_string var, boolean xp)
{
  unsigned e;
  for (e = 0; e < kpse->expansion_len; e++) {
    if (STREQ (kpse->expansions[e].var, var)) {
      kpse->expansions[e].expanding = xp;
      return;
    }
  }

  /* New variable, add it to the list.  */
  kpse->expansion_len++;
  XRETALLOC (kpse->expansions, kpse->expansion_len, expansion_type);
  kpse->expansions[kpse->expansion_len - 1].var = xstrdup (var);
  kpse->expansions[kpse->expansion_len - 1].expanding = xp;
}


/* Return whether VAR is currently being expanding.  */

static boolean
expanding_p (kpathsea kpse, const_string var)
{
  unsigned e;
  for (e = 0; e < kpse->expansion_len; e++) {
    if (STREQ (kpse->expansions[e].var, var))
      return kpse->expansions[e].expanding;
  }

  return false;
}

/* Append the result of value of `var' to EXPANSION, where `var' begins
   at START and ends at END.  If `var' is not set, do not complain.
   Return 1 if `var' was defined, 0 if not.  This is a subroutine for
   the `kpathsea_var_expand' function.  */

static boolean
expand (kpathsea kpse, fn_type *expansion,
        const_string start, const_string end)
{
  boolean ret = false;
  const_string value;
  unsigned len = end - start + 1;
  string var = (string)xmalloc (len + 1);
  strncpy (var, start, len);
  var[len] = 0;

  if (expanding_p (kpse, var)) {
    WARNING1 ("kpathsea: variable `%s' references itself (eventually)", var);
  } else {
    string vtry = concat3 (var, "_", kpse->program_name);
    /* Check for an environment variable.  */
    value = getenv (vtry);
    free (vtry);

    if (!value || !*value)
      value = getenv (var);

    /* If no envvar, check the config files.  */
    if (!value || !*value)
      value = kpathsea_cnf_get (kpse, var);

    if (value) {
      string tmp;
      ret = true;
      expanding (kpse, var, true);
      tmp = kpathsea_expand (kpse, value);
      expanding (kpse, var, false);

      fn_grow (expansion, tmp, strlen (tmp));
      free (tmp);
    }
  }

  free (var);
  return ret;
}

/* Can't think of when it would be useful to change these (and the
   diagnostic messages assume them), but ... */
#ifndef IS_VAR_START /* starts all variable references */
#define IS_VAR_START(c) ((c) == '$')
#endif
#ifndef IS_VAR_CHAR  /* variable name constituent */
#define IS_VAR_CHAR(c) (ISALNUM (c) || (c) == '_')
#endif
#ifndef IS_VAR_BEGIN_DELIMITER /* start delimited variable name (after $) */
#define IS_VAR_BEGIN_DELIMITER(c) ((c) == '{')
#endif
#ifndef IS_VAR_END_DELIMITER
#define IS_VAR_END_DELIMITER(c) ((c) == '}')
#endif


/* Maybe we should support some or all of the various shell ${...}
   constructs, especially ${var-value}.  We do do ~ expansion.  */

string
kpathsea_var_expand (kpathsea kpse, const_string src)
{
  const_string s;
  string ret;
  fn_type expansion;
  expansion = fn_init ();

  /* Copy everything but variable constructs.  */
  for (s = src; *s; s++) {
    if (IS_VAR_START (*s)) {
      s++;

      /* Three cases: `$VAR', `${VAR}', `$<anything-else>'.  */
      if (IS_VAR_CHAR (*s)) {
        /* $V: collect name constituents, then expand.  */
        const_string var_end = s;

        do {
          var_end++;
        } while (IS_VAR_CHAR (*var_end));

        var_end--; /* had to go one past */
        if (!expand (kpse, &expansion, s, var_end)) {
          /* If no expansion, include the literal $x construct,
             so filenames containing dollar signs can be read.
             The first +1 is to get the full variable name,
             the other +1 is to get the dollar sign; we've moved past it.  */
          fn_grow (&expansion, s - 1, var_end - s + 1 + 1);
        }
        s = var_end;

      } else if (IS_VAR_BEGIN_DELIMITER (*s)) {
        /* ${: scan ahead for matching delimiter, then expand.  */
        const_string var_end = ++s;

        while (*var_end && !IS_VAR_END_DELIMITER (*var_end)) {
          var_end++;
        }

        if (! *var_end) {
          WARNING1 ("kpathsea: %s: No matching } for ${", src);
          s = var_end - 1; /* will incr to null at top of loop */
        } else {
          expand (kpse, &expansion, s, var_end - 1);
          s = var_end; /* will incr past } at top of loop*/
        }

      } else {
        /* $<something-else>: warn, but preserve characters; again, so
           filenames containing dollar signs can be read.  */
        WARNING2 ("kpathsea: %s: Unrecognized variable construct `$%c'",
                  src, *s);
        fn_grow (&expansion, s - 1, 2);  /* moved past the $  */
      }
    } else
     fn_1grow (&expansion, *s);
  }
  fn_1grow (&expansion, 0);

  ret = FN_STRING (expansion);
  return ret;
}

#if defined (KPSE_COMPAT_API)
string
kpse_var_expand (const_string src)
{
    return kpathsea_var_expand (kpse_def,src);
}
#endif

/* version.c */

const char *kpathsea_version_string = "kpathsea version 6.2.3/dev";

/* If you are redistributing a modified version of the original
   distribution, please change this address here, among many other
   places.  Thanks.  */

const char *kpathsea_bug_address =
  "Email bug reports to tex-k@tug.org.\n";
