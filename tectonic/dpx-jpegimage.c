/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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

/*
 * JPEG SUPPORT
 *
 * Accroding to Libjpeg document:
 *
 *  CAUTION: it appears that Adobe Photoshop writes inverted data in CMYK
 *  JPEG files: 0 represents 100% ink coverage, rather than 0% ink as you'd
 *  expect....
 *
 * To wrok with this problem, we must detect whether CMYK JPEG file is
 * created by Photoshop. But there are no reliable way to determine this.
 *
 * According to Adobe Technical Note #5516,
 * "Supporting the DCT Filters in PostScript Level 2", Section 18, p.27.
 *
 *  DCTDecode ignores and skips any APPE marker segment does not begin with
 *  the `Adobe' 5-character string.
 *
 * PDF Reference Manual 4th ed., p.61-62.
 *
 *  The JPEG filter implementation in Adobe Acrobat products does not
 *  support features of the JPEG standard that are irrelevant to images.
 *  In addition, certain choices have been made regarding reserved marker
 *  codes and other optional features of the standard. For details, see
 *  Adobe Technical Note #5116, Supporting the DCT Filters in PostScript
 *  Level 2.
 */

#include "dpx-jpegimage.h"

#include <fcntl.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfximage.h"

#define JPEG_DEBUG_STR "JPEG"
#define JPEG_DEBUG     3

#ifdef    HAVE_LIBJPEG
#include <jpeglib.h>
#endif /* HAVE_LIBJPEG */

/* JPEG Markers */
typedef enum {
    JM_SOF0  = 0xc0,
    JM_SOF1  = 0xc1,
    JM_SOF2  = 0xc2,
    JM_SOF3  = 0xc3,
    JM_SOF5  = 0xc5,
    JM_DHT   = 0xc4,
    JM_SOF6  = 0xc6,
    JM_SOF7  = 0xc7,
    JM_SOF9  = 0xc9,
    JM_SOF10 = 0xca,
    JM_SOF11 = 0xcb,
    JM_DAC   = 0xcc,
    JM_SOF13 = 0xcd,
    JM_SOF14 = 0xce,
    JM_SOF15 = 0xcf,

    JM_RST0  = 0xd0,
    JM_RST1  = 0xd1,
    JM_RST2  = 0xd2,
    JM_RST3  = 0xd3,
    JM_RST4  = 0xd4,
    JM_RST5  = 0xd5,
    JM_RST6  = 0xd6,
    JM_RST7  = 0xd7,

    JM_SOI   = 0xd8,
    JM_EOI   = 0xd9,
    JM_SOS   = 0xda,
    JM_DQT   = 0xdb,
    JM_DNL   = 0xdc,
    JM_DRI   = 0xdd,
    JM_DHP   = 0xde,
    JM_EXP   = 0xdf,

    JM_APP0  = 0xe0,
    JM_APP1  = 0xe1,
    JM_APP2  = 0xe2,
    JM_APP14 = 0xee,
    JM_APP15 = 0xef,

    JM_COM   = 0xfe
} JPEG_marker;

typedef enum {
    JS_APPn_JFIF,
    JS_APPn_ADOBE,
    JS_APPn_ICC,
    JS_APPn_XMP
} JPEG_APPn_sig;

struct JPEG_APPn_JFIF  /* APP0 */
{
    uint16_t       version;
    uint8_t        units;      /* 0: only aspect ratio
                                * 1: dots per inch
                                * 2: dots per cm
                                */
    uint16_t       Xdensity;
    uint16_t       Ydensity;
    uint8_t        Xthumbnail;
    uint8_t        Ythumbnail;
    unsigned char *thumbnail;  /* Thumbnail data. */
};

struct JPEG_APPn_ICC   /* APP2 */
{
    uint8_t        seq_id;
    uint8_t        num_chunks;
    unsigned char *chunk;

    /* Length of ICC profile data in this chunk. */
    size_t         length;
};

struct JPEG_APPn_Adobe /* APP14 */
{
    uint16_t version;
    uint16_t flag0;
    uint16_t flag1;
    uint8_t  transform; /* color transform code */
};

struct JPEG_APPn_XMP   /* APP1 */
{
    unsigned char *packet; /* XMP packet */

    /* Length of XMP packet data */
    size_t         length;
};

struct JPEG_ext
{
    JPEG_marker   marker;
    JPEG_APPn_sig app_sig;
    void         *app_data;
};

#define MAX_COUNT 1024
struct  JPEG_info
{
    uint16_t height;
    uint16_t width;

    uint8_t  bits_per_component;
    uint8_t  num_components;

    double xdpi;
    double ydpi;

    /* Application specific extensions */
    int flags;
    int num_appn, max_appn;
    struct JPEG_ext *appn;

