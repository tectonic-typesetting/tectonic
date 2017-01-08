/* tectonic/jpegimage.c -- JPEG-format image handling
   Copyright 2016 the Tectonic Project
   Copyright 1994-2009 SIL International (SIL author: Jonathan Kew)
   Copyright 2002-2015 Jin-Hwan Cho, Shunsaku Hirata, dvipdfmx project team
   Copyright 1998, 1999 Mark A. Wicks
   Licensed under the GPL version 2 or later.
*/

#include <tectonic/jpegimage.h>
#include <tectonic/internals.h>

#define HAVE_APPn_JFIF  (1 << 0)
#define HAVE_APPn_ADOBE (1 << 1)
#define HAVE_APPn_ICC   (1 << 2)
#define HAVE_APPn_Exif  (1 << 3)


/* JPEG Markers */
typedef enum {
    JM_SOF0  = 0xC0,
    JM_SOF1  = 0xC1,
    JM_SOF2  = 0xC2,
    JM_SOF3  = 0xC3,
    JM_SOF5  = 0xC5,
    JM_DHT   = 0xC4,
    JM_SOF6  = 0xC6,
    JM_SOF7  = 0xC7,
    JM_SOF9  = 0xC9,
    JM_SOF10 = 0xCa,
    JM_SOF11 = 0xCb,
    JM_DAC   = 0xCc,
    JM_SOF13 = 0xCd,
    JM_SOF14 = 0xCe,
    JM_SOF15 = 0xCf,

    JM_RST0  = 0xD0,
    JM_RST1  = 0xD1,
    JM_RST2  = 0xD2,
    JM_RST3  = 0xD3,
    JM_RST4  = 0xD4,
    JM_RST5  = 0xD5,
    JM_RST6  = 0xD6,
    JM_RST7  = 0xD7,

    JM_SOI   = 0xD8,
    JM_EOI   = 0xD9,
    JM_SOS   = 0xDa,
    JM_DQT   = 0xDb,
    JM_DNL   = 0xDc,
    JM_DRI   = 0xDd,
    JM_DHP   = 0xDe,
    JM_EXP   = 0xDf,

    JM_APP0  = 0xE0,
    JM_APP1  = 0xE1,
    JM_APP2  = 0xE2,
    JM_APP14 = 0xEe,
    JM_APP15 = 0xEf,

    JM_COM   = 0xFE
} JPEG_marker;

typedef enum {
    JS_APPn_JFIF,
    JS_APPn_ADOBE,
    JS_APPn_ICC
} JPEG_APPn_sig;

struct JPEG_APPn_JFIF  /* APP0 */
{
    unsigned short version;
    unsigned char  units;      /* 0: only aspect ratio
                                * 1: dots per inch
                                * 2: dots per cm
                                */
    unsigned short Xdensity;
    unsigned short Ydensity;
    unsigned char  Xthumbnail;
    unsigned char  Ythumbnail;
    unsigned char *thumbnail;  /* Thumbnail data. */
};

struct JPEG_APPn_ICC   /* APP2 */
{
    unsigned char  seq_id;
    unsigned char  num_chunks;
    unsigned char *chunk;

    /* Length of ICC profile data in this chunk. */
    unsigned short length;
};

struct JPEG_APPn_Adobe /* APP14 */
{
    unsigned short version;
    unsigned short flag0;
    unsigned short flag1;
    unsigned char  transform; /* color transform code */
};

struct JPEG_ext
{
    JPEG_marker   marker;
    JPEG_APPn_sig app_sig;
    void         *app_data;
};


#define RELEASE(p)              free(p)
#define NEW(n, t)               (t*)xmalloc(n * sizeof(t))
#define RENEW(p, n, t)  ((p) ? (t*)xrealloc(p, (n) * sizeof(t)) : NEW(n, t))


