#!/usr/bin/env python3
"""Generate the synthetic cmap04-only.ttf regression-test fixture.

Regenerate with:
  uv run --with fonttools==4.63.0 python tests/assets/generate_cmap04_test_font.py

The glyph outlines are simple shapes authored for this test. The resulting
font is released under the same MIT license as Tectonic.
"""

from pathlib import Path

from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib.tables._c_m_a_p import CmapSubtable


OUTPUT = Path(__file__).with_name("cmap04-only.ttf")
UNITS_PER_EM = 1000


def polygon(points):
    pen = TTGlyphPen(None)
    pen.moveTo(points[0])
    for point in points[1:]:
        pen.lineTo(point)
    pen.closePath()
    return pen.glyph()


def empty_glyph():
    return TTGlyphPen(None).glyph()


def build_font(output):
    glyph_order = [".notdef", "A", "u1D538"]
    glyphs = {
        ".notdef": empty_glyph(),
        "A": polygon([(100, 0), (300, 700), (500, 0)]),
        "u1D538": polygon([(100, 0), (100, 700), (500, 700), (500, 0)]),
    }
    metrics = {name: (600, 0) for name in glyph_order}
    # The non-BMP glyph makes a format-12 full-repertoire cmap natural rather
    # than merely forcing that table format for an all-BMP fixture.
    character_map = {0x0041: "A", 0x1D538: "u1D538"}

    builder = FontBuilder(UNITS_PER_EM, isTTF=True)
    builder.setupGlyphOrder(glyph_order)
    builder.setupCharacterMap(character_map)
    builder.setupGlyf(glyphs)
    builder.setupHorizontalMetrics(metrics)
    builder.setupHorizontalHeader(ascent=800, descent=-200)
    builder.setupNameTable(
        {
            "copyright": "Copyright 2026 the Tectonic Project",
            "familyName": "TectonicCmap04Test",
            "styleName": "Regular",
            "uniqueFontIdentifier": "TectonicCmap04Test-1.0",
            "fullName": "TectonicCmap04Test",
            "psName": "TectonicCmap04Test",
            "version": "Version 1.0",
            "licenseDescription": "MIT License",
            "licenseInfoURL": "https://github.com/tectonic-typesetting/tectonic/blob/master/LICENSE",
        }
    )
    builder.setupOS2(
        sTypoAscender=800,
        sTypoDescender=-200,
        usWinAscent=800,
        usWinDescent=200,
        fsType=0,
    )
    builder.setupPost(keepGlyphNames=False)
    builder.setupMaxp()

    # FontBuilder normally emits several Unicode cmap variants. Replace them
    # after OS/2 setup so the fixture has exactly the one subtable under test.
    cmap = CmapSubtable.newSubtable(12)
    cmap.platformID = 0
    cmap.platEncID = 4
    cmap.language = 0
    cmap.cmap = character_map
    builder.font["cmap"].tables = [cmap]

    builder.font.recalcTimestamp = False
    builder.font["head"].created = 0x7C259DC0  # 1970-01-01 in the SFNT epoch.
    builder.font["head"].modified = 0x7C259DC0
    builder.save(output)


if __name__ == "__main__":
    build_font(OUTPUT)
