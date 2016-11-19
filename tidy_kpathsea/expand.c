/* expand.c: general expansion.

   Copyright 1993, 1994, 1995, 1996, 1997, 2005, 2008, 2009, 2011,
             2012, 2016 Karl Berry.
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

/* Do variable expansion first so ~${USER} works.  (Besides, it's what the
   shells do.)  */

string
kpathsea_expand (kpathsea kpse, const_string s)
{
  string var_expansion = kpathsea_var_expand (kpse, s);
  string tilde_expansion = kpathsea_tilde_expand (kpse, var_expansion);

  /* `kpse_var_expand' always gives us new memory; `kpse_tilde_expand'
     doesn't, necessarily.  So be careful that we don't free what we are
     about to return.  */
  if (tilde_expansion != var_expansion)
    free (var_expansion);

  return tilde_expansion;
}

/* Forward declarations of functions from the original expand.c  */
static str_list_type brace_expand (kpathsea, const_string*);

/* If $KPSE_DOT is defined in the environment, prepend it to any relative
   path components. */

static string
kpathsea_expand_kpse_dot (kpathsea kpse, string path)
{
  string ret, elt;
  string kpse_dot = getenv("KPSE_DOT");

  if (kpse_dot == NULL)
    return path;
  ret = (string)xmalloc(1);
  *ret = 0;

  for (elt = kpathsea_path_element (kpse, path); elt;
       elt = kpathsea_path_element (kpse, NULL)) {
    string save_ret = ret;
    boolean ret_copied = true;
    /* We assume that the !! magic is only used on absolute components.
       Single "." gets special treatment, as does "./" or its equivalent. */
    if (kpathsea_absolute_p (kpse, elt, false)
        || (elt[0] == '!' && elt[1] == '!')) {
      ret = concat3(ret, elt, ENV_SEP_STRING);
    } else if (elt[0] == '.' && elt[1] == 0) {
      ret = concat3 (ret, kpse_dot, ENV_SEP_STRING);
    } else if (elt[0] == '.' && IS_DIR_SEP(elt[1])) {
      ret = concatn (ret, kpse_dot, elt + 1, ENV_SEP_STRING, NULL);
    } else if (*elt) {
      ret = concatn (ret, kpse_dot, DIR_SEP_STRING, elt, ENV_SEP_STRING, NULL);
    } else {
      /* omit empty path elements from TEXMFCNF.
         See http://bugs.debian.org/358330.  */
      ret_copied = false;
    }
    if (ret_copied)
      free (save_ret);
  }

  ret[strlen (ret) - 1] = 0;
  return ret;
}

/* Do brace expansion on ELT; then do variable and ~ expansion on each
   element of the result; then do brace expansion again, in case a
   variable definition contained braces (e.g., $TEXMF).  Return a
   string comprising all of the results separated by ENV_SEP_STRING.  */

static string
kpathsea_brace_expand_element (kpathsea kpse, const_string elt)
{
  unsigned i;
  str_list_type expansions = brace_expand (kpse, &elt);
  string ret = (string)xmalloc (1);
  *ret = 0;

  for (i = 0; i != STR_LIST_LENGTH(expansions); i++) {
    /* Do $ and ~ expansion on each element.  */
    string x = kpathsea_expand (kpse, STR_LIST_ELT(expansions,i));
    string save_ret = ret;
    if (!STREQ (x, STR_LIST_ELT(expansions,i))) {
      /* If we did any expansions, do brace expansion again.  Since
         recursive variable definitions are not allowed, this recursion
         must terminate.  (In practice, it's unlikely there will ever be
         more than one level of recursion.)  */
      string save_x = x;
      x = kpathsea_brace_expand_element (kpse, x);
      free (save_x);
    }
    ret = concat3 (ret, x, ENV_SEP_STRING);
    free (save_ret);
    free (x);
  }
  for (i = 0; i != STR_LIST_LENGTH(expansions); ++i) {
      free(STR_LIST_ELT(expansions,i));
  }
  str_list_free(&expansions);
  ret[strlen (ret) - 1] = 0; /* waste the trailing null */
  return ret;
}

/* Be careful to not waste all the memory we allocate for each element.  */

string
kpathsea_brace_expand (kpathsea kpse, const_string path)
{
  string kpse_dot_expansion;
  string elt;
  unsigned len;
  /* Must do variable expansion first because if we have
       foo = .:~
       TEXINPUTS = $foo
     we want to end up with TEXINPUTS = .:/home/karl.
     Since kpse_path_element is not reentrant, we must get all
     the path elements before we start the loop.  */
  string xpath = kpathsea_var_expand (kpse, path);
  string ret = (string)xmalloc (1);
  *ret = 0;

  for (elt = kpathsea_path_element (kpse, xpath); elt;
       elt = kpathsea_path_element (kpse, NULL)) {
    string save_ret = ret;
    /* Do brace expansion first, so tilde expansion happens in {~ka,~kb}.  */
    string expansion = kpathsea_brace_expand_element (kpse, elt);
    ret = concat3 (ret, expansion, ENV_SEP_STRING);
    free (expansion);
    free (save_ret);
  }

  /* Waste the last byte by overwriting the trailing env_sep with a null.  */
  len = strlen (ret);
  if (len != 0)
    ret[len - 1] = 0;
  free (xpath);

  kpse_dot_expansion = kpathsea_expand_kpse_dot (kpse, ret);
  if (kpse_dot_expansion != ret)
    free (ret);

  return kpse_dot_expansion;
}

