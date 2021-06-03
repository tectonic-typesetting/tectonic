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

#include "dpx-otl_conf.h"

#include <assert.h>
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-agl.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"

#define VERBOSE_LEVEL_MIN 0
static int verbose = 0;
void
otl_conf_set_verbose (int level)
{
  verbose = level;
}

static pdf_obj *
parse_uc_coverage (pdf_obj *gclass, const char **pp, const char *endptr)
{
  pdf_obj *coverage;
  pdf_obj *value;
  int32_t  ucv = 0;
  char    *glyphname, *glyphclass;

  if (*pp + 1 >= endptr)
    return NULL;

  if (**pp == '[')
    (*pp)++;

  coverage = pdf_new_array();

  while (*pp < endptr) {
    skip_white(pp, endptr);
    switch (**pp) {
    case ']': case ';':
      (*pp)++;
      return coverage;
    case ',':
      (*pp)++;
      break;
    case '@':
      {
        pdf_obj *cvalues;
        int      i, size;

        (*pp)++;
        glyphclass = parse_c_ident(pp, endptr);
        cvalues = pdf_lookup_dict(gclass, glyphclass);
        if (!cvalues)
          _tt_abort("%s not defined...", glyphclass);
        size    = pdf_array_length(cvalues);
        for (i = 0; i < size; i++) {
          pdf_add_array(coverage,
                        pdf_link_obj(pdf_get_array(cvalues, i)));
        }
      }
      break;
    default:
      glyphname  = parse_c_ident(pp, endptr);
      if (!glyphname)
        _tt_abort("Invalid Unicode character specified.");

      skip_white(pp, endptr);
      if (*pp + 1 < endptr && **pp == '-') {
        value = pdf_new_array();

        if (agl_get_unicodes(glyphname, &ucv, 1) != 1)
          _tt_abort("Invalid Unicode char: %s", glyphname);
        pdf_add_array(value, pdf_new_number(ucv));
        free(glyphname);

        (*pp)++; skip_white(pp, endptr);
        glyphname = parse_c_ident(pp, endptr);
        if (!glyphname)
          _tt_abort("Invalid Unicode char: %s", glyphname);
        if (agl_get_unicodes(glyphname, &ucv, 1) != 1)
          _tt_abort("Invalid Unicode char: %s", glyphname);
        pdf_add_array(value, pdf_new_number(ucv));
        free(glyphname);

      } else {
        if (agl_get_unicodes(glyphname, &ucv, 1) != 1)
          _tt_abort("Invalid Unicode char: %s", glyphname);
        value = pdf_new_number(ucv);
        free(glyphname);
      }
      pdf_add_array(coverage, value);
      break;
    }
    skip_white(pp, endptr);
  }

  return coverage;
}

static pdf_obj *parse_block (pdf_obj *gclass, const char **pp, const char *endptr);