static unsigned char
get_unsigned_byte (rust_input_handle_t file)
{
    int ch;

    if ((ch = ttstub_input_getc (file)) < 0)
        _tt_abort ("unexpected EOF in get_unsigned_byte()");

    return (unsigned char) ch;
}


static unsigned short
get_unsigned_pair (rust_input_handle_t file)
{
    unsigned short s;

    s = get_unsigned_byte (file);
    return s << 8 + get_unsigned_byte (file);
}


int
tt_check_for_jpeg (rust_input_handle_t file)
{
    unsigned char jpeg_sig[2];

    ttstub_input_seek (file, 0, SEEK_SET);

    if (ttstub_input_read (file, jpeg_sig, 2) != 2)
        return 0;

    if (jpeg_sig[0] != 0xFF || jpeg_sig[1] != JM_SOI)
        return 0;

    return 1;
}


static unsigned int
read_exif_bytes(unsigned char **p, int n, int b)
{
    unsigned int rval = 0;
    unsigned char *pp = *p;

    if (b) {
        switch (n) {
        case 4:
            rval += *pp++; rval <<= 8;
            rval += *pp++; rval <<= 8;
        case 2:
            rval += *pp++; rval <<= 8;
            rval += *pp;
            break;
        }
    } else {
        pp += n;
        switch (n) {
        case 4:
            rval += *--pp; rval <<= 8;
            rval += *--pp; rval <<= 8;
        case 2:
            rval += *--pp; rval <<= 8;
            rval += *--pp;
            break;
        }
    }

    *p += n;
    return rval;
}


static unsigned short
read_APP1_Exif (struct jpeg_info *info, rust_input_handle_t file, unsigned short length)
{
    unsigned char *buffer;
    unsigned char *p, *rp;
    unsigned char *tiff_header;
    char bigendian;
    int i;
    int num_fields, tag, type, value;
    int num = 0, den = 0;
    double xres = 72.0;
    double yres = 72.0;
    double res_unit = 1.0;

    buffer = malloc (length);
    if (buffer == NULL)
	_tt_abort("malloc of %d bytes failed", (int) length);

    if (ttstub_input_read (file, buffer, length) != length)
	return length;

    p = buffer;
    while (p < buffer + length && *p == 0)
        ++p;

    tiff_header = p;

    if (*p == 'M' && *(p+1) == 'M')
        bigendian = 1;
    else if (*p == 'I' && *(p+1) == 'I')
        bigendian = 0;
    else {
	free (buffer);
	return length;
    }

    p += 2;

    i = read_exif_bytes (&p, 2, bigendian);
    if (i != 42) {
	free (buffer);
	return length;
    }

    i = read_exif_bytes (&p, 4, bigendian);
    p = tiff_header + i;
    num_fields = read_exif_bytes (&p, 2, bigendian);

    while (num_fields-- > 0) {
        tag = read_exif_bytes (&p, 2, bigendian);
        type = read_exif_bytes (&p, 2, bigendian);
        read_exif_bytes (&p, 4, bigendian);

        switch (type) {
        case 1: /* byte */
            value = *p++;
            p += 3;
            break;
        case 3: /* short */
            value = read_exif_bytes (&p, 2, bigendian);
            p += 2;
            break;
        case 4: /* long */
        case 9: /* slong */
            value = read_exif_bytes (&p, 4, bigendian);
            break;
        case 5: /* rational */
        case 10: /* srational */
            value = read_exif_bytes (&p, 4, bigendian);
            rp = tiff_header + value;
            num = read_exif_bytes (&rp, 4, bigendian);
            den = read_exif_bytes (&rp, 4, bigendian);
            break;
        case 7: /* undefined */
            value = *p++;
            p += 3;
            break;
        case 2: /* ascii */
        default:
	    value = 0;
            p += 4;
            break;
        }

        switch (tag) {
        case 282: /* x res */
            if (den != 0)
                xres = num / den;
            break;
        case 283: /* y res */
            if (den != 0)
                yres = num / den;
            break;
        case 296: /* res unit */
            switch (value) {
            case 2:
                res_unit = 1.0;
                break;
            case 3:
                res_unit = 2.54;
                break;
            }
        }
    }

    info->xdpi = xres * res_unit;
    info->ydpi = yres * res_unit;

    free (buffer);
    return length;
}


