/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 2010-2014 by Han The Thanh

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

#include "XeTeX_web.h"

#include <map>
#include <iostream>
#include <assert.h>
using namespace std;

typedef pair<int, unsigned int> GlyphId;
typedef map<GlyphId, int>  ProtrusionFactor;
ProtrusionFactor leftProt, rightProt;

void set_cp_code(int fontNum, unsigned int code, int side, int value)
{
    GlyphId id(fontNum, code);
    switch (side) {
    case LEFT_SIDE:
        leftProt[id] = value;
        break;
    case RIGHT_SIDE:
        rightProt[id] = value;
        break;
    default:
        assert(0); // we should not reach here
    }
}

int get_cp_code(int fontNum, unsigned int code, int side)
{
    GlyphId id(fontNum, code);
    ProtrusionFactor* container;
    switch (side) {
    case LEFT_SIDE:
        container = &leftProt;
        break;
    case RIGHT_SIDE:
        container = &rightProt;
        break;
    default:
        assert(0); // we should not reach here
    }
    ProtrusionFactor::iterator it = container->find(id);
    if (it == container->end())
        return 0;
    return it->second;
}