    /* Skip chunks not necessary. */
    char skipbits[MAX_COUNT / 8 + 1];
};

#define HAVE_APPn_JFIF  (1 << 0)
#define HAVE_APPn_ADOBE (1 << 1)
#define HAVE_APPn_ICC   (1 << 2)
#define HAVE_APPn_Exif  (1 << 3)
#define HAVE_APPn_XMP   (1 << 4)

static int      JPEG_scan_file   (struct JPEG_info *j_info, rust_input_handle_t handle);
static int      JPEG_copy_stream (struct JPEG_info *j_info, pdf_obj *stream, rust_input_handle_t handle);

static void     JPEG_info_init   (struct JPEG_info *j_info);
static void     JPEG_info_clear  (struct JPEG_info *j_info);
static pdf_obj *JPEG_get_XMP     (struct JPEG_info *j_info);
static pdf_obj *JPEG_get_iccp    (struct JPEG_info *j_info);
static void     jpeg_get_density (struct JPEG_info *j_info, double *xdensity, double *ydensity);

int
check_for_jpeg (rust_input_handle_t handle)
{
    unsigned char jpeg_sig[2];

    ttstub_input_seek(handle, 0, SEEK_SET);
    if (ttstub_input_read(handle, (char *) jpeg_sig, 2) != 2)
        return 0;
    else if (jpeg_sig[0] != 0xff || jpeg_sig[1] != JM_SOI)
        return 0;

    return 1;
}

int
jpeg_include_image (pdf_ximage *ximage, rust_input_handle_t handle)
{
    pdf_obj         *stream;
    pdf_obj         *stream_dict;
    pdf_obj         *colorspace;
    int              colortype;
    ximage_info      info;
    struct JPEG_info j_info;

    if (!check_for_jpeg(handle)) {
        dpx_warning("%s: Not a JPEG file?", JPEG_DEBUG_STR);
        ttstub_input_seek(handle, 0, SEEK_SET);
        return -1;
    }
    /* File position is 2 here... */

    pdf_ximage_init_image_info(&info);

    JPEG_info_init(&j_info);

    if (JPEG_scan_file(&j_info, handle) < 0) {
        dpx_warning("%s: Not a JPEG file?", JPEG_DEBUG_STR);
        JPEG_info_clear(&j_info);
        return -1;
    }

    switch (j_info.num_components) {
    case 1:
        colortype = PDF_COLORSPACE_TYPE_GRAY;
        break;
    case 3:
        colortype = PDF_COLORSPACE_TYPE_RGB;
        break;
    case 4:
        colortype = PDF_COLORSPACE_TYPE_CMYK;
        break;
    default:
        dpx_warning("%s: Unknown color space (num components: %d)", JPEG_DEBUG_STR, info.num_components);
        JPEG_info_clear(&j_info);
        return -1;
    }

    /* JPEG image use DCTDecode. */
    stream      = pdf_new_stream (0);
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(stream_dict, pdf_new_name("Filter"), pdf_new_name("DCTDecode"));

    /* XMP Metadata */
    if (pdf_get_version() >= 4) {
        if (j_info.flags & HAVE_APPn_XMP) {
            pdf_obj *XMP_stream;

            XMP_stream = JPEG_get_XMP(&j_info);
            pdf_add_dict(stream_dict,
                         pdf_new_name("Metadata"), pdf_ref_obj(XMP_stream));
            pdf_release_obj(XMP_stream);
        }
    }

    /* Check embedded ICC Profile */
    colorspace  = NULL;
    if (j_info.flags & HAVE_APPn_ICC) {
        pdf_obj *icc_stream, *intent;
        int      cspc_id;

        icc_stream = JPEG_get_iccp(&j_info);
        if (!icc_stream)
            colorspace = NULL;
        else {
            if (iccp_check_colorspace(colortype,
                                      pdf_stream_dataptr(icc_stream), pdf_stream_length (icc_stream)) < 0)
                colorspace = NULL;
            else {
                cspc_id = iccp_load_profile(NULL, /* noname */
                                            pdf_stream_dataptr(icc_stream),
                                            pdf_stream_length (icc_stream));
                if (cspc_id < 0)
                    colorspace = NULL;
                else {
                    colorspace = pdf_get_colorspace_reference(cspc_id);
                    intent     = iccp_get_rendering_intent(pdf_stream_dataptr(icc_stream),
                                                           pdf_stream_length (icc_stream));
                    if (intent)
                        pdf_add_dict(stream_dict, pdf_new_name("Intent"), intent);
                }
            }
            pdf_release_obj(icc_stream);
        }
    }
    /* No ICC or invalid ICC profile. */
    if (!colorspace) {
        switch (colortype) {
        case PDF_COLORSPACE_TYPE_GRAY:
            colorspace = pdf_new_name("DeviceGray");
            break;
        case PDF_COLORSPACE_TYPE_RGB:
            colorspace = pdf_new_name("DeviceRGB");
            break;
        case PDF_COLORSPACE_TYPE_CMYK:
            colorspace = pdf_new_name("DeviceCMYK");
            break;
        }
    }
    pdf_add_dict(stream_dict, pdf_new_name("ColorSpace"), colorspace);

#define IS_ADOBE_CMYK(j) (((j).flags & HAVE_APPn_ADOBE) && (j).num_components == 4)
    if (IS_ADOBE_CMYK(j_info)) {
        pdf_obj *decode;
        unsigned int i;

        dpx_warning("Adobe CMYK JPEG: Inverted color assumed.");
        decode = pdf_new_array();
        for (i = 0; i < j_info.num_components; i++) {
            pdf_add_array(decode, pdf_new_number(1.0));
            pdf_add_array(decode, pdf_new_number(0.0));
        }
        pdf_add_dict(stream_dict, pdf_new_name("Decode"), decode);
    }

    /* Copy file */
    JPEG_copy_stream(&j_info, stream, handle);

    info.width              = j_info.width;
    info.height             = j_info.height;
    info.bits_per_component = j_info.bits_per_component;
    info.num_components     = j_info.num_components;

    jpeg_get_density(&j_info, &info.xdensity, &info.ydensity);

    pdf_ximage_set_image(ximage, &info, stream);
    JPEG_info_clear(&j_info);

    return 0;
}


