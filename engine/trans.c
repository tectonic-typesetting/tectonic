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

#include "trans.h"

void make_identity(transform* t)
{
	t->a = 1.0;
	t->b = 0.0;
	t->c = 0.0;
	t->d = 1.0;
	t->x = 0.0;
	t->y = 0.0;
}

void make_scale(transform* t, double xscale, double yscale)
{
	t->a = xscale;
	t->b = 0.0;
	t->c = 0.0;
	t->d = yscale;
	t->x = 0.0;
	t->y = 0.0;
}

void make_translation(transform* t, double dx, double dy)
{
	t->a = 1.0;
	t->b = 0.0;
	t->c = 0.0;
	t->d = 1.0;
	t->x = dx;
	t->y = dy;
}
#include <stdio.h>
void make_rotation(transform* t, double a)
{
	t->a = cos(a);
	t->b = sin(a);
	t->c = -sin(a);
	t->d = cos(a);
	t->x = 0.0;
	t->y = 0.0;
}

void transform_point(real_point* p, const transform* t)
{
	real_point	r;
	r.x = t->a * p->x + t->c * p->y + t->x;
	r.y = t->b * p->x + t->d * p->y + t->y;
	*p = r;
}

void transform_concat(transform* t1, const transform* t2)
{
	transform	r;
	r.a = t1->a * t2->a + t1->b * t2->c + 0.0 * t2->x;
	r.b = t1->a * t2->b + t1->b * t2->d + 0.0 * t2->y;
	r.c = t1->c * t2->a + t1->d * t2->c + 0.0 * t2->x;
	r.d = t1->c * t2->b + t1->d * t2->d + 0.0 * t2->y;
	r.x = t1->x * t2->a + t1->y * t2->c + 1.0 * t2->x;
	r.y = t1->x * t2->b + t1->y * t2->d + 1.0 * t2->y;
	*t1 = r;
}
