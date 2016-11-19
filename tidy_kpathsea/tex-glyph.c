/* tex-glyph.c: search for GF/PK files.

   Copyright 1993, 1994, 1995, 1996, 2008, 2009, 2011 Karl Berry.
   Copyright 1997, 1998, 1999, 2005 Olaf Weber.

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

/* Routines are in bottom-up order.  */

/* Support both cmr10.300pk and dpi300/cmr10.pk.  (Use the latter
   instead of dpi300\cmr10.pk since DOS supports /'s, but Unix doesn't
   support \'s.)  */
#define UNIX_BITMAP_SPEC "$KPATHSEA_NAME.$KPATHSEA_DPI$KPATHSEA_FORMAT"
#define DPI_BITMAP_SPEC  "dpi$KPATHSEA_DPI/$KPATHSEA_NAME.$KPATHSEA_FORMAT"

/* Look up font $KPATHSEA_NAME at resolution $KPATHSEA_DPI in PATH,
   with filename suffix EXTENSION.  Return file found or NULL.  */

static string
try_format (kpathsea kpse, kpse_file_format_type format)
{
  static const_string bitmap_specs[]
    = { UNIX_BITMAP_SPEC, DPI_BITMAP_SPEC,
        NULL };
  const_string *spec;
  boolean must_exist;
  const_string *sfx;
  string ret = NULL;
  const_string path = kpse->format_info[format].path;
  if (!path)
      path = kpathsea_init_format (kpse, format);

  /* Set the suffix on the name we'll be searching for.  */
  sfx = kpse->format_info[format].suffix;
  if (sfx && *sfx)
    kpathsea_xputenv (kpse, "KPATHSEA_FORMAT", *sfx);

  /* OK, the limits on this for loop are a little hokey, but it saves
     having to repeat the body.  We want to do it once with `must_exist'
     false to avoid looking on the disk for cmr10.600pk if
     dpi600/cmr10.pk is in ls-R.  (The time spent in the extra variable
     expansions and db searches is negligible.)  */
  for (must_exist = false; !ret && must_exist <= true; must_exist++)
    {
      for (spec = bitmap_specs; !ret && *spec; spec++)
        {
          string name = kpathsea_var_expand (kpse, *spec);
          ret = kpathsea_path_search (kpse, path, name, must_exist);
          if (name != ret)
            free (name);
        }
    }

  return ret;
}

/* Look for FONTNAME at resolution DPI in format FORMAT.  Search the
   (entire) PK path first, then the GF path, if we're looking for both.
   Return any filename found, and (if we succeeded) fill in GLYPH_FILE.  */

static string
try_size (kpathsea kpse, const_string fontname,  unsigned dpi,
          kpse_file_format_type format,
          kpse_glyph_file_type *glyph_file)
{
  kpse_file_format_type format_found;
  string ret;
  boolean try_gf = format == kpse_gf_format || format == kpse_any_glyph_format;
  boolean try_pk = format == kpse_pk_format || format == kpse_any_glyph_format;

  kpathsea_xputenv_int (kpse, "KPATHSEA_DPI", dpi);

  /* Look for PK first (since it's more likely to be found), then GF.  */
  ret = try_pk ? try_format (kpse, kpse_pk_format) : NULL;
  format_found = kpse_pk_format;

  if (ret == NULL && try_gf)
    {
      ret = try_format (kpse, kpse_gf_format);
      format_found = kpse_gf_format;
    }

  if (ret != NULL && glyph_file)
    { /* Success.  Fill in the return info.  */
      glyph_file->name = fontname;
      glyph_file->dpi = dpi;
      glyph_file->format = format_found;
    }

  return ret;
}

/* Look for FONTNAME at resolution DPI, then at the resolutions within
   KPSE_BITMAP_TOLERANCE of DPI.  */

