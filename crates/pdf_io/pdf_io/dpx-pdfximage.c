/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include "dpx-pdfximage.h"

#include <fcntl.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-bmpimage.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-epdf.h"
#include "dpx-error.h"
#include "dpx-jpegimage.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-pngimage.h"

static int check_for_ps (rust_input_handle_t handle);


#define IMAGE_TYPE_UNKNOWN -1
#define IMAGE_TYPE_PDF      0
#define IMAGE_TYPE_JPEG     1
#define IMAGE_TYPE_PNG      2
#define IMAGE_TYPE_EPS      5
#define IMAGE_TYPE_BMP      6
#define IMAGE_TYPE_JP2      7


struct attr_
{
    int      width, height;
    double   xdensity, ydensity;
    pdf_rect bbox;

    /* Not appropriate place but... someone need them. */
    int      page_no;
    int      page_count;
    int      bbox_type;  /* Ugh */
    pdf_obj *dict;
    char     tempfile;
};

struct pdf_ximage_
{
    char        *ident;
    char         res_name[16];

    int          subtype;

    struct attr_ attr;

    char        *filename;
    pdf_obj     *reference;
    pdf_obj     *resource;

};


/* verbose, verbose, verbose... */
struct opt_
{
    char  *cmdtmpl;
};

static struct opt_ _opts = {
    NULL
};

struct ic_
{
    int         count, capacity;
    pdf_ximage *ximages;
};

static struct ic_  _ic = {
    0, 0, NULL
};

static void
pdf_init_ximage_struct (pdf_ximage *I)
{
    I->ident    = NULL;
    I->filename = NULL;

    I->subtype  = -1;
    memset(I->res_name, 0, 16);
    I->reference = NULL;
    I->resource  = NULL;

    I->attr.width = I->attr.height = 0;
    I->attr.xdensity = I->attr.ydensity = 1.0;
    I->attr.bbox.llx = I->attr.bbox.lly = 0;
    I->attr.bbox.urx = I->attr.bbox.ury = 0;

    I->attr.page_no    = 1;
    I->attr.page_count = 1;
    I->attr.bbox_type  = 0;

    I->attr.dict     = NULL;
    I->attr.tempfile = 0;
}

static void
pdf_clean_ximage_struct (pdf_ximage *I)
{
    free(I->ident);
    free(I->filename);
    pdf_release_obj(I->reference);
    pdf_release_obj(I->resource);/* unsafe? */
    pdf_release_obj(I->attr.dict);
    pdf_init_ximage_struct(I);
}


void
pdf_init_images (void)
{
    struct ic_ *ic = &_ic;
    ic->count    = 0;
    ic->capacity = 0;
    ic->ximages  = NULL;
}

void
pdf_close_images (void)
{
    struct ic_ *ic = &_ic;
    if (ic->ximages) {
        int  i;
        for (i = 0; i < ic->count; i++) {
            pdf_ximage *I = ic->ximages+i;
            if (I->attr.tempfile) {
                /*
                 * It is important to remove temporary files at the end because
                 * we cache file names. Since we use mkstemp to create them, we
                 * might get the same file name again if we delete the first file.
                 * (This happens on NetBSD, reported by Jukka Salmi.)
                 * We also use this to convert a PS file only once if multiple
                 * pages are imported from that file.
                 */
                if (dpx_conf.verbose_level > 1 && dpx_conf.file.keep_cache != 1)
                    dpx_message("pdf_image>> deleting temporary file \"%s\"\n", I->filename);
                dpx_delete_temp_file(I->filename, false); /* temporary filename freed here */
                I->filename = NULL;
            }
            pdf_clean_ximage_struct(I);
        }
        ic->ximages = mfree(ic->ximages);
        ic->count = ic->capacity = 0;
    }

    _opts.cmdtmpl = mfree(_opts.cmdtmpl);
}

static int
source_image_type (rust_input_handle_t handle)
{
    int format = IMAGE_TYPE_UNKNOWN;

    ttstub_input_seek(handle, 0, SEEK_SET);

    /* Original check order: jpeg, jp2, png, bmp, pdf, ps */

    if (check_for_jpeg(handle))
        format = IMAGE_TYPE_JPEG;
    /* else if (check_for_jp2(fp))
     *    format = IMAGE_TYPE_JP2; */
    else if (check_for_png(handle))
        format = IMAGE_TYPE_PNG;
    else if (check_for_bmp(handle))
        format = IMAGE_TYPE_BMP;
    else if (check_for_pdf(handle))
        format = IMAGE_TYPE_PDF;
    else if (check_for_ps(handle))
        format = IMAGE_TYPE_EPS;
    else {
        dpx_warning("Tectonic was unable to detect an image's format");
        format = IMAGE_TYPE_UNKNOWN;
    }

    ttstub_input_seek(handle, 0, SEEK_SET);
    return format;
}

