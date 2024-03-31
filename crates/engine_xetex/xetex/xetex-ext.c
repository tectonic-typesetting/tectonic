/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009, 2011 by Jonathan Kew
 Copyright (c) 2012-2015 by Khaled Hosny
 Copyright (c) 2012, 2013 by Jiang Jiang

 SIL Author(s): Jonathan Kew

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the copyright holders
shall not be used in advertising or otherwise to promote the sale,
use or other dealings in this Software without prior written
authorization from the copyright holders.
\****************************************************************************/

/* XeTeX_ext.c
 * additional plain C extensions for XeTeX - mostly platform-neutral
 */

#include "xetex-core.h"
#include "xetex-ext.h"
#include "teckit-c-Engine.h"
#include "xetex-swap.h"

#include <assert.h>
#include <locale.h>
#include <math.h> /* for fabs() */
#include <signal.h>
#include <time.h>

#ifndef _MSC_VER
#include <sys/time.h>
#endif

#include <unicode/ubidi.h>
#include <unicode/ubrk.h>
#include <unicode/ucnv.h>

#include <graphite2/Font.h>

#include "xetex-xetexd.h"


/* OT-related constants we need */
#define kGSUB HB_TAG('G','S','U','B')
#define kGPOS HB_TAG('G','P','O','S')


static UBreakIterator* brkIter = NULL;
static int brkLocaleStrNum = 0;

void
linebreak_start(int f, int32_t localeStrNum, uint16_t* text, int32_t textLength)
{
    UErrorCode status = U_ZERO_ERROR;
    char* locale = (char*)gettexstring(localeStrNum);

    if (font_area[f] == OTGR_FONT_FLAG && streq_ptr(locale, "G")) {
        XeTeXLayoutEngine engine = (XeTeXLayoutEngine) font_layout_engine[f];
        if (initGraphiteBreaking(engine, text, textLength))
            /* user asked for Graphite line breaking and the font supports it */
            return;
    }

    if ((localeStrNum != brkLocaleStrNum) && (brkIter != NULL)) {
        ubrk_close(brkIter);
        brkIter = NULL;
    }

    if (brkIter == NULL) {
        brkIter = ubrk_open(UBRK_LINE, locale, NULL, 0, &status);
        if (U_FAILURE(status)) {
            begin_diagnostic();
            print_nl('E');
            print_c_string("rror ");
            print_int(status);
            print_c_string(" creating linebreak iterator for locale `");
            print_c_string(locale);
            print_c_string("'; trying default locale `en_us'.");
            end_diagnostic(1);
            if (brkIter != NULL)
                ubrk_close(brkIter);
            status = U_ZERO_ERROR;
            brkIter = ubrk_open(UBRK_LINE, "en_us", NULL, 0, &status);
        }
        free(locale);
        brkLocaleStrNum = localeStrNum;
    }

    if (brkIter == NULL)
        _tt_abort ("failed to create linebreak iterator, status=%d", (int) status);

    ubrk_setText(brkIter, (UChar*) text, textLength, &status);
}

int
linebreak_next(void)
{
    if (brkIter != NULL)
        return ubrk_next((UBreakIterator*)brkIter);
    else
        return findNextGraphiteBreak();
}

int
get_encoding_mode_and_info(int32_t* info)
{
    /* \XeTeXinputencoding "enc-name"
     *   -> name is packed in |nameoffile| as a C string, starting at [1]
     * Check if it's a built-in name; if not, try to open an ICU converter by that name
     */
    UErrorCode err = U_ZERO_ERROR;
    UConverter* cnv;
    *info = 0;
    if (strcasecmp(name_of_file, "auto") == 0) {
        return AUTO;
    }
    if (strcasecmp(name_of_file, "utf8") == 0) {
        return UTF8;
    }
    if (strcasecmp(name_of_file, "utf16") == 0) {   /* depends on host platform */
        return US_NATIVE_UTF16;
    }
    if (strcasecmp(name_of_file, "utf16be") == 0) {
        return UTF16BE;
    }
    if (strcasecmp(name_of_file, "utf16le") == 0) {
        return UTF16LE;
    }
    if (strcasecmp(name_of_file, "bytes") == 0) {
        return RAW;
    }

    /* try for an ICU converter */
    cnv = ucnv_open(name_of_file, &err);
    if (cnv == NULL) {
        begin_diagnostic();
        print_nl('U'); /* ensure message starts on a new line */
        print_c_string("nknown encoding `");
        print_c_string(name_of_file);
        print_c_string("'; reading as raw bytes");
        end_diagnostic(1);
        return RAW;
    } else {
        ucnv_close(cnv);
        *info = maketexstring(name_of_file);
        return ICUMAPPING;
    }
}

void
print_utf8_str(const unsigned char* str, int len)
{
    while (len-- > 0)
        print_raw_char(*(str++), true); /* bypass utf-8 encoding done in print_char() */
}

void
print_chars(const unsigned short* str, int len)
{
    while (len-- > 0)
        print_char(*(str++));
}

static void*
load_mapping_file(const char* s, const char* e, char byteMapping)
{
    TECkit_Converter cnv = 0;
    char* buffer = xmalloc(e - s + 5);
    rust_input_handle_t map;

    strncpy(buffer, s, e - s);
    buffer[e - s] = 0;
    strcat(buffer, ".tec");

    map = ttstub_input_open (buffer, TTBC_FILE_FORMAT_MISC_FONTS, 0);
    if (map) {
        size_t mappingSize = ttstub_input_get_size (map);
        Byte *mapping = xmalloc(mappingSize);
        ssize_t r = ttstub_input_read(map, (char *) mapping, mappingSize);

        if (r < 0 || (size_t) r != mappingSize)
            _tt_abort("could not read mapping file \"%s\"", buffer);

        ttstub_input_close(map);

        if (byteMapping != 0)
            TECkit_CreateConverter(mapping, mappingSize,
                                   false,
                                   UTF16_NATIVE, kForm_Bytes,
                                   &cnv);
        else
            TECkit_CreateConverter(mapping, mappingSize,
                                   true,
                                   UTF16_NATIVE, UTF16_NATIVE,
                                   &cnv);

        if (cnv == NULL)
            font_mapping_warning(buffer, strlen(buffer), 2); /* not loadable */
        else if (get_tracing_fonts_state() > 1)
            font_mapping_warning(buffer, strlen(buffer), 0); /* tracing */

        free(mapping);
    } else {
        font_mapping_warning(buffer, strlen(buffer), 1); /* not found */
    }

    free(buffer);

    return cnv;
}

static char *saved_mapping_name = NULL;
void
check_for_tfm_font_mapping(void)
{
    char* cp = strstr(name_of_file, ":mapping=");
    saved_mapping_name = mfree(saved_mapping_name);
    if (cp != NULL) {
        *cp = 0;
        cp += 9;
        while (*cp && *cp <= ' ')
            ++cp;
        if (*cp)
            saved_mapping_name = xstrdup(cp);
    }
}

void*
load_tfm_font_mapping(void)
{
    void* rval = NULL;
    if (saved_mapping_name != NULL) {
        rval = load_mapping_file(saved_mapping_name,
                saved_mapping_name + strlen(saved_mapping_name), 1);
        saved_mapping_name = mfree(saved_mapping_name);
    }
    return rval;
}

int
apply_tfm_font_mapping(void* cnv, int c)
{
    UniChar in = c;
    Byte out[2];
    UInt32 inUsed, outUsed;
    /* TECkit_Status status; */
    /* status = */ TECkit_ConvertBuffer((TECkit_Converter)cnv,
            (const Byte*)&in, sizeof(in), &inUsed, out, sizeof(out), &outUsed, 1);
    TECkit_ResetConverter((TECkit_Converter) cnv);
    if (outUsed < 1)
        return 0;
    else
        return out[0];
}

double
read_double(const char** s)
{
    int neg = 0;
    double val = 0.0;
    const char* cp = *s;

    while (*cp == ' '|| *cp == '\t')
        ++cp;
    if (*cp == '-') {
        neg = 1;
        ++cp;
    } else if (*cp == '+') {
        ++cp;
    }

    while (*cp >= '0' && *cp <= '9') {
        val = val * 10.0 + *cp - '0';
        ++cp;
    }
    if (*cp == '.') {
        double dec = 10.0;
        ++cp;
        while (*cp >= '0' && *cp <= '9') {
            val = val + (*cp - '0') / dec;
            ++cp;
            dec = dec * 10.0;
        }
    }
    *s = cp;

    return neg ? -val : val;
}

