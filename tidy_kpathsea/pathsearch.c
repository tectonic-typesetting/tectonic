/* pathsearch.c: look up a filename in a path.

   Copyright 1993, 1994, 1995, 1997, 2007, 2009-2012 Karl Berry.
   Copyright 1997-2005 Olaf Weber.

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

#include <time.h> /* for `time' */

/* The very first search is for texmf.cnf, called when someone tries to
   initialize the TFM path or whatever.  init_path calls kpse_cnf_get
   which calls kpse_all_path_search to find all the texmf.cnf's.  We
   need to do various special things in this case, since we obviously
   don't yet have the configuration files when we're searching for the
   configuration files.  */



/* This function is called after every search (except the first, since
   we definitely want to allow enabling the logging in texmf.cnf) to
   record the filename(s) found in $TEXMFLOG.  */

static void
log_search (kpathsea kpse, str_list_type filenames)
{

  if (kpse->log_opened == false) {
    /* Get name from either envvar or config file.  */
      string log_name = kpathsea_var_value (kpse, "TEXMFLOG");
    kpse->log_opened = true;
    if (log_name) {
      kpse->log_file = fopen (log_name, FOPEN_A_MODE);
      if (!kpse->log_file)
        perror (log_name);
      free (log_name);
    }
  }

  if (
#ifdef KPSE_DEBUG
      KPATHSEA_DEBUG_P (KPSE_DEBUG_SEARCH) ||
#endif /* KPSE_DEBUG */
      kpse->log_file) {
    unsigned e;

    /* FILENAMES should never be null, but safety doesn't hurt.  */
    for (e = 0; e < STR_LIST_LENGTH (filenames) && STR_LIST_ELT (filenames, e);
         e++) {
      string filename = STR_LIST_ELT (filenames, e);

      /* Only record absolute filenames, for privacy.  */
      if (kpse->log_file && kpathsea_absolute_p (kpse, filename, false))
        fprintf (kpse->log_file, "%lu %s\n", (long unsigned) time (NULL),
                 filename);

#ifdef KPSE_DEBUG
      /* And show them online, if debugging.  We've already started
         the debugging line in `search', where this is called, so
         just print the filename here, don't use DEBUGF.  */
      if (KPATHSEA_DEBUG_P (KPSE_DEBUG_SEARCH)) {
        putc (' ', stderr);
        fputs (filename, stderr);
      }
#endif /* KPSE_DEBUG */
    }
  }
}

/* Concatenate each element in DIRS with NAME (assume each ends with a
   /, to save time).  If SEARCH_ALL is false, return the first readable
   regular file.  Else continue to search for more.  In any case, if
   none, return a list containing just NULL.

   We keep a single buffer for the potential filenames and reallocate
   only when necessary.  I'm not sure it's noticeably faster, but it
   does seem cleaner.  (We do waste a bit of space in the return
   value, though, since we don't shrink it to the final size returned.)  */

#define INIT_ALLOC 75  /* Doesn't much matter what this number is.  */

static str_list_type
dir_list_search (kpathsea kpse, str_llist_type *dirs,  const_string name,
                    boolean search_all)
{
  str_llist_elt_type *elt;
  str_list_type ret;
  unsigned name_len = strlen (name);
  unsigned allocated = INIT_ALLOC;
  string potential = (string)xmalloc (allocated);

  ret = str_list_init ();

  for (elt = *dirs; elt; elt = STR_LLIST_NEXT (*elt))
    {
      const_string dir = STR_LLIST (*elt);
      unsigned dir_len = strlen (dir);

      while (dir_len + name_len + 1 > allocated)
        {
          allocated += allocated;
          XRETALLOC (potential, allocated, char);
        }

      strcpy (potential, dir);
      strcat (potential, name);

      if (kpathsea_readable_file (kpse, potential))
        {
          str_list_add (&ret, potential);

          /* Move this element towards the top of the list.  */
          str_llist_float (dirs, elt);

          /* If caller only wanted one file returned, no need to
             terminate the list with NULL; the caller knows to only look
             at the first element.  */
          if (!search_all)
            return ret;

          /* Start new filename.  */
          allocated = INIT_ALLOC;
          potential = (string)xmalloc (allocated);
        }
    }

  /* If we get here, either we didn't find any files, or we were finding
     all the files.  But we're done with the last filename, anyway.  */
  free (potential);

  return ret;
}