static int
load_image (const char *ident, const char *fullname, int format, rust_input_handle_t handle,
            load_options options)
{
    struct ic_ *ic = &_ic;
    int id = -1;
    pdf_ximage *I;

    id = ic->count;
    if (ic->count >= ic->capacity) {
        ic->capacity += 16;
        ic->ximages = RENEW(ic->ximages, ic->capacity, pdf_ximage);
    }

    I  = &ic->ximages[id];
    pdf_init_ximage_struct(I);
    if (ident) {
        I->ident = NEW(strlen(ident)+1, char);
        strcpy(I->ident, ident);
    }
    if (fullname) {
        I->filename = NEW(strlen(fullname)+1, char);
        strcpy(I->filename, fullname);
    }

    I->attr.page_no = options.page_no;
    I->attr.bbox_type = options.bbox_type;
    I->attr.dict = options.dict; /* unsafe? */

    switch (format) {
    case IMAGE_TYPE_JPEG:
        if (dpx_conf.verbose_level > 0)
            dpx_message("[JPEG]");
        if (jpeg_include_image(I, handle) < 0)
            goto error;
        I->subtype = PDF_XOBJECT_TYPE_IMAGE;
        break;
    case IMAGE_TYPE_JP2:
        if (dpx_conf.verbose_level > 0)
            dpx_message("[JP2]");
        /*if (jp2_include_image(I, fp) < 0)*/
        dpx_warning("Tectonic: JP2 not yet supported");
        goto error;
        /*I->subtype = PDF_XOBJECT_TYPE_IMAGE;
          break;*/
    case IMAGE_TYPE_PNG:
        if (dpx_conf.verbose_level > 0)
            dpx_message("[PNG]");
        if (png_include_image(I, handle) < 0)
            goto error;
        I->subtype = PDF_XOBJECT_TYPE_IMAGE;
        break;
    case IMAGE_TYPE_BMP:
        if (dpx_conf.verbose_level > 0)
            dpx_message("[BMP]");
        if (bmp_include_image(I, handle) < 0)
            goto error;
        I->subtype = PDF_XOBJECT_TYPE_IMAGE;
        break;
    case IMAGE_TYPE_PDF:
        if (dpx_conf.verbose_level > 0)
            dpx_message("[PDF]");
        {
            int result = pdf_include_page(I, handle, fullname, options);
            /* Tectonic: this used to try ps_include_page() */
            if (result != 0)
                goto error;
        }
        if (dpx_conf.verbose_level > 0)
            dpx_message(",Page:%d", I->attr.page_no);
        I->subtype  = PDF_XOBJECT_TYPE_FORM;
        break;
    case IMAGE_TYPE_EPS:
        if (dpx_conf.verbose_level > 0)
            dpx_message("[EPS]");
        dpx_warning("sorry, PostScript images are not supported by Tectonic");
        dpx_warning("for details, please see https://github.com/tectonic-typesetting/tectonic/issues/27");
        goto error;
    default:
        if (dpx_conf.verbose_level > 0)
            dpx_message("[UNKNOWN]");
        /* Tectonic: this used to try ps_include_page() */
        goto error;
    }

    switch (I->subtype) {
    case PDF_XOBJECT_TYPE_IMAGE:
        sprintf(I->res_name, "Im%d", id);
        break;
    case PDF_XOBJECT_TYPE_FORM:
        sprintf(I->res_name, "Fm%d", id);
        break;
    default:
        _tt_abort("Unknown XObject subtype: %d", I->subtype);
    }

    ic->count++;
    return id;

error:
    pdf_clean_ximage_struct(I);
    return -1;
}

