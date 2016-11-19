/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009, 2011 by Jonathan Kew

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

#include <w2c/config.h>

#include "XeTeX_ext.h"

#include "pdfimage.h"

#include "PDFDoc.h"
#include "Catalog.h"
#include "Page.h"

#include "XeTeX_ext.h"

/* use our own fmin function because it seems to be missing on certain platforms */
inline double
my_fmin(double x, double y)
{
	return (x < y) ? x : y;
}

int
pdf_get_rect(char* filename, int page_num, int pdf_box, real_rect* box)
	/* return the box converted to TeX points */
{
	GooString*	name = new GooString(filename);
	PDFDoc*		doc = new PDFDoc(name);

	if (!doc) {
		delete name;
		return -1;
	}

	/* if the doc got created, it now owns name, so we mustn't delete it! */

	if (!doc->isOk()) {
		delete doc;
		return -1;
	}

	int			pages = doc->getNumPages();
	if (page_num > pages)
		page_num = pages;
	if (page_num < 0)
		page_num = pages + 1 + page_num;
	if (page_num < 1)
		page_num = 1;

	Page*		page = doc->getCatalog()->getPage(page_num);

	PDFRectangle*	r;
	switch (pdf_box) {
		default:
		case pdfbox_crop:
			r = page->getCropBox();
			break;
		case pdfbox_media:
			r = page->getMediaBox();
			break;
		case pdfbox_bleed:
			r = page->getBleedBox();
			break;
		case pdfbox_trim:
			r = page->getTrimBox();
			break;
		case pdfbox_art:
			r = page->getArtBox();
			break;
	}

	box->x  = 72.27 / 72 * my_fmin(r->x1, r->x2);
	box->y  = 72.27 / 72 * my_fmin(r->y1, r->y2);
	box->wd = 72.27 / 72 * fabs(r->x2 - r->x1);
	box->ht = 72.27 / 72 * fabs(r->y2 - r->y1);

	delete doc;

	return 0;
}

int
pdf_count_pages(char* filename)
{
	int			pages = 0;
	GooString*	name = new GooString(filename);
	PDFDoc*		doc = new PDFDoc(name);

	if (!doc) {
		delete name;
		return 0;
	}

	/* if the doc got created, it now owns name, so we mustn't delete it! */

	if (doc->isOk())
		pages = doc->getNumPages();

	delete doc;

	return pages;
}