static void
add_rule (pdf_obj *rule, pdf_obj *gclass,
          char *first, char *second, char *suffix)
{
  pdf_obj *glyph1, *glyph2;
#define MAX_UNICODES 16
  int32_t  unicodes[MAX_UNICODES];
  int      i, n_unicodes;

  if (first[0] == '@') {
    glyph1 = pdf_lookup_dict(gclass, &first[1]);
    if (!glyph1) {
      dpx_warning("No glyph class \"%s\" found.", &first[1]);
      return;
    }
    pdf_link_obj(glyph1);

    if (verbose > VERBOSE_LEVEL_MIN) {
      dpx_message("otl_conf>> Output glyph sequence: %s\n", first);
    }

  } else {
    n_unicodes = agl_get_unicodes(first, unicodes, MAX_UNICODES);
    if (n_unicodes < 1) {
      dpx_warning("Failed to convert glyph \"%s\" to Unicode sequence.",
           first);
      return;
    }
    glyph1 = pdf_new_array();

    if (verbose > VERBOSE_LEVEL_MIN) {
      dpx_message("otl_conf>> Output glyph sequence: %s ->", first);
    }

    for (i = 0; i < n_unicodes; i++) {
      pdf_add_array(glyph1, pdf_new_number(unicodes[i]));

      if (verbose > VERBOSE_LEVEL_MIN) {
        if (unicodes[i] < 0x10000) {
          dpx_message(" U+%04X", unicodes[i]);
        } else {
          dpx_message(" U+%06X", unicodes[i]);
        }
      }
    }

    if (verbose > VERBOSE_LEVEL_MIN) {
      dpx_message("\n");
    }
  }

  if (second[0] == '@') {
    glyph2 = pdf_lookup_dict(gclass, &second[1]);
    if (!glyph2) {
      dpx_warning("No glyph class \"%s\" found.", &second[1]);
      return;
    }
    pdf_link_obj(glyph2);

    if (verbose > VERBOSE_LEVEL_MIN) {
      dpx_message("otl_conf>> Input glyph sequence: %s (%s)\n", second, suffix);
    }

  } else {
    n_unicodes = agl_get_unicodes(second, unicodes, 16);
    if (n_unicodes < 1) {
      dpx_warning("Failed to convert glyph \"%s\" to Unicode sequence.",
           second);
      return;
    }

    if (verbose > VERBOSE_LEVEL_MIN) {
      if (suffix)
        dpx_message("otl_conf>> Input glyph sequence: %s.%s ->", second, suffix);
      else
        dpx_message("otl_conf>> Input glyph sequence: %s ->", second);
    }

    glyph2 = pdf_new_array();
    for (i = 0; i < n_unicodes; i++) {
      pdf_add_array(glyph2, pdf_new_number(unicodes[i]));

      if (verbose > VERBOSE_LEVEL_MIN) {
        if (unicodes[i] < 0x10000) {
          dpx_message(" U+%04X", unicodes[i]);
        } else {
          dpx_message(" U+%06X", unicodes[i]);
        }
      }
    }
    if (verbose > VERBOSE_LEVEL_MIN) {
      dpx_message(" (%s)\n", suffix);
    }
  }

  /* OK */
  if (suffix) {
    pdf_add_array(rule, pdf_new_string(suffix, strlen(suffix)));
  } else {
    pdf_add_array(rule, pdf_new_null());
  }
  pdf_add_array(rule, glyph1);
  pdf_add_array(rule, glyph2);
}

static pdf_obj *
parse_substrule (pdf_obj *gclass, const char **pp, const char *endptr)
{
  pdf_obj *substrule;
  char    *token;

  skip_white(pp, endptr);
  if (*pp < endptr && **pp == '{')
    (*pp)++;

  skip_white(pp, endptr);
  if (*pp >= endptr)
    return NULL;

  substrule = pdf_new_array();
  while (*pp < endptr && **pp != '}') {
    skip_white(pp, endptr);
    if (*pp >= endptr)
      break;

    if (**pp == '#') {
      while (*pp < endptr) {
        if (**pp == '\r' || **pp == '\n') {
          (*pp)++;
          break;
        }
        (*pp)++;
      }
      continue;
    } else if (**pp == ';') {
      (*pp)++;
      continue;
    }

    skip_white(pp, endptr);
    token = parse_c_ident(pp, endptr);
    if (!token)
      break;

    if (streq_ptr(token, "assign") || streq_ptr(token, "substitute")) {
      char *tmp, *first, *second, *suffix;

      skip_white(pp, endptr);

      first = parse_c_ident(pp, endptr);
      if (!first)
        _tt_abort("Syntax error (1)");

      skip_white(pp, endptr);
      tmp = parse_c_ident(pp, endptr);
      if (strcmp(tmp, "by") && strcmp(tmp, "to"))
        _tt_abort("Syntax error (2): %s", *pp);

      skip_white(pp, endptr);
      second = parse_c_ident(pp, endptr); /* allows @ */
      if (!second)
        _tt_abort("Syntax error (3)");

      /* (assign|substitute) tag dst src */
      pdf_add_array(substrule, pdf_new_name(token));
      if (*pp + 1 < endptr && **pp == '.') {
        (*pp)++;
        suffix = parse_c_ident(pp, endptr);
      } else {
        suffix = NULL;
      }
      add_rule(substrule, gclass, first, second, suffix);

      free(first);
      free(tmp);
      free(second);
      free(suffix);
    } else {
      _tt_abort("Unkown command %s.", token);
    }
    free(token);
    skip_white(pp, endptr);
  }

  if (*pp < endptr && **pp == '}')
    (*pp)++;
  return substrule;
}