static hb_tag_t
read_tag_with_param(const char* cp, int* param)
{
    const char* cp2;
    hb_tag_t tag;

    cp2 = cp;
    while (*cp2 && (*cp2 != ':') && (*cp2 != ';') && (*cp2 != ',') && (*cp2 != '='))
        ++cp2;

    tag = hb_tag_from_string(cp, cp2 - cp);

    cp = cp2;
    if (*cp == '=') {
        int neg = 0;
        ++cp;
        if (*cp == '-') {
            ++neg;
            ++cp;
        }
        while (*cp >= '0' && *cp <= '9') {
            *param = *param * 10 + *cp - '0';
            ++cp;
        }
        if (neg)
            *param = -(*param);
    }

    return tag;
}

unsigned int
read_rgb_a(const char** cp)
{
    uint32_t rgbValue = 0;
    uint32_t alpha = 0;
    int i;
    for (i = 0; i < 6; ++i) {
        if ((**cp >= '0') && (**cp <= '9'))
            rgbValue = (rgbValue << 4) + **cp - '0';
        else if ((**cp >= 'A') && (**cp <= 'F'))
            rgbValue = (rgbValue << 4) + **cp - 'A' + 10;
        else if ((**cp >= 'a') && (**cp <= 'f'))
            rgbValue = (rgbValue << 4) + **cp - 'a' + 10;
        else
            return 0x000000FF;
        (*cp)++;
    }
    rgbValue <<= 8;
    for (i = 0; i < 2; ++i) {
        if ((**cp >= '0') && (**cp <= '9'))
            alpha = (alpha << 4) + **cp - '0';
        else if ((**cp >= 'A') && (**cp <= 'F'))
            alpha = (alpha << 4) + **cp - 'A' + 10;
        else if ((**cp >= 'a') && (**cp <= 'f'))
            alpha = (alpha << 4) + **cp - 'a' + 10;
        else
            break;
        (*cp)++;
    }
    if (i == 2)
        rgbValue += alpha;
    else
        rgbValue += 0xFF;
    return rgbValue;
}

int
readCommonFeatures(const char* feat, const char* end, float* extend, float* slant, float* embolden, float* letterspace, uint32_t* rgbValue)
    // returns 1 to go to next_option, -1 for bad_option, 0 to continue
{
    const char* sep;
    sep = strstartswith(feat, "mapping");
    if (sep) {
        if (*sep != '=')
            return -1;
        loaded_font_mapping = load_mapping_file(sep + 1, end, 0);
        return 1;
    }

    sep = strstartswith(feat, "extend");
    if (sep) {
        if (*sep != '=')
            return -1;
        ++sep;
        *extend = read_double(&sep);
        return 1;
    }

    sep = strstartswith(feat, "slant");
    if (sep) {
        if (*sep != '=')
            return -1;
        ++sep;
        *slant = read_double(&sep);
        return 1;
    }

    sep = strstartswith(feat, "embolden");
    if (sep) {
        if (*sep != '=')
            return -1;
        ++sep;
        *embolden = read_double(&sep);
        return 1;
    }

    sep = strstartswith(feat, "letterspace");
    if (sep) {
        if (*sep != '=')
            return -1;
        ++sep;
        *letterspace = read_double(&sep);
        return 1;
    }

    sep = strstartswith(feat, "color");
    if (sep) {
        const char* s;
        if (*sep != '=')
            return -1;
        ++sep;
        s = sep;
        *rgbValue = read_rgb_a(&sep);
        if ((sep == s+6) || (sep == s+8))
            loaded_font_flags |= FONT_FLAGS_COLORED;
        else
            return -1;
        return 1;
    }

    return 0;
}

static bool
readFeatureNumber(const char* s, const char* e, hb_tag_t* f, int* v)
    /* s...e is a "id=setting" string; */
{
    *f = 0;
    *v = 0;
    if (*s < '0' || *s > '9')
        return false;
    while (*s >= '0' && *s <= '9')
        *f = *f * 10 + *s++ - '0';
    while ((*s == ' ') || (*s == '\t'))
        ++s;
    if (*s++ != '=')
        /* no setting was specified */
        return false;

    if (*s < '0' || *s > '9')
        return false;
    while (*s >= '0' && *s <= '9')
        *v = *v * 10 + *s++ - '0';
    while ((*s == ' ') || (*s == '\t'))
        ++s;
    if (s != e)
        return false;
    return true;
}

static void*
loadOTfont(RawPlatformFontRef fontRef, XeTeXFont font, Fixed scaled_size, char* cp1)
{
    XeTeXLayoutEngine engine = NULL;
    hb_tag_t script = HB_TAG_NONE;
    char * language = NULL;
    hb_feature_t* features = NULL;
    char** shapers = NULL; /* NULL-terminated array */
    int nFeatures = 0;
    int nShapers = 0;

    char* cp2;
    const char* cp3;

    hb_tag_t tag;

    uint32_t rgbValue = 0x000000FF;

    float extend = 1.0;
    float slant = 0.0;
    float embolden = 0.0;
    float letterspace = 0.0;

    int i;

    char reqEngine = getReqEngine();

    if (reqEngine == 'O' || reqEngine == 'G') {
        shapers = (char**) xrealloc(shapers, (nShapers + 1) * sizeof(char *));
        if (reqEngine == 'O') {
            static char ot_const[] = "ot";
            shapers[nShapers] = ot_const;
        } else if (reqEngine == 'G') {
            static char graphite2_const[] = "graphite2";
            shapers[nShapers] = graphite2_const;
        }
        nShapers++;
    }

    if (reqEngine == 'G') {
        char* tmpShapers[] = {shapers[0]};
        /* create a default engine so we can query the font for Graphite features;
         * because of font caching, it's cheap to discard this and create the real one later */
        // TODO: this engine is never dropped, but doing so would invalidate `font`
        //        language is always NULL here
        engine = createLayoutEngine(font, script, language,
                features, nFeatures, tmpShapers, rgbValue, extend, slant, embolden);

        if (engine == NULL)
            return NULL;
    }

    /* scan the feature string (if any) */
    if (cp1 != NULL) {
        while (*cp1) {
            if ((*cp1 == ':') || (*cp1 == ';') || (*cp1 == ','))
                ++cp1;
            while ((*cp1 == ' ') || (*cp1 == '\t')) /* skip leading whitespace */
                ++cp1;
            if (*cp1 == 0) /* break if end of string */
                break;

            cp2 = cp1;
            while (*cp2 && (*cp2 != ':') && (*cp2 != ';') && (*cp2 != ','))
                ++cp2;

            cp3 = strstartswith(cp1, "script");
            if (cp3) {
                if (*cp3 != '=')
                    goto bad_option;
                ++cp3;
                script = hb_tag_from_string(cp3, cp2 - cp3);
                goto next_option;
            }

            cp3 = strstartswith(cp1, "language");
            if (cp3) {
                if (*cp3 != '=')
                    goto bad_option;
                ++cp3;
                language = xmalloc(cp2 - cp3 + 1);
                language[cp2 - cp3] = '\0';
                memcpy(language, cp3, cp2 - cp3);
                goto next_option;
            }

            cp3 = strstartswith(cp1, "shaper");
            if (cp3) {
                if (*cp3 != '=')
                    goto bad_option;
                ++cp3;
                shapers = (char**) xrealloc(shapers, (nShapers + 1) * sizeof(char *));
                /* some dumb systems have no strndup() */
                shapers[nShapers] = strdup(cp3);
                shapers[nShapers][cp2 - cp3] = '\0';
                nShapers++;
                goto next_option;
            }

            i = readCommonFeatures(cp1, cp2, &extend, &slant, &embolden, &letterspace, &rgbValue);
            if (i == 1)
                goto next_option;
            else if (i == -1)
                goto bad_option;

            if (reqEngine == 'G') {
                int value = 0;
                if (readFeatureNumber(cp1, cp2, &tag, &value)
                 || findGraphiteFeature(engine, cp1, cp2, &tag, &value)) {
                    features = (hb_feature_t*) xrealloc(features, (nFeatures + 1) * sizeof(hb_feature_t));
                    features[nFeatures].tag = tag;
                    features[nFeatures].value = value;
                    features[nFeatures].start = 0;
                    features[nFeatures].end = (unsigned int) -1;
                    nFeatures++;
                    goto next_option;
                }
            }

            if (*cp1 == '+') {
                int param = 0;
                tag = read_tag_with_param(cp1 + 1, &param);
                features = (hb_feature_t*) xrealloc(features, (nFeatures + 1) * sizeof(hb_feature_t));
                features[nFeatures].tag = tag;
                features[nFeatures].start = 0;
                features[nFeatures].end = (unsigned int) -1;
                // for backward compatibility with pre-0.9999 where feature
                // indices started from 0
                if (param >= 0)
                    param++;
                features[nFeatures].value = param;
                nFeatures++;
                goto next_option;
            }

            if (*cp1 == '-') {
                ++cp1;
                tag = hb_tag_from_string(cp1, cp2 - cp1);
                features = (hb_feature_t*) xrealloc(features, (nFeatures + 1) * sizeof(hb_feature_t));
                features[nFeatures].tag = tag;
                features[nFeatures].start = 0;
                features[nFeatures].end = (unsigned int) -1;
                features[nFeatures].value = 0;
                nFeatures++;
                goto next_option;
            }

            if (strstartswith(cp1, "vertical")) {
                cp3 = cp2;
                if (*cp3 == ';' || *cp3 == ':' || *cp3 == ',')
                    --cp3;
                while (*cp3 == '\0' || *cp3 == ' ' || *cp3 == '\t')
                    --cp3;
                if (*cp3)
                    ++cp3;
                if (cp3 == cp1 + 8) {
                    loaded_font_flags |= FONT_FLAGS_VERTICAL;
                    goto next_option;
                }
            }

        bad_option:
            font_feature_warning((void*) cp1, cp2 - cp1, 0, 0);

        next_option:
            cp1 = cp2;
        }
    }

    if (shapers != NULL) {
        shapers = (char**) xrealloc(shapers, (nShapers + 1) * sizeof(char *));
        shapers[nShapers] = NULL;
    }

    if (embolden != 0.0)
        embolden = embolden * Fix2D(scaled_size) / 100.0;

    if (letterspace != 0.0)
        loaded_font_letter_space = (letterspace / 100.0) * scaled_size;

    if ((loaded_font_flags & FONT_FLAGS_COLORED) == 0)
        rgbValue = 0x000000FF;

    if ((loaded_font_flags & FONT_FLAGS_VERTICAL) != 0)
        setFontLayoutDir(font, 1);

    engine = createLayoutEngine(font, script, language,
                    features, nFeatures, shapers, rgbValue, extend, slant, embolden);

    if (!engine) {
        // only free these if creation failed, otherwise the engine now owns them
        free(features);
        free(shapers);
    } else {
        native_font_type_flag = OTGR_FONT_FLAG;
    }

    return engine;
}