int
pdf_ximage_findresource (const char *ident, load_options options)
{
    struct ic_ *ic = &_ic;
    int id = -1;
    pdf_ximage *I;
    int format;
    rust_input_handle_t handle;

    /* "I don't understand why there is comparision against I->attr.dict here...
     * I->attr.dict and options.dict are simply pointers to PDF dictionaries."
     */
    for (id = 0; id < ic->count; id++) {
        I = &ic->ximages[id];
        if (I->ident && streq_ptr(ident, I->ident)) {
            if (I->attr.page_no == options.page_no /* Not sure */
                && I->attr.dict == options.dict    /* ????? */
                && I->attr.bbox_type == options.bbox_type) {
                return id;
            }
        }
    }

    /* This happens if we've already inserted the image into the PDF output.
     * In my one test case, it seems to just work to plunge along merrily
     * ahead ...
     *
     * if (f) {
     *   <"we already have converted this file; f is the temporary file name">
     *   fullname = NEW(strlen(f)+1, char);
     *   strcpy(fullname, f);
     * } else { kpse_find_file() }
     */

    handle = ttstub_input_open(ident, TTBC_FILE_FORMAT_PICT, 0);
    if (handle == NULL) {
        dpx_warning("Error locating image file \"%s\"", ident);
        return -1;
    }

    if (dpx_conf.verbose_level > 0)
        dpx_message("(Image:%s", ident);

    format = source_image_type(handle);
    id = load_image(ident, ident, format, handle, options);

    ttstub_input_close(handle);

    if (dpx_conf.verbose_level > 0)
        dpx_message(")");

    if (id < 0)
        dpx_warning("pdf: image inclusion failed for \"%s\".", ident);

    return id;
}

/* Reference: PDF Reference 1.5 v6, pp.321--322
 *
 * TABLE 4.42 Additional entries specific to a type 1 form dictionary
 *
 * BBox rectangle (Required) An array of four numbers in the form coordinate
 *                system, giving the coordinates of the left, bottom, right,
 *                and top edges, respectively, of the form XObject's bounding
 *                box. These boundaries are used to clip the form XObject and
 *                to determine its size for caching.
 *
 * Matrix array   (Optional) An array of six numbers specifying the form
 *                matrix, which maps form space into user space.
 *                Default value: the identity matrix [1 0 0 1 0 0].
 */
void
pdf_ximage_init_form_info (xform_info *info)
{
    info->flags    = 0;
    info->bbox.llx = 0;
    info->bbox.lly = 0;
    info->bbox.urx = 0;
    info->bbox.ury = 0;
    info->matrix.a = 1.0;
    info->matrix.b = 0.0;
    info->matrix.c = 0.0;
    info->matrix.d = 1.0;
    info->matrix.e = 0.0;
    info->matrix.f = 0.0;
}

/* Reference: PDF Reference 1.5 v6, pp.303--306
 *
 * TABLE 4.42 Additional entries specific to an image dictionary
 *
 * Width integer  (Required) The width of the image, in samples.
 *
 * Height integer (Required) The height of the image, in samples.
 *
 * ColorSpace name or array
 *                (Required for images, except those that use the JPXDecode
 *                filter; not allowed for image masks) The color space in
 *                which image samples are specified. This may be any type
 *                of color space except Patter.
 *
 *                If the image uses the JPXDecode filter, this entry is
 *                optional.
 *
 * BitsPerComponent integer
 *                (Required except for image masks and images that use the
 *                JPXDecode filter) The number of bits used to represent
 *                each color component. Only a single value may be specified;
 *                the number of bits is the same for all color components.
 *                Valid values are 1,2,4,8, and (in PDF1.5) 16. If ImageMask
 *                is true, this entry is optional, and if speficified, its
 *                value must be 1.
 *
 *                If the image stream uses the JPXDecode filter, this entry
 *                is optional and ignored if present. The bit depth is
 *                determined in the process of decoding the JPEG2000 image.
 */
void
pdf_ximage_init_image_info (ximage_info *info)
{
    info->flags  = 0;
    info->width  = 0;
    info->height = 0;
    info->bits_per_component = 0;
    info->num_components = 0;
    info->min_dpi = 0;
    info->xdensity = info->ydensity = 1.0;
}