static pdf_obj *
parse_block (pdf_obj *gclass, const char **pp, const char *endptr)
{
  pdf_obj *rule;
  char    *token, *tmp;

  skip_white(pp, endptr);
  if (*pp < endptr && **pp == '{')
    (*pp)++;

  skip_white(pp, endptr);
  if (*pp >= endptr)
    return NULL;

  rule   = pdf_new_dict();
  while (*pp < endptr && **pp != '}') {
    skip_white(pp, endptr);
    if (*pp >= endptr)
      break;
    if (**pp == '#') {
      while (*pp < endptr) {
        if (**pp == '\r' || **pp == '\n') {
          (*pp)++;
          break;
        }
        (*pp)++;
      }
      continue;
    } else if (**pp == ';') {
      (*pp)++;
      continue;
    }

    skip_white(pp, endptr);
    token = parse_c_ident(pp, endptr);
    if (!token)
      break;

    if (streq_ptr(token, "script") ||
        streq_ptr(token, "language")) {
      int  i, len;

      skip_white(pp, endptr);
      len = 0;
      while (*pp + len < endptr && *(*pp + len) != ';') {
        len++;
      }
      if (len > 0) {
        tmp = NEW(len+1, char);
        memset(tmp, 0, len+1);
        for (i = 0; i < len; i++) {
          if (!isspace((unsigned char)**pp))
            tmp[i] = **pp;
          (*pp)++;
        }
        pdf_add_dict(rule,
                     pdf_new_name(token),
                     pdf_new_string(tmp, strlen(tmp)));

        if (verbose > VERBOSE_LEVEL_MIN) {
          dpx_message("otl_conf>> Current %s set to \"%s\"\n", token, tmp);
        }

        free(tmp);
      }
    } else if (streq_ptr(token, "option")) {
      pdf_obj *opt_dict, *opt_rule;

      opt_dict = pdf_lookup_dict(rule, "option");
      if (!opt_dict) {
        opt_dict = pdf_new_dict();
        pdf_add_dict(rule,
                     pdf_new_name("option"), opt_dict);
      }

      skip_white(pp, endptr);
      tmp = parse_c_ident(pp, endptr);

      if (verbose > VERBOSE_LEVEL_MIN) {
        dpx_message("otl_conf>> Reading option \"%s\"\n", tmp);
      }

      skip_white(pp, endptr);
      opt_rule = parse_block(gclass, pp, endptr);
      pdf_add_dict(opt_dict, pdf_new_name(tmp), opt_rule);

      free(tmp);
    } else if (streq_ptr(token, "prefered") ||
               streq_ptr(token, "required") ||
               streq_ptr(token, "optional")) {
      pdf_obj *subst, *rule_block;

      if (verbose > VERBOSE_LEVEL_MIN) {
        dpx_message("otl_conf>> Reading block (%s)\n", token);
      }

      skip_white(pp, endptr);
      if (*pp >= endptr || **pp != '{')
        _tt_abort("Syntax error (1)");

      rule_block = parse_substrule(gclass, pp, endptr);
      subst = pdf_lookup_dict(rule, "rule");
      if (!subst) {
        subst = pdf_new_array();
        pdf_add_dict(rule, pdf_new_name("rule"), subst);
      }
      pdf_add_array(subst, pdf_new_number(token[0]));
      pdf_add_array(subst, rule_block);
    } else if (token[0] == '@') {
      pdf_obj *coverage;

      skip_white(pp, endptr);
      (*pp)++; /* = */
      skip_white(pp, endptr);

      if (verbose > VERBOSE_LEVEL_MIN) {
        dpx_message("otl_conf>> Glyph class \"%s\"\n", token);
      }

      coverage = parse_uc_coverage(gclass, pp, endptr);
      if (!coverage)
        _tt_abort("No valid Unicode characters...");

      pdf_add_dict(gclass,
                   pdf_new_name(&token[1]), coverage);
    }
    free(token);
    skip_white(pp, endptr);
  }

  if (*pp < endptr && **pp == '}')
    (*pp)++;
  return rule;
}