static void
splitFontName(char* name, char** var, char** feat, char** end, int* index)
{
    *var = NULL;
    *feat = NULL;
    *index = 0;
    if (*name == '[') {
        int withinFileName = 1;
        ++name;
        while (*name) {
            if (withinFileName && *name == ']') {
                withinFileName = 0;
                if (*var == NULL)
                    *var = name;
            } else if (*name == ':') {
                if (withinFileName && *var == NULL) {
                    *var = name;
                    ++name;
                    while (*name >= '0' && *name <= '9')
                        *index = *index * 10 + *name++ - '0';
                    --name;
                } else if (!withinFileName && *feat == NULL)
                    *feat = name;
            }
            ++name;
        }
        *end = name;
    } else {
        while (*name) {
            if (*name == '/' && *var == NULL && *feat == NULL)
                *var = name;
            else if (*name == ':' && *feat == NULL)
                *feat = name;
            ++name;
        }
        *end = name;
    }
    if (*feat == NULL)
        *feat = name;
    if (*var == NULL)
        *var = *feat;
}

void*
find_native_font(char* uname, int32_t scaled_size)
    /* scaled_size here is in TeX points, or is a negative integer for 'scaled_t' */
{
    void* rval = NULL;
    char* nameString;
    char* var;
    char* feat;
    char* end;
    char* name = (char*)uname;
    char* varString = NULL;
    char* featString = NULL;
    RawPlatformFontRef fontRef;
    XeTeXFont font = NULL;
    int index = 0;

    loaded_font_mapping = NULL;
    loaded_font_flags = 0;
    loaded_font_letter_space = 0;

    splitFontName(name, &var, &feat, &end, &index);
    nameString = xmalloc(var - name + 1);
    strncpy(nameString, name, var - name);
    nameString[var - name] = 0;

    if (feat > var) {
        varString = xmalloc(feat - var);
        strncpy(varString, var + 1, feat - var - 1);
        varString[feat - var - 1] = 0;
    }

    if (end > feat) {
        featString = xmalloc(end - feat);
        strncpy(featString, feat + 1, end - feat - 1);
        featString[end - feat - 1] = 0;
    }

    // check for "[filename]" form, don't search maps in this case
    if (nameString[0] == '[') {
        if (scaled_size < 0) {
            font = createFontFromFile(nameString + 1, index, 655360L);
            if (font != NULL) {
                Fixed dsize = D2Fix(getDesignSize(font));
                if (scaled_size == -1000)
                    scaled_size = dsize;
                else
                    scaled_size = xn_over_d(dsize, -scaled_size, 1000);
                deleteFont(font);
            }
        }
        font = createFontFromFile(nameString + 1, index, scaled_size);
        if (font != NULL) {
            set_loaded_font_design_size(D2Fix(getDesignSize(font)));

            /* This is duplicated in XeTeXFontMgr::findFont! */
            setReqEngine(0);
            if (varString) {
                if (strstartswith(varString, "/AAT"))
                    setReqEngine('A');
                else if ((strstartswith(varString, "/OT")) || (strstartswith(varString, "/ICU")))
                    setReqEngine('O');
                else if (strstartswith(varString, "/GR"))
                    setReqEngine('G');
            }

            rval = loadOTfont(0, font, scaled_size, featString);
            if (rval == NULL)
                deleteFont(font);
            if (rval != NULL && get_tracing_fonts_state() > 0) {
                begin_diagnostic();
                print_nl(' ');
                print_c_string("-> ");
                print_c_string(nameString + 1);
                end_diagnostic(0);
            }
        }
    } else {
        fontRef = findFontByName(nameString, varString, Fix2D(scaled_size));

        /* Tectonic: this used to live in XeTeXFontMgr::findFont(), but we needed to
         * move it here to preserve encapsulation.
         */

        if (get_tracing_fonts_state() > 0) {
            begin_diagnostic();
            print_nl(' ');
            print_c_string("-> ");
            print_c_string(ttxl_platfont_get_desc(fontRef));
            end_diagnostic(0);
        }

        if (fontRef) {
            /* update name_of_file to the full name of the font, for error messages during font loading */
            const char* fullName = getFullName(fontRef);
            name_length = strlen(fullName);
            if (featString != NULL)
                name_length += strlen(featString) + 1;
            if (varString != NULL)
                name_length += strlen(varString) + 1;
            free(name_of_file);
            name_of_file = xmalloc(name_length + 1);
            strcpy(name_of_file, fullName);

            if (scaled_size < 0) {
                font = createFont(fontRef, scaled_size);
                if (font != NULL) {
                    Fixed dsize = D2Fix(getDesignSize(font));
                    if (scaled_size == -1000)
                        scaled_size = dsize;
                    else
                        scaled_size = xn_over_d(dsize, -scaled_size, 1000);
                    deleteFont(font);
                }
            }

            font = createFont(fontRef, scaled_size);
            if (font != NULL) {
#ifdef XETEX_MAC
                /* decide whether to use AAT or OpenType rendering with this font */
                if (getReqEngine() == 'A') {
                    rval = loadAATfont(fontRef, scaled_size, featString);
                    if (rval == NULL)
                        deleteFont(font);
                } else {
                    if (getReqEngine() == 'O' || getReqEngine() == 'G' ||
                            hasFontTable(font, kGSUB) || hasFontTable(font, kGPOS))
                        rval = loadOTfont(fontRef, font, scaled_size, featString);

                    /* loadOTfont failed or the above check was false */
                    if (rval == NULL)
                        rval = loadAATfont(fontRef, scaled_size, featString);

                    if (rval == NULL)
                        deleteFont(font);
                }
#else
                rval = loadOTfont(fontRef, font, scaled_size, featString);
                if (rval == NULL)
                    deleteFont(font);
#endif
            }

            /* append the style and feature strings, so that \show\fontID will give a full result */
            if (varString != NULL && *varString != 0) {
                strcat(name_of_file, "/");
                strcat(name_of_file, varString);
            }
            if (featString != NULL && *featString != 0) {
                strcat(name_of_file, ":");
                strcat(name_of_file, featString);
            }
            name_length = strlen(name_of_file);
        }
    }

    free(varString);

    free(featString);

    free(nameString);

    return rval;
}

void
release_font_engine(void* engine, int type_flag)
{
#ifdef XETEX_MAC
    if (type_flag == AAT_FONT_FLAG) {
        CFRelease((CFDictionaryRef)engine);
    } else
#endif
    if (type_flag == OTGR_FONT_FLAG) {
        deleteLayoutEngine((XeTeXLayoutEngine)engine);
    }
}