void
pdf_ximage_set_image (pdf_ximage *I, void *image_info, pdf_obj *resource)
{
    pdf_obj     *dict;
    ximage_info *info = image_info;

    if (!PDF_OBJ_STREAMTYPE(resource))
        _tt_abort("Image XObject must be of stream type.");

    I->subtype = PDF_XOBJECT_TYPE_IMAGE;

    I->attr.width  = info->width;  /* The width of the image, in samples */
    I->attr.height = info->height; /* The height of the image, in samples */
    I->attr.xdensity = info->xdensity;
    I->attr.ydensity = info->ydensity;

    I->reference = pdf_ref_obj(resource);

    dict = pdf_stream_dict(resource);
    pdf_add_dict(dict, pdf_new_name("Type"),    pdf_new_name("XObject"));
    pdf_add_dict(dict, pdf_new_name("Subtype"), pdf_new_name("Image"));
    pdf_add_dict(dict, pdf_new_name("Width"),   pdf_new_number(info->width));
    pdf_add_dict(dict, pdf_new_name("Height"),  pdf_new_number(info->height));
    if (info->bits_per_component > 0) /* Ignored for JPXDecode filter. FIXME */
        pdf_add_dict(dict, pdf_new_name("BitsPerComponent"),
                     pdf_new_number(info->bits_per_component));
    if (I->attr.dict)
        pdf_merge_dict(dict, I->attr.dict);

    pdf_release_obj(resource); /* Caller don't know we are using reference. */
    I->resource  = NULL;
}

void
pdf_ximage_set_form (pdf_ximage *I, void *form_info, pdf_obj *resource)
{
    xform_info *info = form_info;
    pdf_coord p1, p2, p3, p4;

    I->subtype   = PDF_XOBJECT_TYPE_FORM;

    /* Image's attribute "bbox" here is affected by /Rotate entry of included
     * PDF page.
     */
    p1.x = info->bbox.llx; p1.y = info->bbox.lly;
    pdf_dev_transform(&p1, &info->matrix);
    p2.x = info->bbox.urx; p2.y = info->bbox.lly;
    pdf_dev_transform(&p2, &info->matrix);
    p3.x = info->bbox.urx; p3.y = info->bbox.ury;
    pdf_dev_transform(&p3, &info->matrix);
    p4.x = info->bbox.llx; p4.y = info->bbox.ury;
    pdf_dev_transform(&p4, &info->matrix);

    I->attr.bbox.llx = min4(p1.x, p2.x, p3.x, p4.x);
    I->attr.bbox.lly = min4(p1.y, p2.y, p3.y, p4.y);
    I->attr.bbox.urx = max4(p1.x, p2.x, p3.x, p4.x);
    I->attr.bbox.ury = max4(p1.y, p2.y, p3.y, p4.y);

    I->reference = pdf_ref_obj(resource);

    pdf_release_obj(resource); /* Caller don't know we are using reference. */
    I->resource  = NULL;
}

int
pdf_ximage_get_page (pdf_ximage *I)
{
    return I->attr.page_no;
}

#define CHECK_ID(c,n) do {                              \
        if ((n) < 0 || (n) >= (c)->count) {             \
            _tt_abort("Invalid XObject ID: %d", (n));       \
        }                                               \
    } while (0)
#define GET_IMAGE(c,n) (&((c)->ximages[(n)]))

pdf_obj *
pdf_ximage_get_reference (int id)
{
    struct ic_ *ic = &_ic;
    pdf_ximage *I;

    CHECK_ID(ic, id);

    I = GET_IMAGE(ic, id);
    if (!I->reference)
        I->reference = pdf_ref_obj(I->resource);

    return pdf_link_obj(I->reference);
}

/* called from pdfdoc.c only for late binding */
int
pdf_ximage_defineresource (const char *ident,
                           int subtype, void *info, pdf_obj *resource)
{
    struct ic_ *ic = &_ic;
    int         id;
    pdf_ximage *I;

    id = ic->count;
    if (ic->count >= ic->capacity) {
        ic->capacity += 16;
        ic->ximages   = RENEW(ic->ximages, ic->capacity, pdf_ximage);
    }

    I = &ic->ximages[id];

    pdf_init_ximage_struct(I);

    if (ident) {
        I->ident = NEW(strlen(ident)+1, char);
        strcpy(I->ident, ident);
    }

    switch (subtype) {
    case PDF_XOBJECT_TYPE_IMAGE:
        pdf_ximage_set_image(I, info, resource);
        sprintf(I->res_name, "Im%d", id);
        break;
    case PDF_XOBJECT_TYPE_FORM:
        pdf_ximage_set_form (I, info, resource);
        sprintf(I->res_name, "Fm%d", id);
        break;
    default:
        _tt_abort("Unknown XObject subtype: %d", subtype);
    }
    ic->count++;

    return  id;
}


char *
pdf_ximage_get_resname (int id)
{
    struct ic_ *ic = &_ic;
    pdf_ximage *I;

    CHECK_ID(ic, id);

    I = GET_IMAGE(ic, id);

    return I->res_name;
}