#if defined(KPSE_COMPAT_API)
string
kpse_brace_expand (const_string path)
{
    return kpathsea_brace_expand (kpse_def, path);
}
#endif

/* Expand all special constructs in a path, and include only the actually
   existing directories in the result. */
string
kpathsea_path_expand (kpathsea kpse, const_string path)
{
  string ret;
  string xpath;
  string elt;
  unsigned len;
  const_string ypath;

  /* Initialise ret to the empty string. */
  ret = (string)xmalloc (1);
  *ret = 0;
  len = 0;

  ypath = path;

  /* Expand variables and braces first.  */
  xpath = kpathsea_brace_expand (kpse, ypath);

  /* Now expand each of the path elements, printing the results */
  for (elt = kpathsea_path_element (kpse, xpath); elt;
       elt = kpathsea_path_element (kpse, NULL)) {
    str_llist_type *dirs;

    /* Skip and ignore magic leading chars.  */
    if (*elt == '!' && *(elt + 1) == '!')
      elt += 2;

    /* Search the disk for all dirs in the component specified.
       Be faster to check the database, but this is more reliable.  */
    dirs = kpathsea_element_dirs (kpse, elt);
    if (dirs && *dirs) {
      str_llist_elt_type *dir;

      for (dir = *dirs; dir; dir = STR_LLIST_NEXT (*dir)) {
        string thedir = STR_LLIST (*dir);
        unsigned dirlen = strlen (thedir);
        string save_ret = ret;
        /* We need to retain trailing slash if that's the root directory.
         * On unix, "/" is root dir, "" often taken to be current dir.
         * On windows, "C:/" is root dir of drive C, and "C:" is current
         * on drive C.  There's no need to look at other cases, like UNC
         * names.
         */
        if (dirlen == 1 || (dirlen == 3 && NAME_BEGINS_WITH_DEVICE (thedir)
                            && IS_DIR_SEP (thedir[2]))) {
          ret = concat3 (ret, thedir, ENV_SEP_STRING);
          len += dirlen + 1;
          ret[len - 1] = ENV_SEP;
        } else {
          ret = concat (ret, thedir);
          len += dirlen;
          ret [len - 1] = ENV_SEP;
        }
        free (save_ret);
      }
    }
  }
  /* Get rid of trailing ':', if any. */
  if (len != 0)
    ret[len - 1] = 0;
  return ret;
}

#if defined(KPSE_COMPAT_API)
string
kpse_path_expand (const_string path)
{
    return kpathsea_path_expand (kpse_def, path);
}
#endif

/* ... */
static void expand_append (str_list_type* partial,
                              const_string text, const_string p)
{
    string new_string;
    unsigned len;
    str_list_type tmp;
    tmp = str_list_init();
    len = p - text;
    new_string = (string)xmalloc(len+1);
    strncpy(new_string, text, len);
    new_string[len]=0;
    str_list_add(&tmp, new_string);
    str_list_concat_elements(partial, tmp);
}


static str_list_type
brace_expand (kpathsea kpse, const_string *text)
{
    str_list_type result, partial, recurse;
    const_string p;
    result = str_list_init();
    partial = str_list_init();
    for (p = *text; *p && *p != '}'; ++p) {
        /* FIXME: Should be IS_ENV_SEP(*p) */
        if (*p == ENV_SEP || *p == ',') {
            expand_append(&partial, *text, p);
            str_list_concat(&result, partial);
            str_list_free(&partial);
            *text = p+1;
            partial = str_list_init();
        } else if (*p == '{') {
            expand_append(&partial, *text, p);
            ++p;
            recurse = brace_expand(kpse, &p);
            str_list_concat_elements(&partial, recurse);
            str_list_free(&recurse);
            /* Check for missing closing brace. */
            if (*p != '}') {
                WARNING1 ("kpathsea: %s: Unmatched {", *text);
            }
            *text = p+1;
        } else if (*p == '$') {
            /* Skip ${VAR} */
            if (*(p+1) == '{')
                for (p+=2; *p!='}';++p);
        }
    }
    expand_append(&partial, *text, p);
    str_list_concat(&result, partial);
    str_list_free(&partial);
    *text = p;
    return result;
}