void
ot_get_font_metrics(void* pEngine, scaled_t* ascent, scaled_t* descent, scaled_t* xheight, scaled_t* capheight, scaled_t* slant)
{
    XeTeXLayoutEngine engine = (XeTeXLayoutEngine)pEngine;
    float a, d;

    getAscentAndDescent(engine, &a, &d);
    *ascent = D2Fix(a);
    *descent = D2Fix(d);

    *slant = D2Fix(Fix2D(getSlant(getFont(engine))) * getExtendFactor(engine)
                    + getSlantFactor(engine));

    /* get cap and x height from OS/2 table */
    getCapAndXHeight(engine, &a, &d);
    *capheight = D2Fix(a);
    *xheight = D2Fix(d);

    /* fallback in case the font does not have OS/2 table */
    if (*xheight == 0) {
        int glyphID = mapCharToGlyph(engine, 'x');
        if (glyphID != 0) {
            getGlyphHeightDepth(engine, glyphID, &a, &d);
            *xheight = D2Fix(a);
        } else {
            *xheight = *ascent / 2; /* arbitrary figure if there's no 'x' in the font */
        }
    }

    if (*capheight == 0) {
        int glyphID = mapCharToGlyph(engine, 'X');
        if (glyphID != 0) {
            getGlyphHeightDepth(engine, glyphID, &a, &d);
            *capheight = D2Fix(a);
        } else {
            *capheight = *ascent; /* arbitrary figure if there's no 'X' in the font */
        }
    }
}

int32_t
ot_font_get(int32_t what, void* pEngine)
{
    XeTeXLayoutEngine engine = (XeTeXLayoutEngine)pEngine;
    XeTeXFont fontInst = getFont(engine);
    switch (what) {
        case XeTeX_count_glyphs:
            return countGlyphs(fontInst);
            break;

        case XeTeX_count_features: /* ie Graphite features */
            return countGraphiteFeatures(engine);
            break;

        case XeTeX_OT_count_scripts:
            return countScripts(fontInst);
            break;
    }
    return 0;
}


int32_t
ot_font_get_1(int32_t what, void* pEngine, int32_t param)
{
    XeTeXLayoutEngine engine = (XeTeXLayoutEngine)pEngine;
    XeTeXFont fontInst = getFont(engine);
    switch (what) {
        case XeTeX_OT_count_languages:
            return countLanguages(fontInst, param);
            break;

        case XeTeX_OT_script_code:
            return getIndScript(fontInst, param);
            break;

        /* for graphite fonts...*/
        case XeTeX_feature_code:
            return getGraphiteFeatureCode(engine, param);
            break;
        case XeTeX_is_exclusive_feature:
            return 1;
            break;
        case XeTeX_count_selectors:
            return countGraphiteFeatureSettings(engine, param);
            break;
    }
    return 0;
}


int32_t
ot_font_get_2(int32_t what, void* pEngine, int32_t param1, int32_t param2)
{
    XeTeXLayoutEngine engine = (XeTeXLayoutEngine)pEngine;
    XeTeXFont fontInst = getFont(engine);
    switch (what) {
        case XeTeX_OT_language_code:
            return getIndLanguage(fontInst, param1, param2);
            break;

        case XeTeX_OT_count_features:
            return countFeatures(fontInst, param1, param2);
            break;

        /* for graphite fonts */
        case XeTeX_selector_code:
            return getGraphiteFeatureSettingCode(engine, param1, param2);
            break;
        case XeTeX_is_default_selector:
            return getGraphiteFeatureDefaultSetting(engine, param1) == param2;
            break;
    }

    return 0;
}


int32_t
ot_font_get_3(int32_t what, void* pEngine, int32_t param1, int32_t param2, int32_t param3)
{
    XeTeXLayoutEngine engine = (XeTeXLayoutEngine)pEngine;
    XeTeXFont fontInst = getFont(engine);
    switch (what) {
        case XeTeX_OT_feature_code:
            return getIndFeature(fontInst, param1, param2, param3);
            break;
    }

    return 0;
}

void
gr_print_font_name(int32_t what, void* pEngine, int32_t param1, int32_t param2)
{
    char* name = NULL;
    XeTeXLayoutEngine engine = (XeTeXLayoutEngine)pEngine;
    switch (what) {
        case XETEX_FEATURE_NAME_CODE:
            name = getGraphiteFeatureLabel(engine, param1);
            break;
        case XETEX_SELECTOR_NAME_CODE:
            name = getGraphiteFeatureSettingLabel(engine, param1, param2);
            break;
    }

    if (name != NULL) {
        print_c_string(name);
        gr_label_destroy(name);
    }
}

int32_t
gr_font_get_named(int32_t what, void* pEngine)
{
    long rval = -1;
    XeTeXLayoutEngine engine = (XeTeXLayoutEngine)pEngine;
    switch (what) {
        case XeTeX_find_feature_by_name:
            rval = findGraphiteFeatureNamed(engine, name_of_file, name_length);
            break;
    }
    return rval;
}

int32_t
gr_font_get_named_1(int32_t what, void* pEngine, int32_t param)
{
    long rval = -1;
    XeTeXLayoutEngine engine = (XeTeXLayoutEngine)pEngine;
    switch (what) {
        case XeTeX_find_selector_by_name:
            rval = findGraphiteFeatureSettingNamed(engine, param, name_of_file, name_length);
            break;
    }
    return rval;
}

#define XDV_FLAG_VERTICAL       0x0100
#define XDV_FLAG_COLORED        0x0200
#define XDV_FLAG_EXTEND         0x1000
#define XDV_FLAG_SLANT          0x2000
#define XDV_FLAG_EMBOLDEN       0x4000

#ifdef XETEX_MAC
static UInt32
cgColorToRGBA32(CGColorRef color)
{
    const CGFloat *components = CGColorGetComponents(color);

    UInt32 rval = (UInt8)(components[0] * 255.0 + 0.5);
    rval <<= 8;
    rval += (UInt8)(components[1] * 255.0 + 0.5);
    rval <<= 8;
    rval += (UInt8)(components[2] * 255.0 + 0.5);
    rval <<= 8;
    rval += (UInt8)(components[3] * 255.0 + 0.5);
    return rval;
}
#endif

static int xdvBufSize = 0;

int
makeXDVGlyphArrayData(void* pNode)
{
    unsigned char* cp;
    uint16_t* glyphIDs;
    memory_word* p = (memory_word*) pNode;
    void* glyph_info;
    FixedPoint* locations;
    Fixed width;
    uint16_t glyphCount = native_glyph_count(p);

    int i = glyphCount * native_glyph_info_size + 8; /* to guarantee enough space in the buffer */
    if (i > xdvBufSize) {
        free(xdv_buffer);
        xdvBufSize = ((i / 1024) + 1) * 1024;
        xdv_buffer = xmalloc(xdvBufSize);
    }

    glyph_info = native_glyph_info_ptr(p);
    locations = (FixedPoint*)glyph_info;
    glyphIDs = (uint16_t*)(locations + glyphCount);

    cp = (unsigned char*)xdv_buffer;

    width = node_width(p);
    *cp++ = (width >> 24) & 0xff;
    *cp++ = (width >> 16) & 0xff;
    *cp++ = (width >> 8) & 0xff;
    *cp++ = width & 0xff;

    *cp++ = (glyphCount >> 8) & 0xff;
    *cp++ = glyphCount & 0xff;

    for (i = 0; i < glyphCount; ++i) {
        Fixed x = locations[i].x;
        Fixed y = locations[i].y;
        *cp++ = (x >> 24) & 0xff;
        *cp++ = (x >> 16) & 0xff;
        *cp++ = (x >> 8) & 0xff;
        *cp++ = x & 0xff;
        *cp++ = (y >> 24) & 0xff;
        *cp++ = (y >> 16) & 0xff;
        *cp++ = (y >> 8) & 0xff;
        *cp++ = y & 0xff;
    }

    for (i = 0; i < glyphCount; ++i) {
        uint16_t g = glyphIDs[i];
        *cp++ = (g >> 8) & 0xff;
        *cp++ = g & 0xff;
    }

    return ((char*)cp - xdv_buffer);
}