static string
try_resolution (kpathsea kpse, const_string fontname,  unsigned dpi,
                kpse_file_format_type format,
                kpse_glyph_file_type *glyph_file)
{
  string ret = try_size (kpse, fontname, dpi, format, glyph_file);

  if (!ret) {
    unsigned r;
    unsigned tolerance = KPSE_BITMAP_TOLERANCE (dpi);
    /* Cast to unsigned to shut up stupid compilers. */
    unsigned lower_bound = (int) (dpi - tolerance) < 0
                           ? 0 : (unsigned)(dpi - tolerance);
    unsigned upper_bound = (unsigned)(dpi + tolerance);

    /* Prefer scaling up to scaling down, since scaling down can omit
       character features (Tom did this in dvips).  */
    for (r = lower_bound; !ret && r <= upper_bound; r++)
      if (r != dpi)
        ret = try_size (kpse, fontname, r, format, glyph_file);
  }

  return ret;
}

/* Look up *FONTNAME_PTR in format FORMAT at DPI in the texfonts.map files
   that we can find, returning the filename found and GLYPH_FILE.  Also
   set *FONTNAME_PTR to the real name corresponding to the alias found
   or the first alias, if that is not an alias itself.  (This allows
   mktexpk to only deal with real names.)  */

static string
try_fontmap (kpathsea kpse, const_string *fontname_ptr,  unsigned dpi,
             kpse_file_format_type format,
             kpse_glyph_file_type *glyph_file)
{
  const_string *mapped_names;
  const_string fontname = *fontname_ptr;
  string ret = NULL;

  mapped_names = kpathsea_fontmap_lookup (kpse, fontname);
  if (mapped_names) {
    const_string mapped_name;
    const_string first_name = *mapped_names;
    while (!ret && (mapped_name = *mapped_names++)) {
      kpathsea_xputenv (kpse, "KPATHSEA_NAME", mapped_name);
      ret = try_resolution (kpse, mapped_name, dpi, format, glyph_file);
    }
    if (ret) {
      /* If some alias succeeeded, return that alias.  */
      *fontname_ptr = xstrdup (mapped_name);
    /* Return first alias name, unless that itself is an alias,
       in which case do nothing.  */
    } else if (!kpathsea_fontmap_lookup (kpse, first_name)) {
      *fontname_ptr = xstrdup (first_name);
    }
  }

  return ret;
}

/* Look for FONTNAME in `kpse_fallback_resolutions', omitting DPI if we
   happen across it.  Return NULL if nothing found.  Pass GLYPH_FILE
   along as usual.  Assume `kpse_fallback_resolutions' is sorted.  */

static string
try_fallback_resolutions (kpathsea kpse,
                          const_string fontname,  unsigned dpi,
                          kpse_file_format_type format,
                          kpse_glyph_file_type *glyph_file)
{
  unsigned s;
  int loc, max_loc;
  int lower_loc, upper_loc;
  unsigned lower_diff, upper_diff;
  unsigned closest_diff = UINT_MAX;
  string ret = NULL; /* In case the only fallback resolution is DPI.  */
  loc = 0; /* -Wall */
  /* First find the fallback size closest to DPI, even including DPI.  */
  for (s = 0; kpse->fallback_resolutions[s] != 0; s++)
    {
      unsigned this_diff = abs (kpse->fallback_resolutions[s] - dpi);
      if (this_diff < closest_diff)
        {
          closest_diff = this_diff;
          loc = s;
        }
    }
  if (s == 0)
    return ret; /* If nothing in list, quit now.  */

  max_loc = s;
  lower_loc = loc - 1;
  upper_loc = loc + 1;

  for (;;)
    {
      unsigned fallback = kpse->fallback_resolutions[loc];
      /* Don't bother to try DPI itself again.  */
      if (fallback != dpi)
        {
          ret = try_resolution (kpse, fontname, fallback, format, glyph_file);
          if (ret)
            break;
        }

      /* That didn't work. How far away are the locs above or below?  */
      lower_diff = lower_loc > -1
                   ? dpi - kpse->fallback_resolutions[lower_loc] : INT_MAX;
      upper_diff = upper_loc < max_loc
                   ? kpse->fallback_resolutions[upper_loc] - dpi : INT_MAX;

      /* But if we're at the end in both directions, quit.  */
      if (lower_diff == INT_MAX && upper_diff == INT_MAX)
        break;

      /* Go in whichever direction is closest.  */
      if (lower_diff < upper_diff)
        {
          loc = lower_loc;
          lower_loc--;
        }
      else
        {
          loc = upper_loc;
          upper_loc++;
        }
    }

  return ret;
}