static void
jpeg_get_density (struct JPEG_info *j_info, double *xdensity, double *ydensity)
{
    /*
     * j_info->xdpi and j_info->ydpi are determined in most cases
     * in JPEG_scan_file(). FIXME: However, in some kinds of JPEG files,
     * j_info->xdpi, and j_info->ydpi are not determined in
     * JPEG_scan_file(). In this case we assume
     * that j_info->xdpi = j_info->ydpi = 72.0.
     */

    if (j_info->xdpi < 0.1 && j_info->ydpi < 0.1)
        j_info->xdpi = j_info->ydpi = 72.0;

    *xdensity = 72.0 / j_info->xdpi;
    *ydensity = 72.0 / j_info->ydpi;
}


static void
JPEG_info_init (struct JPEG_info *j_info)
{
    j_info->width              = 0;
    j_info->height             = 0;
    j_info->bits_per_component = 0;
    j_info->num_components     = 0;

    j_info->xdpi     = 0.0;
    j_info->ydpi     = 0.0;

    j_info->flags    = 0;
    j_info->num_appn = 0;
    j_info->max_appn = 0;
    j_info->appn     = NULL;

    memset(j_info->skipbits, 0, MAX_COUNT / 8 + 1);
}

static void
JPEG_release_APPn_data (JPEG_marker marker, JPEG_APPn_sig app_sig, void *app_data)
{
    if (marker  == JM_APP0 &&
        app_sig == JS_APPn_JFIF) {
        struct JPEG_APPn_JFIF *data;

        data = (struct JPEG_APPn_JFIF *) app_data;
        data->thumbnail = mfree(data->thumbnail);

        free(data);
    } else if (marker  == JM_APP2 && app_sig == JS_APPn_ICC) {
        struct JPEG_APPn_ICC *data;

        data = (struct JPEG_APPn_ICC *) app_data;
        data->chunk = mfree(data->chunk);

        free(data);
    } else if (marker  == JM_APP14 && app_sig == JS_APPn_ADOBE) {
        struct JPEG_APPn_Adobe *data;

        data = (struct JPEG_APPn_Adobe *) app_data;

        free(data);
    } else if (marker == JM_APP1 && app_sig == JS_APPn_XMP) {
        struct JPEG_APPn_XMP *data;

        data = (struct JPEG_APPn_XMP *) app_data;
        free(data->packet);

        free(data);
    }
}

static void
JPEG_info_clear (struct JPEG_info *j_info)
{
    if (j_info->num_appn > 0 &&
        j_info->appn    != NULL) {
        int i;

        for (i = 0; i < j_info->num_appn; i++)
            JPEG_release_APPn_data(j_info->appn[i].marker,
                                   j_info->appn[i].app_sig, j_info->appn[i].app_data);
        free(j_info->appn);
    }
    j_info->appn     = NULL;
    j_info->num_appn = 0;
    j_info->max_appn = 0;
    j_info->flags    = 0;
}