int
make_font_def(int32_t f)
{
    uint16_t flags = 0;
    uint32_t rgba;
    Fixed size;
    char* filename;
    uint32_t index;
    uint8_t filenameLen;
    int fontDefLength;
    char* cp;
    /* RawPlatformFontRef fontRef = 0; */
    float extend = 1.0;
    float slant = 0.0;
    float embolden = 0.0;

#ifdef XETEX_MAC
    CFDictionaryRef attributes = NULL;

    if (font_area[f] == AAT_FONT_FLAG) {
        CTFontRef font;
        CGColorRef color;
        CGAffineTransform t;
        CFNumberRef emboldenNumber;
        CGFloat fSize;

        attributes = (CFDictionaryRef) font_layout_engine[f];
        font = CFDictionaryGetValue(attributes, kCTFontAttributeName);

        filename = getFileNameFromCTFont(font, &index);
        assert(filename);

        if (CFDictionaryGetValue(attributes, kCTVerticalFormsAttributeName))
            flags |= XDV_FLAG_VERTICAL;

        color = (CGColorRef) CFDictionaryGetValue(attributes, kCTForegroundColorAttributeName);
        if (color)
            rgba = cgColorToRGBA32(color);

        t = CTFontGetMatrix(font);
        extend = t.a;
        slant = t.c;

        emboldenNumber = CFDictionaryGetValue(attributes, kXeTeXEmboldenAttributeName);
        if (emboldenNumber)
            CFNumberGetValue(emboldenNumber, kCFNumberFloatType, &embolden);

        fSize = CTFontGetSize(font);
        size = D2Fix(fSize);
    } else
#endif
    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXLayoutEngine engine;

        engine = (XeTeXLayoutEngine)font_layout_engine[f];
        /* fontRef = getFontRef(engine); */
        filename = getFontFilename(engine, &index);
        assert(filename);

        rgba = getRgbValue(engine);
        if ((font_flags[f] & FONT_FLAGS_VERTICAL) != 0)
            flags |= XDV_FLAG_VERTICAL;

        extend = getExtendFactor(engine);
        slant = getSlantFactor(engine);
        embolden = getEmboldenFactor(engine);

        size = D2Fix(getPointSize(engine));
    } else {
        _tt_abort("bad native font flag in `make_font_def`");
    }

    filenameLen = strlen(filename);

    /* parameters after internal font ID:
    //  size[4]
    //  flags[2]
    //  l[1] n[l]
    //  if flags & COLORED:
    //      c[4]
    */

    fontDefLength
        = 4 /* size */
        + 2 /* flags */
        + 1 /* name length */
        + filenameLen
        + 4 /* face index */;

    if ((font_flags[f] & FONT_FLAGS_COLORED) != 0) {
        fontDefLength += 4; /* 32-bit RGBA value */
        flags |= XDV_FLAG_COLORED;
    }

    if (extend != 1.0) {
        fontDefLength += 4;
        flags |= XDV_FLAG_EXTEND;
    }
    if (slant != 0.0) {
        fontDefLength += 4;
        flags |= XDV_FLAG_SLANT;
    }
    if (embolden != 0.0) {
        fontDefLength += 4;
        flags |= XDV_FLAG_EMBOLDEN;
    }

    if (fontDefLength > xdvBufSize) {
        free(xdv_buffer);
        xdvBufSize = ((fontDefLength / 1024) + 1) * 1024;
        xdv_buffer = xmalloc(xdvBufSize);
    }
    cp = xdv_buffer;

    *(Fixed*)cp = SWAP32(size);
    cp += 4;

    *(uint16_t*)cp = SWAP16(flags);
    cp += 2;

    *(uint8_t*)cp = filenameLen;
    cp += 1;
    memcpy(cp, filename, filenameLen);
    cp += filenameLen;

    *(uint32_t*)cp = SWAP32(index);
    cp += 4;

    if ((font_flags[f] & FONT_FLAGS_COLORED) != 0) {
        *(uint32_t*)cp = SWAP32(rgba);
        cp += 4;
    }

    if (flags & XDV_FLAG_EXTEND) {
        Fixed f = D2Fix(extend);
        *(uint32_t*)(cp) = SWAP32(f);
        cp += 4;
    }
    if (flags & XDV_FLAG_SLANT) {
        Fixed f = D2Fix(slant);
        *(uint32_t*)(cp) = SWAP32(f);
        cp += 4;
    }
    if (flags & XDV_FLAG_EMBOLDEN) {
        Fixed f = D2Fix(embolden);
        *(uint32_t*)(cp) = SWAP32(f);
        cp += 4;
    }

    freeFontFilename((char*) filename);

    return fontDefLength;
}

int
apply_mapping(void* pCnv, uint16_t* txtPtr, int txtLen)
{
    TECkit_Converter cnv = (TECkit_Converter)pCnv;
    UInt32 inUsed, outUsed;
    TECkit_Status status;
    static UInt32 outLength = 0;

    /* allocate outBuffer if not big enough */
    if (outLength < txtLen * sizeof(UniChar) + 32) {
        free(mapped_text);
        outLength = txtLen * sizeof(UniChar) + 32;
        mapped_text = xmalloc(outLength);
    }

    /* try the mapping */
retry:
    status = TECkit_ConvertBuffer(cnv,
            (Byte*)txtPtr, txtLen * sizeof(UniChar), &inUsed,
            (Byte*)mapped_text, outLength, &outUsed, true);
    TECkit_ResetConverter(cnv);

    switch (status) {
        case kStatus_NoError:
            txtPtr = (UniChar*)mapped_text;
            return outUsed / sizeof(UniChar);

        case kStatus_OutputBufferFull:
            outLength += (txtLen * sizeof(UniChar)) + 32;
            free(mapped_text);
            mapped_text = xmalloc(outLength);
            goto retry;

        default:
            return 0;
    }
}

static void
snap_zone(scaled_t* value, scaled_t snap_value, scaled_t fuzz)
{
    scaled_t difference = *value - snap_value;
    if (difference <= fuzz && difference >= -fuzz)
        *value = snap_value;
}

void
get_native_char_height_depth(int32_t font, int32_t ch, scaled_t* height, scaled_t* depth)
{
#define QUAD(f)         font_info[6+param_base[f]].b32.s1
#define X_HEIGHT(f)     font_info[5+param_base[f]].b32.s1
#define CAP_HEIGHT(f)   font_info[8+param_base[f]].b32.s1

    float ht = 0.0;
    float dp = 0.0;
    Fixed fuzz;

#ifdef XETEX_MAC
    if (font_area[font] == AAT_FONT_FLAG) {
        CFDictionaryRef attributes = (CFDictionaryRef)(font_layout_engine[font]);
        int gid = MapCharToGlyph_AAT(attributes, ch);
        GetGlyphHeightDepth_AAT(attributes, gid, &ht, &dp);
    } else
#endif
    if (font_area[font] == OTGR_FONT_FLAG) {
        XeTeXLayoutEngine engine = (XeTeXLayoutEngine)font_layout_engine[font];
        int gid = mapCharToGlyph(engine, ch);
        getGlyphHeightDepth(engine, gid, &ht, &dp);
    } else {
        _tt_abort("bad native font flag in `get_native_char_height_depth`");
    }

    *height = D2Fix(ht);
    *depth = D2Fix(dp);

    /* snap to "known" zones for baseline, x-height, cap-height if within 4% of em-size */
    fuzz = QUAD(font) / 25;
    snap_zone(depth, 0, fuzz);
    snap_zone(height, 0, fuzz);
    snap_zone(height, X_HEIGHT(font), fuzz);
    snap_zone(height, CAP_HEIGHT(font), fuzz);
}

scaled_t
getnativecharht(int32_t f, int32_t c)
{
    scaled_t h, d;
    get_native_char_height_depth(f, c, &h, &d);
    return h;
}

scaled_t
getnativechardp(int32_t f, int32_t c)
{
    scaled_t h, d;
    get_native_char_height_depth(f, c, &h, &d);
    return d;
}

void
get_native_char_sidebearings(int32_t font, int32_t ch, scaled_t* lsb, scaled_t* rsb)
{
    float l, r;

#ifdef XETEX_MAC
    if (font_area[font] == AAT_FONT_FLAG) {
        CFDictionaryRef attributes = (CFDictionaryRef)(font_layout_engine[font]);
        int gid = MapCharToGlyph_AAT(attributes, ch);
        GetGlyphSidebearings_AAT(attributes, gid, &l, &r);
    } else
#endif
    if (font_area[font] == OTGR_FONT_FLAG) {
        XeTeXLayoutEngine engine = (XeTeXLayoutEngine)font_layout_engine[font];
        int gid = mapCharToGlyph(engine, ch);
        getGlyphSidebearings(engine, gid, &l, &r);
    } else {
        _tt_abort("bad native font flag in `get_native_char_side_bearings`");
    }

    *lsb = D2Fix(l);
    *rsb = D2Fix(r);
}

