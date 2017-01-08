/* tectonic/pngimage.c -- PNG-format image handling
   Copyright 2016 the Tectonic Project
   Copyright 1994-2006 SIL International (SIL author: Jonathan Kew)
   Copyright 2002 Jin-Hwan Cho, Shunsaku Hirata, dvipdfmx project team
   Copyright 1998, 1999 Mark A. Wicks
   Licensed under the GPL version 2 or later.
*/

#include <tectonic/pngimage.h>
#include <tectonic/internals.h>
#include <png.h>

#define NSIGBYTES 8


static void
_png_warning_callback (png_structp png_ptr, png_const_charp msg)
{
    (void)png_ptr;
    (void)msg;
}


int
tt_check_for_png (rust_input_handle_t file)
{
    unsigned char sigbytes[NSIGBYTES];

    ttstub_input_seek (file, 0, SEEK_SET);

    if (ttstub_input_read (file, sigbytes, NSIGBYTES) != NSIGBYTES)
        return 0;

    if (png_sig_cmp (sigbytes, 0, NSIGBYTES))
        return 0;

    return 1;
}


static void
_png_read (png_structp png_ptr, png_bytep outbytes, png_size_t n)
{
    rust_input_handle_t handle = png_get_io_ptr (png_ptr);

    if (ttstub_input_read (handle, outbytes, n) != n)
	_tt_abort ("error reading PNG");
}


int
png_scan_file (struct png_info *info, rust_input_handle_t file)
{
    png_structp png_ptr;
    png_infop png_info_ptr;
    png_byte bpc;
    png_uint_32 width, height;

    ttstub_input_seek (file, 0, SEEK_SET);

    png_ptr = png_create_read_struct (PNG_LIBPNG_VER_STRING, NULL, NULL, _png_warning_callback);
    if (png_ptr == NULL)
	return -1;

    png_info_ptr = png_create_info_struct (png_ptr);
    if (png_info_ptr == NULL) {
	png_destroy_read_struct (&png_ptr, NULL, NULL);
        return -1;
    }

    png_set_read_fn (png_ptr, file, _png_read);
    /* NOTE: could use png_set_sig_bytes() to tell libpng if we started at non-zero file offset */

    png_read_info (png_ptr, png_info_ptr);
    info->width = png_get_image_width (png_ptr, png_info_ptr);
    info->height = png_get_image_height (png_ptr, png_info_ptr);
    info->bits_per_component = png_get_bit_depth (png_ptr, png_info_ptr);
    info->xdpi = png_get_x_pixels_per_meter(png_ptr, png_info_ptr) * 0.0254; /* pix per meter => DPI */
    info->ydpi = png_get_y_pixels_per_meter(png_ptr, png_info_ptr) * 0.0254;

    if (info->xdpi == 0)
        info->xdpi = 72;
    if (info->ydpi == 0)
        info->ydpi = 72;

    png_destroy_info_struct (png_ptr, &png_info_ptr);
    png_destroy_read_struct (&png_ptr, NULL, NULL);
    return 0;
}
