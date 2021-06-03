/* synctex.h

Copyright (c) 2008, 2009 jerome DOT laurens AT u-bourgogne DOT fr

This file is part of the SyncTeX package.

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE

Acknowledgments:
----------------
The author received useful remarks from the pdfTeX developers, especially Hahn The Thanh,
and significant help from XeTeX developer Jonathan Kew

Nota Bene:
----------
If you include or use a significant part of the synctex package into a software,
I would appreciate to be listed as contributor and see "SyncTeX" highlighted.

Version 1
Latest Revision: Wed Jul  1 08:17:50 UTC 2009

*/

#  ifndef __SYNCTEX_HEADER__
#    define __SYNCTEX_HEADER__

/*  Send this message to init the synctex command value to the command line option.
 *  Sending this message too early will cause a bus error.  */
void synctex_init_command(void);

/*  Send this message to clean memory, and close the file.  */
void synctex_terminate(bool log_opened);

/*  Free all memory used, close and remove the file if any. */
void synctex_abort(bool log_opened);

/*  Send this message when starting a new input.  */
void synctex_start_input(void);

/*  Recording the "{..." line.  In *tex.web, use synctex_sheet(pdf_output) at
 *  the very beginning of the ship_out procedure.
*/
void synctex_sheet(int32_t mag);

/*  Recording the "}..." line.  In *tex.web, use synctex_teehs at
 *  the very end of the ship_out procedure.
*/
void synctex_teehs(void);
 
/*  Recording the "<..." line.  In pdftex.web, use synctex_pdfxform(p) at
 *  the very beginning of the pdf_ship_out procedure.
 */
void synctex_pdfxform(int32_t p);

/*  Recording the ">" line.  In pdftex.web, use synctex_mrofxfdp at
 *  the very end of the ship_out procedure.
 */
void synctex_mrofxfdp(void);

/*  This message is sent when a vlist will be shipped out, more precisely at
 *  the beginning of the vlist_out procedure in *TeX.web.  It will be balanced
 *  by a synctex_tsilv, sent at the end of the vlist_out procedure.  p is the
 *  address of the vlist We assume that p is really a vlist node! */
void synctex_vlist(int32_t this_box);

/*  Recording a "}" line ending a vbox: this message is sent whenever a vlist
 *  has been shipped out. It is used to close the vlist nesting level. It is
 *  sent at the end of each vlist_out procedure in *TeX.web to balance a former
 *  synctex_vlist sent at the beginning of that procedure.    */
void synctex_tsilv(int32_t this_box);

/*  This message is sent when a void vlist will be shipped out.
 *  There is no need to balance a void vlist.  */
void synctex_void_vlist(int32_t p, int32_t this_box);

/*  Send this message when an hlist will be shipped out, more precisely at
 *  the beginning of the hlist_out procedure in *TeX.web.  It must be balanced
 *  by a synctex_tsilh, sent at the end of the hlist_out procedure.  p is the
 *  address of the hlist. */
void synctex_hlist(int32_t this_box);

/*  Send this message at the end of the various hlist_out procedure in *TeX.web
 *  to balance a former synctex_hlist.    */
void synctex_tsilh(int32_t this_box);

/*  This message is sent when a void hlist will be shipped out.
 *  There is no need to balance a void hlist.  */
void synctex_void_hlist(int32_t p, int32_t this_box);

/*  Send this message whenever an inline math node will ship out. */
void synctex_math(int32_t p, int32_t this_box);

/*  Send this message whenever an horizontal rule or glue node will ship out. */
void synctex_horizontal_rule_or_glue(int32_t p, int32_t this_box);

/*  Send this message whenever a kern node will ship out. */
void synctex_kern(int32_t p, int32_t this_box);

void synctex_pdfrefxform(int objnum);

/*  For debugging purpose only    */
void synctex_current(void);

#  endif