scaled_t
get_glyph_bounds(int32_t font, int32_t edge, int32_t gid)
{
/* edge codes 1,2,3,4 => L T R B */
    float a, b;

#ifdef XETEX_MAC
    if (font_area[font] == AAT_FONT_FLAG) {
        CFDictionaryRef attributes = (CFDictionaryRef)(font_layout_engine[font]);
        if (edge & 1)
            GetGlyphSidebearings_AAT(attributes, gid, &a, &b);
        else
            GetGlyphHeightDepth_AAT(attributes, gid, &a, &b);
    } else
#endif
    if (font_area[font] == OTGR_FONT_FLAG) {
        XeTeXLayoutEngine engine = (XeTeXLayoutEngine)font_layout_engine[font];
        if (edge & 1)
            getGlyphSidebearings(engine, gid, &a, &b);
        else
            getGlyphHeightDepth(engine, gid, &a, &b);
    } else {
        _tt_abort("bad native font flag in `get_glyph_bounds`");
    }
    return D2Fix((edge <= 2) ? a : b);
}

scaled_t
getnativecharic(int32_t f, int32_t c)
{
    scaled_t lsb, rsb;
    get_native_char_sidebearings(f, c, &lsb, &rsb);
    if (rsb < 0)
        return font_letter_space[f] - rsb;
    else
        return font_letter_space[f];
}

scaled_t
getnativecharwd(int32_t f, int32_t c)
{
    scaled_t wd = 0;
#ifdef XETEX_MAC
    if (font_area[f] == AAT_FONT_FLAG) {
        CFDictionaryRef attributes = (CFDictionaryRef)(font_layout_engine[f]);
        int gid = MapCharToGlyph_AAT(attributes, c);
        wd = D2Fix(GetGlyphWidth_AAT(attributes, gid));
    } else
#endif
    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXLayoutEngine engine = (XeTeXLayoutEngine)font_layout_engine[f];
        int gid = mapCharToGlyph(engine, c);
        wd = D2Fix(getGlyphWidthFromEngine(engine, gid));
    } else {
        _tt_abort("bad native font flag in `get_native_char_wd`");
    }
    return wd;
}

uint16_t
real_get_native_glyph(void* pNode, unsigned int index)
{
    memory_word* node = (memory_word*)pNode;
    FixedPoint* locations = (FixedPoint*)native_glyph_info_ptr(node);
    uint16_t* glyphIDs = (uint16_t*)(locations + native_glyph_count(node));
    if (index >= native_glyph_count(node))
        return 0;
    else
        return glyphIDs[index];
}

void
store_justified_native_glyphs(void* pNode)
{
    memory_word* node = (memory_word*)pNode;
    unsigned int f = native_font(node);

#ifdef XETEX_MAC /* separate Mac-only codepath for AAT fonts */
    if (font_area[f] == AAT_FONT_FLAG) {
        (void)DoAATLayout(node, 1);
        return;
    }
#endif

    /* save desired width */
    int savedWidth = node_width(node);

    measure_native_node(node, 0);

    if (node_width(node) != savedWidth) {
        /* see how much adjustment is needed overall */
        double justAmount = Fix2D(savedWidth - node_width(node));

        /* apply justification to spaces (or if there are none, distribute it to all glyphs as a last resort) */
        FixedPoint* locations = (FixedPoint*)native_glyph_info_ptr(node);
        uint16_t* glyphIDs = (uint16_t*)(locations + native_glyph_count(node));
        int glyphCount = native_glyph_count(node);
        int spaceCount = 0, i;

        int spaceGlyph = map_char_to_glyph(f, ' ');
        for (i = 0; i < glyphCount; ++i)
            if (glyphIDs[i] == spaceGlyph)
                spaceCount++;

        if (spaceCount > 0) {
            double adjustment = 0;
            int spaceIndex = 0;
            for (i = 0; i < glyphCount; ++i) {
                locations[i].x = D2Fix(Fix2D(locations[i].x) + adjustment);
                if (glyphIDs[i] == spaceGlyph) {
                    spaceIndex++;
                    adjustment = justAmount * spaceIndex / spaceCount;
                }
            }
        } else {
            for (i = 1; i < glyphCount; ++i)
                locations[i].x = D2Fix(Fix2D(locations[i].x) + justAmount * i / (glyphCount - 1));
        }

        node_width(node) = savedWidth;
    }
}

void
measure_native_node(void* pNode, int use_glyph_metrics)
{
    memory_word* node = (memory_word*)pNode;
    int txtLen = native_length(node);
    uint16_t* txtPtr = (uint16_t*)(node + NATIVE_NODE_SIZE);

    unsigned int f = native_font(node);

#ifdef XETEX_MAC
    if (font_area[f] == AAT_FONT_FLAG) {
        /* we're using this font in AAT mode, so font_layout_engine[f] is actually a CFDictionaryRef */
        DoAATLayout(node, 0);
    } else
#endif
    if (font_area[f] == OTGR_FONT_FLAG) {
        /* using this font in OT Layout mode, so font_layout_engine[f] is actually a XeTeXLayoutEngine */

        XeTeXLayoutEngine engine = (XeTeXLayoutEngine)(font_layout_engine[f]);

        FixedPoint* locations = NULL;
        uint16_t* glyphIDs;
        Fixed* glyphAdvances = NULL;
        int totalGlyphCount = 0;

        /* need to find direction runs within the text, and call layoutChars separately for each */

        UBiDiDirection dir;
        void* glyph_info = 0;
        static FloatPoint* positions = 0;
        static float* advances = 0;
        static uint32_t* glyphs = 0;

        UBiDi* pBiDi = ubidi_open();

        UErrorCode errorCode = U_ZERO_ERROR;
        ubidi_setPara(pBiDi, (const UChar*) txtPtr, txtLen, getDefaultDirection(engine), NULL, &errorCode);

        dir = ubidi_getDirection(pBiDi);
        if (dir == UBIDI_MIXED) {
            /* we actually do the layout twice here, once to count glyphs and then again to get them;
               which is inefficient, but i figure that MIXED is a relatively rare occurrence, so i can't be
               bothered to deal with the memory reallocation headache of doing it differently
            */
            int nRuns = ubidi_countRuns(pBiDi, &errorCode);
            double width = 0;
            int i, runIndex;
            int32_t logicalStart, length;
            for (runIndex = 0; runIndex < nRuns; ++runIndex) {
                dir = ubidi_getVisualRun(pBiDi, runIndex, &logicalStart, &length);
                totalGlyphCount += layoutChars(engine, txtPtr, logicalStart, length, txtLen, (dir == UBIDI_RTL));
            }

            if (totalGlyphCount > 0) {
                double x, y;
                glyph_info = xcalloc(totalGlyphCount, native_glyph_info_size);
                locations = (FixedPoint*)glyph_info;
                glyphIDs = (uint16_t*)(locations + totalGlyphCount);
                glyphAdvances = xcalloc(totalGlyphCount, sizeof(Fixed));
                totalGlyphCount = 0;

                x = y = 0.0;
                for (runIndex = 0; runIndex < nRuns; ++runIndex) {
                    int nGlyphs;
                    dir = ubidi_getVisualRun(pBiDi, runIndex, &logicalStart, &length);
                    nGlyphs = layoutChars(engine, txtPtr, logicalStart, length, txtLen,
                                            (dir == UBIDI_RTL));

                    glyphs = xcalloc(nGlyphs, sizeof(uint32_t));
                    positions = xcalloc(nGlyphs + 1, sizeof(FloatPoint));
                    advances = xcalloc(nGlyphs, sizeof(float));

                    getGlyphs(engine, glyphs);
                    getGlyphAdvances(engine, advances);
                    getGlyphPositions(engine, positions);

                    for (i = 0; i < nGlyphs; ++i) {
                        glyphIDs[totalGlyphCount] = glyphs[i];
                        locations[totalGlyphCount].x = D2Fix(positions[i].x + x);
                        locations[totalGlyphCount].y = D2Fix(positions[i].y + y);
                        glyphAdvances[totalGlyphCount] = D2Fix(advances[i]);
                        ++totalGlyphCount;
                    }
                    x += positions[nGlyphs].x;
                    y += positions[nGlyphs].y;

                    free(glyphs);
                    free(positions);
                    free(advances);
                }
                width = x;
            }

            node_width(node) = D2Fix(width);
            native_glyph_count(node) = totalGlyphCount;
            native_glyph_info_ptr(node) = glyph_info;
        } else {
            double width = 0;
            totalGlyphCount = layoutChars(engine, txtPtr, 0, txtLen, txtLen, (dir == UBIDI_RTL));

            glyphs = xcalloc(totalGlyphCount, sizeof(uint32_t));
            positions = xcalloc(totalGlyphCount + 1, sizeof(FloatPoint));
            advances = xcalloc(totalGlyphCount, sizeof(float));

            getGlyphs(engine, glyphs);
            getGlyphAdvances(engine, advances);
            getGlyphPositions(engine, positions);

            if (totalGlyphCount > 0) {
                int i;
                glyph_info = xcalloc(totalGlyphCount, native_glyph_info_size);
                locations = (FixedPoint*)glyph_info;
                glyphIDs = (uint16_t*)(locations + totalGlyphCount);
                glyphAdvances = xcalloc(totalGlyphCount, sizeof(Fixed));
                for (i = 0; i < totalGlyphCount; ++i) {
                    glyphIDs[i] = glyphs[i];
                    glyphAdvances[i] = D2Fix(advances[i]);
                    locations[i].x = D2Fix(positions[i].x);
                    locations[i].y = D2Fix(positions[i].y);
                }
                width = positions[totalGlyphCount].x;
            }

            node_width(node) = D2Fix(width);
            native_glyph_count(node) = totalGlyphCount;
            native_glyph_info_ptr(node) = glyph_info;

            free(glyphs);
            free(positions);
            free(advances);
        }

        ubidi_close(pBiDi);


        if (font_letter_space[f] != 0) {
            Fixed lsDelta = 0;
            Fixed lsUnit = font_letter_space[f];
            int i;
            for (i = 0; i < totalGlyphCount; ++i) {
                if (glyphAdvances[i] == 0 && lsDelta != 0)
                    lsDelta -= lsUnit;
                locations[i].x += lsDelta;
                lsDelta += lsUnit;
            }
            if (lsDelta != 0) {
                lsDelta -= lsUnit;
                node_width(node) += lsDelta;
            }
        }
        free(glyphAdvances);
    } else {
        _tt_abort("bad native font flag in `measure_native_node`");
    }

    if (use_glyph_metrics == 0 || native_glyph_count(node) == 0) {
        /* for efficiency, height and depth are the font's ascent/descent,
            not true values based on the actual content of the word,
            unless use_glyph_metrics is non-zero */
        node_height(node) = height_base[f];
        node_depth(node) = depth_base[f];
    } else {
        /* this iterates over the glyph data whether it comes from AAT or OT layout */
        FixedPoint* locations = (FixedPoint*)native_glyph_info_ptr(node);
        uint16_t* glyphIDs = (uint16_t*)(locations + native_glyph_count(node));
        float yMin = 65536.0;
        float yMax = -65536.0;
        int i;
        for (i = 0; i < native_glyph_count(node); ++i) {
            float ht, dp;
            float y = Fix2D(-locations[i].y); /* NB negative is upwards in locations[].y! */

            GlyphBBox bbox;
            if (getCachedGlyphBBox(f, glyphIDs[i], &bbox) == 0) {
#ifdef XETEX_MAC
                if (font_area[f] == AAT_FONT_FLAG)
                    GetGlyphBBox_AAT((CFDictionaryRef)(font_layout_engine[f]), glyphIDs[i], &bbox);
                else
#endif
                if (font_area[f] == OTGR_FONT_FLAG)
                    getGlyphBounds((XeTeXLayoutEngine)(font_layout_engine[f]), glyphIDs[i], &bbox);

                cacheGlyphBBox(f, glyphIDs[i], &bbox);
            }

            ht = bbox.yMax;
            dp = -bbox.yMin;

            if (y + ht > yMax)
                yMax = y + ht;
            if (y - dp < yMin)
                yMin = y - dp;
        }
        node_height(node) = D2Fix(yMax);
        node_depth(node) = -D2Fix(yMin);
    }
}