int
pdf_ximage_get_subtype (int id)
{
    struct ic_ *ic = &_ic;
    pdf_ximage *I;

    CHECK_ID(ic, id);

    I = GET_IMAGE(ic, id);

    return I->subtype;
}

void
pdf_ximage_set_attr (int id, int width, int height, double xdensity, double ydensity, double llx, double lly, double urx, double ury)
{
    struct ic_ *ic = &_ic;
    pdf_ximage *I;

    CHECK_ID(ic, id);

    I = GET_IMAGE(ic, id);
    I->attr.width = width;
    I->attr.height = height;
    I->attr.xdensity = xdensity;
    I->attr.ydensity = ydensity;
    I->attr.bbox.llx = llx;
    I->attr.bbox.lly = lly;
    I->attr.bbox.urx = urx;
    I->attr.bbox.ury = ury;
}

/* depth...
 * Dvipdfm treat "depth" as "yoffset" for pdf:image and pdf:uxobj
 * not as vertical dimension of scaled image. (And there are bugs.)
 * This part contains incompatibile behaviour than dvipdfm!
 */
#define EBB_DPI 72

static void
scale_to_fit_I (pdf_tmatrix    *T,
                transform_info *p,
                pdf_ximage     *I)
{
    double  s_x, s_y, d_x, d_y;
    double  wd0, ht0, dp, xscale, yscale;

    if (p->flags & INFO_HAS_USER_BBOX) {
        wd0 =  p->bbox.urx - p->bbox.llx;
        ht0 =  p->bbox.ury - p->bbox.lly;
        xscale = I->attr.width * I->attr.xdensity / wd0;
        yscale = I->attr.height * I->attr.ydensity / ht0;
        d_x = -p->bbox.llx / wd0;
        d_y = -p->bbox.lly / ht0;
    } else {
        wd0 = I->attr.width * I->attr.xdensity;
        ht0 = I->attr.height * I->attr.ydensity;
        xscale = yscale = 1.0;
        d_x = 0.0;
        d_y = 0.0;
    }

    if (wd0 == 0.0) {
        dpx_warning("Image width=0.0!");
        wd0 = 1.0;
    }
    if (ht0 == 0.0) {
        dpx_warning("Image height=0.0!");
        ht0 = 1.0;
    }

    if ( (p->flags & INFO_HAS_WIDTH ) &&
         (p->flags & INFO_HAS_HEIGHT) ) {
        s_x = p->width * xscale;
        s_y = (p->height + p->depth) * yscale;
        dp  = p->depth * yscale;
    } else if ( p->flags & INFO_HAS_WIDTH ) {
        s_x = p->width * xscale;
        s_y = s_x * ((double)I->attr.height / I->attr.width);
        dp  = 0.0;
    } else if ( p->flags & INFO_HAS_HEIGHT) {
        s_y = (p->height + p->depth) * yscale;
        s_x = s_y * ((double)I->attr.width / I->attr.height);
        dp  = p->depth * yscale;
    } else {
        s_x = wd0;
        s_y = ht0;
        dp  = 0.0;
    }
    T->a = s_x; T->c = 0.0;
    T->b = 0.0; T->d = s_y;
    T->e = d_x * s_x / xscale; T->f = d_y * s_y / yscale - dp;

    return;
}


static void
scale_to_fit_F (pdf_tmatrix    *T,
                transform_info *p,
                pdf_ximage     *I)
{
    double  s_x, s_y, d_x, d_y;
    double  wd0, ht0, dp;

    if (p->flags & INFO_HAS_USER_BBOX) {
        wd0 =  p->bbox.urx - p->bbox.llx;
        ht0 =  p->bbox.ury - p->bbox.lly;
        d_x = -p->bbox.llx;
        d_y = -p->bbox.lly;
    } else {
        wd0 = I->attr.bbox.urx - I->attr.bbox.llx;
        ht0 = I->attr.bbox.ury - I->attr.bbox.lly;
        d_x = 0.0;
        d_y = 0.0;
    }

    if (wd0 == 0.0) {
        dpx_warning("Image width=0.0!");
        wd0 = 1.0;
    }
    if (ht0 == 0.0) {
        dpx_warning("Image height=0.0!");
        ht0 = 1.0;
    }

    if ( (p->flags & INFO_HAS_WIDTH ) &&
         (p->flags & INFO_HAS_HEIGHT) ) {
        s_x = p->width  / wd0;
        s_y = (p->height + p->depth) / ht0;
        dp  = p->depth;
    } else if ( p->flags & INFO_HAS_WIDTH ) {
        s_x = p->width  / wd0;
        s_y = s_x;
        dp  = 0.0;
    } else if ( p->flags & INFO_HAS_HEIGHT) {
        s_y = (p->height + p->depth) / ht0;
        s_x = s_y;
        dp  = p->depth;
    } else {
        s_x = s_y = 1.0;
        dp  = 0.0;
    }

    T->a = s_x; T->c = 0.0;
    T->b = 0.0; T->d = s_y;
    T->e = s_x * d_x; T->f = s_y * d_y - dp;

    return;
}