/* Note: NAMES[i] is not modified.  */
static str_list_type
dir_list_search_list (kpathsea kpse, str_llist_type *dirs, string* names,
                      boolean search_all)
{
  str_llist_elt_type *elt;
  str_list_type ret;
  unsigned allocated = INIT_ALLOC;
  string potential = XTALLOC(allocated, char);

  ret = str_list_init ();

  for (elt = *dirs; elt; elt = STR_LLIST_NEXT(*elt)) {
      const_string dir = STR_LLIST (*elt);
      unsigned dir_len = strlen (dir);
      int i;

      for (i = 0; names[i]; i++) {
          const_string name = names[i];
          unsigned name_len;

          /* Don't bother with absolute & explicit relative. */
          if (kpathsea_absolute_p(kpse, name, true))
              continue;

          name_len = strlen(name);

          while (dir_len + name_len + 1 > allocated) {
              allocated += allocated;
              XRETALLOC (potential, allocated, char);
          }

          strcpy (potential, dir);
          strcat (potential+dir_len, name);

          if (kpathsea_readable_file (kpse, potential)) {
              str_list_add (&ret, potential);

              /* Move this element towards the top of the list.  */
              str_llist_float (dirs, elt);

              /* If caller only wanted one file returned, no need to
                 terminate the list with NULL; the caller knows to only look
                 at the first element.  */
              if (!search_all)
                  return ret;

              /* Start new filename. */
              allocated = INIT_ALLOC;
              potential = XTALLOC(allocated, char);
          }
      }
  }

  /* If we get here, either we didn't find any files, or we were finding
     all the files.  But we're done with the last filename, anyway.  */
  free (potential);

  return ret;
}

/* This is called when NAME is absolute or explicitly relative; if it's
   readable, return (a list containing) it; otherwise, return NULL.  */

static str_list_type
absolute_search (kpathsea kpse, string name)
{
  str_list_type ret_list;
  string found = kpathsea_readable_file (kpse, name);

  /* Some old compilers can't initialize structs.  */
  ret_list = str_list_init ();

  /* If NAME wasn't found, free the expansion.  */
  if (name != found)
    free (name);

  /* Add `found' to the return list even if it's null; that tells
     the caller we didn't find anything.  */
  str_list_add (&ret_list, found);

  return ret_list;
}

/* This is the hard case -- look for NAME in PATH.  If ALL is false,
   return the first file found.  Otherwise, search all elements of PATH.  */

static str_list_type
path_search (kpathsea kpse, const_string path,  string name,
             boolean must_exist,  boolean all)
{
  string elt;
  str_list_type ret_list;
  boolean done = false;
  ret_list = str_list_init (); /* some compilers lack struct initialization */

  for (elt = kpathsea_path_element (kpse, path); !done && elt;
       elt = kpathsea_path_element (kpse, NULL)) {
    str_list_type *found;
    boolean allow_disk_search = true;

    if (*elt == '!' && *(elt + 1) == '!') {
      /* Those magic leading chars in a path element means don't search the
         disk for this elt.  And move past the magic to get to the name.  */
      allow_disk_search = false;
      elt += 2;
    }

    /* See elt-dirs.c for side effects of this function */
    kpathsea_normalize_path(kpse, elt);

    /* Try ls-R, unless we're searching for texmf.cnf.  Our caller
       (search), also tests first_search, and does the resetting.  */
    found = kpse->followup_search ? kpathsea_db_search (kpse, name, elt, all)
                                  : NULL;

    /* Search the filesystem if (1) the path spec allows it, and either
         (2a) we are searching for texmf.cnf ; or
         (2b) no db exists; or
         (2c) no db's are relevant to this elt; or
         (3) MUST_EXIST && NAME was not in the db.
       In (2*), `found' will be NULL.
       In (3),  `found' will be an empty list. */
    if (allow_disk_search && (!found || (must_exist && !STR_LIST (*found)))) {
        str_llist_type *dirs = kpathsea_element_dirs (kpse, elt);
      if (dirs && *dirs) {
        if (!found)
          found = XTALLOC1 (str_list_type);
        *found = dir_list_search (kpse, dirs, name, all);
      }
    }

    /* Did we find anything anywhere?  */
    if (found && STR_LIST (*found)) {
      if (all)
        str_list_concat (&ret_list, *found);
      else {
        str_list_add (&ret_list, STR_LIST_ELT (*found, 0));
        done = true;
      }
    }

    /* Free the list space, if any (but not the elements).  */
    if (found) {
      str_list_free (found);
      free (found);
    }
  }

  /* Free the expanded name we were passed.  It can't be in the return
     list, since the path directories got unconditionally prepended.  */
  free (name);

  return ret_list;
}