Fixed
real_get_native_italic_correction(void* pNode)
{
    memory_word* node = (memory_word*) pNode;
    unsigned int f = native_font(node);
    unsigned int n = native_glyph_count(node);
    if (n > 0) {
        FixedPoint* locations = (FixedPoint*)native_glyph_info_ptr(node);
        uint16_t* glyphIDs = (uint16_t*)(locations + n);

#ifdef XETEX_MAC
        if (font_area[f] == AAT_FONT_FLAG)
            return D2Fix(GetGlyphItalCorr_AAT((CFDictionaryRef)(font_layout_engine[f]), glyphIDs[n-1]))
                    + font_letter_space[f];
#endif
        if (font_area[f] == OTGR_FONT_FLAG)
            return D2Fix(getGlyphItalCorr((XeTeXLayoutEngine)(font_layout_engine[f]), glyphIDs[n-1]))
                    + font_letter_space[f];
    }

    return 0;
}


Fixed
real_get_native_glyph_italic_correction(void* pNode)
{
    memory_word* node = (memory_word*) pNode;
    uint16_t gid = native_glyph(node);
    unsigned int f = native_font(node);

#ifdef XETEX_MAC
    if (font_area[f] == AAT_FONT_FLAG)
        return D2Fix(GetGlyphItalCorr_AAT((CFDictionaryRef)(font_layout_engine[f]), gid));
#endif
    if (font_area[f] == OTGR_FONT_FLAG)
        return D2Fix(getGlyphItalCorr((XeTeXLayoutEngine)(font_layout_engine[f]), gid));

    return 0;   /* can't actually happen */
}

void
measure_native_glyph(void* pNode, int use_glyph_metrics)
{
    memory_word* node = (memory_word*) pNode;
    uint16_t gid = native_glyph(node);
    unsigned int f = native_font(node);

    float ht = 0.0;
    float dp = 0.0;

#ifdef XETEX_MAC
    if (font_area[f] == AAT_FONT_FLAG) {
        CFDictionaryRef attributes = (CFDictionaryRef)(font_layout_engine[f]);
        node_width(node) = D2Fix(GetGlyphWidth_AAT(attributes, gid));
        if (use_glyph_metrics)
            GetGlyphHeightDepth_AAT(attributes, gid, &ht, &dp);
    } else
#endif
    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXLayoutEngine engine = (XeTeXLayoutEngine)font_layout_engine[f];
        XeTeXFont fontInst = getFont(engine);
        node_width(node) = D2Fix(getGlyphWidth(fontInst, gid));
        if (use_glyph_metrics)
            getGlyphHeightDepth(engine, gid, &ht, &dp);
    } else {
        _tt_abort("bad native font flag in `measure_native_glyph`");
    }

    if (use_glyph_metrics) {
        node_height(node) = D2Fix(ht);
        node_depth(node) = D2Fix(dp);
    } else {
        node_height(node) = height_base[f];
        node_depth(node) = depth_base[f];
    }
}

int32_t
map_char_to_glyph(int32_t font, int32_t ch)
{
    if (ch > 0x10ffff || ((ch >= 0xd800) && (ch <= 0xdfff)))
        return 0;
#ifdef XETEX_MAC
    if (font_area[font] == AAT_FONT_FLAG)
        return MapCharToGlyph_AAT((CFDictionaryRef)(font_layout_engine[font]), ch);
    else
#endif
    if (font_area[font] == OTGR_FONT_FLAG)
        return mapCharToGlyph((XeTeXLayoutEngine)(font_layout_engine[font]), ch);
    else {
        _tt_abort("bad native font flag in `map_char_to_glyph`");
    }
}

int32_t
map_glyph_to_index(int32_t font)
    /* glyph name is at name_of_file */
{
#ifdef XETEX_MAC
    if (font_area[font] == AAT_FONT_FLAG)
        return MapGlyphToIndex_AAT((CFDictionaryRef)(font_layout_engine[font]), name_of_file);
    else
#endif
    if (font_area[font] == OTGR_FONT_FLAG)
        return mapGlyphToIndex((XeTeXLayoutEngine)(font_layout_engine[font]), name_of_file);
    else
        _tt_abort("bad native font flag in `map_glyph_to_index`");
}

int32_t
get_font_char_range(int32_t font, int first)
{
#ifdef XETEX_MAC
    if (font_area[font] == AAT_FONT_FLAG)
        return GetFontCharRange_AAT((CFDictionaryRef)(font_layout_engine[font]), first);
    else
#endif
    if (font_area[font] == OTGR_FONT_FLAG)
        return getFontCharRange((XeTeXLayoutEngine)(font_layout_engine[font]), first);
    else
        _tt_abort("bad native font flag in `get_font_char_range'`");
}

Fixed D2Fix(double d)
{
    Fixed rval = (int)(d * 65536.0 + 0.5);
    return rval;
}

double Fix2D(Fixed f)
{
    double rval = f / 65536.0;
    return rval;
}

/* these are here, not XeTeX_mac.c, because we need stubs on other platforms */
void
aat_get_font_metrics(CFDictionaryRef attributes, int32_t* ascent, int32_t* descent, int32_t* xheight, int32_t* capheight, int32_t* slant)
{
#ifdef XETEX_MAC
    CTFontRef font = fontFromAttributes(attributes);

    *ascent = D2Fix(CTFontGetAscent(font));
    *descent = D2Fix(CTFontGetDescent(font));
    *xheight = D2Fix(CTFontGetXHeight(font));
    *capheight = D2Fix(CTFontGetCapHeight(font));
    *slant = D2Fix(tan(-CTFontGetSlantAngle(font) * M_PI / 180.0));
#endif
}