static unsigned short
read_APP0_JFIF (struct jpeg_info *info, rust_input_handle_t file)
{
    unsigned short units, xdensity, ydensity;
    unsigned char xthumb, ythumb;

    get_unsigned_pair (file); /* version: ignore */
    units = get_unsigned_byte (file);
    xdensity = get_unsigned_pair (file);
    ydensity = get_unsigned_pair (file);

    switch (units) {
    case 1:
        info->xdpi = xdensity;
        info->ydpi = ydensity;
        break;
    case 2: /* density is in pixels per cm */
        info->xdpi = xdensity * 2.54;
        info->ydpi = ydensity * 2.54;
        break;
    default: /* FIXME: not sure what to do with this.... */
        info->xdpi = 72.0;
        info->ydpi = 72.0;
        break;
    }

    return 7;
}


static JPEG_marker
jpeg_get_marker (rust_input_handle_t file)
{
    int c;

    c = ttstub_input_getc (file);
    if (c != 255)
        return -1;

    for (;;) {
	c = ttstub_input_getc (file);
        if (c < 0)
            return -1;
        if (c > 0 && c < 255)
            return c;
    }

    return -1;
}


int
jpeg_scan_file (struct jpeg_info *info, rust_input_handle_t file)
{
    JPEG_marker marker;
    unsigned short length;
    int found_SOFn, count;
    char app_sig[128];

    info->width  = 0;
    info->height = 0;
    info->bits_per_component = 0;
    info->num_components = 0;
    info->xdpi = 0.0;
    info->ydpi = 0.0;

    ttstub_input_seek (file, 0, SEEK_SET);

    count = 0;
    found_SOFn = 0;

    while (!found_SOFn && (marker = jpeg_get_marker(file)) != (JPEG_marker) -1) {
        if (marker == JM_SOI || (marker >= JM_RST0 && marker <= JM_RST7)) {
            count++;
            continue;
        }

        length = get_unsigned_pair(file) - 2;

        switch (marker) {
        case JM_SOF0:
	case JM_SOF1:
	case JM_SOF2:
	case JM_SOF3:
        case JM_SOF5:
	case JM_SOF6:
	case JM_SOF7:
	case JM_SOF9:
        case JM_SOF10:
	case JM_SOF11:
	case JM_SOF13:
	case JM_SOF14:
        case JM_SOF15:
            info->bits_per_component = get_unsigned_byte (file);
            info->height = get_unsigned_pair (file);
            info->width = get_unsigned_pair (file);
            info->num_components = get_unsigned_byte (file);
            found_SOFn = 1;
            break;

        case JM_APP0:
	    /* This could be a JFIF section, which has DPI info. */
            if (length > 5) {
                if (ttstub_input_read (file, app_sig, 5) != 5)
                    return -1;

                length -= 5;

                if (!memcmp (app_sig, "JFIF\0", 5))
                    length -= read_APP0_JFIF(info, file);
            }
            ttstub_input_seek (file, length, SEEK_CUR);
            break;

        case JM_APP1:
	    /* This could be an EXIF section, which has DPI info. */
            if (length > 5) {
                if (ttstub_input_read (file, app_sig, 5) != 5)
                    return -1;

                length -= 5;

                if (!memcmp (app_sig, "Exif\0", 5))
                    length -= read_APP1_Exif(info, file, length);
            }
            ttstub_input_seek (file, length, SEEK_CUR);
            break;

        default:
            ttstub_input_seek (file, length, SEEK_CUR);
            break;
        }

    }

    if (found_SOFn)
	return 0;
    return -1;
}