static pdf_obj *
JPEG_get_iccp (struct JPEG_info *j_info)
{
    pdf_obj              *icc_stream;
    struct JPEG_APPn_ICC *icc;
    int    i, prev_id = 0, num_icc_seg = -1;

    icc_stream = pdf_new_stream(STREAM_COMPRESS);
    for (i = 0; i < j_info->num_appn; i++) {
        if (j_info->appn[i].marker  != JM_APP2 ||
            j_info->appn[i].app_sig != JS_APPn_ICC)
            continue;
        icc = (struct JPEG_APPn_ICC *) j_info->appn[i].app_data;
        if (num_icc_seg < 0 && prev_id == 0) {
            num_icc_seg = icc->num_chunks;
            /* ICC chunks are sorted? */
        } else if (icc->seq_id != prev_id + 1 ||
                   num_icc_seg != icc->num_chunks || icc->seq_id  > icc->num_chunks) {
            dpx_warning("Invalid JPEG ICC chunk: %d (p:%d, n:%d)", icc->seq_id, prev_id, icc->num_chunks);
            pdf_release_obj(icc_stream);
            icc_stream = NULL;
            break;
        }
        pdf_add_stream(icc_stream, icc->chunk, icc->length);
        prev_id     = icc->seq_id;
        num_icc_seg = icc->num_chunks;
    }

    return icc_stream;
}

static pdf_obj *
JPEG_get_XMP (struct JPEG_info *j_info)
{
    pdf_obj              *XMP_stream, *stream_dict;
    struct JPEG_APPn_XMP *XMP;
    int    i, count = 0;

    /* I don't know if XMP Metadata should be compressed here.*/
    XMP_stream  = pdf_new_stream(STREAM_COMPRESS);
    stream_dict = pdf_stream_dict(XMP_stream);
    pdf_add_dict(stream_dict,
                 pdf_new_name("Type"), pdf_new_name("Metadata"));
    pdf_add_dict(stream_dict,
                 pdf_new_name("Subtype"), pdf_new_name("XML"));
    for (i = 0; i < j_info->num_appn; i++) {
        /* Not sure for the case of multiple segments */
        if (j_info->appn[i].marker  != JM_APP1 ||
            j_info->appn[i].app_sig != JS_APPn_XMP)
            continue;
        XMP = (struct JPEG_APPn_XMP *) j_info->appn[i].app_data;
        pdf_add_stream(XMP_stream, XMP->packet, XMP->length);
        count++;
    }
    if (count > 1)
        dpx_warning("%s: Multiple XMP segments found in JPEG file. (untested)", JPEG_DEBUG_STR);

    return XMP_stream;
}

static JPEG_marker
JPEG_get_marker (rust_input_handle_t handle)
{
    int c;

    c = ttstub_input_getc(handle);
    if (c != 255)
        return -1;

    for (;;) {
        c = ttstub_input_getc(handle);
        if (c < 0)
            return -1;
        else if (c > 0 && c < 255) {
            return c;
        }
    }
}

static int
add_APPn_marker (struct JPEG_info *j_info, JPEG_marker marker, int app_sig, void *app_data)
{
    int n;

    if (j_info->num_appn >= j_info->max_appn) {
        j_info->max_appn += 16;
        j_info->appn = RENEW(j_info->appn, j_info->max_appn, struct JPEG_ext);
    }
    n = j_info->num_appn;

    j_info->appn[n].marker   = marker;
    j_info->appn[n].app_sig  = app_sig;
    j_info->appn[n].app_data = app_data;

    j_info->num_appn += 1;

    return n;
}

static unsigned short
read_APP14_Adobe (struct JPEG_info *j_info, rust_input_handle_t handle)
{
    struct JPEG_APPn_Adobe *app_data;

    app_data = NEW(1, struct JPEG_APPn_Adobe);
    app_data->version   = tt_get_unsigned_pair(handle);
    app_data->flag0     = tt_get_unsigned_pair(handle);
    app_data->flag1     = tt_get_unsigned_pair(handle);
    app_data->transform = tt_get_unsigned_byte(handle);

    add_APPn_marker(j_info, JM_APP14, JS_APPn_ADOBE, app_data);

    return 7;
}

#define JPEG_EXIF_BIGENDIAN    0
#define JPEG_EXIF_LITTLEENDIAN 1
static int
read_exif_bytes (unsigned char **pp, int n, int endian)
{
    int            rval = 0;
    unsigned char *p   = *pp;
    int            i;

    switch (endian) {
    case JPEG_EXIF_BIGENDIAN:
        for (i = 0; i < n; i++) {
            rval = (rval << 8) + p[i];
        }
        break;
    case JPEG_EXIF_LITTLEENDIAN:
        for (i = n - 1; i >= 0; i--) {
            rval = (rval << 8) + p[i];
        }
        break;
    }

    *pp += n;
    return rval;
}