int
aat_font_get(int what, CFDictionaryRef attributes)
{
    int rval = -1;

#ifdef XETEX_MAC
    CTFontRef font = fontFromAttributes(attributes);
    CFArrayRef list;

    switch (what) {
        case XeTeX_count_glyphs:
            rval = CTFontGetGlyphCount(font);
            break;

        case XeTeX_count_features:
            list = CTFontCopyFeatures(font);
            if (list) {
                rval = CFArrayGetCount(list);
                CFRelease(list);
            }
            break;
    }
#endif
    return rval;
}

int
aat_font_get_1(int what, CFDictionaryRef attributes, int param)
{
    int rval = -1;

#ifdef XETEX_MAC
    CTFontRef font = fontFromAttributes(attributes);

    switch (what) {
        case XeTeX_feature_code:
        {
            CFArrayRef features = CTFontCopyFeatures(font);
            if (features) {
                if (CFArrayGetCount(features) > param) {
                    CFDictionaryRef feature = CFArrayGetValueAtIndex(features, param);
                    CFNumberRef identifier = CFDictionaryGetValue(feature, kCTFontFeatureTypeIdentifierKey);
                    if (identifier)
                        CFNumberGetValue(identifier, kCFNumberIntType, &rval);
                }
                CFRelease(features);
            }
            break;
        }

        case XeTeX_is_exclusive_feature:
        {
            CFArrayRef features = CTFontCopyFeatures(font);
            if (features) {
                CFBooleanRef value;
                CFDictionaryRef feature = findDictionaryInArrayWithIdentifier(features, kCTFontFeatureTypeIdentifierKey, param);
                Boolean found = CFDictionaryGetValueIfPresent(feature, kCTFontFeatureTypeExclusiveKey, (const void **)&value);
                if (found)
                    rval = CFBooleanGetValue(value);
                CFRelease(features);
            }
            break;
        }

        case XeTeX_count_selectors:
        {
            CFArrayRef features = CTFontCopyFeatures(font);
            if (features) {
                CFDictionaryRef feature = findDictionaryInArrayWithIdentifier(features, kCTFontFeatureTypeIdentifierKey, param);
                if (feature) {
                    CFArrayRef selectors = CFDictionaryGetValue(feature, kCTFontFeatureTypeSelectorsKey);
                    if (selectors)
                        rval = CFArrayGetCount(selectors);
                }
                CFRelease(features);
            }
            break;
        }
    }
#endif

    return rval;
}

int
aat_font_get_2(int what, CFDictionaryRef attributes, int param1, int param2)
{
    int rval = -1;

#ifdef XETEX_MAC
    CTFontRef font = fontFromAttributes(attributes);
    CFArrayRef features = CTFontCopyFeatures(font);
    if (features) {
        CFDictionaryRef feature = findDictionaryInArrayWithIdentifier(features, kCTFontFeatureTypeIdentifierKey, param1);
        if (feature) {
            CFArrayRef selectors = CFDictionaryGetValue(feature, kCTFontFeatureTypeSelectorsKey);
            if (selectors) {
                CFDictionaryRef selector;
                switch (what) {
                    case XeTeX_selector_code:
                        if (CFArrayGetCount(selectors) > param2) {
                            CFNumberRef identifier;
                            selector = CFArrayGetValueAtIndex(selectors, param2);
                            identifier = CFDictionaryGetValue(selector, kCTFontFeatureSelectorIdentifierKey);
                            if (identifier)
                                CFNumberGetValue(identifier, kCFNumberIntType, &rval);
                        }
                        break;
                    case XeTeX_is_default_selector:
                        selector = findDictionaryInArrayWithIdentifier(selectors, kCTFontFeatureSelectorIdentifierKey, param2);
                        if (selector) {
                            CFBooleanRef isDefault;
                            Boolean found = CFDictionaryGetValueIfPresent(selector, kCTFontFeatureSelectorDefaultKey, (const void **)&isDefault);
                            if (found)
                                rval = CFBooleanGetValue(isDefault);
                        }
                        break;
                }
            }
        }
        CFRelease(features);
    }
#endif

    return rval;
}

int
aat_font_get_named(int what, CFDictionaryRef attributes)
{
    int rval = -1;

#ifdef XETEX_MAC
    if (what == XeTeX_find_feature_by_name)
        {
            CTFontRef font = fontFromAttributes(attributes);
            CFArrayRef features = CTFontCopyFeatures(font);
            if (features) {
                CFDictionaryRef feature = findDictionaryInArray(features, kCTFontFeatureTypeNameKey,
                                                                name_of_file, name_length);
                if (feature) {
                    CFNumberRef identifier = CFDictionaryGetValue(feature, kCTFontFeatureTypeIdentifierKey);
                    CFNumberGetValue(identifier, kCFNumberIntType, &rval);
                }
                CFRelease(features);
            }
        }
#endif

    return rval;
}

int
aat_font_get_named_1(int what, CFDictionaryRef attributes, int param)
{
    int rval = -1;

#ifdef XETEX_MAC
    CTFontRef font = fontFromAttributes(attributes);

    if (what == XeTeX_find_selector_by_name) {
        CFArrayRef features = CTFontCopyFeatures(font);
        if (features) {
            CFDictionaryRef feature = findDictionaryInArrayWithIdentifier(features, kCTFontFeatureTypeIdentifierKey, param);
            if (feature) {
                CFNumberRef selector = findSelectorByName(feature, name_of_file, name_length);
                if (selector)
                    CFNumberGetValue(selector, kCFNumberIntType, &rval);
            }
            CFRelease(features);
        }
    }
#endif

    return rval;
}

void
aat_print_font_name(int what, CFDictionaryRef attributes, int param1, int param2)
{
    /* Tectonic: this function is called for XETEX_VARIATION_NAME_CODE but doesn't handle it */
#ifdef XETEX_MAC
    CFStringRef name = NULL;
    if (what == XETEX_FEATURE_NAME_CODE || what == XETEX_SELECTOR_NAME_CODE) {
        CTFontRef font = fontFromAttributes(attributes);
        CFArrayRef features = CTFontCopyFeatures(font);
        if (features) {
            CFDictionaryRef feature = findDictionaryInArrayWithIdentifier(features,
                                                                          kCTFontFeatureTypeIdentifierKey,
                                                                          param1);
            if (feature) {
                if (what == XETEX_FEATURE_NAME_CODE)
                    name = CFDictionaryGetValue(feature, kCTFontFeatureTypeNameKey);
                else {
                    CFArrayRef selectors = CFDictionaryGetValue(feature, kCTFontFeatureTypeSelectorsKey);
                    CFDictionaryRef selector = findDictionaryInArrayWithIdentifier(selectors,
                                                                                   kCTFontFeatureSelectorIdentifierKey,
                                                                                   param2);
                    if (selector)
                        name = CFDictionaryGetValue(selector, kCTFontFeatureSelectorNameKey);
                }
            }
            CFRelease(features);
        }
    }

    if (name) {
        CFIndex len = CFStringGetLength(name);
        UniChar* buf = xcalloc(len, sizeof(UniChar));
        CFStringGetCharacters(name, CFRangeMake(0, len), buf);
        print_chars(buf, len);
        free(buf);
    }
#endif
}

void
print_glyph_name(int32_t font, int32_t gid)
{
    const char* s = NULL;
    int len = 0;
#ifdef XETEX_MAC
    if (font_area[font] == AAT_FONT_FLAG) {
        s = GetGlyphNameFromCTFont(fontFromInteger(font), gid, &len);
    } else
#endif
    if (font_area[font] == OTGR_FONT_FLAG) {
        XeTeXLayoutEngine engine = (XeTeXLayoutEngine)font_layout_engine[font];
        s = getGlyphName(getFont(engine), gid, &len);
    } else {
        _tt_abort("bad native font flag in `print_glyph_name`");
    }
    while (len-- > 0)
        print_char(*s++);
    if (s)
    	freeGlyphName(s);
}

int32_t real_get_native_word_cp(void* pNode, int side)
{
    memory_word* node = (memory_word*)pNode;
    FixedPoint* locations = (FixedPoint*)native_glyph_info_ptr(node);
    uint16_t* glyphIDs = (uint16_t*)(locations + native_glyph_count(node));
    uint16_t glyphCount = native_glyph_count(node);
    int32_t f = native_font(node);
    uint16_t actual_glyph;

    if (glyphCount == 0)
        return 0;

    switch (side) {
    case LEFT_SIDE:
        actual_glyph = *glyphIDs;
        break;
    case RIGHT_SIDE:
        actual_glyph = glyphIDs[glyphCount - 1];
        break;
    default:
        assert(0); // we should not reach this point
    }
    return get_cp_code(f, actual_glyph, side);
}
