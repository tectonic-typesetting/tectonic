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

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/XeTeX_ext.h>

#include <stdlib.h>

#include "PDFDoc.h"
#include "Stream.h"
#include "Catalog.h"
#include "Page.h"


static PDFDoc *
_rust_to_pdfdoc (rust_input_handle_t file)
{
    /* We could do better by writing a Stream implementation for the Rust I/O
     * layer, but for now we are going to be totally lame and just snarf the
     * whole PDF file. */

    size_t sz = ttstub_input_get_size (file);

    unsigned char *buf = new unsigned char[sz];
    if (buf == NULL)
        return NULL;

    if (ttstub_input_read (file, buf, sz) != sz) {
        delete[] buf;
        return NULL;
    }

    /* XXX: are we leaking buf here? Depends on whether MemStream/PDFDoc take
     * ownership. */

    Object obj;
    obj.initNull();

    MemStream *ms = new MemStream ((char *) buf, 0, sz, &obj);

    return new PDFDoc (ms);
}


int
pdf_get_rect (rust_input_handle_t file, int page_num, int pdf_box, real_rect* box)
{
    PDFDoc *doc = _rust_to_pdfdoc (file);

    if (!doc)
        return -1;

    if (!doc->isOk ()) {
        delete doc;
        return -1;
    }

    int pages = doc->getNumPages ();
    if (page_num > pages)
        page_num = pages;
    if (page_num < 0)
        page_num = pages + 1 + page_num;
    if (page_num < 1)
        page_num = 1;

    Page *page = doc->getCatalog()->getPage(page_num);

    PDFRectangle *r;
    switch (pdf_box) {
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
    case pdfbox_crop:
    default:
        r = page->getCropBox();
        break;
    }

    box->x = 72.27 / 72 * fmin(r->x1, r->x2);
    box->y = 72.27 / 72 * fmin(r->y1, r->y2);
    box->wd = 72.27 / 72 * fabs(r->x2 - r->x1);
    box->ht = 72.27 / 72 * fabs(r->y2 - r->y1);

    delete doc;
    return 0;
}


int
pdf_count_pages (rust_input_handle_t file)
{
    PDFDoc *doc = _rust_to_pdfdoc (file);

    if (!doc)
        return -1;

    if (!doc->isOk ()) {
        delete doc;
        return -1;
    }

    int pages = doc->getNumPages ();
    delete doc;
    return pages;
}