#define JPEG_EXIF_TYPE_BYTE             1
#define JPEG_EXIF_TYPE_ASCII            2
#define JPEG_EXIF_TYPE_SHORT            3
#define JPEG_EXIF_TYPE_LONG             4
#define JPEG_EXIF_TYPE_RATIONAL         5
#define JPEG_EXIF_TYPE_UNDEFINED        7
#define JPEG_EXIF_TYPE_SLONG            9
#define JPEG_EXIF_TYPE_SRATIONAL       10

#define JPEG_EXIF_TAG_XRESOLUTION     282
#define JPEG_EXIF_TAG_YRESOLUTION     283
#define JPEG_EXIF_TAG_RESOLUTIONUNIT  296
#define JPEG_EXIF_TAG_RESUNIT_MS      0x5110
#define JPEG_EXIF_TAG_XRES_MS         0x5111
#define JPEG_EXIF_TAG_YRES_MS         0x5112

static size_t
read_APP1_Exif (struct JPEG_info *info, rust_input_handle_t handle, size_t length)
{
    unsigned char *buffer, *endptr;
    unsigned char *p, *rp;
    unsigned char *tiff_header;
    char bigendian;
    int i;
    int num_fields, tag, type, value;
    int num = 0, den = 0;
    double xres = 0.0;
    double yres = 0.0;
    double res_unit = 1.0;
    unsigned int xres_ms = 0;
    unsigned int yres_ms = 0;
    double res_unit_ms = 0.0;
    double exifxdpi = 0.0;
    double exifydpi = 0.0;
    ssize_t r;

    buffer = xmalloc (length);

    r = ttstub_input_read (handle, (char *) buffer, length);
    if (r < 0 || (size_t) r != length)
        goto err;

    p = buffer;
    endptr = buffer + length;

    while (p < buffer + length && *p == 0)
        ++p;

    if (p + 8 >= endptr)
        goto err;

    tiff_header = p;

    if (*p == 'M' && *(p+1) == 'M')
        bigendian = JPEG_EXIF_BIGENDIAN;
    else if (*p == 'I' && *(p+1) == 'I')
        bigendian = JPEG_EXIF_LITTLEENDIAN;
    else {
        dpx_warning("JPEG: Invalid value in Exif TIFF header.");
        goto err;
    }

    p += 2;

    i = read_exif_bytes (&p, 2, bigendian);
    if (i != 42) {
        dpx_warning("JPEG: Invalid value in Exif TIFF header.");
        goto err;
    }

    i = read_exif_bytes (&p, 4, bigendian);
    p = tiff_header + i;
    num_fields = read_exif_bytes (&p, 2, bigendian);

    while (num_fields-- > 0) {
        int count;

        tag = read_exif_bytes (&p, 2, bigendian);
        type = read_exif_bytes (&p, 2, bigendian);
        count = read_exif_bytes (&p, 4, bigendian);

        switch (type) {
        case JPEG_EXIF_TYPE_BYTE:
            value = *p++;
            p += 3;
            break;
        case JPEG_EXIF_TYPE_SHORT:
            value = read_exif_bytes (&p, 2, bigendian);
            p += 2;
            break;
        case JPEG_EXIF_TYPE_LONG:
        case JPEG_EXIF_TYPE_SLONG:
            value = read_exif_bytes (&p, 4, bigendian);
            break;
        case JPEG_EXIF_TYPE_RATIONAL:
        case JPEG_EXIF_TYPE_SRATIONAL:
            value = read_exif_bytes (&p, 4, bigendian);
            rp = tiff_header + value;
            num = read_exif_bytes (&rp, 4, bigendian);
            den = read_exif_bytes (&rp, 4, bigendian);
            break;
        case JPEG_EXIF_TYPE_UNDEFINED:
            value = *p++;
            p += 3;
            break;
        case JPEG_EXIF_TYPE_ASCII:
        default:
            value = 0;
            p += 4;
            break;
        }

        switch (tag) {
        case JPEG_EXIF_TAG_XRESOLUTION:
            if (den != 0)
                xres = num / den;
            break;
        case JPEG_EXIF_TAG_YRESOLUTION:
            if (den != 0)
                yres = num / den;
            break;
        case JPEG_EXIF_TAG_RESOLUTIONUNIT:
            switch (value) {
            case 2: /* inch */
                res_unit = 1.0;
                break;
            case 3: /* cm */
                res_unit = 2.54;
                break;
            }
        case JPEG_EXIF_TAG_RESUNIT_MS: /* PixelUnit */
            if (type != JPEG_EXIF_TYPE_BYTE || count != 1) {
                dpx_warning("%s: Invalid data for ResolutionUnit in Exif chunk.", JPEG_DEBUG_STR);
                goto err;
            }
            value = read_exif_bytes(&p, 1, bigendian);
            p += 3;
            if (value == 1)
                res_unit_ms = 0.0254; /* Unit is meter */
            else
                res_unit_ms = 0.0;
            break;
        case JPEG_EXIF_TAG_XRES_MS: /* PixelPerUnitX */
            if (type != JPEG_EXIF_TYPE_LONG || count != 1) {
                dpx_warning("%s: Invalid data for PixelPerUnitX in Exif chunk.", JPEG_DEBUG_STR);
                goto err;
            }
            value = read_exif_bytes(&p, 4, bigendian);
            xres_ms = value;
            break;
        case JPEG_EXIF_TAG_YRES_MS: /* PixelPerUnitY */
            if (type != JPEG_EXIF_TYPE_LONG || count != 1) {
                dpx_warning("%s: Invalid data for PixelPerUnitY in Exif chunk.", JPEG_DEBUG_STR);
                goto err;
            }
            value = read_exif_bytes(&p, 4, bigendian);
            yres_ms = value;
            break;
        }
    }

    /* Calculate Exif resolution, if given. */

    if (xres > 0.0 && yres > 0.0) {
        exifxdpi = xres * res_unit;
        exifydpi = yres * res_unit;
    } else if (xres_ms > 0 && yres_ms > 0 && res_unit_ms > 0.0) {
        exifxdpi = xres_ms * res_unit_ms;
        exifydpi = yres_ms * res_unit_ms;
    } else {
        exifxdpi = 72.0 * res_unit;
        exifydpi = 72.0 * res_unit;
    }

    /* Do not overwrite if already specified in JFIF */

    if (info->xdpi < 0.1 && info->ydpi < 0.1) {
        info->xdpi = exifxdpi;
        info->ydpi = exifydpi;
    } else {
        double xxx1 = floor(exifxdpi + 0.5);
        double xxx2 = floor(info->xdpi + 0.5);
        double yyy1 = floor(exifydpi + 0.5);
        double yyy2 = floor(info->ydpi + 0.5);

        if (xxx1 != xxx2 || yyy1 != yyy2) {
            dpx_warning("JPEG: Inconsistent resolution may have been "
                        "specified in Exif and JFIF: %gx%g - %gx%g",
                        xres * res_unit, yres * res_unit, info->xdpi, info->ydpi);
        }
    }

err:
    free (buffer);
    return length;
}

