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

#define EXTERN extern
#include "xetexd.h"

#include "XeTeX_ext.h"

#include <tidy_kpathutil/public.h>
#include <tidy_kpathsea/public.h>

#include "pdfimage.h"
#include "image/pngimage.h"
#include "image/jpegimage.h"
#include "image/bmpimage.h"


int
count_pdf_file_pages(void)
{
	int	rval = 0;

    char*		pic_path = kpse_find_file((char*)name_of_file + 1, kpse_pict_format, 1);
	if (pic_path) {
		rval = pdf_count_pages(pic_path);
		free(pic_path);
	}

	return rval;
}


/*
	locate picture file from /nameoffile+1/ using kpathsearch
	pdfBoxType indicates which pdf bounding box to use (0 for \XeTeXpicfile)
	page indicates which page is wanted (0-based)
	return 0 for success, or non-zero error code for failure
	return full path in *path
	return bounds (tex points) in *bounds
*/
int
find_pic_file(char** path, real_rect* bounds, int pdfBoxType, int page)
{
	int		err = -1;
	FILE*	fp = NULL;
    char*	pic_path = kpse_find_file((char*)name_of_file + 1, kpse_pict_format, 1);

	*path = NULL;
	bounds->x = bounds->y = bounds->wd = bounds->ht = 0.0;

	if (pic_path == NULL)
		goto done;

	/* if cmd was \XeTeXpdffile, use xpdflib to read it */
	if (pdfBoxType != 0) {
		err = pdf_get_rect(pic_path, page, pdfBoxType, bounds);
		goto done;
	}

	/* otherwise try graphics formats that we know */
	fp = fopen(pic_path, FOPEN_RBIN_MODE);
	if (fp == NULL)
		goto done;

	if (check_for_jpeg(fp)) {
		struct JPEG_info	info;
		err = JPEG_scan_file(&info, fp);
		if (err == 0) {
			bounds->wd = (info.width * 72.27) / info.xdpi;
			bounds->ht = (info.height * 72.27) / info.ydpi;
		}
		goto done;
	}

	if (check_for_bmp(fp)) {
		struct bmp_info	info;
		err = bmp_scan_file(&info, fp);
		if (err == 0) {
			bounds->wd = (info.width * 72.27) / info.xdpi;
			bounds->ht = (info.height * 72.27) / info.ydpi;
		}
		goto done;
	}

	if (check_for_png(fp)) {
		struct png_info	info;
		err = png_scan_file(&info, fp);
		if (err == 0) {
			bounds->wd = (info.width * 72.27) / info.xdpi;
			bounds->ht = (info.height * 72.27) / info.ydpi;
		}
		goto done;
	}

	/* could support other file types here (TIFF, WMF, etc?) */

done:
	if (fp != NULL)
		fclose(fp);

	if (err == 0)
		*path = pic_path;
	else {
		if (pic_path != NULL)
			free(pic_path);
	}

	return err;
}
