/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew

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

/*
XeTeX_pic.c
   interface between xetex and graphics files
   only needs to get image dimensions, not actually load/process the file
*/

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/xetexd.h>
#include <tectonic/XeTeX_ext.h>
#include <tectonic/stubs.h>
#include <tectonic/pngimage.h>
#include <tectonic/jpegimage.h>
#include <tectonic/bmpimage.h>


int
count_pdf_file_pages (void)
{
    int pages;
    rust_input_handle_t handle;

    handle = ttstub_input_open (name_of_file + 1, kpse_pict_format, 0);
    if (handle == NULL)
	return 0;

    pages = pdf_count_pages(handle);
    ttstub_input_close(handle);
    return pages;
}


/*
  pdfBoxType indicates which pdf bounding box to use (0 for \XeTeXpicfile)
  page indicates which page is wanted (0-based)
  return 0 for success, or non-zero error code for failure
  return full path in *path
  return bounds (tex points) in *bounds
*/
int
find_pic_file (char **path, real_rect *bounds, int pdfBoxType, int page)
{
    char *in_path = (char *) name_of_file + 1;
    int err = -1;
    rust_input_handle_t handle;

    handle = ttstub_input_open (in_path, kpse_pict_format, 0);
    bounds->x = bounds->y = bounds->wd = bounds->ht = 0.0;

    if (handle == NULL)
	goto done;

    /* if cmd was \XeTeXpdffile, use xpdflib to read it */
    if (pdfBoxType != 0) {
	err = pdf_get_rect (handle, page, pdfBoxType, bounds);
	goto done;
    }

    /* otherwise try graphics formats that we know */
    if (tt_check_for_jpeg(handle)) {
	struct jpeg_info info;

	err = jpeg_scan_file(&info, handle);
	if (err == 0) {
	    bounds->wd = (info.width * 72.27) / info.xdpi;
	    bounds->ht = (info.height * 72.27) / info.ydpi;
	}
	goto done;
    }

    if (tt_check_for_bmp(handle)) {
	struct bmp_info	info;

	err = bmp_scan_file(&info, handle);
	if (err == 0) {
	    bounds->wd = (info.width * 72.27) / info.xdpi;
	    bounds->ht = (info.height * 72.27) / info.ydpi;
	}
	goto done;
    }

    if (tt_check_for_png(handle)) {
	struct png_info	info;
	err = png_scan_file(&info, handle);
	if (err == 0) {
	    bounds->wd = (info.width * 72.27) / info.xdpi;
	    bounds->ht = (info.height * 72.27) / info.ydpi;
	}
	goto done;
    }

    /* could support other file types here (TIFF, WMF, etc?) */

done:
    if (handle != NULL)
	ttstub_input_close (handle);

    if (err == 0)
	*path = xstrdup(in_path);

    return err;
}