static size_t
read_APP0_JFIF (struct JPEG_info *j_info, rust_input_handle_t handle)
{
    struct JPEG_APPn_JFIF *app_data;
    size_t thumb_data_len;

    app_data = NEW(1, struct JPEG_APPn_JFIF);
    app_data->version    = tt_get_unsigned_pair(handle);
    app_data->units      = tt_get_unsigned_byte(handle);
    app_data->Xdensity   = tt_get_unsigned_pair(handle);
    app_data->Ydensity   = tt_get_unsigned_pair(handle);
    app_data->Xthumbnail = tt_get_unsigned_byte(handle);
    app_data->Ythumbnail = tt_get_unsigned_byte(handle);
    thumb_data_len = 3 * app_data->Xthumbnail * app_data->Ythumbnail;
    if (thumb_data_len > 0) {
        app_data->thumbnail = NEW(thumb_data_len, unsigned char);
        ttstub_input_read(handle, (char *) app_data->thumbnail, thumb_data_len);
    } else {
        app_data->thumbnail = NULL;
    }

    add_APPn_marker(j_info, JM_APP0, JS_APPn_JFIF, app_data);

    switch (app_data->units) {
    case 1:
        j_info->xdpi = app_data->Xdensity;
        j_info->ydpi = app_data->Ydensity;
        break;
    case 2: /* density is in pixels per cm */
        j_info->xdpi = app_data->Xdensity * 2.54;
        j_info->ydpi = app_data->Ydensity * 2.54;
        break;
    default: /* FIXME: not sure what to do with this.... */
        j_info->xdpi = 72.0;
        j_info->ydpi = 72.0;
        break;
    }

    return (9 + thumb_data_len);
}

static size_t
read_APP0_JFXX (rust_input_handle_t handle, size_t length)
{
    tt_get_unsigned_byte(handle);
    /* Extension Code:
     *
     * 0x10: Thumbnail coded using JPEG
     * 0x11: Thumbnail stored using 1 byte/pixel
     * 0x13: Thumbnail stored using 3 bytes/pixel
     */
    ttstub_input_seek(handle, length - 1, SEEK_CUR); /* Thunbnail image */

    /* Ignore */

    return length;
}

