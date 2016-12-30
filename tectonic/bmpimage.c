/* tectonic/bmpimage.c -- BMP-format image handling
   Copyright 2016 the Tectonic Project
   Copyright 1994-2006 SIL International (SIL author: Jonathan Kew)
   Copyright 2002 Jin-Hwan Cho, Shunsaku Hirata, dvipdfmx project team
   Copyright 1998, 1999 Mark A. Wicks
   Licensed under the GPL version 2 or later.
*/

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/bmpimage.h>

#define DIB_FILE_HEADER_SIZE 14
#define DIB_CORE_HEADER_SIZE 12
#define DIB_INFO_HEADER_SIZE 40
#define DIB_INFO_HEADER_SIZE2 64
#define DIB_INFO_HEADER_SIZE4 108
#define DIB_INFO_HEADER_SIZE5 124

#define DIB_COMPRESS_NONE 0
#define DIB_COMPRESS_RLE8 1
#define DIB_COMPRESS_RLE4 2

#define DIB_HEADER_SIZE_MAX (DIB_FILE_HEADER_SIZE + DIB_INFO_HEADER_SIZE5)

int
check_for_bmp (rust_input_handle_t file)
{
    unsigned char sigbytes[2];

    if (file == NULL)
        return 0;

    ttstub_input_seek (file, 0, SEEK_SET);

    if (ttstub_input_read (file, sigbytes, 2) != 2)
        return 0;

    if (sigbytes[0] == 'B' && sigbytes[1] == 'M')
        return 1;

    return 0;
}


#define USHORT_LE(p) (p[0] + (p[1] << 8))
#define ULONG_LE(b) (b[0] + (b[1] << 8) + (b[2] << 16) + (b[3] << 24))

int
bmp_scan_file(struct bmp_info *info, rust_input_handle_t file)
{
    unsigned char buf[DIB_HEADER_SIZE_MAX+4];
    unsigned char *p;
    long offset, hsize;
    long psize; /* Bytes per palette color: 3 for OS2, 4 for Win */
    unsigned short bit_count; /* Bits per pix */
    int num_palette;

    p = buf;

    ttstub_input_seek (file, 0, SEEK_SET);

    if (ttstub_input_read (file, buf, DIB_FILE_HEADER_SIZE + 4) != DIB_FILE_HEADER_SIZE + 4)
        return -1;

    if (p[0] != 'B' || p[1] != 'M')
        return -1;

    p += 6;

    if (ULONG_LE(p) != 0)
        return -1; /* not a BMP file after all? */

    p += 4;
    offset = ULONG_LE(p);
    p += 4;

    /* info header */

    hsize = ULONG_LE(p);
    p += 4;

    if (ttstub_input_read (file, p, hsize - 4) != hsize - 4)
        return -1;

    if (hsize == DIB_CORE_HEADER_SIZE) {
        info->width = USHORT_LE(p);
        p += 2;
        info->height = USHORT_LE(p);
        p += 2;
        info->xdpi = 72.0; /* assume 72 DPI */
        info->ydpi = 72.0; /* assume 72 DPI */
        if (USHORT_LE(p) != 1)
            return -1; /* unknown bcPlanes value */
        p += 2;
        bit_count = USHORT_LE(p);
        p += 2;
        psize = 3;
    } else if (hsize == DIB_INFO_HEADER_SIZE || hsize == DIB_INFO_HEADER_SIZE2 ||
               hsize == DIB_INFO_HEADER_SIZE4 || hsize == DIB_INFO_HEADER_SIZE5) {
        info->width = ULONG_LE(p);
        p += 4;
        info->height = ULONG_LE(p);
        if (info->height < 0)
            info->height = -info->height;
        p += 4;
        if (USHORT_LE(p) != 1)
            return -1; /* unknown biPlanes value */
        p += 2;
        bit_count = USHORT_LE(p);
        p += 10; /* ignore compression, biSizeImage */
        info->xdpi = ULONG_LE(p) * 0.0254; /* pixels per meter => DPI */
        p += 4;
        info->ydpi = ULONG_LE(p) * 0.0254;
        p += 4;
        psize = 4;
    } else {
        return -1; /* unexpected header */
    }

    if (bit_count < 24) {
        if (bit_count != 1 && bit_count != 4 && bit_count != 8)
            return -1; /* unexpected palette size */

        num_palette = (offset - hsize - DIB_FILE_HEADER_SIZE) / psize;
        info->bits_per_component = bit_count;
        info->num_components = 1;
    } else if (bit_count == 24) { /* full color */
        num_palette = 1; /* dummy */
        info->bits_per_component = 8;
        info->num_components = 3;
    } else {
        return -1;
    }

    if (info->width == 0 || info->height == 0 || num_palette < 1)
        return -1;

    return 0;
}