/* See the .h file for description.  This is the entry point.  */

string
kpathsea_find_glyph (kpathsea kpse,
                     const_string passed_fontname,  unsigned dpi,
                     kpse_file_format_type format,
                     kpse_glyph_file_type *glyph_file)
{
  string ret;
  kpse_glyph_source_type source;
  const_string fontname = passed_fontname;

  /* Start the search: try the name we're given.  */
  source = kpse_glyph_source_normal;
  kpathsea_xputenv (kpse, "KPATHSEA_NAME", fontname);
  ret = try_resolution (kpse, fontname, dpi, format, glyph_file);

  /* Try all the various possibilities in order of preference.  */
  if (!ret) {
    /* Maybe FONTNAME was an alias.  */
    source = kpse_glyph_source_alias;
    ret = try_fontmap (kpse, &fontname, dpi, format, glyph_file);

    /* If not an alias, try creating it on the fly with mktexpk,
       unless FONTNAME is absolute or explicitly relative.  */
    if (!ret && !kpathsea_absolute_p (kpse, fontname, true)) {
      source = kpse_glyph_source_maketex;
      /* `try_resolution' leaves the envvar set randomly.  */
      kpathsea_xputenv_int (kpse, "KPATHSEA_DPI", dpi);
      ret = kpathsea_make_tex (kpse, format, fontname);
    }

    /* If mktex... succeeded, set return struct.  Doesn't make sense for
       `kpse_make_tex' to set it, since it can only succeed or fail,
       unlike the other routines.  */
    if (ret && glyph_file) {
      KPSE_GLYPH_FILE_DPI (*glyph_file) = dpi;
      KPSE_GLYPH_FILE_NAME (*glyph_file) = fontname;
    }

    /* If mktex... failed, try any fallback resolutions.  */
    else {
      if (kpse->fallback_resolutions)
        ret = try_fallback_resolutions (kpse, fontname, dpi, format, glyph_file);

      /* We're down to the font of last resort.  */
      if (!ret && kpse->fallback_font) {
        const_string name = kpse->fallback_font;
        source = kpse_glyph_source_fallback;
        kpathsea_xputenv (kpse, "KPATHSEA_NAME", name);

        /* As before, first try it at the given size.  */
        ret = try_resolution (kpse, name, dpi, format, glyph_file);

        /* The fallback font at the fallback resolutions.  */
        if (!ret && kpse->fallback_resolutions)
          ret = try_fallback_resolutions (kpse, name, dpi, format, glyph_file);
      }
    }
  }

  /* If RET is null, then the caller is not supposed to look at GLYPH_FILE,
     so it doesn't matter if we assign something incorrect.  */
  if (glyph_file)
    KPSE_GLYPH_FILE_SOURCE (*glyph_file) = source;

  /* FIXME: fontname may have been allocated, but (worse) it may also
     have been assigned to struct that's passed out of this function.
  if (fontname != passed_fontname)
    free (fontname);
  */

  return ret;
}

#if defined (KPSE_COMPAT_API)
string
kpse_find_glyph (const_string passed_fontname,  unsigned dpi,
                 kpse_file_format_type format,
                 kpse_glyph_file_type *glyph_file)
{
    return kpathsea_find_glyph (kpse_def, passed_fontname, dpi, format,
                                glyph_file);
}
#endif


/* The tolerances change whether we base things on DPI1 or DPI2.  */

boolean
kpathsea_bitmap_tolerance (kpathsea kpse, double dpi1,  double dpi2)
{
  unsigned tolerance = KPSE_BITMAP_TOLERANCE (dpi2);
  unsigned lower_bound = (int) (dpi2 - tolerance) < 0 ? 0 : dpi2 - tolerance;
  unsigned upper_bound = dpi2 + tolerance;
  (void)kpse; /* currenty not used */

  return lower_bound <= dpi1 && dpi1 <= upper_bound;
}

#if defined (KPSE_COMPAT_API)
boolean
kpse_bitmap_tolerance (double dpi1,  double dpi2)
{
    return kpathsea_bitmap_tolerance (kpse_def, dpi1, dpi2);
}
#endif