/* Search PATH for ORIGINAL_NAME.  If ALL is false, or ORIGINAL_NAME is
   absolute_p, check ORIGINAL_NAME itself.  Otherwise, look at each
   element of PATH for the first readable ORIGINAL_NAME.

   Always return a list; if no files are found, the list will
   contain just NULL.  If ALL is true, the list will be
   terminated with NULL.  */

static string *
search (kpathsea kpse, const_string path,  const_string original_name,
        boolean must_exist,  boolean all)
{
  str_list_type ret_list;
  string name;
  boolean absolute_p;

  /* Make a leading ~ count as an absolute filename, and expand $FOO's.  */
  name = kpathsea_expand (kpse, original_name);

  /* If the first name is absolute or explicitly relative, no need to
     consider PATH at all.  */
  absolute_p = kpathsea_absolute_p (kpse, name, true);

#ifdef KPSE_DEBUG
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_SEARCH))
    DEBUGF4 ("start search(file=%s, must_exist=%d, find_all=%d, path=%s).\n",
             name, must_exist, all, path);
#endif /* KPSE_DEBUG */

  /* Find the file(s). */
  ret_list = absolute_p ? absolute_search (kpse, name)
                        : path_search (kpse, path, name, must_exist, all);

  /* Append NULL terminator if we didn't find anything at all, or we're
     supposed to find ALL and the list doesn't end in NULL now.  */
  if (STR_LIST_LENGTH (ret_list) == 0
      || (all && STR_LIST_LAST_ELT (ret_list) != NULL))
    str_list_add (&ret_list, NULL);

  /* The very first search is for texmf.cnf.  We can't log that, since
     we want to allow setting TEXMFLOG in texmf.cnf.  */
  if (kpse->followup_search == false) {
    kpse->followup_search = true;
  } else {
    /* Record the filenames we found, if desired.  And wrap them in a
       debugging line if we're doing that.  */
#ifdef KPSE_DEBUG
    if (KPATHSEA_DEBUG_P (KPSE_DEBUG_SEARCH))
      DEBUGF1 ("search(%s) =>", original_name);
#endif /* KPSE_DEBUG */
    log_search (kpse, ret_list);
#ifdef KPSE_DEBUG
    if (KPATHSEA_DEBUG_P (KPSE_DEBUG_SEARCH))
      putc ('\n', stderr);
#endif /* KPSE_DEBUG */
  }

  return STR_LIST (ret_list);
}

/* Search PATH for NAMES.

   Always return a list; if no files are found, the list will
   contain just NULL.  If ALL is true, the list will be
   terminated with NULL.  */

string *
kpathsea_path_search_list_generic (kpathsea kpse,
                                   const_string path, string* names,
                                   boolean must_exist, boolean all)
{
  str_list_type ret_list;
  string* namep;
  string elt;
  boolean done = false;
  boolean all_absolute = true;

  ret_list = str_list_init();

#ifdef KPSE_DEBUG
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_SEARCH)) {
    DEBUGF1  ("start search(files=[%s", *names);
    for (namep = names+1; *namep != NULL; namep++) {
      fputc(' ', stderr);
      fputs(*namep, stderr);
    }
    fprintf (stderr, "], must_exist=%d, find_all=%d, path=%s).\n",
             must_exist, all, path);
  }