static pdf_obj *
otl_read_conf (const char *conf_name)
{
  pdf_obj *rule;
  pdf_obj *gclass;
  rust_input_handle_t handle = NULL;
  char    *filename, *wbuf, *p, *endptr;
  const char *pp;
  int      size, len;

  filename = NEW(strlen(conf_name)+strlen(".otl")+1, char);
  strcpy(filename, conf_name);
  strcat(filename, ".otl");

  handle = ttstub_input_open(filename, TTBC_FILE_FORMAT_CNF, 0);
  if (handle == NULL) {
    free(filename);
    return NULL;
  }

  size = ttstub_input_get_size(handle);

  if (verbose > VERBOSE_LEVEL_MIN) {
    dpx_message("\n");
    dpx_message("otl_conf>> Layout config. \"%s\" found: file=\"%s\" (%d bytes)\n",
         conf_name, filename, size);
  }
  free(filename);
  if (size < 1)
    return NULL;

  wbuf = NEW(size, char);
  p = wbuf; endptr = p + size;

  while (size > 0 && p < endptr) {
    len = ttstub_input_read(handle, p, size);
    if (len < 0) {
        ttstub_input_close(handle);
        _tt_abort("error reading OTL configuration file \"%s\"", filename);
    }

    p    += len;
    size -= len;
  }

  ttstub_input_close(handle);
  pp     = wbuf;
  gclass = pdf_new_dict();
  rule   = parse_block(gclass, &pp, endptr);
  pdf_release_obj(gclass);

  free(wbuf);

  return rule;
}

static pdf_obj *otl_confs = NULL;

pdf_obj *
otl_find_conf (const char *conf_name)
{
  pdf_obj *rule;
  pdf_obj *script, *language;
  pdf_obj *options;

  return  NULL;

  if (otl_confs)
    rule = pdf_lookup_dict(otl_confs, conf_name);
  else {
    otl_confs = pdf_new_dict();
    rule = NULL;
  }

  if (!rule) {
    rule = otl_read_conf(conf_name);
    if (rule) {
      pdf_add_dict(otl_confs,
                   pdf_new_name(conf_name), rule);
      script   = pdf_lookup_dict(rule, "script");
      language = pdf_lookup_dict(rule, "language");
      options  = pdf_lookup_dict(rule, "option");
      if (!script) {
        script = pdf_new_string("*", 1);
        pdf_add_dict(rule,
                     pdf_new_name("script"),
                     script);
        dpx_warning("Script unspecified in \"%s\"...", conf_name);
      }
      if (!language) {
        language = pdf_new_string("dflt", 4);
        pdf_add_dict(rule,
                     pdf_new_name("language"),
                     language);
        dpx_warning("Language unspecified in \"%s\"...", conf_name);
      }

      if (options) {
        pdf_obj *optkeys, *opt, *key;
        int      i, num_opts;

        optkeys  = pdf_dict_keys(options);
        num_opts = pdf_array_length(optkeys);
        for (i = 0; i < num_opts; i++) {
          key = pdf_get_array(optkeys, i);
          opt = pdf_lookup_dict(options, pdf_name_value(key));
          if (!pdf_lookup_dict(opt, "script"))
            pdf_add_dict(opt,
                         pdf_new_name("script"),
                         pdf_link_obj(script));
          if (!pdf_lookup_dict(opt, "language"))
            pdf_add_dict(opt,
                         pdf_new_name("language"),
                         pdf_link_obj(language));
        }
        pdf_release_obj(optkeys);
      }

    }
  }

  return rule;
}


char *
otl_conf_get_script (pdf_obj *conf)
{
  pdf_obj *script;

  assert(conf);

  script = pdf_lookup_dict(conf, "script");

  return pdf_string_value(script);
}

char *
otl_conf_get_language (pdf_obj *conf)
{
  pdf_obj *language;

  assert(conf);

  language = pdf_lookup_dict(conf, "language");

  return pdf_string_value(language);
}

pdf_obj *
otl_conf_get_rule (pdf_obj *conf)
{
  assert(conf);
  return pdf_lookup_dict(conf, "rule");
}

pdf_obj *
otl_conf_find_opt (pdf_obj *conf, const char *opt_tag)
{
  pdf_obj *opt_conf = NULL;
  pdf_obj *options;

  assert(conf);

  options = pdf_lookup_dict(conf, "option");
  if (options && opt_tag)
    opt_conf = pdf_lookup_dict(options, opt_tag);
  else
    opt_conf = NULL;

  return opt_conf;
}

void
otl_init_conf (void)
{
  pdf_release_obj(otl_confs);
  otl_confs = pdf_new_dict();

  if (verbose > VERBOSE_LEVEL_MIN + 10) {
    pdf_release_obj(pdf_ref_obj(otl_confs));
  }
}

void
otl_close_conf (void)
{
  pdf_release_obj(otl_confs);
  otl_confs = NULL;
}