static size_t
read_APP1_XMP (struct JPEG_info *j_info, rust_input_handle_t handle, size_t length)
{
    struct JPEG_APPn_XMP *app_data;

    app_data = NEW(1, struct JPEG_APPn_XMP);
    app_data->length = length;
    app_data->packet = NEW(app_data->length, unsigned char);
    ttstub_input_read(handle, (char *) app_data->packet, app_data->length);

    add_APPn_marker(j_info, JM_APP1, JS_APPn_XMP, app_data);

    return length;
}

static size_t
read_APP2_ICC (struct JPEG_info *j_info, rust_input_handle_t handle, size_t length)
{
    struct JPEG_APPn_ICC *app_data;

    app_data = NEW(1, struct JPEG_APPn_ICC);
    app_data->seq_id      = tt_get_unsigned_byte(handle); /* Starting at 1 */
    app_data->num_chunks  = tt_get_unsigned_byte(handle);
    app_data->length      = length - 2;
    app_data->chunk       = NEW(app_data->length, unsigned char);
    ttstub_input_read(handle, (char *) app_data->chunk, app_data->length);

    add_APPn_marker(j_info, JM_APP2, JS_APPn_ICC, app_data);

    return length;
}

static int
JPEG_copy_stream (struct JPEG_info *j_info, pdf_obj *stream, rust_input_handle_t handle)
{
    JPEG_marker marker;
    int         length;
    int         found_SOFn, count;

#define SKIP_CHUNK(j,c) ((j)->skipbits[(c) / 8] & (1 << (7 - (c) % 8)))
#define COPY_CHUNK(f,s,l) while ((l) > 0) {                             \
        int nb_read = ttstub_input_read((f), work_buffer, MIN((l), WORK_BUFFER_SIZE)); \
        if (nb_read > 0)                                                \
            pdf_add_stream((s), work_buffer, nb_read);                  \
        (l) -= nb_read;                                                 \
    }
    ttstub_input_seek(handle, 0, SEEK_SET);
    count      = 0;
    found_SOFn = 0;
    while (!found_SOFn && count < MAX_COUNT &&
           (marker = JPEG_get_marker(handle)) != (JPEG_marker) - 1) {
        if ( marker == JM_SOI  ||
             (marker >= JM_RST0 && marker <= JM_RST7)) {
            work_buffer[0] = (char) 0xff;
            work_buffer[1] = (char) marker;
            pdf_add_stream(stream, work_buffer, 2);
        } else {
            length = tt_get_unsigned_pair(handle) - 2;
            switch (marker) {
            case JM_SOF0:  case JM_SOF1:  case JM_SOF2:  case JM_SOF3:
            case JM_SOF5:  case JM_SOF6:  case JM_SOF7:  case JM_SOF9:
            case JM_SOF10: case JM_SOF11: case JM_SOF13: case JM_SOF14:
            case JM_SOF15:
                work_buffer[0] = (char) 0xff;
                work_buffer[1] = (char) marker;
                work_buffer[2] = ((length + 2) >> 8) & 0xff;
                work_buffer[3] =  (length + 2) & 0xff;
                pdf_add_stream(stream, work_buffer, 4);
                COPY_CHUNK(handle, stream, length);
                found_SOFn = 1;
                break;
            default:
                if (SKIP_CHUNK(j_info, count)) {
                    ttstub_input_seek(handle, length, SEEK_CUR);
                } else {
                    work_buffer[0] = (char) 0xff;
                    work_buffer[1] = (char) marker;
                    work_buffer[2] = ((length + 2) >> 8) & 0xff;
                    work_buffer[3] =  (length + 2) & 0xff;
                    pdf_add_stream(stream, work_buffer, 4);
                    COPY_CHUNK(handle, stream, length);
                }
            }
        }
        count++;
    }

    {
        size_t total_size = ttstub_input_get_size(handle);
        size_t pos = ttstub_input_seek(handle, 0, SEEK_CUR);

        while ((length = ttstub_input_read(handle, work_buffer, MIN(WORK_BUFFER_SIZE, total_size - pos))) > 0) {
            pdf_add_stream(stream, work_buffer, length);
            pos += length;
        }
    }

    return (found_SOFn ? 0 : -1);
}

#define SET_SKIP(j,c) if ((c) < MAX_COUNT) {                    \
        (j)->skipbits[(c) / 8] |= (1 << (7 - ((c) % 8)));       \
    }