/* called from pdfdev.c and spc_html.c */
int
pdf_ximage_scale_image (int            id,
                        pdf_tmatrix    *M, /* return value for trans matrix */
                        pdf_rect       *r, /* return value for clipping */
                        transform_info *p  /* argument from specials */
    )
{
    struct ic_ *ic = &_ic;
    pdf_ximage *I;

    CHECK_ID(ic, id);

    I = GET_IMAGE(ic, id);

    pdf_setmatrix(M, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

    switch (I->subtype) {
        /* Reference: PDF Reference 1.5 v6, p.302
         *
         * An image can be placed on the output page in any desired position,
         * orientation, and size by using the cm operator to modify the current
         * transformation matrix (CTM) so as to map the unit square of user space
         * to the rectangle or parallelogram in which the image is to be painted.
         *
         * There is neither BBox nor Matrix key in the image XObject.
         * Everything must be controlled by the cm operator.
         *
         * The argument [p] contains the user-defined bounding box, the scailing
         * factor of which is bp as EPS and PDF. On the other hand, I->attr
         * contains the (sampling) width and the (sampling) height of the image.
         *
         * There is no problem if a bitmap image has density information.
         * Otherwise, DVIPDFM's ebb generates bounding box as 100px = 72bp = 1in.
         * In this case, screen captured images look bad. Moreover, DVIPDFM's ebb
         * ignores all density information and use just 100px = 72bp = 1in.
         *
         * On the other hand, pdfTeX uses 100px = 100bp to get a better quality
         * for screen captured images.
         *
         * DVIPDFMx's xbb generates bounding box as 100px = 100bp in the same
         * way as pdfTeX. Furthermore, it takes care of density information too.
         */
    case PDF_XOBJECT_TYPE_IMAGE:
        scale_to_fit_I(M, p, I);
        if (p->flags & INFO_HAS_USER_BBOX) {
            r->llx = p->bbox.llx / (I->attr.width * I->attr.xdensity);
            r->lly = p->bbox.lly / (I->attr.height * I->attr.ydensity);
            r->urx = p->bbox.urx / (I->attr.width * I->attr.xdensity);
            r->ury = p->bbox.ury / (I->attr.height * I->attr.ydensity);
        } else {
            r->llx = 0.0;
            r->lly = 0.0;
            r->urx = 1.0;
            r->ury = 1.0;
        }
        break;
        /* User-defined transformation and clipping are controlled by
         * the cm operator and W operator, explicitly */
    case PDF_XOBJECT_TYPE_FORM:
        scale_to_fit_F(M, p, I);
        if (p->flags & INFO_HAS_USER_BBOX) {
            r->llx = p->bbox.llx;
            r->lly = p->bbox.lly;
            r->urx = p->bbox.urx;
            r->ury = p->bbox.ury;
        } else { /* I->attr.bbox from the image bounding box */
            r->llx = I->attr.bbox.llx;
            r->lly = I->attr.bbox.lly;
            r->urx = I->attr.bbox.urx;
            r->ury = I->attr.bbox.ury;
        }
        break;
    }

    return  0;
}


/* Migrated from psimage.c */

void set_distiller_template (char *s)
{
    free(_opts.cmdtmpl);
    if (!s || *s == '\0')
        _opts.cmdtmpl = NULL;
    else {
        _opts.cmdtmpl = NEW(strlen(s) + 1, char);
        strcpy(_opts.cmdtmpl, s);
    }
    return;
}

char *get_distiller_template (void)
{
    return _opts.cmdtmpl;
}

static int
check_for_ps (rust_input_handle_t handle)
{
    ttstub_input_seek(handle, 0, SEEK_SET);
    tt_mfgets (work_buffer, WORK_BUFFER_SIZE, handle);
    if (strstartswith(work_buffer, "%!"))
        return 1;
    return 0;
}