#endif /* KPSE_DEBUG */

  /* FIXME: is this really true?  No need to do any expansion on names.  */

  /* First catch any absolute or explicit relative names. */
  for (namep = names; *namep; namep++) {
    if (kpathsea_absolute_p (kpse, *namep, true)) {
      if (kpathsea_readable_file (kpse, *namep)) {
        str_list_add (&ret_list, xstrdup(*namep));
        if (!all)
          goto out;
      }
    } else {
      all_absolute = false;
    }
  }
  /* Shortcut: if we were only given absolute/explicit relative names,
     we can skip the rest.  Typically, if one name is absolute, they
     all are, because our caller derived them from each other. */
  if (all_absolute)
      goto out;

  /* Look at each path element in turn. */
  for (elt = kpathsea_path_element (kpse, path); !done && elt;
       elt = kpathsea_path_element (kpse, NULL))
  {
    str_list_type *found;
    boolean allow_disk_search = true;
    if (elt[0] == '!' && elt[1] == '!') {
      /* !! magic -> disallow disk searches. */
      allow_disk_search = false;
      elt += 2;
    }

    /* See elt-dirs.c for side effects of this function. */
    kpathsea_normalize_path (kpse, elt);

    /* Try ls-R, unless we're searching for texmf.cnf. */
    found = kpse->followup_search
            ? kpathsea_db_search_list (kpse, names, elt, all) : NULL;

    /* Search the filesystem if (1) the path spec allows it, and either
         (2a) we are searching for texmf.cnf ; or
         (2b) no db exists; or
         (2c) no db's are relevant to this elt; or
         (3) MUST_EXIST && NAME was not in the db.
       In (2*), `found' will be NULL.
       In (3),  `found' will be an empty list. */
    if (allow_disk_search && (!found || (must_exist && !STR_LIST(*found)))) {
        str_llist_type *dirs = kpathsea_element_dirs (kpse, elt);
      if (dirs && *dirs) {
        if (!found)
          found = XTALLOC1 (str_list_type);
        *found = dir_list_search_list (kpse, dirs, names, all);
      }
    }

    /* Did we find anything? */
    if (found && STR_LIST (*found)) {
      if (all) {
        str_list_concat (&ret_list, *found);
      } else {
        str_list_add (&ret_list, STR_LIST_ELT (*found, 0));
        done = true;
      }
    }
  }

 out:
  /* Uniqify, since our paths can often end up finding the same file
     more than once.  */
  str_list_uniqify (&ret_list);

  /* Add NULL if we will be returning multiple elements.  */
  if (STR_LIST_LENGTH (ret_list) == 0
      || (all && STR_LIST_LAST_ELT (ret_list) != NULL))
    str_list_add (&ret_list, NULL);

  if (kpse->followup_search == false) {
    kpse->followup_search = true;
  } else {
    /* Record the filenames we found, if desired.  And wrap them in a
       debugging line if we're doing that.  */
#ifdef KPSE_DEBUG
    if (KPATHSEA_DEBUG_P (KPSE_DEBUG_SEARCH)) {
      DEBUGF1 ("search([%s", *names);
      for (namep = names+1; *namep != NULL; namep++) {
        fputc (' ', stderr);
        fputs (*namep, stderr);
      }
      fputs ("]) =>", stderr);
    }
#endif /* KPSE_DEBUG */
    log_search (kpse, ret_list);
#ifdef KPSE_DEBUG
    if (KPATHSEA_DEBUG_P (KPSE_DEBUG_SEARCH))
      putc ('\n', stderr);
#endif /* KPSE_DEBUG */
  }

  return STR_LIST (ret_list);
}

/* Search PATH for the first NAME according to MUST_EXIST.  */

string
kpathsea_path_search (kpathsea kpse, const_string path, const_string name,
                      boolean must_exist)
{
  string *ret_list = search (kpse, path, name, must_exist, false);
  string ret = *ret_list;
  free (ret_list);
  return ret;
}

/* Search PATH for all files named NAME.  Might have been better not
   to assert `must_exist' here, but it's too late to change.  */

string *
kpathsea_all_path_search (kpathsea kpse, const_string path, const_string name)
{
    string *ret = search (kpse, path, name, true, true);
  return ret;
}

#if defined (KPSE_COMPAT_API)

string
kpse_path_search (const_string path,  const_string name, boolean must_exist)
{
    return kpathsea_path_search (kpse_def, path,  name, must_exist);
}

string *
kpse_all_path_search (const_string path,  const_string name)
{
    return kpathsea_all_path_search (kpse_def,  path, name);
}
#endif