static int
JPEG_scan_file (struct JPEG_info *j_info, rust_input_handle_t handle)
{
    JPEG_marker marker;
    int         found_SOFn, count;
    char        app_sig[128];

    ttstub_input_seek(handle, 0, SEEK_SET);
    count      = 0;
    found_SOFn = 0;
    while (!found_SOFn &&
           (marker = JPEG_get_marker(handle)) != (JPEG_marker) -1) {
        if ( marker != JM_SOI  &&
             (marker  < JM_RST0 || marker > JM_RST7)) {
            int length = tt_get_unsigned_pair(handle) - 2;
            switch (marker) {
            case JM_SOF0:  case JM_SOF1:  case JM_SOF2:  case JM_SOF3:
            case JM_SOF5:  case JM_SOF6:  case JM_SOF7:  case JM_SOF9:
            case JM_SOF10: case JM_SOF11: case JM_SOF13: case JM_SOF14:
            case JM_SOF15:
                j_info->bits_per_component = tt_get_unsigned_byte(handle);
                j_info->height             = tt_get_unsigned_pair(handle);
                j_info->width              = tt_get_unsigned_pair(handle);
                j_info->num_components     = tt_get_unsigned_byte(handle);
                found_SOFn = 1;
                break;
            case JM_APP0:
                if (length > 5) {
                    if (ttstub_input_read(handle, app_sig, 5) != 5)
                        return -1;
                    length -= 5;
                    if (!memcmp(app_sig, "JFIF\000", 5)) {
                        j_info->flags |= HAVE_APPn_JFIF;
                        length -= read_APP0_JFIF(j_info, handle);
                    } else if (!memcmp(app_sig, "JFXX", 5)) {
                        length -= read_APP0_JFXX(handle, length);
                    }
                }
                ttstub_input_seek(handle, length, SEEK_CUR);
                break;
            case JM_APP1:
                if (length > 5) {
                    if (ttstub_input_read(handle, app_sig, 5) != 5)
                        return -1;
                    length -= 5;
                    if (!memcmp(app_sig, "Exif\000", 5)) {
                        j_info->flags |= HAVE_APPn_Exif;
                        length -= read_APP1_Exif(j_info, handle, length);
                    } else if (!memcmp(app_sig, "http:", 5) && length > 24) {
                        if (ttstub_input_read(handle, app_sig, 24) != 24)
                            return -1;
                        length -= 24;
                        if (!memcmp(app_sig, "//ns.adobe.com/xap/1.0/\000", 24)) {
                            j_info->flags |= HAVE_APPn_XMP;
                            length -= read_APP1_XMP(j_info, handle, length);
                            SET_SKIP(j_info, count);
                        }
                    }
                }
                ttstub_input_seek(handle, length, SEEK_CUR);
                break;
            case JM_APP2:
                if (length >= 14) {
                    if (ttstub_input_read(handle, app_sig, 12) != 12)
                        return -1;
                    length -= 12;
                    if (!memcmp(app_sig, "ICC_PROFILE\000", 12)) {
                        j_info->flags |= HAVE_APPn_ICC;
                        length -= read_APP2_ICC(j_info, handle, length);
                        SET_SKIP(j_info, count);
                    }
                }
                ttstub_input_seek(handle, length, SEEK_CUR);
                break;
            case JM_APP14:
                if (length > 5) {
                    if (ttstub_input_read(handle, app_sig, 5) != 5)
                        return -1;
                    length -= 5;
                    if (!memcmp(app_sig, "Adobe", 5)) {
                        j_info->flags |= HAVE_APPn_ADOBE;
                        length -= read_APP14_Adobe(j_info, handle);
                    } else {
                        SET_SKIP(j_info, count);
                    }
                }
                ttstub_input_seek(handle, length, SEEK_CUR);
                break;
            default:
                ttstub_input_seek(handle, length, SEEK_CUR);
                if (marker >= JM_APP0 && marker <= JM_APP15) {
                    SET_SKIP(j_info, count);
                }
                break;
            }
        }
        count++;
    }

    /*
     * If j_info->xdpi, and j_info->ydpi are not yet determined,
     * they are assumed to be 72.0 to avoid division by zero.
     */
    if (j_info->xdpi < 0.1 && j_info->ydpi < 0.1)
        j_info->xdpi = j_info->ydpi = 72.0;

    return (found_SOFn ? 0 : -1);
}

int
jpeg_get_bbox (rust_input_handle_t handle, unsigned int *width, unsigned int *height, double *xdensity, double *ydensity)
{
    struct JPEG_info j_info;

    JPEG_info_init(&j_info);

    if (JPEG_scan_file(&j_info, handle) < 0) {
        dpx_warning("%s: Not a JPEG file?", JPEG_DEBUG_STR);
        JPEG_info_clear(&j_info);
        return -1;
    }

    *width  = j_info.width;
    *height = j_info.height;

    jpeg_get_density(&j_info, xdensity, ydensity);

    JPEG_info_clear(&j_info);

    return 0;
}
