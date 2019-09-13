#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           extern_types,
           ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type XeTeXLayoutEngine_rec;
    pub type UBreakIterator;
    pub type UConverter;
    pub type __CFString;
    /* Macs provide Fixed and FixedPoint */
    pub type XeTeXFont_rec;
    pub type __CFDictionary;
    pub type Opaque_TECkit_Converter;
    pub type __CFArray;
    pub type __CFBoolean;
    pub type __CFNumber;
    pub type CGColor;
    pub type __CTFontDescriptor;
    pub type __CTFont;
    pub type UBiDi;
    #[no_mangle]
    fn CFBooleanGetValue(boolean: CFBooleanRef) -> Boolean;
    #[no_mangle]
    static kCTFontAttributeName: CFStringRef;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn hb_tag_from_string(str: *const libc::c_char, len: libc::c_int) -> hb_tag_t;
    #[no_mangle]
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn xcalloc(nelem: size_t, elsize: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(old_address: *mut libc::c_void, new_size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    /* tectonic/core-bridge.h: declarations of C/C++ => Rust bridge API
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    /* Both XeTeX and bibtex use this enum: */
    /* The weird enum values are historical and could be rationalized. But it is
     * good to write them explicitly since they must be kept in sync with
     * `src/engines/mod.rs`.
     */
    /* quasi-hack to get the primary input */
    /* Bridge API. Keep synchronized with src/engines/mod.rs. */
    /* These functions are not meant to be used in the C/C++ code. They define the
     * API that we expose to the Rust side of things. */
    /* The internal, C/C++ interface: */
    /* Global symbols that route through the global API variable. Hopefully we
     * will one day eliminate all of the global state and get rid of all of
     * these. */
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const libc::c_char,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn tan(_: libc::c_double) -> libc::c_double;
    /* info for each glyph is location (FixedPoint) + glyph ID (uint16_t) */
    /* glyph ID field in a glyph_node */
    /* For Unicode encoding form interpretation... */
    #[no_mangle]
    fn getkXeTeXEmboldenAttributeName() -> CFStringRef;
    #[no_mangle]
    fn maketexstring(s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn get_cp_code(fontNum: libc::c_int, code: libc::c_uint, side: libc::c_int) -> libc::c_int;
    /* functions in XeTeX_mac.c */
    #[no_mangle]
    fn loadAATfont(
        descriptor: CTFontDescriptorRef,
        scaled_size: int32_t,
        cp1: *const libc::c_char,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn DoAATLayout(node: *mut libc::c_void, justify: libc::c_int);
    #[no_mangle]
    fn GetGlyphBBox_AAT(fontAttrs: CFDictionaryRef, gid: uint16_t, bbox: *mut GlyphBBox);
    #[no_mangle]
    fn GetGlyphWidth_AAT(fontAttrs: CFDictionaryRef, gid: uint16_t) -> libc::c_double;
    #[no_mangle]
    fn GetGlyphHeightDepth_AAT(
        fontAttrs: CFDictionaryRef,
        gid: uint16_t,
        ht: *mut libc::c_float,
        dp: *mut libc::c_float,
    );
    #[no_mangle]
    fn GetGlyphSidebearings_AAT(
        fontAttrs: CFDictionaryRef,
        gid: uint16_t,
        lsb: *mut libc::c_float,
        rsb: *mut libc::c_float,
    );
    #[no_mangle]
    fn GetGlyphItalCorr_AAT(fontAttrs: CFDictionaryRef, gid: uint16_t) -> libc::c_double;
    #[no_mangle]
    fn MapCharToGlyph_AAT(fontAttrs: CFDictionaryRef, ch: UInt32) -> libc::c_int;
    #[no_mangle]
    fn MapGlyphToIndex_AAT(
        attributes: CFDictionaryRef,
        glyphName: *const libc::c_char,
    ) -> libc::c_int;
    /* ***************************************************************************\
     Part of the XeTeX typesetting system
     Copyright (c) 1994-2008 by SIL International
     Copyright (c) 2009 by Jonathan Kew
     Copyright (c) 2012-2015 by Khaled Hosny

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
    /* graphite interface functions... */
    #[no_mangle]
    fn countGraphiteFeatures(engine: XeTeXLayoutEngine) -> uint32_t;
    #[no_mangle]
    fn findGraphiteFeature(
        engine: XeTeXLayoutEngine,
        s: *const libc::c_char,
        e: *const libc::c_char,
        f: *mut hb_tag_t,
        v: *mut libc::c_int,
    ) -> bool;
    #[no_mangle]
    fn countScripts(font: XeTeXFont) -> libc::c_uint;
    #[no_mangle]
    fn getReqEngine() -> libc::c_char;
    #[no_mangle]
    fn findNextGraphiteBreak() -> libc::c_int;
    #[no_mangle]
    fn initGraphiteBreaking(
        engine: XeTeXLayoutEngine,
        txtPtr: *const uint16_t,
        txtLen: libc::c_int,
    ) -> bool;
    #[no_mangle]
    fn getFontCharRange(engine: XeTeXLayoutEngine, reqFirst: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getGlyphName(font: XeTeXFont, gid: uint16_t, len: *mut libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn mapGlyphToIndex(engine: XeTeXLayoutEngine, glyphName: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn mapCharToGlyph(engine: XeTeXLayoutEngine, charCode: uint32_t) -> uint32_t;
    #[no_mangle]
    fn getGlyphItalCorr(engine: XeTeXLayoutEngine, glyphID: uint32_t) -> libc::c_float;
    #[no_mangle]
    fn getGlyphSidebearings(
        engine: XeTeXLayoutEngine,
        glyphID: uint32_t,
        lsb: *mut libc::c_float,
        rsb: *mut libc::c_float,
    );
    #[no_mangle]
    fn getGlyphHeightDepth(
        engine: XeTeXLayoutEngine,
        glyphID: uint32_t,
        height: *mut libc::c_float,
        depth: *mut libc::c_float,
    );
    #[no_mangle]
    fn getGlyphWidthFromEngine(engine: XeTeXLayoutEngine, glyphID: uint32_t) -> libc::c_float;
    #[no_mangle]
    fn getGlyphBounds(engine: XeTeXLayoutEngine, glyphID: uint32_t, bbox: *mut GlyphBBox);
    #[no_mangle]
    fn countLanguages(font: XeTeXFont, script: hb_tag_t) -> libc::c_uint;
    #[no_mangle]
    fn getFileNameFromCTFont(ctFontRef: CTFontRef, index: *mut uint32_t) -> *mut libc::c_char;
    #[no_mangle]
    fn GetFontCharRange_AAT(fontAttrs: CFDictionaryRef, reqFirst: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fontFromInteger(font: int32_t) -> CTFontRef;
    #[no_mangle]
    fn GetGlyphNameFromCTFont(
        ctFontRef: CTFontRef,
        gid: uint16_t,
        len: *mut libc::c_int,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn findGraphiteFeatureSettingNamed(
        engine: XeTeXLayoutEngine,
        feature: uint32_t,
        name: *const libc::c_char,
        namelength: libc::c_int,
    ) -> libc::c_long;
    #[no_mangle]
    fn findGraphiteFeatureNamed(
        engine: XeTeXLayoutEngine,
        name: *const libc::c_char,
        namelength: libc::c_int,
    ) -> libc::c_long;
    #[no_mangle]
    fn fontFromAttributes(fontAttrs: CFDictionaryRef) -> CTFontRef;
    #[no_mangle]
    fn getGraphiteFeatureSettingLabel(
        engine: XeTeXLayoutEngine,
        feature: uint32_t,
        setting: uint32_t,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn getGraphiteFeatureLabel(engine: XeTeXLayoutEngine, feature: uint32_t) -> *mut libc::c_char;
    #[no_mangle]
    fn getGraphiteFeatureDefaultSetting(engine: XeTeXLayoutEngine, feature: uint32_t) -> uint32_t;
    #[no_mangle]
    fn getGraphiteFeatureSettingCode(
        engine: XeTeXLayoutEngine,
        feature: uint32_t,
        index: uint32_t,
    ) -> uint32_t;
    #[no_mangle]
    fn getFullName(fontRef: PlatformFontRef) -> *const libc::c_char;
    #[no_mangle]
    fn getGraphiteFeatureCode(engine: XeTeXLayoutEngine, index: uint32_t) -> uint32_t;
    #[no_mangle]
    fn getRgbValue(engine: XeTeXLayoutEngine) -> uint32_t;
    #[no_mangle]
    fn getDefaultDirection(engine: XeTeXLayoutEngine) -> libc::c_int;
    #[no_mangle]
    fn getCapAndXHeight(
        engine: XeTeXLayoutEngine,
        capheight: *mut libc::c_float,
        xheight: *mut libc::c_float,
    );
    #[no_mangle]
    fn findSelectorByName(
        feature: CFDictionaryRef,
        name: *const libc::c_char,
        nameLength: libc::c_int,
    ) -> CFNumberRef;
    #[no_mangle]
    fn getAscentAndDescent(
        engine: XeTeXLayoutEngine,
        ascent: *mut libc::c_float,
        descent: *mut libc::c_float,
    );
    #[no_mangle]
    fn getPointSize(engine: XeTeXLayoutEngine) -> libc::c_float;
    #[no_mangle]
    fn findDictionaryInArray(
        array: CFArrayRef,
        nameKey: *const libc::c_void,
        name: *const libc::c_char,
        nameLength: libc::c_int,
    ) -> CFDictionaryRef;
    #[no_mangle]
    fn findDictionaryInArrayWithIdentifier(
        array: CFArrayRef,
        identifierKey: *const libc::c_void,
        identifier: libc::c_int,
    ) -> CFDictionaryRef;
    #[no_mangle]
    fn getGlyphPositions(engine: XeTeXLayoutEngine, positions: *mut FloatPoint);
    #[no_mangle]
    fn getCachedGlyphBBox(fontID: uint16_t, glyphID: uint16_t, bbox: *mut GlyphBBox)
        -> libc::c_int;
    #[no_mangle]
    fn cacheGlyphBBox(fontID: uint16_t, glyphID: uint16_t, bbox: *const GlyphBBox);
    #[no_mangle]
    fn createFont(fontRef: PlatformFontRef, pointSize: Fixed) -> XeTeXFont;
    #[no_mangle]
    fn createFontFromFile(
        filename: *const libc::c_char,
        index: libc::c_int,
        pointSize: Fixed,
    ) -> XeTeXFont;
    #[no_mangle]
    fn setFontLayoutDir(font: XeTeXFont, vertical: libc::c_int);
    #[no_mangle]
    fn findFontByName(
        name: *const libc::c_char,
        var: *mut libc::c_char,
        size: libc::c_double,
    ) -> PlatformFontRef;
    #[no_mangle]
    fn deleteFont(font: XeTeXFont);
    #[no_mangle]
    fn getFontFilename(engine: XeTeXLayoutEngine, index: *mut uint32_t) -> *mut libc::c_char;
    #[no_mangle]
    fn getDesignSize(font: XeTeXFont) -> libc::c_double;
    #[no_mangle]
    fn setReqEngine(reqEngine: libc::c_char);
    #[no_mangle]
    fn getFontTablePtr(font: XeTeXFont, tableTag: uint32_t) -> *mut libc::c_void;
    #[no_mangle]
    fn getSlant(font: XeTeXFont) -> Fixed;
    #[no_mangle]
    fn countFeatures(font: XeTeXFont, script: hb_tag_t, language: hb_tag_t) -> libc::c_uint;
    #[no_mangle]
    fn countGraphiteFeatureSettings(engine: XeTeXLayoutEngine, feature: uint32_t) -> uint32_t;
    #[no_mangle]
    fn countGlyphs(font: XeTeXFont) -> libc::c_uint;
    #[no_mangle]
    fn getIndScript(font: XeTeXFont, index: libc::c_uint) -> hb_tag_t;
    #[no_mangle]
    fn getIndLanguage(font: XeTeXFont, script: hb_tag_t, index: libc::c_uint) -> hb_tag_t;
    #[no_mangle]
    fn getIndFeature(
        font: XeTeXFont,
        script: hb_tag_t,
        language: hb_tag_t,
        index: libc::c_uint,
    ) -> hb_tag_t;
    #[no_mangle]
    fn getGlyphWidth(font: XeTeXFont, gid: uint32_t) -> libc::c_float;
    #[no_mangle]
    fn createLayoutEngine(
        fontRef: PlatformFontRef,
        font: XeTeXFont,
        script: hb_tag_t,
        language: *mut libc::c_char,
        features: *mut hb_feature_t,
        nFeatures: libc::c_int,
        shapers: *mut *mut libc::c_char,
        rgbValue: uint32_t,
        extend: libc::c_float,
        slant: libc::c_float,
        embolden: libc::c_float,
    ) -> XeTeXLayoutEngine;
    #[no_mangle]
    fn deleteLayoutEngine(engine: XeTeXLayoutEngine);
    #[no_mangle]
    fn getFont(engine: XeTeXLayoutEngine) -> XeTeXFont;
    #[no_mangle]
    fn getFontRef(engine: XeTeXLayoutEngine) -> PlatformFontRef;
    #[no_mangle]
    fn getExtendFactor(engine: XeTeXLayoutEngine) -> libc::c_float;
    #[no_mangle]
    fn getSlantFactor(engine: XeTeXLayoutEngine) -> libc::c_float;
    #[no_mangle]
    fn getEmboldenFactor(engine: XeTeXLayoutEngine) -> libc::c_float;
    #[no_mangle]
    fn layoutChars(
        engine: XeTeXLayoutEngine,
        chars: *mut uint16_t,
        offset: int32_t,
        count: int32_t,
        max: int32_t,
        rightToLeft: bool,
    ) -> libc::c_int;
    #[no_mangle]
    fn getGlyphs(engine: XeTeXLayoutEngine, glyphs: *mut uint32_t);
    #[no_mangle]
    fn getGlyphAdvances(engine: XeTeXLayoutEngine, advances: *mut libc::c_float);
    /* not the MS compiler, so try Metrowerks' platform macros */
    /* this seems to be needed for a gcc-mingw32 build to work... */
    /*
        Create a converter object from a compiled mapping
    */
    #[no_mangle]
    fn TECkit_CreateConverter(
        mapping: *mut Byte,
        mappingSize: UInt32,
        mapForward: Byte,
        sourceForm: UInt16,
        targetForm: UInt16,
        converter: *mut TECkit_Converter,
    ) -> TECkit_Status;
    /*
        Convert text from a buffer in memory
    */
    #[no_mangle]
    fn TECkit_ConvertBuffer(
        converter: TECkit_Converter,
        inBuffer: *const Byte,
        inLength: UInt32,
        inUsed: *mut UInt32,
        outBuffer: *mut Byte,
        outLength: UInt32,
        outUsed: *mut UInt32,
        inputIsComplete: Byte,
    ) -> TECkit_Status;
    /* *
     * Allocate a <code>UBiDi</code> structure.
     * Such an object is initially empty. It is assigned
     * the Bidi properties of a piece of text containing one or more paragraphs
     * by <code>ubidi_setPara()</code>
     * or the Bidi properties of a line within a paragraph by
     * <code>ubidi_setLine()</code>.<p>
     * This object can be reused for as long as it is not deallocated
     * by calling <code>ubidi_close()</code>.<p>
     * <code>ubidi_setPara()</code> and <code>ubidi_setLine()</code> will allocate
     * additional memory for internal structures as necessary.
     *
     * @return An empty <code>UBiDi</code> object.
     * @stable ICU 2.0
     */
    #[no_mangle]
    fn ubidi_open_64() -> *mut UBiDi;
    /* *
     * Allocate a <code>UBiDi</code> structure with preallocated memory
     * for internal structures.
     * This function provides a <code>UBiDi</code> object like <code>ubidi_open()</code>
     * with no arguments, but it also preallocates memory for internal structures
     * according to the sizings supplied by the caller.<p>
     * Subsequent functions will not allocate any more memory, and are thus
     * guaranteed not to fail because of lack of memory.<p>
     * The preallocation can be limited to some of the internal memory
     * by setting some values to 0 here. That means that if, e.g.,
     * <code>maxRunCount</code> cannot be reasonably predetermined and should not
     * be set to <code>maxLength</code> (the only failproof value) to avoid
     * wasting memory, then <code>maxRunCount</code> could be set to 0 here
     * and the internal structures that are associated with it will be allocated
     * on demand, just like with <code>ubidi_open()</code>.
     *
     * @param maxLength is the maximum text or line length that internal memory
     *        will be preallocated for. An attempt to associate this object with a
     *        longer text will fail, unless this value is 0, which leaves the allocation
     *        up to the implementation.
     *
     * @param maxRunCount is the maximum anticipated number of same-level runs
     *        that internal memory will be preallocated for. An attempt to access
     *        visual runs on an object that was not preallocated for as many runs
     *        as the text was actually resolved to will fail,
     *        unless this value is 0, which leaves the allocation up to the implementation.<br><br>
     *        The number of runs depends on the actual text and maybe anywhere between
     *        1 and <code>maxLength</code>. It is typically small.
     *
     * @param pErrorCode must be a valid pointer to an error code value.
     *
     * @return An empty <code>UBiDi</code> object with preallocated memory.
     * @stable ICU 2.0
     */
    /* *
     * <code>ubidi_close()</code> must be called to free the memory
     * associated with a UBiDi object.<p>
     *
     * <strong>Important: </strong>
     * A parent <code>UBiDi</code> object must not be destroyed or reused if
     * it still has children.
     * If a <code>UBiDi</code> object has become the <i>child</i>
     * of another one (its <i>parent</i>) by calling
     * <code>ubidi_setLine()</code>, then the child object must
     * be destroyed (closed) or reused (by calling
     * <code>ubidi_setPara()</code> or <code>ubidi_setLine()</code>)
     * before the parent object.
     *
     * @param pBiDi is a <code>UBiDi</code> object.
     *
     * @see ubidi_setPara
     * @see ubidi_setLine
     * @stable ICU 2.0
     */
    #[no_mangle]
    fn ubidi_close_64(pBiDi: *mut UBiDi);
    /* *
     * Modify the operation of the Bidi algorithm such that it
     * approximates an "inverse Bidi" algorithm. This function
     * must be called before <code>ubidi_setPara()</code>.
     *
     * <p>The normal operation of the Bidi algorithm as described
     * in the Unicode Technical Report is to take text stored in logical
     * (keyboard, typing) order and to determine the reordering of it for visual
     * rendering.
     * Some legacy systems store text in visual order, and for operations
     * with standard, Unicode-based algorithms, the text needs to be transformed
     * to logical order. This is effectively the inverse algorithm of the
     * described Bidi algorithm. Note that there is no standard algorithm for
     * this "inverse Bidi" and that the current implementation provides only an
     * approximation of "inverse Bidi".</p>
     *
     * <p>With <code>isInverse</code> set to <code>TRUE</code>,
     * this function changes the behavior of some of the subsequent functions
     * in a way that they can be used for the inverse Bidi algorithm.
     * Specifically, runs of text with numeric characters will be treated in a
     * special way and may need to be surrounded with LRM characters when they are
     * written in reordered sequence.</p>
     *
     * <p>Output runs should be retrieved using <code>ubidi_getVisualRun()</code>.
     * Since the actual input for "inverse Bidi" is visually ordered text and
     * <code>ubidi_getVisualRun()</code> gets the reordered runs, these are actually
     * the runs of the logically ordered output.</p>
     *
     * <p>Calling this function with argument <code>isInverse</code> set to
     * <code>TRUE</code> is equivalent to calling
     * <code>ubidi_setReorderingMode</code> with argument
     * <code>reorderingMode</code>
     * set to <code>#UBIDI_REORDER_INVERSE_NUMBERS_AS_L</code>.<br>
     * Calling this function with argument <code>isInverse</code> set to
     * <code>FALSE</code> is equivalent to calling
     * <code>ubidi_setReorderingMode</code> with argument
     * <code>reorderingMode</code>
     * set to <code>#UBIDI_REORDER_DEFAULT</code>.
     *
     * @param pBiDi is a <code>UBiDi</code> object.
     *
     * @param isInverse specifies "forward" or "inverse" Bidi operation.
     *
     * @see ubidi_setPara
     * @see ubidi_writeReordered
     * @see ubidi_setReorderingMode
     * @stable ICU 2.0
     */
    /* *
     * Is this Bidi object set to perform the inverse Bidi algorithm?
     * <p>Note: calling this function after setting the reordering mode with
     * <code>ubidi_setReorderingMode</code> will return <code>TRUE</code> if the
     * reordering mode was set to <code>#UBIDI_REORDER_INVERSE_NUMBERS_AS_L</code>,
     * <code>FALSE</code> for all other values.</p>
     *
     * @param pBiDi is a <code>UBiDi</code> object.
     * @return TRUE if the Bidi object is set to perform the inverse Bidi algorithm
     * by handling numbers as L.
     *
     * @see ubidi_setInverse
     * @see ubidi_setReorderingMode
     * @stable ICU 2.0
     */
    /* *
     * Specify whether block separators must be allocated level zero,
     * so that successive paragraphs will progress from left to right.
     * This function must be called before <code>ubidi_setPara()</code>.
     * Paragraph separators (B) may appear in the text.  Setting them to level zero
     * means that all paragraph separators (including one possibly appearing
     * in the last text position) are kept in the reordered text after the text
     * that they follow in the source text.
     * When this feature is not enabled, a paragraph separator at the last
     * position of the text before reordering will go to the first position
     * of the reordered text when the paragraph level is odd.
     *
     * @param pBiDi is a <code>UBiDi</code> object.
     *
     * @param orderParagraphsLTR specifies whether paragraph separators (B) must
     * receive level 0, so that successive paragraphs progress from left to right.
     *
     * @see ubidi_setPara
     * @stable ICU 3.4
     */
    /* *
     * Is this Bidi object set to allocate level 0 to block separators so that
     * successive paragraphs progress from left to right?
     *
     * @param pBiDi is a <code>UBiDi</code> object.
     * @return TRUE if the Bidi object is set to allocate level 0 to block
     *         separators.
     *
     * @see ubidi_orderParagraphsLTR
     * @stable ICU 3.4
     */
    /* *
     * <code>UBiDiReorderingMode</code> values indicate which variant of the Bidi
     * algorithm to use.
     *
     * @see ubidi_setReorderingMode
     * @stable ICU 3.6
     */
    /* * Regular Logical to Visual Bidi algorithm according to Unicode.
     * This is a 0 value.
     * @stable ICU 3.6 */
    /* * Logical to Visual algorithm which handles numbers in a way which
     * mimics the behavior of Windows XP.
     * @stable ICU 3.6 */
    /* * Logical to Visual algorithm grouping numbers with adjacent R characters
     * (reversible algorithm).
     * @stable ICU 3.6 */
    /* * Reorder runs only to transform a Logical LTR string to the Logical RTL
     * string with the same display, or vice-versa.<br>
     * If this mode is set together with option
     * <code>#UBIDI_OPTION_INSERT_MARKS</code>, some Bidi controls in the source
     * text may be removed and other controls may be added to produce the
     * minimum combination which has the required display.
     * @stable ICU 3.6 */
    /* * Visual to Logical algorithm which handles numbers like L
     * (same algorithm as selected by <code>ubidi_setInverse(TRUE)</code>.
     * @see ubidi_setInverse
     * @stable ICU 3.6 */
    /* * Visual to Logical algorithm equivalent to the regular Logical to Visual
     * algorithm.
     * @stable ICU 3.6 */
    /* * Inverse Bidi (Visual to Logical) algorithm for the
     * <code>UBIDI_REORDER_NUMBERS_SPECIAL</code> Bidi algorithm.
     * @stable ICU 3.6 */
    /* *
     * Number of values for reordering mode.
     * @deprecated ICU 58 The numeric value may change over time, see ICU ticket #12420.
     */
    // U_HIDE_DEPRECATED_API
    /* *
     * Modify the operation of the Bidi algorithm such that it implements some
     * variant to the basic Bidi algorithm or approximates an "inverse Bidi"
     * algorithm, depending on different values of the "reordering mode".
     * This function must be called before <code>ubidi_setPara()</code>, and stays
     * in effect until called again with a different argument.
     *
     * <p>The normal operation of the Bidi algorithm as described
     * in the Unicode Standard Annex #9 is to take text stored in logical
     * (keyboard, typing) order and to determine how to reorder it for visual
     * rendering.</p>
     *
     * <p>With the reordering mode set to a value other than
     * <code>#UBIDI_REORDER_DEFAULT</code>, this function changes the behavior of
     * some of the subsequent functions in a way such that they implement an
     * inverse Bidi algorithm or some other algorithm variants.</p>
     *
     * <p>Some legacy systems store text in visual order, and for operations
     * with standard, Unicode-based algorithms, the text needs to be transformed
     * into logical order. This is effectively the inverse algorithm of the
     * described Bidi algorithm. Note that there is no standard algorithm for
     * this "inverse Bidi", so a number of variants are implemented here.</p>
     *
     * <p>In other cases, it may be desirable to emulate some variant of the
     * Logical to Visual algorithm (e.g. one used in MS Windows), or perform a
     * Logical to Logical transformation.</p>
     *
     * <ul>
     * <li>When the reordering mode is set to <code>#UBIDI_REORDER_DEFAULT</code>,
     * the standard Bidi Logical to Visual algorithm is applied.</li>
     *
     * <li>When the reordering mode is set to
     * <code>#UBIDI_REORDER_NUMBERS_SPECIAL</code>,
     * the algorithm used to perform Bidi transformations when calling
     * <code>ubidi_setPara</code> should approximate the algorithm used in
     * Microsoft Windows XP rather than strictly conform to the Unicode Bidi
     * algorithm.
     * <br>
     * The differences between the basic algorithm and the algorithm addressed
     * by this option are as follows:
     * <ul>
     *   <li>Within text at an even embedding level, the sequence "123AB"
     *   (where AB represent R or AL letters) is transformed to "123BA" by the
     *   Unicode algorithm and to "BA123" by the Windows algorithm.</li>
     *   <li>Arabic-Indic numbers (AN) are handled by the Windows algorithm just
     *   like regular numbers (EN).</li>
     * </ul></li>
     *
     * <li>When the reordering mode is set to
     * <code>#UBIDI_REORDER_GROUP_NUMBERS_WITH_R</code>,
     * numbers located between LTR text and RTL text are associated with the RTL
     * text. For instance, an LTR paragraph with content "abc 123 DEF" (where
     * upper case letters represent RTL characters) will be transformed to
     * "abc FED 123" (and not "abc 123 FED"), "DEF 123 abc" will be transformed
     * to "123 FED abc" and "123 FED abc" will be transformed to "DEF 123 abc".
     * This makes the algorithm reversible and makes it useful when round trip
     * (from visual to logical and back to visual) must be achieved without
     * adding LRM characters. However, this is a variation from the standard
     * Unicode Bidi algorithm.<br>
     * The source text should not contain Bidi control characters other than LRM
     * or RLM.</li>
     *
     * <li>When the reordering mode is set to
     * <code>#UBIDI_REORDER_RUNS_ONLY</code>,
     * a "Logical to Logical" transformation must be performed:
     * <ul>
     * <li>If the default text level of the source text (argument <code>paraLevel</code>
     * in <code>ubidi_setPara</code>) is even, the source text will be handled as
     * LTR logical text and will be transformed to the RTL logical text which has
     * the same LTR visual display.</li>
     * <li>If the default level of the source text is odd, the source text
     * will be handled as RTL logical text and will be transformed to the
     * LTR logical text which has the same LTR visual display.</li>
     * </ul>
     * This mode may be needed when logical text which is basically Arabic or
     * Hebrew, with possible included numbers or phrases in English, has to be
     * displayed as if it had an even embedding level (this can happen if the
     * displaying application treats all text as if it was basically LTR).
     * <br>
     * This mode may also be needed in the reverse case, when logical text which is
     * basically English, with possible included phrases in Arabic or Hebrew, has to
     * be displayed as if it had an odd embedding level.
     * <br>
     * Both cases could be handled by adding LRE or RLE at the head of the text,
     * if the display subsystem supports these formatting controls. If it does not,
     * the problem may be handled by transforming the source text in this mode
     * before displaying it, so that it will be displayed properly.<br>
     * The source text should not contain Bidi control characters other than LRM
     * or RLM.</li>
     *
     * <li>When the reordering mode is set to
     * <code>#UBIDI_REORDER_INVERSE_NUMBERS_AS_L</code>, an "inverse Bidi" algorithm
     * is applied.
     * Runs of text with numeric characters will be treated like LTR letters and
     * may need to be surrounded with LRM characters when they are written in
     * reordered sequence (the option <code>#UBIDI_INSERT_LRM_FOR_NUMERIC</code> can
     * be used with function <code>ubidi_writeReordered</code> to this end. This
     * mode is equivalent to calling <code>ubidi_setInverse()</code> with
     * argument <code>isInverse</code> set to <code>TRUE</code>.</li>
     *
     * <li>When the reordering mode is set to
     * <code>#UBIDI_REORDER_INVERSE_LIKE_DIRECT</code>, the "direct" Logical to Visual
     * Bidi algorithm is used as an approximation of an "inverse Bidi" algorithm.
     * This mode is similar to mode <code>#UBIDI_REORDER_INVERSE_NUMBERS_AS_L</code>
     * but is closer to the regular Bidi algorithm.
     * <br>
     * For example, an LTR paragraph with the content "FED 123 456 CBA" (where
     * upper case represents RTL characters) will be transformed to
     * "ABC 456 123 DEF", as opposed to "DEF 123 456 ABC"
     * with mode <code>UBIDI_REORDER_INVERSE_NUMBERS_AS_L</code>.<br>
     * When used in conjunction with option
     * <code>#UBIDI_OPTION_INSERT_MARKS</code>, this mode generally
     * adds Bidi marks to the output significantly more sparingly than mode
     * <code>#UBIDI_REORDER_INVERSE_NUMBERS_AS_L</code> with option
     * <code>#UBIDI_INSERT_LRM_FOR_NUMERIC</code> in calls to
     * <code>ubidi_writeReordered</code>.</li>
     *
     * <li>When the reordering mode is set to
     * <code>#UBIDI_REORDER_INVERSE_FOR_NUMBERS_SPECIAL</code>, the Logical to Visual
     * Bidi algorithm used in Windows XP is used as an approximation of an "inverse Bidi" algorithm.
     * <br>
     * For example, an LTR paragraph with the content "abc FED123" (where
     * upper case represents RTL characters) will be transformed to "abc 123DEF."</li>
     * </ul>
     *
     * <p>In all the reordering modes specifying an "inverse Bidi" algorithm
     * (i.e. those with a name starting with <code>UBIDI_REORDER_INVERSE</code>),
     * output runs should be retrieved using
     * <code>ubidi_getVisualRun()</code>, and the output text with
     * <code>ubidi_writeReordered()</code>. The caller should keep in mind that in
     * "inverse Bidi" modes the input is actually visually ordered text and
     * reordered output returned by <code>ubidi_getVisualRun()</code> or
     * <code>ubidi_writeReordered()</code> are actually runs or character string
     * of logically ordered output.<br>
     * For all the "inverse Bidi" modes, the source text should not contain
     * Bidi control characters other than LRM or RLM.</p>
     *
     * <p>Note that option <code>#UBIDI_OUTPUT_REVERSE</code> of
     * <code>ubidi_writeReordered</code> has no useful meaning and should not be
     * used in conjunction with any value of the reordering mode specifying
     * "inverse Bidi" or with value <code>UBIDI_REORDER_RUNS_ONLY</code>.
     *
     * @param pBiDi is a <code>UBiDi</code> object.
     * @param reorderingMode specifies the required variant of the Bidi algorithm.
     *
     * @see UBiDiReorderingMode
     * @see ubidi_setInverse
     * @see ubidi_setPara
     * @see ubidi_writeReordered
     * @stable ICU 3.6
     */
    /* *
     * What is the requested reordering mode for a given Bidi object?
     *
     * @param pBiDi is a <code>UBiDi</code> object.
     * @return the current reordering mode of the Bidi object
     * @see ubidi_setReorderingMode
     * @stable ICU 3.6
     */
    /* *
     * <code>UBiDiReorderingOption</code> values indicate which options are
     * specified to affect the Bidi algorithm.
     *
     * @see ubidi_setReorderingOptions
     * @stable ICU 3.6
     */
    /* *
     * option value for <code>ubidi_setReorderingOptions</code>:
     * disable all the options which can be set with this function
     * @see ubidi_setReorderingOptions
     * @stable ICU 3.6
     */
    /* *
     * option bit for <code>ubidi_setReorderingOptions</code>:
     * insert Bidi marks (LRM or RLM) when needed to ensure correct result of
     * a reordering to a Logical order
     *
     * <p>This option must be set or reset before calling
     * <code>ubidi_setPara</code>.</p>
     *
     * <p>This option is significant only with reordering modes which generate
     * a result with Logical order, specifically:</p>
     * <ul>
     *   <li><code>#UBIDI_REORDER_RUNS_ONLY</code></li>
     *   <li><code>#UBIDI_REORDER_INVERSE_NUMBERS_AS_L</code></li>
     *   <li><code>#UBIDI_REORDER_INVERSE_LIKE_DIRECT</code></li>
     *   <li><code>#UBIDI_REORDER_INVERSE_FOR_NUMBERS_SPECIAL</code></li>
     * </ul>
     *
     * <p>If this option is set in conjunction with reordering mode
     * <code>#UBIDI_REORDER_INVERSE_NUMBERS_AS_L</code> or with calling
     * <code>ubidi_setInverse(TRUE)</code>, it implies
     * option <code>#UBIDI_INSERT_LRM_FOR_NUMERIC</code>
     * in calls to function <code>ubidi_writeReordered()</code>.</p>
     *
     * <p>For other reordering modes, a minimum number of LRM or RLM characters
     * will be added to the source text after reordering it so as to ensure
     * round trip, i.e. when applying the inverse reordering mode on the
     * resulting logical text with removal of Bidi marks
     * (option <code>#UBIDI_OPTION_REMOVE_CONTROLS</code> set before calling
     * <code>ubidi_setPara()</code> or option <code>#UBIDI_REMOVE_BIDI_CONTROLS</code>
     * in <code>ubidi_writeReordered</code>), the result will be identical to the
     * source text in the first transformation.
     *
     * <p>This option will be ignored if specified together with option
     * <code>#UBIDI_OPTION_REMOVE_CONTROLS</code>. It inhibits option
     * <code>UBIDI_REMOVE_BIDI_CONTROLS</code> in calls to function
     * <code>ubidi_writeReordered()</code> and it implies option
     * <code>#UBIDI_INSERT_LRM_FOR_NUMERIC</code> in calls to function
     * <code>ubidi_writeReordered()</code> if the reordering mode is
     * <code>#UBIDI_REORDER_INVERSE_NUMBERS_AS_L</code>.</p>
     *
     * @see ubidi_setReorderingMode
     * @see ubidi_setReorderingOptions
     * @stable ICU 3.6
     */
    /* *
     * option bit for <code>ubidi_setReorderingOptions</code>:
     * remove Bidi control characters
     *
     * <p>This option must be set or reset before calling
     * <code>ubidi_setPara</code>.</p>
     *
     * <p>This option nullifies option <code>#UBIDI_OPTION_INSERT_MARKS</code>.
     * It inhibits option <code>#UBIDI_INSERT_LRM_FOR_NUMERIC</code> in calls
     * to function <code>ubidi_writeReordered()</code> and it implies option
     * <code>#UBIDI_REMOVE_BIDI_CONTROLS</code> in calls to that function.</p>
     *
     * @see ubidi_setReorderingMode
     * @see ubidi_setReorderingOptions
     * @stable ICU 3.6
     */
    /* *
     * option bit for <code>ubidi_setReorderingOptions</code>:
     * process the output as part of a stream to be continued
     *
     * <p>This option must be set or reset before calling
     * <code>ubidi_setPara</code>.</p>
     *
     * <p>This option specifies that the caller is interested in processing large
     * text object in parts.
     * The results of the successive calls are expected to be concatenated by the
     * caller. Only the call for the last part will have this option bit off.</p>
     *
     * <p>When this option bit is on, <code>ubidi_setPara()</code> may process
     * less than the full source text in order to truncate the text at a meaningful
     * boundary. The caller should call <code>ubidi_getProcessedLength()</code>
     * immediately after calling <code>ubidi_setPara()</code> in order to
     * determine how much of the source text has been processed.
     * Source text beyond that length should be resubmitted in following calls to
     * <code>ubidi_setPara</code>. The processed length may be less than
     * the length of the source text if a character preceding the last character of
     * the source text constitutes a reasonable boundary (like a block separator)
     * for text to be continued.<br>
     * If the last character of the source text constitutes a reasonable
     * boundary, the whole text will be processed at once.<br>
     * If nowhere in the source text there exists
     * such a reasonable boundary, the processed length will be zero.<br>
     * The caller should check for such an occurrence and do one of the following:
     * <ul><li>submit a larger amount of text with a better chance to include
     *         a reasonable boundary.</li>
     *     <li>resubmit the same text after turning off option
     *         <code>UBIDI_OPTION_STREAMING</code>.</li></ul>
     * In all cases, this option should be turned off before processing the last
     * part of the text.</p>
     *
     * <p>When the <code>UBIDI_OPTION_STREAMING</code> option is used,
     * it is recommended to call <code>ubidi_orderParagraphsLTR()</code> with
     * argument <code>orderParagraphsLTR</code> set to <code>TRUE</code> before
     * calling <code>ubidi_setPara</code> so that later paragraphs may be
     * concatenated to previous paragraphs on the right.</p>
     *
     * @see ubidi_setReorderingMode
     * @see ubidi_setReorderingOptions
     * @see ubidi_getProcessedLength
     * @see ubidi_orderParagraphsLTR
     * @stable ICU 3.6
     */
    /* *
     * Specify which of the reordering options
     * should be applied during Bidi transformations.
     *
     * @param pBiDi is a <code>UBiDi</code> object.
     * @param reorderingOptions is a combination of zero or more of the following
     * options:
     * <code>#UBIDI_OPTION_DEFAULT</code>, <code>#UBIDI_OPTION_INSERT_MARKS</code>,
     * <code>#UBIDI_OPTION_REMOVE_CONTROLS</code>, <code>#UBIDI_OPTION_STREAMING</code>.
     *
     * @see ubidi_getReorderingOptions
     * @stable ICU 3.6
     */
    /* *
     * What are the reordering options applied to a given Bidi object?
     *
     * @param pBiDi is a <code>UBiDi</code> object.
     * @return the current reordering options of the Bidi object
     * @see ubidi_setReorderingOptions
     * @stable ICU 3.6
     */
    /* *
     * Set the context before a call to ubidi_setPara().<p>
     *
     * ubidi_setPara() computes the left-right directionality for a given piece
     * of text which is supplied as one of its arguments. Sometimes this piece
     * of text (the "main text") should be considered in context, because text
     * appearing before ("prologue") and/or after ("epilogue") the main text
     * may affect the result of this computation.<p>
     *
     * This function specifies the prologue and/or the epilogue for the next
     * call to ubidi_setPara(). The characters specified as prologue and
     * epilogue should not be modified by the calling program until the call
     * to ubidi_setPara() has returned. If successive calls to ubidi_setPara()
     * all need specification of a context, ubidi_setContext() must be called
     * before each call to ubidi_setPara(). In other words, a context is not
     * "remembered" after the following successful call to ubidi_setPara().<p>
     *
     * If a call to ubidi_setPara() specifies UBIDI_DEFAULT_LTR or
     * UBIDI_DEFAULT_RTL as paraLevel and is preceded by a call to
     * ubidi_setContext() which specifies a prologue, the paragraph level will
     * be computed taking in consideration the text in the prologue.<p>
     *
     * When ubidi_setPara() is called without a previous call to
     * ubidi_setContext, the main text is handled as if preceded and followed
     * by strong directional characters at the current paragraph level.
     * Calling ubidi_setContext() with specification of a prologue will change
     * this behavior by handling the main text as if preceded by the last
     * strong character appearing in the prologue, if any.
     * Calling ubidi_setContext() with specification of an epilogue will change
     * the behavior of ubidi_setPara() by handling the main text as if followed
     * by the first strong character or digit appearing in the epilogue, if any.<p>
     *
     * Note 1: if <code>ubidi_setContext</code> is called repeatedly without
     *         calling <code>ubidi_setPara</code>, the earlier calls have no effect,
     *         only the last call will be remembered for the next call to
     *         <code>ubidi_setPara</code>.<p>
     *
     * Note 2: calling <code>ubidi_setContext(pBiDi, NULL, 0, NULL, 0, &errorCode)</code>
     *         cancels any previous setting of non-empty prologue or epilogue.
     *         The next call to <code>ubidi_setPara()</code> will process no
     *         prologue or epilogue.<p>
     *
     * Note 3: users must be aware that even after setting the context
     *         before a call to ubidi_setPara() to perform e.g. a logical to visual
     *         transformation, the resulting string may not be identical to what it
     *         would have been if all the text, including prologue and epilogue, had
     *         been processed together.<br>
     * Example (upper case letters represent RTL characters):<br>
     * &nbsp;&nbsp;prologue = "<code>abc DE</code>"<br>
     * &nbsp;&nbsp;epilogue = none<br>
     * &nbsp;&nbsp;main text = "<code>FGH xyz</code>"<br>
     * &nbsp;&nbsp;paraLevel = UBIDI_LTR<br>
     * &nbsp;&nbsp;display without prologue = "<code>HGF xyz</code>"
     *             ("HGF" is adjacent to "xyz")<br>
     * &nbsp;&nbsp;display with prologue = "<code>abc HGFED xyz</code>"
     *             ("HGF" is not adjacent to "xyz")<br>
     *
     * @param pBiDi is a paragraph <code>UBiDi</code> object.
     *
     * @param prologue is a pointer to the text which precedes the text that
     *        will be specified in a coming call to ubidi_setPara().
     *        If there is no prologue to consider, then <code>proLength</code>
     *        must be zero and this pointer can be NULL.
     *
     * @param proLength is the length of the prologue; if <code>proLength==-1</code>
     *        then the prologue must be zero-terminated.
     *        Otherwise proLength must be >= 0. If <code>proLength==0</code>, it means
     *        that there is no prologue to consider.
     *
     * @param epilogue is a pointer to the text which follows the text that
     *        will be specified in a coming call to ubidi_setPara().
     *        If there is no epilogue to consider, then <code>epiLength</code>
     *        must be zero and this pointer can be NULL.
     *
     * @param epiLength is the length of the epilogue; if <code>epiLength==-1</code>
     *        then the epilogue must be zero-terminated.
     *        Otherwise epiLength must be >= 0. If <code>epiLength==0</code>, it means
     *        that there is no epilogue to consider.
     *
     * @param pErrorCode must be a valid pointer to an error code value.
     *
     * @see ubidi_setPara
     * @stable ICU 4.8
     */
    /* *
     * Perform the Unicode Bidi algorithm. It is defined in the
     * <a href="http://www.unicode.org/unicode/reports/tr9/">Unicode Standard Annex #9</a>,
     * version 13,
     * also described in The Unicode Standard, Version 4.0 .<p>
     *
     * This function takes a piece of plain text containing one or more paragraphs,
     * with or without externally specified embedding levels from <i>styled</i>
     * text and computes the left-right-directionality of each character.<p>
     *
     * If the entire text is all of the same directionality, then
     * the function may not perform all the steps described by the algorithm,
     * i.e., some levels may not be the same as if all steps were performed.
     * This is not relevant for unidirectional text.<br>
     * For example, in pure LTR text with numbers the numbers would get
     * a resolved level of 2 higher than the surrounding text according to
     * the algorithm. This implementation may set all resolved levels to
     * the same value in such a case.<p>
     *
     * The text can be composed of multiple paragraphs. Occurrence of a block
     * separator in the text terminates a paragraph, and whatever comes next starts
     * a new paragraph. The exception to this rule is when a Carriage Return (CR)
     * is followed by a Line Feed (LF). Both CR and LF are block separators, but
     * in that case, the pair of characters is considered as terminating the
     * preceding paragraph, and a new paragraph will be started by a character
     * coming after the LF.
     *
     * @param pBiDi A <code>UBiDi</code> object allocated with <code>ubidi_open()</code>
     *        which will be set to contain the reordering information,
     *        especially the resolved levels for all the characters in <code>text</code>.
     *
     * @param text is a pointer to the text that the Bidi algorithm will be performed on.
     *        This pointer is stored in the UBiDi object and can be retrieved
     *        with <code>ubidi_getText()</code>.<br>
     *        <strong>Note:</strong> the text must be (at least) <code>length</code> long.
     *
     * @param length is the length of the text; if <code>length==-1</code> then
     *        the text must be zero-terminated.
     *
     * @param paraLevel specifies the default level for the text;
     *        it is typically 0 (LTR) or 1 (RTL).
     *        If the function shall determine the paragraph level from the text,
     *        then <code>paraLevel</code> can be set to
     *        either <code>#UBIDI_DEFAULT_LTR</code>
     *        or <code>#UBIDI_DEFAULT_RTL</code>; if the text contains multiple
     *        paragraphs, the paragraph level shall be determined separately for
     *        each paragraph; if a paragraph does not include any strongly typed
     *        character, then the desired default is used (0 for LTR or 1 for RTL).
     *        Any other value between 0 and <code>#UBIDI_MAX_EXPLICIT_LEVEL</code>
     *        is also valid, with odd levels indicating RTL.
     *
     * @param embeddingLevels (in) may be used to preset the embedding and override levels,
     *        ignoring characters like LRE and PDF in the text.
     *        A level overrides the directional property of its corresponding
     *        (same index) character if the level has the
     *        <code>#UBIDI_LEVEL_OVERRIDE</code> bit set.<br><br>
     *        Aside from that bit, it must be
     *        <code>paraLevel<=embeddingLevels[]<=UBIDI_MAX_EXPLICIT_LEVEL</code>,
     *        except that level 0 is always allowed.
     *        Level 0 for a paragraph separator prevents reordering of paragraphs;
     *        this only works reliably if <code>#UBIDI_LEVEL_OVERRIDE</code>
     *        is also set for paragraph separators.
     *        Level 0 for other characters is treated as a wildcard
     *        and is lifted up to the resolved level of the surrounding paragraph.<br><br>
     *        <strong>Caution: </strong>A copy of this pointer, not of the levels,
     *        will be stored in the <code>UBiDi</code> object;
     *        the <code>embeddingLevels</code> array must not be
     *        deallocated before the <code>UBiDi</code> structure is destroyed or reused,
     *        and the <code>embeddingLevels</code>
     *        should not be modified to avoid unexpected results on subsequent Bidi operations.
     *        However, the <code>ubidi_setPara()</code> and
     *        <code>ubidi_setLine()</code> functions may modify some or all of the levels.<br><br>
     *        After the <code>UBiDi</code> object is reused or destroyed, the caller
     *        must take care of the deallocation of the <code>embeddingLevels</code> array.<br><br>
     *        <strong>Note:</strong> the <code>embeddingLevels</code> array must be
     *        at least <code>length</code> long.
     *        This pointer can be <code>NULL</code> if this
     *        value is not necessary.
     *
     * @param pErrorCode must be a valid pointer to an error code value.
     * @stable ICU 2.0
     */
    /* *
     * <code>ubidi_setLine()</code> sets a <code>UBiDi</code> to
     * contain the reordering information, especially the resolved levels,
     * for all the characters in a line of text. This line of text is
     * specified by referring to a <code>UBiDi</code> object representing
     * this information for a piece of text containing one or more paragraphs,
     * and by specifying a range of indexes in this text.<p>
     * In the new line object, the indexes will range from 0 to <code>limit-start-1</code>.<p>
     *
     * This is used after calling <code>ubidi_setPara()</code>
     * for a piece of text, and after line-breaking on that text.
     * It is not necessary if each paragraph is treated as a single line.<p>
     *
     * After line-breaking, rules (L1) and (L2) for the treatment of
     * trailing WS and for reordering are performed on
     * a <code>UBiDi</code> object that represents a line.<p>
     *
     * <strong>Important: </strong><code>pLineBiDi</code> shares data with
     * <code>pParaBiDi</code>.
     * You must destroy or reuse <code>pLineBiDi</code> before <code>pParaBiDi</code>.
     * In other words, you must destroy or reuse the <code>UBiDi</code> object for a line
     * before the object for its parent paragraph.<p>
     *
     * The text pointer that was stored in <code>pParaBiDi</code> is also copied,
     * and <code>start</code> is added to it so that it points to the beginning of the
     * line for this object.
     *
     * @param pParaBiDi is the parent paragraph object. It must have been set
     * by a successful call to ubidi_setPara.
     *
     * @param start is the line's first index into the text.
     *
     * @param limit is just behind the line's last index into the text
     *        (its last index +1).<br>
     *        It must be <code>0<=start<limit<=</code>containing paragraph limit.
     *        If the specified line crosses a paragraph boundary, the function
     *        will terminate with error code U_ILLEGAL_ARGUMENT_ERROR.
     *
     * @param pLineBiDi is the object that will now represent a line of the text.
     *
     * @param pErrorCode must be a valid pointer to an error code value.
     *
     * @see ubidi_setPara
     * @see ubidi_getProcessedLength
     * @stable ICU 2.0
     */
    /* *
     * Get the directionality of the text.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @return a value of <code>UBIDI_LTR</code>, <code>UBIDI_RTL</code>
     *         or <code>UBIDI_MIXED</code>
     *         that indicates if the entire text
     *         represented by this object is unidirectional,
     *         and which direction, or if it is mixed-directional.
     * Note -  The value <code>UBIDI_NEUTRAL</code> is never returned from this method.
     *
     * @see UBiDiDirection
     * @stable ICU 2.0
     */
    /* *
     * Gets the base direction of the text provided according
     * to the Unicode Bidirectional Algorithm. The base direction
     * is derived from the first character in the string with bidirectional
     * character type L, R, or AL. If the first such character has type L,
     * <code>UBIDI_LTR</code> is returned. If the first such character has
     * type R or AL, <code>UBIDI_RTL</code> is returned. If the string does
     * not contain any character of these types, then
     * <code>UBIDI_NEUTRAL</code> is returned.
     *
     * This is a lightweight function for use when only the base direction
     * is needed and no further bidi processing of the text is needed.
     *
     * @param text is a pointer to the text whose base
     *             direction is needed.
     * Note: the text must be (at least) @c length long.
     *
     * @param length is the length of the text;
     *               if <code>length==-1</code> then the text
     *               must be zero-terminated.
     *
     * @return  <code>UBIDI_LTR</code>, <code>UBIDI_RTL</code>,
     *          <code>UBIDI_NEUTRAL</code>
     *
     * @see UBiDiDirection
     * @stable ICU 4.6
     */
    /* *
     * Get the pointer to the text.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @return The pointer to the text that the UBiDi object was created for.
     *
     * @see ubidi_setPara
     * @see ubidi_setLine
     * @stable ICU 2.0
     */
    /* *
     * Get the length of the text.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @return The length of the text that the UBiDi object was created for.
     * @stable ICU 2.0
     */
    /* *
     * Get the paragraph level of the text.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @return The paragraph level. If there are multiple paragraphs, their
     *         level may vary if the required paraLevel is UBIDI_DEFAULT_LTR or
     *         UBIDI_DEFAULT_RTL.  In that case, the level of the first paragraph
     *         is returned.
     *
     * @see UBiDiLevel
     * @see ubidi_getParagraph
     * @see ubidi_getParagraphByIndex
     * @stable ICU 2.0
     */
    /* *
     * Get the number of paragraphs.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @return The number of paragraphs.
     * @stable ICU 3.4
     */
    /* *
     * Get a paragraph, given a position within the text.
     * This function returns information about a paragraph.<br>
     * Note: if the paragraph index is known, it is more efficient to
     * retrieve the paragraph information using ubidi_getParagraphByIndex().<p>
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @param charIndex is the index of a character within the text, in the
     *        range <code>[0..ubidi_getProcessedLength(pBiDi)-1]</code>.
     *
     * @param pParaStart will receive the index of the first character of the
     *        paragraph in the text.
     *        This pointer can be <code>NULL</code> if this
     *        value is not necessary.
     *
     * @param pParaLimit will receive the limit of the paragraph.
     *        The l-value that you point to here may be the
     *        same expression (variable) as the one for
     *        <code>charIndex</code>.
     *        This pointer can be <code>NULL</code> if this
     *        value is not necessary.
     *
     * @param pParaLevel will receive the level of the paragraph.
     *        This pointer can be <code>NULL</code> if this
     *        value is not necessary.
     *
     * @param pErrorCode must be a valid pointer to an error code value.
     *
     * @return The index of the paragraph containing the specified position.
     *
     * @see ubidi_getProcessedLength
     * @stable ICU 3.4
     */
    /* *
     * Get a paragraph, given the index of this paragraph.
     *
     * This function returns information about a paragraph.<p>
     *
     * @param pBiDi is the paragraph <code>UBiDi</code> object.
     *
     * @param paraIndex is the number of the paragraph, in the
     *        range <code>[0..ubidi_countParagraphs(pBiDi)-1]</code>.
     *
     * @param pParaStart will receive the index of the first character of the
     *        paragraph in the text.
     *        This pointer can be <code>NULL</code> if this
     *        value is not necessary.
     *
     * @param pParaLimit will receive the limit of the paragraph.
     *        This pointer can be <code>NULL</code> if this
     *        value is not necessary.
     *
     * @param pParaLevel will receive the level of the paragraph.
     *        This pointer can be <code>NULL</code> if this
     *        value is not necessary.
     *
     * @param pErrorCode must be a valid pointer to an error code value.
     *
     * @stable ICU 3.4
     */
    /* *
     * Get the level for one character.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @param charIndex the index of a character. It must be in the range
     *         [0..ubidi_getProcessedLength(pBiDi)].
     *
     * @return The level for the character at charIndex (0 if charIndex is not
     *         in the valid range).
     *
     * @see UBiDiLevel
     * @see ubidi_getProcessedLength
     * @stable ICU 2.0
     */
    /* *
     * Get an array of levels for each character.<p>
     *
     * Note that this function may allocate memory under some
     * circumstances, unlike <code>ubidi_getLevelAt()</code>.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object, whose
     *        text length must be strictly positive.
     *
     * @param pErrorCode must be a valid pointer to an error code value.
     *
     * @return The levels array for the text,
     *         or <code>NULL</code> if an error occurs.
     *
     * @see UBiDiLevel
     * @see ubidi_getProcessedLength
     * @stable ICU 2.0
     */
    /* *
     * Get a logical run.
     * This function returns information about a run and is used
     * to retrieve runs in logical order.<p>
     * This is especially useful for line-breaking on a paragraph.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @param logicalPosition is a logical position within the source text.
     *
     * @param pLogicalLimit will receive the limit of the corresponding run.
     *        The l-value that you point to here may be the
     *        same expression (variable) as the one for
     *        <code>logicalPosition</code>.
     *        This pointer can be <code>NULL</code> if this
     *        value is not necessary.
     *
     * @param pLevel will receive the level of the corresponding run.
     *        This pointer can be <code>NULL</code> if this
     *        value is not necessary.
     *
     * @see ubidi_getProcessedLength
     * @stable ICU 2.0
     */
    /* *
     * Get the number of runs.
     * This function may invoke the actual reordering on the
     * <code>UBiDi</code> object, after <code>ubidi_setPara()</code>
     * may have resolved only the levels of the text. Therefore,
     * <code>ubidi_countRuns()</code> may have to allocate memory,
     * and may fail doing so.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @param pErrorCode must be a valid pointer to an error code value.
     *
     * @return The number of runs.
     * @stable ICU 2.0
     */
    /* *
     * Get one run's logical start, length, and directionality,
     * which can be 0 for LTR or 1 for RTL.
     * In an RTL run, the character at the logical start is
     * visually on the right of the displayed run.
     * The length is the number of characters in the run.<p>
     * <code>ubidi_countRuns()</code> should be called
     * before the runs are retrieved.
     *
     * @param pBiDi is the paragraph or line <code>UBiDi</code> object.
     *
     * @param runIndex is the number of the run in visual order, in the
     *        range <code>[0..ubidi_countRuns(pBiDi)-1]</code>.
     *
     * @param pLogicalStart is the first logical character index in the text.
     *        The pointer may be <code>NULL</code> if this index is not needed.
     *
     * @param pLength is the number of characters (at least one) in the run.
     *        The pointer may be <code>NULL</code> if this is not needed.
     *
     * @return the directionality of the run,
     *         <code>UBIDI_LTR==0</code> or <code>UBIDI_RTL==1</code>,
     *         never <code>UBIDI_MIXED</code>,
     *         never <code>UBIDI_NEUTRAL</code>.
     *
     * @see ubidi_countRuns
     *
     * Example:
     * <pre>
     * \code
     * int32_t i, count=ubidi_countRuns(pBiDi),
     *         logicalStart, visualIndex=0, length;
     * for(i=0; i<count; ++i) {
     *    if(UBIDI_LTR==ubidi_getVisualRun(pBiDi, i, &logicalStart, &length)) {
     *         do { // LTR
     *             show_char(text[logicalStart++], visualIndex++);
     *         } while(--length>0);
     *     } else {
     *         logicalStart+=length;  // logicalLimit
     *         do { // RTL
     *             show_char(text[--logicalStart], visualIndex++);
     *         } while(--length>0);
     *     }
     * }
     *\endcode
     * </pre>
     *
     * Note that in right-to-left runs, code like this places
     * second surrogates before first ones (which is generally a bad idea)
     * and combining characters before base characters.
     * <p>
     * Use of <code>ubidi_writeReordered()</code>, optionally with the
     * <code>#UBIDI_KEEP_BASE_COMBINING</code> option, can be considered in order
     * to avoid these issues.
     * @stable ICU 2.0
     */
    #[no_mangle]
    fn ubidi_getVisualRun_64(
        pBiDi: *mut UBiDi,
        runIndex: int32_t,
        pLogicalStart: *mut int32_t,
        pLength: *mut int32_t,
    ) -> UBiDiDirection;
    #[no_mangle]
    fn ubidi_countRuns_64(pBiDi: *mut UBiDi, pErrorCode: *mut UErrorCode) -> int32_t;
    #[no_mangle]
    fn ubidi_getDirection_64(pBiDi: *const UBiDi) -> UBiDiDirection;
    #[no_mangle]
    fn ubidi_setPara_64(
        pBiDi: *mut UBiDi,
        text: *const UChar,
        length: int32_t,
        paraLevel: UBiDiLevel,
        embeddingLevels: *mut UBiDiLevel,
        pErrorCode: *mut UErrorCode,
    );
    /* *
     * A recommended size (in bytes) for the memory buffer to be passed to ubrk_saveClone().
     * @deprecated ICU 52. Do not rely on ubrk_safeClone() cloning into any provided buffer.
     */
    /* U_HIDE_DEPRECATED_API */
    /* *
    * Close a UBreakIterator.
    * Once closed, a UBreakIterator may no longer be used.
    * @param bi The break iterator to close.
     * @stable ICU 2.0
    */
    #[no_mangle]
    fn ubrk_close_64(bi: *mut UBreakIterator);
    #[no_mangle]
    fn ubrk_open_64(
        type_0: UBreakIteratorType,
        locale: *const libc::c_char,
        text: *const UChar,
        textLength: int32_t,
        status: *mut UErrorCode,
    ) -> *mut UBreakIterator;
    #[no_mangle]
    fn ubrk_setText_64(
        bi: *mut UBreakIterator,
        text: *const UChar,
        textLength: int32_t,
        status: *mut UErrorCode,
    );
    #[no_mangle]
    fn ubrk_next_64(bi: *mut UBreakIterator) -> int32_t;
    /* *
     * Creates a UConverter object with the name of a coded character set specified as a C string.
     * The actual name will be resolved with the alias file
     * using a case-insensitive string comparison that ignores
     * leading zeroes and all non-alphanumeric characters.
     * E.g., the names "UTF8", "utf-8", "u*T@f08" and "Utf 8" are all equivalent.
     * (See also ucnv_compareNames().)
     * If <code>NULL</code> is passed for the converter name, it will create one with the
     * getDefaultName return value.
     *
     * <p>A converter name for ICU 1.5 and above may contain options
     * like a locale specification to control the specific behavior of
     * the newly instantiated converter.
     * The meaning of the options depends on the particular converter.
     * If an option is not defined for or recognized by a given converter, then it is ignored.</p>
     *
     * <p>Options are appended to the converter name string, with a
     * <code>UCNV_OPTION_SEP_CHAR</code> between the name and the first option and
     * also between adjacent options.</p>
     *
     * <p>If the alias is ambiguous, then the preferred converter is used
     * and the status is set to U_AMBIGUOUS_ALIAS_WARNING.</p>
     *
     * <p>The conversion behavior and names can vary between platforms. ICU may
     * convert some characters differently from other platforms. Details on this topic
     * are in the <a href="http://icu-project.org/userguide/conversion.html">User's
     * Guide</a>. Aliases starting with a "cp" prefix have no specific meaning
     * other than its an alias starting with the letters "cp". Please do not
     * associate any meaning to these aliases.</p>
     *
     * \snippet samples/ucnv/convsamp.cpp ucnv_open
     *
     * @param converterName Name of the coded character set table.
     *          This may have options appended to the string.
     *          IANA alias character set names, IBM CCSIDs starting with "ibm-",
     *          Windows codepage numbers starting with "windows-" are frequently
     *          used for this parameter. See ucnv_getAvailableName and
     *          ucnv_getAlias for a complete list that is available.
     *          If this parameter is NULL, the default converter will be used.
     * @param err outgoing error status <TT>U_MEMORY_ALLOCATION_ERROR, U_FILE_ACCESS_ERROR</TT>
     * @return the created Unicode converter object, or <TT>NULL</TT> if an error occurred
     * @see ucnv_openU
     * @see ucnv_openCCSID
     * @see ucnv_getAvailableName
     * @see ucnv_getAlias
     * @see ucnv_getDefaultName
     * @see ucnv_close
     * @see ucnv_compareNames
     * @stable ICU 2.0
     */
    #[no_mangle]
    fn ucnv_open_64(converterName: *const libc::c_char, err: *mut UErrorCode) -> *mut UConverter;
    /* *
     * \def U_CNV_SAFECLONE_BUFFERSIZE
     * Definition of a buffer size that is designed to be large enough for
     * converters to be cloned with ucnv_safeClone().
     * @deprecated ICU 52. Do not rely on ucnv_safeClone() cloning into any provided buffer.
     */
    /* U_HIDE_DEPRECATED_API */
    /* *
     * Deletes the unicode converter and releases resources associated
     * with just this instance.
     * Does not free up shared converter tables.
     *
     * @param converter the converter object to be deleted
     * @see ucnv_open
     * @see ucnv_openU
     * @see ucnv_openCCSID
     * @stable ICU 2.0
     */
    #[no_mangle]
    fn ucnv_close_64(converter: *mut UConverter);
    #[no_mangle]
    fn gr_label_destroy(label: *mut libc::c_void);
    #[no_mangle]
    fn gettexstring(_: str_number) -> *mut libc::c_char;
    #[no_mangle]
    static mut name_of_file: *mut libc::c_char;
    #[no_mangle]
    static mut name_length: int32_t;
    #[no_mangle]
    static mut font_info: *mut memory_word;
    #[no_mangle]
    static mut font_area: *mut str_number;
    #[no_mangle]
    static mut font_layout_engine: *mut *mut libc::c_void;
    #[no_mangle]
    static mut font_flags: *mut libc::c_char;
    #[no_mangle]
    static mut font_letter_space: *mut scaled_t;
    #[no_mangle]
    static mut loaded_font_mapping: *mut libc::c_void;
    #[no_mangle]
    static mut loaded_font_flags: libc::c_char;
    #[no_mangle]
    static mut loaded_font_letter_space: scaled_t;
    #[no_mangle]
    static mut loaded_font_design_size: scaled_t;
    #[no_mangle]
    static mut mapped_text: *mut UTF16_code;
    #[no_mangle]
    static mut xdv_buffer: *mut libc::c_char;
    #[no_mangle]
    static mut height_base: *mut int32_t;
    #[no_mangle]
    static mut depth_base: *mut int32_t;
    #[no_mangle]
    static mut param_base: *mut int32_t;
    #[no_mangle]
    static mut native_font_type_flag: int32_t;
    #[no_mangle]
    fn begin_diagnostic();
    #[no_mangle]
    fn end_diagnostic(blank_line: bool);
    #[no_mangle]
    fn font_feature_warning(
        featureNameP: *const libc::c_void,
        featLen: int32_t,
        settingNameP: *const libc::c_void,
        setLen: int32_t,
    );
    #[no_mangle]
    fn font_mapping_warning(
        mappingNameP: *const libc::c_void,
        mappingNameLen: int32_t,
        warningType: int32_t,
    );
    #[no_mangle]
    fn get_tracing_fonts_state() -> int32_t;
    #[no_mangle]
    fn print_raw_char(s: UTF16_code, incr_offset: bool);
    #[no_mangle]
    fn print_char(s: int32_t);
    #[no_mangle]
    fn print_nl(s: str_number);
    #[no_mangle]
    fn print_int(n: int32_t);
    #[no_mangle]
    fn xn_over_d(x: scaled_t, n: int32_t, d: int32_t) -> scaled_t;
    #[no_mangle]
    fn CFNumberGetValue(
        number: CFNumberRef,
        theType: CFNumberType,
        valuePtr: *mut libc::c_void,
    ) -> Boolean;
    #[no_mangle]
    fn CGColorGetComponents(color: CGColorRef) -> *const CGFloat;
    #[no_mangle]
    static kCTForegroundColorAttributeName: CFStringRef;
    #[no_mangle]
    static kCTVerticalFormsAttributeName: CFStringRef;
    #[no_mangle]
    fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    #[no_mangle]
    fn CFStringGetLength(theString: CFStringRef) -> CFIndex;
    #[no_mangle]
    fn CFStringGetCharacters(theString: CFStringRef, range: CFRange, buffer: *mut UniChar);
    #[no_mangle]
    fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *const libc::c_void;
    #[no_mangle]
    fn CFDictionaryGetValue(
        theDict: CFDictionaryRef,
        key: *const libc::c_void,
    ) -> *const libc::c_void;
    #[no_mangle]
    fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CTFontGetMatrix(font: CTFontRef) -> CGAffineTransform;
    #[no_mangle]
    fn CFDictionaryGetValueIfPresent(
        theDict: CFDictionaryRef,
        key: *const libc::c_void,
        value: *mut *const libc::c_void,
    ) -> Boolean;
    #[no_mangle]
    fn CFRelease(cf: CFTypeRef);
    #[no_mangle]
    fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CTFontGetGlyphCount(font: CTFontRef) -> CFIndex;
    #[no_mangle]
    fn CTFontGetSlantAngle(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CTFontGetCapHeight(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    static kCTFontFeatureTypeIdentifierKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureTypeNameKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureTypeExclusiveKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureTypeSelectorsKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureSelectorIdentifierKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureSelectorNameKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureSelectorDefaultKey: CFStringRef;
    #[no_mangle]
    fn CTFontCopyFeatures(font: CTFontRef) -> CFArrayRef;
}
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type size_t = __darwin_size_t;
pub type int32_t = libc::c_int;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type ssize_t = __darwin_ssize_t;
pub type tt_input_format_type = libc::c_uint;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
pub type XeTeXLayoutEngine = *mut XeTeXLayoutEngine_rec;
#[derive(Copy, Clone)]
#[repr(C, packed(2))]
pub struct FixedPoint {
    pub x: Fixed,
    pub y: Fixed,
}
pub type Fixed = SInt32;
pub type SInt32 = libc::c_int;
pub type UChar = uint16_t;
pub type UniChar = UInt16;
pub type UInt16 = libc::c_ushort;
pub type CFStringRef = *const __CFString;
pub type UBiDiLevel = uint8_t;
pub type XeTeXFont = *mut XeTeXFont_rec;
pub type CFDictionaryRef = *const __CFDictionary;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hb_feature_t {
    pub tag: hb_tag_t,
    pub value: uint32_t,
    pub start: libc::c_uint,
    pub end: libc::c_uint,
}
pub type hb_tag_t = uint32_t;
/*------------------------------------------------------------------------
Copyright (C) 2002-2014 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.

File: TECkit_Engine.h
Responsibility: Jonathan Kew
Last reviewed: Not yet.

Description:
    Public API to the TECkit conversion engine.
-------------------------------------------------------------------------*/
/*
    TECkit_Engine.h

    Public API to the TECkit encoding conversion library.

    18-Jan-2008		jk	added EXPORTED to declarations, for mingw32 cross-build
    18-Mar-2005		jk	moved version number to TECkit_Common.h as it is shared with the compiler
    19-Mar-2004		jk	updated minor version for 2.2 engine (improved matching functionality)
    23-Sep-2003		jk	updated for version 2.1 - new "...Opt" APIs
     5-Jul-2002		jk	corrected placement of WINAPI to keep MS compiler happy
    14-May-2002		jk	added WINAPI to function declarations
    22-Dec-2001		jk	initial version
*/
/* formFlags bits for normalization; if none are set, then this side of the mapping is normalization-form-agnostic on input, and may generate an unspecified mixture */
/* expects fully composed text (NC) */
/* expects fully decomposed text (NCD) */
/* generates fully composed text (NC) */
/* generates fully decomposed text (NCD) */
/* if VisualOrder is set, this side of the mapping deals with visual-order rather than logical-order text (only relevant for bidi scripts) */
/* visual rather than logical order */
/* if Unicode is set, the encoding is Unicode on this side of the mapping */
/* this is Unicode rather than a byte encoding */
/* required names */
/* "source" or LHS encoding name, e.g. "SIL-EEG_URDU-2001" */
/* "destination" or RHS encoding name, e.g. "UNICODE-3-1" */
/* source encoding description, e.g. "SIL East Eurasia Group Extended Urdu (Mac OS)" */
/* destination description, e.g. "Unicode 3.1" */
/* additional recommended names (parallel to UTR-22) */
/* "1.0b1" */
/* "mailto:nrsi@sil.org" */
/* "SIL International" */
/* "Greek (Galatia)" */
/* "(c)2002 SIL International" */
/* additional name IDs may be defined in the future */
/*
    encoding form options for TECkit_CreateConverter
*/
/*
    end of text value for TECkit_DataSource functions to return
*/
/*
    A converter object is an opaque pointer
*/
pub type TECkit_Converter = *mut Opaque_TECkit_Converter;
pub type Byte = UInt8;
pub type UInt8 = libc::c_uchar;
pub type Boolean = libc::c_uchar;
pub type UErrorCode = libc::c_int;
pub const U_ERROR_LIMIT: UErrorCode = 66818;
pub const U_PLUGIN_ERROR_LIMIT: UErrorCode = 66818;
pub const U_PLUGIN_DIDNT_SET_LEVEL: UErrorCode = 66817;
pub const U_PLUGIN_TOO_HIGH: UErrorCode = 66816;
pub const U_PLUGIN_ERROR_START: UErrorCode = 66816;
pub const U_STRINGPREP_CHECK_BIDI_ERROR: UErrorCode = 66562;
pub const U_STRINGPREP_UNASSIGNED_ERROR: UErrorCode = 66561;
pub const U_STRINGPREP_PROHIBITED_ERROR: UErrorCode = 66560;
pub const U_IDNA_ERROR_LIMIT: UErrorCode = 66569;
pub const U_IDNA_DOMAIN_NAME_TOO_LONG_ERROR: UErrorCode = 66568;
pub const U_IDNA_ZERO_LENGTH_LABEL_ERROR: UErrorCode = 66567;
pub const U_IDNA_LABEL_TOO_LONG_ERROR: UErrorCode = 66566;
pub const U_IDNA_VERIFICATION_ERROR: UErrorCode = 66565;
pub const U_IDNA_ACE_PREFIX_ERROR: UErrorCode = 66564;
pub const U_IDNA_STD3_ASCII_RULES_ERROR: UErrorCode = 66563;
pub const U_IDNA_CHECK_BIDI_ERROR: UErrorCode = 66562;
pub const U_IDNA_UNASSIGNED_ERROR: UErrorCode = 66561;
pub const U_IDNA_ERROR_START: UErrorCode = 66560;
pub const U_IDNA_PROHIBITED_ERROR: UErrorCode = 66560;
pub const U_REGEX_ERROR_LIMIT: UErrorCode = 66326;
pub const U_REGEX_INVALID_CAPTURE_GROUP_NAME: UErrorCode = 66325;
pub const U_REGEX_PATTERN_TOO_BIG: UErrorCode = 66324;
pub const U_REGEX_STOPPED_BY_CALLER: UErrorCode = 66323;
pub const U_REGEX_TIME_OUT: UErrorCode = 66322;
pub const U_REGEX_STACK_OVERFLOW: UErrorCode = 66321;
pub const U_REGEX_INVALID_RANGE: UErrorCode = 66320;
pub const U_REGEX_MISSING_CLOSE_BRACKET: UErrorCode = 66319;
pub const U_REGEX_OCTAL_TOO_BIG: UErrorCode = 66318;
pub const U_REGEX_SET_CONTAINS_STRING: UErrorCode = 66317;
pub const U_REGEX_LOOK_BEHIND_LIMIT: UErrorCode = 66316;
pub const U_REGEX_INVALID_FLAG: UErrorCode = 66315;
pub const U_REGEX_INVALID_BACK_REF: UErrorCode = 66314;
pub const U_REGEX_MAX_LT_MIN: UErrorCode = 66313;
pub const U_REGEX_BAD_INTERVAL: UErrorCode = 66312;
pub const U_REGEX_NUMBER_TOO_BIG: UErrorCode = 66311;
pub const U_REGEX_MISMATCHED_PAREN: UErrorCode = 66310;
pub const U_REGEX_UNIMPLEMENTED: UErrorCode = 66309;
pub const U_REGEX_PROPERTY_SYNTAX: UErrorCode = 66308;
pub const U_REGEX_BAD_ESCAPE_SEQUENCE: UErrorCode = 66307;
pub const U_REGEX_INVALID_STATE: UErrorCode = 66306;
pub const U_REGEX_RULE_SYNTAX: UErrorCode = 66305;
pub const U_REGEX_ERROR_START: UErrorCode = 66304;
pub const U_REGEX_INTERNAL_ERROR: UErrorCode = 66304;
pub const U_BRK_ERROR_LIMIT: UErrorCode = 66062;
pub const U_BRK_MALFORMED_RULE_TAG: UErrorCode = 66061;
pub const U_BRK_UNRECOGNIZED_OPTION: UErrorCode = 66060;
pub const U_BRK_RULE_EMPTY_SET: UErrorCode = 66059;
pub const U_BRK_INIT_ERROR: UErrorCode = 66058;
pub const U_BRK_UNDEFINED_VARIABLE: UErrorCode = 66057;
pub const U_BRK_NEW_LINE_IN_QUOTED_STRING: UErrorCode = 66056;
pub const U_BRK_MISMATCHED_PAREN: UErrorCode = 66055;
pub const U_BRK_VARIABLE_REDFINITION: UErrorCode = 66054;
pub const U_BRK_ASSIGN_ERROR: UErrorCode = 66053;
pub const U_BRK_UNCLOSED_SET: UErrorCode = 66052;
pub const U_BRK_RULE_SYNTAX: UErrorCode = 66051;
pub const U_BRK_SEMICOLON_EXPECTED: UErrorCode = 66050;
pub const U_BRK_HEX_DIGITS_EXPECTED: UErrorCode = 66049;
pub const U_BRK_ERROR_START: UErrorCode = 66048;
pub const U_BRK_INTERNAL_ERROR: UErrorCode = 66048;
pub const U_FMT_PARSE_ERROR_LIMIT: UErrorCode = 65812;
pub const U_NUMBER_SKELETON_SYNTAX_ERROR: UErrorCode = 65811;
pub const U_NUMBER_ARG_OUTOFBOUNDS_ERROR: UErrorCode = 65810;
pub const U_FORMAT_INEXACT_ERROR: UErrorCode = 65809;
pub const U_DECIMAL_NUMBER_SYNTAX_ERROR: UErrorCode = 65808;
pub const U_DEFAULT_KEYWORD_MISSING: UErrorCode = 65807;
pub const U_UNDEFINED_KEYWORD: UErrorCode = 65806;
pub const U_DUPLICATE_KEYWORD: UErrorCode = 65805;
pub const U_ARGUMENT_TYPE_MISMATCH: UErrorCode = 65804;
pub const U_UNSUPPORTED_ATTRIBUTE: UErrorCode = 65803;
pub const U_UNSUPPORTED_PROPERTY: UErrorCode = 65802;
pub const U_UNMATCHED_BRACES: UErrorCode = 65801;
pub const U_ILLEGAL_PAD_POSITION: UErrorCode = 65800;
pub const U_PATTERN_SYNTAX_ERROR: UErrorCode = 65799;
pub const U_MULTIPLE_PAD_SPECIFIERS: UErrorCode = 65798;
pub const U_MULTIPLE_PERMILL_SYMBOLS: UErrorCode = 65797;
pub const U_MULTIPLE_PERCENT_SYMBOLS: UErrorCode = 65796;
pub const U_MALFORMED_EXPONENTIAL_PATTERN: UErrorCode = 65795;
pub const U_MULTIPLE_EXPONENTIAL_SYMBOLS: UErrorCode = 65794;
pub const U_MULTIPLE_DECIMAL_SEPERATORS: UErrorCode = 65793;
pub const U_MULTIPLE_DECIMAL_SEPARATORS: UErrorCode = 65793;
pub const U_FMT_PARSE_ERROR_START: UErrorCode = 65792;
pub const U_UNEXPECTED_TOKEN: UErrorCode = 65792;
pub const U_PARSE_ERROR_LIMIT: UErrorCode = 65571;
pub const U_INVALID_FUNCTION: UErrorCode = 65570;
pub const U_INVALID_ID: UErrorCode = 65569;
pub const U_INTERNAL_TRANSLITERATOR_ERROR: UErrorCode = 65568;
pub const U_ILLEGAL_CHARACTER: UErrorCode = 65567;
pub const U_VARIABLE_RANGE_OVERLAP: UErrorCode = 65566;
pub const U_VARIABLE_RANGE_EXHAUSTED: UErrorCode = 65565;
pub const U_ILLEGAL_CHAR_IN_SEGMENT: UErrorCode = 65564;
pub const U_UNCLOSED_SEGMENT: UErrorCode = 65563;
pub const U_MALFORMED_PRAGMA: UErrorCode = 65562;
pub const U_INVALID_PROPERTY_PATTERN: UErrorCode = 65561;
pub const U_INVALID_RBT_SYNTAX: UErrorCode = 65560;
pub const U_MULTIPLE_COMPOUND_FILTERS: UErrorCode = 65559;
pub const U_MISPLACED_COMPOUND_FILTER: UErrorCode = 65558;
pub const U_RULE_MASK_ERROR: UErrorCode = 65557;
pub const U_UNTERMINATED_QUOTE: UErrorCode = 65556;
pub const U_UNQUOTED_SPECIAL: UErrorCode = 65555;
pub const U_UNDEFINED_VARIABLE: UErrorCode = 65554;
pub const U_UNDEFINED_SEGMENT_REFERENCE: UErrorCode = 65553;
pub const U_TRAILING_BACKSLASH: UErrorCode = 65552;
pub const U_MULTIPLE_POST_CONTEXTS: UErrorCode = 65551;
pub const U_MULTIPLE_CURSORS: UErrorCode = 65550;
pub const U_MULTIPLE_ANTE_CONTEXTS: UErrorCode = 65549;
pub const U_MISSING_SEGMENT_CLOSE: UErrorCode = 65548;
pub const U_MISSING_OPERATOR: UErrorCode = 65547;
pub const U_MISPLACED_QUANTIFIER: UErrorCode = 65546;
pub const U_MISPLACED_CURSOR_OFFSET: UErrorCode = 65545;
pub const U_MISPLACED_ANCHOR_START: UErrorCode = 65544;
pub const U_MISMATCHED_SEGMENT_DELIMITERS: UErrorCode = 65543;
pub const U_MALFORMED_VARIABLE_REFERENCE: UErrorCode = 65542;
pub const U_MALFORMED_VARIABLE_DEFINITION: UErrorCode = 65541;
pub const U_MALFORMED_UNICODE_ESCAPE: UErrorCode = 65540;
pub const U_MALFORMED_SYMBOL_REFERENCE: UErrorCode = 65539;
pub const U_MALFORMED_SET: UErrorCode = 65538;
pub const U_MALFORMED_RULE: UErrorCode = 65537;
pub const U_PARSE_ERROR_START: UErrorCode = 65536;
pub const U_BAD_VARIABLE_DEFINITION: UErrorCode = 65536;
pub const U_STANDARD_ERROR_LIMIT: UErrorCode = 31;
pub const U_NO_WRITE_PERMISSION: UErrorCode = 30;
pub const U_USELESS_COLLATOR_ERROR: UErrorCode = 29;
pub const U_COLLATOR_VERSION_MISMATCH: UErrorCode = 28;
pub const U_INVALID_STATE_ERROR: UErrorCode = 27;
pub const U_INVARIANT_CONVERSION_ERROR: UErrorCode = 26;
pub const U_ENUM_OUT_OF_SYNC_ERROR: UErrorCode = 25;
pub const U_TOO_MANY_ALIASES_ERROR: UErrorCode = 24;
pub const U_STATE_TOO_OLD_ERROR: UErrorCode = 23;
pub const U_PRIMARY_TOO_LONG_ERROR: UErrorCode = 22;
pub const U_CE_NOT_FOUND_ERROR: UErrorCode = 21;
pub const U_NO_SPACE_AVAILABLE: UErrorCode = 20;
pub const U_UNSUPPORTED_ESCAPE_SEQUENCE: UErrorCode = 19;
pub const U_ILLEGAL_ESCAPE_SEQUENCE: UErrorCode = 18;
pub const U_RESOURCE_TYPE_MISMATCH: UErrorCode = 17;
pub const U_UNSUPPORTED_ERROR: UErrorCode = 16;
pub const U_BUFFER_OVERFLOW_ERROR: UErrorCode = 15;
pub const U_INVALID_TABLE_FILE: UErrorCode = 14;
pub const U_INVALID_TABLE_FORMAT: UErrorCode = 13;
pub const U_ILLEGAL_CHAR_FOUND: UErrorCode = 12;
pub const U_TRUNCATED_CHAR_FOUND: UErrorCode = 11;
pub const U_INVALID_CHAR_FOUND: UErrorCode = 10;
pub const U_PARSE_ERROR: UErrorCode = 9;
pub const U_INDEX_OUTOFBOUNDS_ERROR: UErrorCode = 8;
pub const U_MEMORY_ALLOCATION_ERROR: UErrorCode = 7;
pub const U_MESSAGE_PARSE_ERROR: UErrorCode = 6;
pub const U_INTERNAL_PROGRAM_ERROR: UErrorCode = 5;
pub const U_FILE_ACCESS_ERROR: UErrorCode = 4;
pub const U_INVALID_FORMAT_ERROR: UErrorCode = 3;
pub const U_MISSING_RESOURCE_ERROR: UErrorCode = 2;
pub const U_ILLEGAL_ARGUMENT_ERROR: UErrorCode = 1;
pub const U_ZERO_ERROR: UErrorCode = 0;
pub const U_ERROR_WARNING_LIMIT: UErrorCode = -119;
pub const U_PLUGIN_CHANGED_LEVEL_WARNING: UErrorCode = -120;
pub const U_DIFFERENT_UCA_VERSION: UErrorCode = -121;
pub const U_AMBIGUOUS_ALIAS_WARNING: UErrorCode = -122;
pub const U_SORT_KEY_TOO_SHORT_WARNING: UErrorCode = -123;
pub const U_STRING_NOT_TERMINATED_WARNING: UErrorCode = -124;
pub const U_STATE_OLD_WARNING: UErrorCode = -125;
pub const U_SAFECLONE_ALLOCATED_WARNING: UErrorCode = -126;
pub const U_USING_DEFAULT_WARNING: UErrorCode = -127;
pub const U_ERROR_WARNING_START: UErrorCode = -128;
pub const U_USING_FALLBACK_WARNING: UErrorCode = -128;
/* tectonic/xetex-core.h: core XeTeX types and #includes.
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/
// defines U_IS_BIG_ENDIAN for us
/* fontconfig */
/* freetype */
/* harfbuzz */
/* Endianness foo */
/* our typedefs */
pub type scaled_t = int32_t;
pub type UInt32 = libc::c_uint;
pub type CFIndex = libc::c_long;
pub type CFTypeRef = *const libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}
pub type CFArrayRef = *const __CFArray;
pub type CFBooleanRef = *const __CFBoolean;
pub type CFNumberType = CFIndex;
pub type C2RustUnnamed = libc::c_uint;
pub const kCFNumberMaxType: C2RustUnnamed = 16;
pub const kCFNumberCGFloatType: C2RustUnnamed = 16;
pub const kCFNumberNSIntegerType: C2RustUnnamed = 15;
pub const kCFNumberCFIndexType: C2RustUnnamed = 14;
pub const kCFNumberDoubleType: C2RustUnnamed = 13;
pub const kCFNumberFloatType: C2RustUnnamed = 12;
pub const kCFNumberLongLongType: C2RustUnnamed = 11;
pub const kCFNumberLongType: C2RustUnnamed = 10;
pub const kCFNumberIntType: C2RustUnnamed = 9;
pub const kCFNumberShortType: C2RustUnnamed = 8;
pub const kCFNumberCharType: C2RustUnnamed = 7;
pub const kCFNumberFloat64Type: C2RustUnnamed = 6;
pub const kCFNumberFloat32Type: C2RustUnnamed = 5;
pub const kCFNumberSInt64Type: C2RustUnnamed = 4;
pub const kCFNumberSInt32Type: C2RustUnnamed = 3;
pub const kCFNumberSInt16Type: C2RustUnnamed = 2;
pub const kCFNumberSInt8Type: C2RustUnnamed = 1;
pub type CFNumberRef = *const __CFNumber;
pub type CGFloat = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}
pub type CGColorRef = *mut CGColor;
pub type CTFontDescriptorRef = *const __CTFontDescriptor;
pub type CTFontRef = *const __CTFont;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FloatPoint {
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphBBox {
    pub xMin: libc::c_float,
    pub yMin: libc::c_float,
    pub xMax: libc::c_float,
    pub yMax: libc::c_float,
}
pub type PlatformFontRef = CTFontDescriptorRef;
pub type str_number = int32_t;
pub type UBreakIteratorType = libc::c_uint;
pub const UBRK_COUNT: UBreakIteratorType = 5;
pub const UBRK_TITLE: UBreakIteratorType = 4;
pub const UBRK_SENTENCE: UBreakIteratorType = 3;
pub const UBRK_LINE: UBreakIteratorType = 2;
pub const UBRK_WORD: UBreakIteratorType = 1;
pub const UBRK_CHARACTER: UBreakIteratorType = 0;
/* tectonic/xetex-xetexd.h -- many, many XeTeX symbol definitions
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/
/* include this here to avoid conflict between clang's emmintrin.h and
 * texmfmem.h. Should be removed once a fixed clang is widely available
 * http://llvm.org/bugs/show_bug.cgi?id=14964 */
/* Extra stuff used in various change files for various reasons.  */
/* Array allocations. Add 1 to size to account for Pascal indexing convention. */
/*11:*/
/*18: */
pub type UTF16_code = libc::c_ushort;
/*------------------------------------------------------------------------
Copyright (C) 2002-2016 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.

File: TECkit_Common.h
Responsibility: Jonathan Kew
Last reviewed: Not yet.

Description:
    Public definitions used by TECkit engine and compiler
-------------------------------------------------------------------------*/
/*
    Common types and defines for the engine and compiler

History:
    16-Sep-2006		jk	updated version to 2.4 (adding new compiler APIs for Bob E)
    23-May-2005		jk	patch for 64-bit architectures (thanks to Ulrik P)
    18-Mar-2005		jk	updated minor version for 2.3 (engine unchanged, XML option in compiler)
    23-Sep-2003		jk	updated for version 2.1 - extended status values
    xx-xxx-2002		jk	version 2.0 initial release
*/
/* 16.16 version number */
/*
    all public functions return a status code
*/
pub type TECkit_Status = libc::c_long;
/* The annoying `memory_word` type. We have to make sure the byte-swapping
 * that the (un)dumping routines do suffices to put things in the right place
 * in memory.
 *
 * This set of data used to be a huge mess (see comment after the
 * definitions). It is now (IMO) a lot more reasonable, but there will no
 * doubt be carryover weird terminology around the code.
 *
 * ## ENDIANNESS (cheat sheet because I'm lame)
 *
 * Intel is little-endian. Say that we have a 32-bit integer stored in memory
 * with `p` being a `uint8` pointer to its location. In little-endian land,
 * `p[0]` is least significant byte and `p[3]` is its most significant byte.
 *
 * Conversely, in big-endian land, `p[0]` is its most significant byte and
 * `p[3]` is its least significant byte.
 *
 * ## MEMORY_WORD LAYOUT
 *
 * Little endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [lsb......s0.......msb] [lsb......s1.......msb]
 *   b16:   [l..s0...m] [l..s1...m] [l..s2...m] [l..s3...m]
 *
 * Big endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [msb......s1.......lsb] [msb......s0.......lsb]
 *   b16:   [m..s3...l] [m..s2...l] [m..s1...l] [m...s0..l]
 *
 */
pub type b32x2 = b32x2_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2_le_t {
    pub s0: int32_t,
    pub s1: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: libc::c_double,
    pub ptr: *mut libc::c_void,
}
pub type b16x4 = b16x4_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4_le_t {
    pub s0: uint16_t,
    pub s1: uint16_t,
    pub s2: uint16_t,
    pub s3: uint16_t,
}
pub const UBIDI_RTL: UBiDiDirection = 1;
pub type UBiDiDirection = libc::c_uint;
pub const UBIDI_NEUTRAL: UBiDiDirection = 3;
pub const UBIDI_MIXED: UBiDiDirection = 2;
pub const UBIDI_LTR: UBiDiDirection = 0;
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn strstartswith(
    mut s: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    return 0 as *const libc::c_char;
}
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
/* ***************************************************************************\
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
#[inline]
unsafe extern "C" fn SWAP16(p: uint16_t) -> uint16_t {
    return ((p as libc::c_int >> 8i32) + ((p as libc::c_int) << 8i32)) as uint16_t;
}
#[inline]
unsafe extern "C" fn SWAP32(p: uint32_t) -> uint32_t {
    return (p >> 24i32)
        .wrapping_add(p >> 8i32 & 0xff00i32 as libc::c_uint)
        .wrapping_add(p << 8i32 & 0xff0000i32 as libc::c_uint)
        .wrapping_add(p << 24i32);
}
#[inline]
unsafe extern "C" fn print_c_string(mut str: *const libc::c_char) {
    while *str != 0 {
        let fresh0 = str;
        str = str.offset(1);
        print_char(*fresh0 as int32_t);
    }
}
static mut brkIter: *mut UBreakIterator = 0 as *const UBreakIterator as *mut UBreakIterator;
static mut brkLocaleStrNum: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn linebreak_start(
    mut f: libc::c_int,
    mut localeStrNum: int32_t,
    mut text: *mut uint16_t,
    mut textLength: int32_t,
) {
    let mut status: UErrorCode = U_ZERO_ERROR;
    let mut locale: *mut libc::c_char = gettexstring(localeStrNum);
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32
        && streq_ptr(locale, b"G\x00" as *const u8 as *const libc::c_char) as libc::c_int != 0
    {
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
        if initGraphiteBreaking(engine, text, textLength) {
            /* user asked for Graphite line breaking and the font supports it */
            return;
        }
    }
    if localeStrNum != brkLocaleStrNum && !brkIter.is_null() {
        ubrk_close_64(brkIter);
        brkIter = 0 as *mut UBreakIterator
    }
    if brkIter.is_null() {
        brkIter = ubrk_open_64(UBRK_LINE, locale, 0 as *const UChar, 0i32, &mut status);
        if status as libc::c_int > U_ZERO_ERROR as libc::c_int {
            begin_diagnostic();
            print_nl('E' as i32);
            print_c_string(b"rror \x00" as *const u8 as *const libc::c_char);
            print_int(status as int32_t);
            print_c_string(
                b" creating linebreak iterator for locale `\x00" as *const u8
                    as *const libc::c_char,
            );
            print_c_string(locale);
            print_c_string(
                b"\'; trying default locale `en_us\'.\x00" as *const u8 as *const libc::c_char,
            );
            end_diagnostic(1i32 != 0);
            if !brkIter.is_null() {
                ubrk_close_64(brkIter);
            }
            status = U_ZERO_ERROR;
            brkIter = ubrk_open_64(
                UBRK_LINE,
                b"en_us\x00" as *const u8 as *const libc::c_char,
                0 as *const UChar,
                0i32,
                &mut status,
            )
        }
        free(locale as *mut libc::c_void);
        brkLocaleStrNum = localeStrNum
    }
    if brkIter.is_null() {
        _tt_abort(
            b"failed to create linebreak iterator, status=%d\x00" as *const u8
                as *const libc::c_char,
            status as libc::c_int,
        );
    }
    ubrk_setText_64(brkIter, text as *mut UChar, textLength, &mut status);
}
#[no_mangle]
pub unsafe extern "C" fn linebreak_next() -> libc::c_int {
    if !brkIter.is_null() {
        return ubrk_next_64(brkIter);
    } else {
        return findNextGraphiteBreak();
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_encoding_mode_and_info(mut info: *mut int32_t) -> libc::c_int {
    /* \XeTeXinputencoding "enc-name"
     *   -> name is packed in |nameoffile| as a C string, starting at [1]
     * Check if it's a built-in name; if not, try to open an ICU converter by that name
     */
    let mut err: UErrorCode = U_ZERO_ERROR;
    let mut cnv: *mut UConverter = 0 as *mut UConverter;
    *info = 0i32;
    if strcasecmp(
        name_of_file,
        b"auto\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        return 0i32;
    }
    if strcasecmp(
        name_of_file,
        b"utf8\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        return 1i32;
    }
    if strcasecmp(
        name_of_file,
        b"utf16\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        /* depends on host platform */
        return 3i32;
    }
    if strcasecmp(
        name_of_file,
        b"utf16be\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        return 2i32;
    }
    if strcasecmp(
        name_of_file,
        b"utf16le\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        return 3i32;
    }
    if strcasecmp(
        name_of_file,
        b"bytes\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        return 4i32;
    }
    /* try for an ICU converter */
    cnv = ucnv_open_64(name_of_file, &mut err); /* ensure message starts on a new line */
    if cnv.is_null() {
        begin_diagnostic();
        print_nl('U' as i32);
        print_c_string(b"nknown encoding `\x00" as *const u8 as *const libc::c_char);
        print_c_string(name_of_file);
        print_c_string(b"\'; reading as raw bytes\x00" as *const u8 as *const libc::c_char);
        end_diagnostic(1i32 != 0);
        return 4i32;
    } else {
        ucnv_close_64(cnv);
        *info = maketexstring(name_of_file);
        return 5i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_utf8_str(mut str: *const libc::c_uchar, mut len: libc::c_int) {
    loop {
        let fresh1 = len;
        len = len - 1;
        if !(fresh1 > 0i32) {
            break;
        }
        let fresh2 = str;
        str = str.offset(1);
        print_raw_char(*fresh2 as UTF16_code, 1i32 != 0);
    }
    /* bypass utf-8 encoding done in print_char() */
}
#[no_mangle]
pub unsafe extern "C" fn print_chars(mut str: *const libc::c_ushort, mut len: libc::c_int) {
    loop {
        let fresh3 = len;
        len = len - 1;
        if !(fresh3 > 0i32) {
            break;
        }
        let fresh4 = str;
        str = str.offset(1);
        print_char(*fresh4 as int32_t);
    }
}
unsafe extern "C" fn load_mapping_file(
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
    mut byteMapping: libc::c_char,
) -> *mut libc::c_void {
    let mut cnv: TECkit_Converter = 0 as TECkit_Converter;
    let mut buffer: *mut libc::c_char =
        xmalloc((e.wrapping_offset_from(s) as libc::c_long + 5i32 as libc::c_long) as size_t)
            as *mut libc::c_char;
    let mut map: rust_input_handle_t = 0 as *mut libc::c_void;
    strncpy(
        buffer,
        s,
        e.wrapping_offset_from(s) as libc::c_long as libc::c_ulong,
    );
    *buffer.offset(e.wrapping_offset_from(s) as libc::c_long as isize) = 0i32 as libc::c_char;
    strcat(buffer, b".tec\x00" as *const u8 as *const libc::c_char);
    map = ttstub_input_open(buffer, TTIF_MISCFONTS, 0i32);
    if !map.is_null() {
        let mut mappingSize: size_t = ttstub_input_get_size(map);
        let mut mapping: *mut Byte = xmalloc(mappingSize) as *mut Byte;
        let mut r: ssize_t = ttstub_input_read(map, mapping as *mut libc::c_char, mappingSize);
        if r < 0i32 as libc::c_long || r as size_t != mappingSize {
            _tt_abort(
                b"could not read mapping file \"%s\"\x00" as *const u8 as *const libc::c_char,
                buffer,
            );
        }
        ttstub_input_close(map);
        if byteMapping as libc::c_int != 0i32 {
            TECkit_CreateConverter(
                mapping,
                mappingSize as UInt32,
                0i32 as Byte,
                4i32 as UInt16,
                1i32 as UInt16,
                &mut cnv,
            );
        } else {
            TECkit_CreateConverter(
                mapping,
                mappingSize as UInt32,
                1i32 as Byte,
                4i32 as UInt16,
                4i32 as UInt16,
                &mut cnv,
            );
        }
        if cnv.is_null() {
            /* tracing */
            font_mapping_warning(
                buffer as *const libc::c_void,
                strlen(buffer) as int32_t,
                2i32,
            ); /* not loadable */
        } else if get_tracing_fonts_state() > 1i32 {
            font_mapping_warning(
                buffer as *const libc::c_void,
                strlen(buffer) as int32_t,
                0i32,
            );
        }
    } else {
        font_mapping_warning(
            buffer as *const libc::c_void,
            strlen(buffer) as int32_t,
            1i32,
        );
        /* not found */
    }
    free(buffer as *mut libc::c_void);
    return cnv as *mut libc::c_void;
}
static mut saved_mapping_name: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn check_for_tfm_font_mapping() {
    let mut cp: *mut libc::c_char = strstr(
        name_of_file,
        b":mapping=\x00" as *const u8 as *const libc::c_char,
    );
    saved_mapping_name = mfree(saved_mapping_name as *mut libc::c_void) as *mut libc::c_char;
    if !cp.is_null() {
        *cp = 0i32 as libc::c_char;
        cp = cp.offset(9);
        while *cp as libc::c_int != 0 && *cp as libc::c_int <= ' ' as i32 {
            cp = cp.offset(1)
        }
        if *cp != 0 {
            saved_mapping_name = xstrdup(cp)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn load_tfm_font_mapping() -> *mut libc::c_void {
    let mut rval: *mut libc::c_void = 0 as *mut libc::c_void;
    if !saved_mapping_name.is_null() {
        rval = load_mapping_file(
            saved_mapping_name,
            saved_mapping_name.offset(strlen(saved_mapping_name) as isize),
            1i32 as libc::c_char,
        );
        saved_mapping_name = mfree(saved_mapping_name as *mut libc::c_void) as *mut libc::c_char
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn apply_tfm_font_mapping(
    mut cnv: *mut libc::c_void,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut in_0: UniChar = c as UniChar;
    let mut out: [Byte; 2] = [0; 2];
    let mut inUsed: UInt32 = 0;
    let mut outUsed: UInt32 = 0;
    /* TECkit_Status status; */
    /* status = */
    TECkit_ConvertBuffer(
        cnv as TECkit_Converter,
        &mut in_0 as *mut UniChar as *const Byte,
        ::std::mem::size_of::<UniChar>() as libc::c_ulong as UInt32,
        &mut inUsed,
        out.as_mut_ptr(),
        ::std::mem::size_of::<[Byte; 2]>() as libc::c_ulong as UInt32,
        &mut outUsed,
        1i32 as Byte,
    );
    if outUsed < 1i32 as libc::c_uint {
        return 0i32;
    } else {
        return out[0] as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn read_double(mut s: *mut *const libc::c_char) -> libc::c_double {
    let mut neg: libc::c_int = 0i32;
    let mut val: libc::c_double = 0.0f64;
    let mut cp: *const libc::c_char = *s;
    while *cp as libc::c_int == ' ' as i32 || *cp as libc::c_int == '\t' as i32 {
        cp = cp.offset(1)
    }
    if *cp as libc::c_int == '-' as i32 {
        neg = 1i32;
        cp = cp.offset(1)
    } else if *cp as libc::c_int == '+' as i32 {
        cp = cp.offset(1)
    }
    while *cp as libc::c_int >= '0' as i32 && *cp as libc::c_int <= '9' as i32 {
        val = val * 10.0f64 + *cp as libc::c_int as libc::c_double - '0' as i32 as libc::c_double;
        cp = cp.offset(1)
    }
    if *cp as libc::c_int == '.' as i32 {
        let mut dec: libc::c_double = 10.0f64;
        cp = cp.offset(1);
        while *cp as libc::c_int >= '0' as i32 && *cp as libc::c_int <= '9' as i32 {
            val = val + (*cp as libc::c_int - '0' as i32) as libc::c_double / dec;
            cp = cp.offset(1);
            dec = dec * 10.0f64
        }
    }
    *s = cp;
    return if neg != 0 { -val } else { val };
}
unsafe extern "C" fn read_tag_with_param(
    mut cp: *const libc::c_char,
    mut param: *mut libc::c_int,
) -> hb_tag_t {
    let mut cp2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tag: hb_tag_t = 0;
    cp2 = cp;
    while *cp2 as libc::c_int != 0
        && *cp2 as libc::c_int != ':' as i32
        && *cp2 as libc::c_int != ';' as i32
        && *cp2 as libc::c_int != ',' as i32
        && *cp2 as libc::c_int != '=' as i32
    {
        cp2 = cp2.offset(1)
    }
    tag = hb_tag_from_string(
        cp,
        cp2.wrapping_offset_from(cp) as libc::c_long as libc::c_int,
    );
    cp = cp2;
    if *cp as libc::c_int == '=' as i32 {
        let mut neg: libc::c_int = 0i32;
        cp = cp.offset(1);
        if *cp as libc::c_int == '-' as i32 {
            neg += 1;
            cp = cp.offset(1)
        }
        while *cp as libc::c_int >= '0' as i32 && *cp as libc::c_int <= '9' as i32 {
            *param = *param * 10i32 + *cp as libc::c_int - '0' as i32;
            cp = cp.offset(1)
        }
        if neg != 0 {
            *param = -*param
        }
    }
    return tag;
}
#[no_mangle]
pub unsafe extern "C" fn read_rgb_a(mut cp: *mut *const libc::c_char) -> libc::c_uint {
    let mut rgbValue: uint32_t = 0i32 as uint32_t;
    let mut alpha: uint32_t = 0i32 as uint32_t;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 6i32 {
        if **cp as libc::c_int >= '0' as i32 && **cp as libc::c_int <= '9' as i32 {
            rgbValue = (rgbValue << 4i32)
                .wrapping_add(**cp as libc::c_uint)
                .wrapping_sub('0' as i32 as libc::c_uint)
        } else if **cp as libc::c_int >= 'A' as i32 && **cp as libc::c_int <= 'F' as i32 {
            rgbValue = (rgbValue << 4i32)
                .wrapping_add(**cp as libc::c_uint)
                .wrapping_sub('A' as i32 as libc::c_uint)
                .wrapping_add(10i32 as libc::c_uint)
        } else if **cp as libc::c_int >= 'a' as i32 && **cp as libc::c_int <= 'f' as i32 {
            rgbValue = (rgbValue << 4i32)
                .wrapping_add(**cp as libc::c_uint)
                .wrapping_sub('a' as i32 as libc::c_uint)
                .wrapping_add(10i32 as libc::c_uint)
        } else {
            return 0xffi32 as libc::c_uint;
        }
        *cp = (*cp).offset(1);
        i += 1
    }
    rgbValue <<= 8i32;
    i = 0i32;
    while i < 2i32 {
        if **cp as libc::c_int >= '0' as i32 && **cp as libc::c_int <= '9' as i32 {
            alpha = (alpha << 4i32)
                .wrapping_add(**cp as libc::c_uint)
                .wrapping_sub('0' as i32 as libc::c_uint)
        } else if **cp as libc::c_int >= 'A' as i32 && **cp as libc::c_int <= 'F' as i32 {
            alpha = (alpha << 4i32)
                .wrapping_add(**cp as libc::c_uint)
                .wrapping_sub('A' as i32 as libc::c_uint)
                .wrapping_add(10i32 as libc::c_uint)
        } else {
            if !(**cp as libc::c_int >= 'a' as i32 && **cp as libc::c_int <= 'f' as i32) {
                break;
            }
            alpha = (alpha << 4i32)
                .wrapping_add(**cp as libc::c_uint)
                .wrapping_sub('a' as i32 as libc::c_uint)
                .wrapping_add(10i32 as libc::c_uint)
        }
        *cp = (*cp).offset(1);
        i += 1
    }
    if i == 2i32 {
        rgbValue = (rgbValue as libc::c_uint).wrapping_add(alpha) as uint32_t as uint32_t
    } else {
        rgbValue =
            (rgbValue as libc::c_uint).wrapping_add(0xffi32 as libc::c_uint) as uint32_t as uint32_t
    }
    return rgbValue;
}
#[no_mangle]
pub unsafe extern "C" fn readCommonFeatures(
    mut feat: *const libc::c_char,
    mut end: *const libc::c_char,
    mut extend: *mut libc::c_float,
    mut slant: *mut libc::c_float,
    mut embolden: *mut libc::c_float,
    mut letterspace: *mut libc::c_float,
    mut rgbValue: *mut uint32_t,
) -> libc::c_int
// returns 1 to go to next_option, -1 for bad_option, 0 to continue
{
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    sep = strstartswith(feat, b"mapping\x00" as *const u8 as *const libc::c_char);
    if !sep.is_null() {
        if *sep as libc::c_int != '=' as i32 {
            return -1i32;
        }
        loaded_font_mapping = load_mapping_file(sep.offset(1), end, 0i32 as libc::c_char);
        return 1i32;
    }
    sep = strstartswith(feat, b"extend\x00" as *const u8 as *const libc::c_char);
    if !sep.is_null() {
        if *sep as libc::c_int != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        *extend = read_double(&mut sep) as libc::c_float;
        return 1i32;
    }
    sep = strstartswith(feat, b"slant\x00" as *const u8 as *const libc::c_char);
    if !sep.is_null() {
        if *sep as libc::c_int != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        *slant = read_double(&mut sep) as libc::c_float;
        return 1i32;
    }
    sep = strstartswith(feat, b"embolden\x00" as *const u8 as *const libc::c_char);
    if !sep.is_null() {
        if *sep as libc::c_int != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        *embolden = read_double(&mut sep) as libc::c_float;
        return 1i32;
    }
    sep = strstartswith(feat, b"letterspace\x00" as *const u8 as *const libc::c_char);
    if !sep.is_null() {
        if *sep as libc::c_int != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        *letterspace = read_double(&mut sep) as libc::c_float;
        return 1i32;
    }
    sep = strstartswith(feat, b"color\x00" as *const u8 as *const libc::c_char);
    if !sep.is_null() {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        if *sep as libc::c_int != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        s = sep;
        *rgbValue = read_rgb_a(&mut sep);
        if sep == s.offset(6) || sep == s.offset(8) {
            loaded_font_flags = (loaded_font_flags as libc::c_int | 0x1i32) as libc::c_char
        } else {
            return -1i32;
        }
        return 1i32;
    }
    return 0i32;
}
unsafe extern "C" fn readFeatureNumber(
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
    mut f: *mut hb_tag_t,
    mut v: *mut libc::c_int,
) -> bool
/* s...e is a "id=setting" string; */ {
    *f = 0i32 as hb_tag_t;
    *v = 0i32;
    if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
        return 0i32 != 0;
    }
    while *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
        let fresh5 = s;
        s = s.offset(1);
        *f = (*f)
            .wrapping_mul(10i32 as libc::c_uint)
            .wrapping_add(*fresh5 as libc::c_uint)
            .wrapping_sub('0' as i32 as libc::c_uint)
    }
    while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32 {
        s = s.offset(1)
    }
    let fresh6 = s;
    s = s.offset(1);
    if *fresh6 as libc::c_int != '=' as i32 {
        /* no setting was specified */
        return 0i32 != 0;
    } /* NULL-terminated array */
    if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
        return 0i32 != 0;
    }
    while *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
        let fresh7 = s;
        s = s.offset(1);
        *v = *v * 10i32 + *fresh7 as libc::c_int - '0' as i32
    }
    while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32 {
        s = s.offset(1)
    }
    if s != e {
        return 0i32 != 0;
    }
    return 1i32 != 0;
}
unsafe extern "C" fn loadOTfont(
    mut fontRef: PlatformFontRef,
    mut font: XeTeXFont,
    mut scaled_size: Fixed,
    mut cp1: *mut libc::c_char,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut engine: XeTeXLayoutEngine = 0 as XeTeXLayoutEngine;
    let mut script: hb_tag_t = (0i32 as uint32_t & 0xffi32 as libc::c_uint) << 24i32
        | (0i32 as uint32_t & 0xffi32 as libc::c_uint) << 16i32
        | (0i32 as uint32_t & 0xffi32 as libc::c_uint) << 8i32
        | 0i32 as uint32_t & 0xffi32 as libc::c_uint;
    let mut language: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut features: *mut hb_feature_t = 0 as *mut hb_feature_t;
    let mut shapers: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nFeatures: libc::c_int = 0i32;
    let mut nShapers: libc::c_int = 0i32;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tag: hb_tag_t = 0;
    let mut rgbValue: uint32_t = 0xffi32 as uint32_t;
    let mut extend: libc::c_float = 1.0f64 as libc::c_float;
    let mut slant: libc::c_float = 0.0f64 as libc::c_float;
    let mut embolden: libc::c_float = 0.0f64 as libc::c_float;
    let mut letterspace: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut reqEngine: libc::c_char = getReqEngine();
    if reqEngine as libc::c_int == 'O' as i32 || reqEngine as libc::c_int == 'G' as i32 {
        shapers = xrealloc(
            shapers as *mut libc::c_void,
            ((nShapers + 1i32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        if reqEngine as libc::c_int == 'O' as i32 {
            static mut ot_const: [libc::c_char; 3] = [111, 116, 0];
            let ref mut fresh8 = *shapers.offset(nShapers as isize);
            *fresh8 = ot_const.as_mut_ptr()
        } else if reqEngine as libc::c_int == 'G' as i32 {
            static mut graphite2_const: [libc::c_char; 10] =
                [103, 114, 97, 112, 104, 105, 116, 101, 50, 0];
            let ref mut fresh9 = *shapers.offset(nShapers as isize);
            *fresh9 = graphite2_const.as_mut_ptr()
        }
        nShapers += 1
    }
    if reqEngine as libc::c_int == 'G' as i32 {
        let mut tmpShapers: [*mut libc::c_char; 1] = [*shapers.offset(0)];
        /* create a default engine so we can query the font for Graphite features;
         * because of font caching, it's cheap to discard this and create the real one later */
        engine = createLayoutEngine(
            fontRef,
            font,
            script,
            language,
            features,
            nFeatures,
            tmpShapers.as_mut_ptr(),
            rgbValue,
            extend,
            slant,
            embolden,
        );
        if engine.is_null() {
            return 0 as *mut libc::c_void;
        }
    }
    /* scan the feature string (if any) */
    if !cp1.is_null() {
        while *cp1 != 0 {
            if *cp1 as libc::c_int == ':' as i32
                || *cp1 as libc::c_int == ';' as i32
                || *cp1 as libc::c_int == ',' as i32
            {
                cp1 = cp1.offset(1)
            }
            while *cp1 as libc::c_int == ' ' as i32 || *cp1 as libc::c_int == '\t' as i32 {
                /* skip leading whitespace */
                cp1 = cp1.offset(1)
            }
            if *cp1 as libc::c_int == 0i32 {
                break;
            }
            cp2 = cp1;
            while *cp2 as libc::c_int != 0
                && *cp2 as libc::c_int != ':' as i32
                && *cp2 as libc::c_int != ';' as i32
                && *cp2 as libc::c_int != ',' as i32
            {
                cp2 = cp2.offset(1)
            }
            cp3 = strstartswith(cp1, b"script\x00" as *const u8 as *const libc::c_char);
            if !cp3.is_null() {
                if *cp3 as libc::c_int != '=' as i32 {
                    current_block = 8329762136643326403;
                } else {
                    cp3 = cp3.offset(1);
                    script = hb_tag_from_string(
                        cp3,
                        cp2.wrapping_offset_from(cp3) as libc::c_long as libc::c_int,
                    );
                    current_block = 7725042370488302866;
                }
            } else {
                cp3 = strstartswith(cp1, b"language\x00" as *const u8 as *const libc::c_char);
                if !cp3.is_null() {
                    if *cp3 as libc::c_int != '=' as i32 {
                        current_block = 8329762136643326403;
                    } else {
                        cp3 = cp3.offset(1);
                        language = xmalloc(
                            (cp2.wrapping_offset_from(cp3) as libc::c_long + 1i32 as libc::c_long)
                                as size_t,
                        ) as *mut libc::c_char;
                        *language.offset(cp2.wrapping_offset_from(cp3) as libc::c_long as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        memcpy(
                            language as *mut libc::c_void,
                            cp3 as *const libc::c_void,
                            cp2.wrapping_offset_from(cp3) as libc::c_long as libc::c_ulong,
                        );
                        current_block = 7725042370488302866;
                    }
                } else {
                    cp3 = strstartswith(cp1, b"shaper\x00" as *const u8 as *const libc::c_char);
                    if !cp3.is_null() {
                        if *cp3 as libc::c_int != '=' as i32 {
                            current_block = 8329762136643326403;
                        } else {
                            cp3 = cp3.offset(1);
                            shapers = xrealloc(
                                shapers as *mut libc::c_void,
                                ((nShapers + 1i32) as libc::c_ulong).wrapping_mul(
                                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ),
                            ) as *mut *mut libc::c_char;
                            /* some dumb systems have no strndup() */
                            let ref mut fresh10 = *shapers.offset(nShapers as isize);
                            *fresh10 = strdup(cp3);
                            *(*shapers.offset(nShapers as isize))
                                .offset(cp2.wrapping_offset_from(cp3) as libc::c_long as isize) =
                                '\u{0}' as i32 as libc::c_char;
                            nShapers += 1;
                            current_block = 7725042370488302866;
                        }
                    } else {
                        i = readCommonFeatures(
                            cp1,
                            cp2,
                            &mut extend,
                            &mut slant,
                            &mut embolden,
                            &mut letterspace,
                            &mut rgbValue,
                        );
                        if i == 1i32 {
                            current_block = 7725042370488302866;
                        } else if i == -1i32 {
                            current_block = 8329762136643326403;
                        } else {
                            if reqEngine as libc::c_int == 'G' as i32 {
                                let mut value: libc::c_int = 0i32;
                                if readFeatureNumber(cp1, cp2, &mut tag, &mut value) as libc::c_int
                                    != 0
                                    || findGraphiteFeature(engine, cp1, cp2, &mut tag, &mut value)
                                        as libc::c_int
                                        != 0
                                {
                                    features = xrealloc(
                                        features as *mut libc::c_void,
                                        ((nFeatures + 1i32) as libc::c_ulong)
                                            .wrapping_mul(::std::mem::size_of::<hb_feature_t>()
                                                as libc::c_ulong),
                                    )
                                        as *mut hb_feature_t;
                                    (*features.offset(nFeatures as isize)).tag = tag;
                                    (*features.offset(nFeatures as isize)).value =
                                        value as uint32_t;
                                    (*features.offset(nFeatures as isize)).start =
                                        0i32 as libc::c_uint;
                                    (*features.offset(nFeatures as isize)).end =
                                        -1i32 as libc::c_uint;
                                    nFeatures += 1;
                                    current_block = 7725042370488302866;
                                } else {
                                    current_block = 15669289850109000831;
                                }
                            } else {
                                current_block = 15669289850109000831;
                            }
                            match current_block {
                                7725042370488302866 => {}
                                _ => {
                                    if *cp1 as libc::c_int == '+' as i32 {
                                        let mut param: libc::c_int = 0i32;
                                        tag = read_tag_with_param(cp1.offset(1), &mut param);
                                        features = xrealloc(
                                            features as *mut libc::c_void,
                                            ((nFeatures + 1i32) as libc::c_ulong).wrapping_mul(
                                                ::std::mem::size_of::<hb_feature_t>()
                                                    as libc::c_ulong,
                                            ),
                                        )
                                            as *mut hb_feature_t;
                                        (*features.offset(nFeatures as isize)).tag = tag;
                                        (*features.offset(nFeatures as isize)).start =
                                            0i32 as libc::c_uint;
                                        (*features.offset(nFeatures as isize)).end =
                                            -1i32 as libc::c_uint;
                                        // for backward compatibility with pre-0.9999 where feature
                                        // indices started from 0
                                        if param >= 0i32 {
                                            param += 1
                                        }
                                        (*features.offset(nFeatures as isize)).value =
                                            param as uint32_t;
                                        nFeatures += 1;
                                        current_block = 7725042370488302866;
                                    } else if *cp1 as libc::c_int == '-' as i32 {
                                        cp1 = cp1.offset(1);
                                        tag = hb_tag_from_string(
                                            cp1,
                                            cp2.wrapping_offset_from(cp1) as libc::c_long
                                                as libc::c_int,
                                        );
                                        features = xrealloc(
                                            features as *mut libc::c_void,
                                            ((nFeatures + 1i32) as libc::c_ulong).wrapping_mul(
                                                ::std::mem::size_of::<hb_feature_t>()
                                                    as libc::c_ulong,
                                            ),
                                        )
                                            as *mut hb_feature_t;
                                        (*features.offset(nFeatures as isize)).tag = tag;
                                        (*features.offset(nFeatures as isize)).start =
                                            0i32 as libc::c_uint;
                                        (*features.offset(nFeatures as isize)).end =
                                            -1i32 as libc::c_uint;
                                        (*features.offset(nFeatures as isize)).value =
                                            0i32 as uint32_t;
                                        nFeatures += 1;
                                        current_block = 7725042370488302866;
                                    } else if !strstartswith(
                                        cp1,
                                        b"vertical\x00" as *const u8 as *const libc::c_char,
                                    )
                                    .is_null()
                                    {
                                        cp3 = cp2;
                                        if *cp3 as libc::c_int == ';' as i32
                                            || *cp3 as libc::c_int == ':' as i32
                                            || *cp3 as libc::c_int == ',' as i32
                                        {
                                            cp3 = cp3.offset(-1)
                                        }
                                        while *cp3 as libc::c_int == '\u{0}' as i32
                                            || *cp3 as libc::c_int == ' ' as i32
                                            || *cp3 as libc::c_int == '\t' as i32
                                        {
                                            cp3 = cp3.offset(-1)
                                        }
                                        if *cp3 != 0 {
                                            cp3 = cp3.offset(1)
                                        }
                                        if cp3 == cp1.offset(8) as *const libc::c_char {
                                            loaded_font_flags = (loaded_font_flags as libc::c_int
                                                | 0x2i32)
                                                as libc::c_char;
                                            current_block = 7725042370488302866;
                                        } else {
                                            current_block = 8329762136643326403;
                                        }
                                    } else {
                                        current_block = 8329762136643326403;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                8329762136643326403 => {
                    font_feature_warning(
                        cp1 as *mut libc::c_void,
                        cp2.wrapping_offset_from(cp1) as libc::c_long as int32_t,
                        0 as *const libc::c_void,
                        0i32,
                    );
                }
                _ => {}
            }
            cp1 = cp2
        }
    }
    /* break if end of string */
    if !shapers.is_null() {
        shapers = xrealloc(
            shapers as *mut libc::c_void,
            ((nShapers + 1i32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        let ref mut fresh11 = *shapers.offset(nShapers as isize);
        *fresh11 = 0 as *mut libc::c_char
    }
    if embolden as libc::c_double != 0.0f64 {
        embolden = (embolden as libc::c_double * Fix2D(scaled_size) / 100.0f64) as libc::c_float
    }
    if letterspace as libc::c_double != 0.0f64 {
        loaded_font_letter_space =
            (letterspace as libc::c_double / 100.0f64 * scaled_size as libc::c_double) as scaled_t
    }
    if loaded_font_flags as libc::c_int & 0x1i32 == 0i32 {
        rgbValue = 0xffi32 as uint32_t
    }
    if loaded_font_flags as libc::c_int & 0x2i32 != 0i32 {
        setFontLayoutDir(font, 1i32);
    }
    engine = createLayoutEngine(
        fontRef, font, script, language, features, nFeatures, shapers, rgbValue, extend, slant,
        embolden,
    );
    if engine.is_null() {
        // only free these if creation failed, otherwise the engine now owns them
        free(features as *mut libc::c_void);
        free(shapers as *mut libc::c_void);
    } else {
        native_font_type_flag = 0xfffeu32 as int32_t
    }
    return engine as *mut libc::c_void;
}
#[inline(always)]
unsafe extern "C" fn CFRangeMake(mut loc: CFIndex, mut len: CFIndex) -> CFRange {
    let mut range: CFRange = CFRange {
        location: 0,
        length: 0,
    };
    range.location = loc;
    range.length = len;
    return range;
}
unsafe extern "C" fn splitFontName(
    mut name: *mut libc::c_char,
    mut var: *mut *mut libc::c_char,
    mut feat: *mut *mut libc::c_char,
    mut end: *mut *mut libc::c_char,
    mut index: *mut libc::c_int,
) {
    *var = 0 as *mut libc::c_char;
    *feat = 0 as *mut libc::c_char;
    *index = 0i32;
    if *name as libc::c_int == '[' as i32 {
        let mut withinFileName: libc::c_int = 1i32;
        name = name.offset(1);
        while *name != 0 {
            if withinFileName != 0 && *name as libc::c_int == ']' as i32 {
                withinFileName = 0i32;
                if (*var).is_null() {
                    *var = name
                }
            } else if *name as libc::c_int == ':' as i32 {
                if withinFileName != 0 && (*var).is_null() {
                    *var = name;
                    name = name.offset(1);
                    while *name as libc::c_int >= '0' as i32 && *name as libc::c_int <= '9' as i32 {
                        let fresh12 = name;
                        name = name.offset(1);
                        *index = *index * 10i32 + *fresh12 as libc::c_int - '0' as i32
                    }
                    name = name.offset(-1)
                } else if withinFileName == 0 && (*feat).is_null() {
                    *feat = name
                }
            }
            name = name.offset(1)
        }
        *end = name
    } else {
        while *name != 0 {
            if *name as libc::c_int == '/' as i32 && (*var).is_null() && (*feat).is_null() {
                *var = name
            } else if *name as libc::c_int == ':' as i32 && (*feat).is_null() {
                *feat = name
            }
            name = name.offset(1)
        }
        *end = name
    }
    if (*feat).is_null() {
        *feat = name
    }
    if (*var).is_null() {
        *var = *feat
    };
}
#[no_mangle]
pub unsafe extern "C" fn find_native_font(
    mut uname: *mut libc::c_char,
    mut scaled_size: int32_t,
) -> *mut libc::c_void
/* scaled_size here is in TeX points, or is a negative integer for 'scaled_t' */ {
    let mut rval: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nameString: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut feat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = uname;
    let mut varString: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut featString: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fontRef: PlatformFontRef = 0 as *const __CTFontDescriptor;
    let mut font: XeTeXFont = 0 as XeTeXFont;
    let mut index: libc::c_int = 0i32;
    loaded_font_mapping = 0 as *mut libc::c_void;
    loaded_font_flags = 0i32 as libc::c_char;
    loaded_font_letter_space = 0i32;
    splitFontName(name, &mut var, &mut feat, &mut end, &mut index);
    nameString =
        xmalloc((var.wrapping_offset_from(name) as libc::c_long + 1i32 as libc::c_long) as size_t)
            as *mut libc::c_char;
    strncpy(
        nameString,
        name,
        var.wrapping_offset_from(name) as libc::c_long as libc::c_ulong,
    );
    *nameString.offset(var.wrapping_offset_from(name) as libc::c_long as isize) =
        0i32 as libc::c_char;
    if feat > var {
        varString =
            xmalloc(feat.wrapping_offset_from(var) as libc::c_long as size_t) as *mut libc::c_char;
        strncpy(
            varString,
            var.offset(1),
            (feat.wrapping_offset_from(var) as libc::c_long - 1i32 as libc::c_long)
                as libc::c_ulong,
        );
        *varString.offset(
            (feat.wrapping_offset_from(var) as libc::c_long - 1i32 as libc::c_long) as isize,
        ) = 0i32 as libc::c_char
    }
    if end > feat {
        featString =
            xmalloc(end.wrapping_offset_from(feat) as libc::c_long as size_t) as *mut libc::c_char;
        strncpy(
            featString,
            feat.offset(1),
            (end.wrapping_offset_from(feat) as libc::c_long - 1i32 as libc::c_long)
                as libc::c_ulong,
        );
        *featString.offset(
            (end.wrapping_offset_from(feat) as libc::c_long - 1i32 as libc::c_long) as isize,
        ) = 0i32 as libc::c_char
    }
    // check for "[filename]" form, don't search maps in this case
    if *nameString.offset(0) as libc::c_int == '[' as i32 {
        if scaled_size < 0i32 {
            font = createFontFromFile(nameString.offset(1), index, 655360i64 as Fixed);
            if !font.is_null() {
                let mut dsize: Fixed = D2Fix(getDesignSize(font));
                if scaled_size == -1000i32 {
                    scaled_size = dsize
                } else {
                    scaled_size = xn_over_d(dsize, -scaled_size, 1000i32)
                }
                deleteFont(font);
            }
        }
        font = createFontFromFile(nameString.offset(1), index, scaled_size);
        if !font.is_null() {
            loaded_font_design_size = D2Fix(getDesignSize(font));
            /* This is duplicated in XeTeXFontMgr::findFont! */
            setReqEngine(0i32 as libc::c_char);
            if !varString.is_null() {
                if !strstartswith(varString, b"/AAT\x00" as *const u8 as *const libc::c_char)
                    .is_null()
                {
                    setReqEngine('A' as i32 as libc::c_char);
                } else if !strstartswith(varString, b"/OT\x00" as *const u8 as *const libc::c_char)
                    .is_null()
                    || !strstartswith(varString, b"/ICU\x00" as *const u8 as *const libc::c_char)
                        .is_null()
                {
                    setReqEngine('O' as i32 as libc::c_char);
                } else if !strstartswith(varString, b"/GR\x00" as *const u8 as *const libc::c_char)
                    .is_null()
                {
                    setReqEngine('G' as i32 as libc::c_char);
                }
            }
            rval = loadOTfont(0 as PlatformFontRef, font, scaled_size, featString);
            if rval.is_null() {
                deleteFont(font);
            }
            if !rval.is_null() && get_tracing_fonts_state() > 0i32 {
                begin_diagnostic();
                print_nl(' ' as i32);
                print_c_string(b"-> \x00" as *const u8 as *const libc::c_char);
                print_c_string(nameString.offset(1));
                end_diagnostic(0i32 != 0);
            }
        }
    } else {
        fontRef = findFontByName(nameString, varString, Fix2D(scaled_size));
        if !fontRef.is_null() {
            /* update name_of_file to the full name of the font, for error messages during font loading */
            let mut fullName: *const libc::c_char = getFullName(fontRef);
            name_length = strlen(fullName) as int32_t;
            if !featString.is_null() {
                name_length = (name_length as libc::c_ulong)
                    .wrapping_add(strlen(featString).wrapping_add(1i32 as libc::c_ulong))
                    as int32_t as int32_t
            }
            if !varString.is_null() {
                name_length = (name_length as libc::c_ulong)
                    .wrapping_add(strlen(varString).wrapping_add(1i32 as libc::c_ulong))
                    as int32_t as int32_t
            }
            free(name_of_file as *mut libc::c_void);
            name_of_file = xmalloc((name_length + 1i32) as size_t) as *mut libc::c_char;
            strcpy(name_of_file, fullName);
            if scaled_size < 0i32 {
                font = createFont(fontRef, scaled_size);
                if !font.is_null() {
                    let mut dsize_0: Fixed = D2Fix(getDesignSize(font));
                    if scaled_size == -1000i32 {
                        scaled_size = dsize_0
                    } else {
                        scaled_size = xn_over_d(dsize_0, -scaled_size, 1000i32)
                    }
                    deleteFont(font);
                }
            }
            font = createFont(fontRef, scaled_size);
            if !font.is_null() {
                /* decide whether to use AAT or OpenType rendering with this font */
                if getReqEngine() as libc::c_int == 'A' as i32 {
                    rval = loadAATfont(fontRef, scaled_size, featString);
                    if rval.is_null() {
                        deleteFont(font);
                    }
                } else {
                    if getReqEngine() as libc::c_int == 'O' as i32
                        || getReqEngine() as libc::c_int == 'G' as i32
                        || !getFontTablePtr(
                            font,
                            ('G' as i32 as uint32_t & 0xffi32 as libc::c_uint) << 24i32
                                | ('S' as i32 as uint32_t & 0xffi32 as libc::c_uint) << 16i32
                                | ('U' as i32 as uint32_t & 0xffi32 as libc::c_uint) << 8i32
                                | 'B' as i32 as uint32_t & 0xffi32 as libc::c_uint,
                        )
                        .is_null()
                        || !getFontTablePtr(
                            font,
                            ('G' as i32 as uint32_t & 0xffi32 as libc::c_uint) << 24i32
                                | ('P' as i32 as uint32_t & 0xffi32 as libc::c_uint) << 16i32
                                | ('O' as i32 as uint32_t & 0xffi32 as libc::c_uint) << 8i32
                                | 'S' as i32 as uint32_t & 0xffi32 as libc::c_uint,
                        )
                        .is_null()
                    {
                        rval = loadOTfont(fontRef, font, scaled_size, featString)
                    }
                    /* loadOTfont failed or the above check was false */
                    if rval.is_null() {
                        rval = loadAATfont(fontRef, scaled_size, featString)
                    }
                    if rval.is_null() {
                        deleteFont(font);
                    }
                }
            }
            /* append the style and feature strings, so that \show\fontID will give a full result */
            if !varString.is_null() && *varString as libc::c_int != 0i32 {
                strcat(name_of_file, b"/\x00" as *const u8 as *const libc::c_char);
                strcat(name_of_file, varString);
            }
            if !featString.is_null() && *featString as libc::c_int != 0i32 {
                strcat(name_of_file, b":\x00" as *const u8 as *const libc::c_char);
                strcat(name_of_file, featString);
            }
            name_length = strlen(name_of_file) as int32_t
        }
    }
    free(varString as *mut libc::c_void);
    free(featString as *mut libc::c_void);
    free(nameString as *mut libc::c_void);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn release_font_engine(
    mut engine: *mut libc::c_void,
    mut type_flag: libc::c_int,
) {
    if type_flag as libc::c_uint == 0xffffu32 {
        CFRelease(engine as CFDictionaryRef as CFTypeRef);
    } else if type_flag as libc::c_uint == 0xfffeu32 {
        deleteLayoutEngine(engine as XeTeXLayoutEngine);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ot_get_font_metrics(
    mut pEngine: *mut libc::c_void,
    mut ascent: *mut scaled_t,
    mut descent: *mut scaled_t,
    mut xheight: *mut scaled_t,
    mut capheight: *mut scaled_t,
    mut slant: *mut scaled_t,
) {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut a: libc::c_float = 0.;
    let mut d: libc::c_float = 0.;
    getAscentAndDescent(engine, &mut a, &mut d);
    *ascent = D2Fix(a as libc::c_double);
    *descent = D2Fix(d as libc::c_double);
    *slant = D2Fix(
        Fix2D(getSlant(getFont(engine))) * getExtendFactor(engine) as libc::c_double
            + getSlantFactor(engine) as libc::c_double,
    );
    /* get cap and x height from OS/2 table */
    getCapAndXHeight(engine, &mut a, &mut d);
    *capheight = D2Fix(a as libc::c_double);
    *xheight = D2Fix(d as libc::c_double);
    /* fallback in case the font does not have OS/2 table */
    if *xheight == 0i32 {
        let mut glyphID: libc::c_int =
            mapCharToGlyph(engine, 'x' as i32 as uint32_t) as libc::c_int;
        if glyphID != 0i32 {
            getGlyphHeightDepth(engine, glyphID as uint32_t, &mut a, &mut d);
            *xheight = D2Fix(a as libc::c_double)
        } else {
            *xheight = *ascent / 2i32
            /* arbitrary figure if there's no 'x' in the font */
        }
    }
    if *capheight == 0i32 {
        let mut glyphID_0: libc::c_int =
            mapCharToGlyph(engine, 'X' as i32 as uint32_t) as libc::c_int;
        if glyphID_0 != 0i32 {
            getGlyphHeightDepth(engine, glyphID_0 as uint32_t, &mut a, &mut d);
            *capheight = D2Fix(a as libc::c_double)
        } else {
            *capheight = *ascent
            /* arbitrary figure if there's no 'X' in the font */
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ot_font_get(mut what: int32_t, mut pEngine: *mut libc::c_void) -> int32_t {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut fontInst: XeTeXFont = getFont(engine);
    match what {
        1 => return countGlyphs(fontInst) as int32_t,
        8 => {
            /* ie Graphite features */
            return countGraphiteFeatures(engine) as int32_t;
        }
        16 => return countScripts(fontInst) as int32_t,
        _ => {}
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ot_font_get_1(
    mut what: int32_t,
    mut pEngine: *mut libc::c_void,
    mut param: int32_t,
) -> int32_t {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut fontInst: XeTeXFont = getFont(engine);
    match what {
        17 => return countLanguages(fontInst, param as hb_tag_t) as int32_t,
        19 => return getIndScript(fontInst, param as libc::c_uint) as int32_t,
        9 => {
            /* for graphite fonts...*/
            return getGraphiteFeatureCode(engine, param as uint32_t) as int32_t;
        }
        11 => return 1i32,
        12 => return countGraphiteFeatureSettings(engine, param as uint32_t) as int32_t,
        _ => {}
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ot_font_get_2(
    mut what: int32_t,
    mut pEngine: *mut libc::c_void,
    mut param1: int32_t,
    mut param2: int32_t,
) -> int32_t {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut fontInst: XeTeXFont = getFont(engine);
    match what {
        20 => {
            return getIndLanguage(fontInst, param1 as hb_tag_t, param2 as libc::c_uint) as int32_t
        }
        18 => return countFeatures(fontInst, param1 as hb_tag_t, param2 as hb_tag_t) as int32_t,
        13 => {
            /* for graphite fonts */
            return getGraphiteFeatureSettingCode(engine, param1 as uint32_t, param2 as uint32_t)
                as int32_t;
        }
        15 => {
            return (getGraphiteFeatureDefaultSetting(engine, param1 as uint32_t)
                == param2 as libc::c_uint) as libc::c_int
        }
        _ => {}
    } /* to guarantee enough space in the buffer */
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ot_font_get_3(
    mut what: int32_t,
    mut pEngine: *mut libc::c_void,
    mut param1: int32_t,
    mut param2: int32_t,
    mut param3: int32_t,
) -> int32_t {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut fontInst: XeTeXFont = getFont(engine);
    match what {
        21 => {
            return getIndFeature(
                fontInst,
                param1 as hb_tag_t,
                param2 as hb_tag_t,
                param3 as libc::c_uint,
            ) as int32_t
        }
        _ => {}
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn gr_print_font_name(
    mut what: int32_t,
    mut pEngine: *mut libc::c_void,
    mut param1: int32_t,
    mut param2: int32_t,
) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    match what {
        8 => name = getGraphiteFeatureLabel(engine, param1 as uint32_t),
        9 => name = getGraphiteFeatureSettingLabel(engine, param1 as uint32_t, param2 as uint32_t),
        _ => {}
    }
    if !name.is_null() {
        print_c_string(name);
        gr_label_destroy(name as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gr_font_get_named(
    mut what: int32_t,
    mut pEngine: *mut libc::c_void,
) -> int32_t {
    let mut rval: libc::c_long = -1i32 as libc::c_long;
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    match what {
        10 => rval = findGraphiteFeatureNamed(engine, name_of_file, name_length),
        _ => {}
    }
    return rval as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn gr_font_get_named_1(
    mut what: int32_t,
    mut pEngine: *mut libc::c_void,
    mut param: int32_t,
) -> int32_t {
    let mut rval: libc::c_long = -1i32 as libc::c_long;
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    match what {
        14 => {
            rval = findGraphiteFeatureSettingNamed(
                engine,
                param as uint32_t,
                name_of_file,
                name_length,
            )
        }
        _ => {}
    }
    return rval as int32_t;
}
unsafe extern "C" fn cgColorToRGBA32(mut color: CGColorRef) -> UInt32 {
    let mut components: *const CGFloat = CGColorGetComponents(color);
    let mut rval: UInt32 = (*components.offset(0) * 255.0f64 + 0.5f64) as UInt8 as UInt32;
    rval <<= 8i32;
    rval = (rval as libc::c_uint)
        .wrapping_add((*components.offset(1) * 255.0f64 + 0.5f64) as UInt8 as libc::c_uint)
        as UInt32 as UInt32;
    rval <<= 8i32;
    rval = (rval as libc::c_uint)
        .wrapping_add((*components.offset(2) * 255.0f64 + 0.5f64) as UInt8 as libc::c_uint)
        as UInt32 as UInt32;
    rval <<= 8i32;
    rval = (rval as libc::c_uint)
        .wrapping_add((*components.offset(3) * 255.0f64 + 0.5f64) as UInt8 as libc::c_uint)
        as UInt32 as UInt32;
    return rval;
}
static mut xdvBufSize: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn makeXDVGlyphArrayData(mut pNode: *mut libc::c_void) -> libc::c_int {
    let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut glyphIDs: *mut uint16_t = 0 as *mut uint16_t;
    let mut p: *mut memory_word = pNode as *mut memory_word;
    let mut glyph_info: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut locations: *mut FixedPoint = 0 as *mut FixedPoint;
    let mut width: Fixed = 0;
    let mut glyphCount: uint16_t = (*p.offset(4)).b16.s0;
    let mut i: libc::c_int = glyphCount as libc::c_int * 10i32 + 8i32;
    if i > xdvBufSize {
        free(xdv_buffer as *mut libc::c_void);
        xdvBufSize = (i / 1024i32 + 1i32) * 1024i32;
        xdv_buffer = xmalloc(xdvBufSize as size_t) as *mut libc::c_char
    }
    glyph_info = (*p.offset(5)).ptr;
    locations = glyph_info as *mut FixedPoint;
    glyphIDs = locations.offset(glyphCount as libc::c_int as isize) as *mut uint16_t;
    cp = xdv_buffer as *mut libc::c_uchar;
    width = (*p.offset(1)).b32.s1;
    let fresh13 = cp;
    cp = cp.offset(1);
    *fresh13 = (width >> 24i32 & 0xffi32) as libc::c_uchar;
    let fresh14 = cp;
    cp = cp.offset(1);
    *fresh14 = (width >> 16i32 & 0xffi32) as libc::c_uchar;
    let fresh15 = cp;
    cp = cp.offset(1);
    *fresh15 = (width >> 8i32 & 0xffi32) as libc::c_uchar;
    let fresh16 = cp;
    cp = cp.offset(1);
    *fresh16 = (width & 0xffi32) as libc::c_uchar;
    let fresh17 = cp;
    cp = cp.offset(1);
    *fresh17 = (glyphCount as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
    let fresh18 = cp;
    cp = cp.offset(1);
    *fresh18 = (glyphCount as libc::c_int & 0xffi32) as libc::c_uchar;
    i = 0i32;
    while i < glyphCount as libc::c_int {
        let mut x: Fixed = (*locations.offset(i as isize)).x;
        let mut y: Fixed = (*locations.offset(i as isize)).y;
        let fresh19 = cp;
        cp = cp.offset(1);
        *fresh19 = (x >> 24i32 & 0xffi32) as libc::c_uchar;
        let fresh20 = cp;
        cp = cp.offset(1);
        *fresh20 = (x >> 16i32 & 0xffi32) as libc::c_uchar;
        let fresh21 = cp;
        cp = cp.offset(1);
        *fresh21 = (x >> 8i32 & 0xffi32) as libc::c_uchar;
        let fresh22 = cp;
        cp = cp.offset(1);
        *fresh22 = (x & 0xffi32) as libc::c_uchar;
        let fresh23 = cp;
        cp = cp.offset(1);
        *fresh23 = (y >> 24i32 & 0xffi32) as libc::c_uchar;
        let fresh24 = cp;
        cp = cp.offset(1);
        *fresh24 = (y >> 16i32 & 0xffi32) as libc::c_uchar;
        let fresh25 = cp;
        cp = cp.offset(1);
        *fresh25 = (y >> 8i32 & 0xffi32) as libc::c_uchar;
        let fresh26 = cp;
        cp = cp.offset(1);
        *fresh26 = (y & 0xffi32) as libc::c_uchar;
        i += 1
    }
    i = 0i32;
    while i < glyphCount as libc::c_int {
        let mut g: uint16_t = *glyphIDs.offset(i as isize);
        let fresh27 = cp;
        cp = cp.offset(1);
        *fresh27 = (g as libc::c_int >> 8i32 & 0xffi32) as libc::c_uchar;
        let fresh28 = cp;
        cp = cp.offset(1);
        *fresh28 = (g as libc::c_int & 0xffi32) as libc::c_uchar;
        i += 1
    }
    return (cp as *mut libc::c_char).wrapping_offset_from(xdv_buffer) as libc::c_long
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn make_font_def(mut f: int32_t) -> libc::c_int {
    let mut flags: uint16_t = 0i32 as uint16_t;
    let mut rgba: uint32_t = 0;
    let mut size: Fixed = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: uint32_t = 0;
    let mut filenameLen: uint8_t = 0;
    let mut fontDefLength: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* PlatformFontRef fontRef = 0; */
    let mut extend: libc::c_float = 1.0f64 as libc::c_float;
    let mut slant: libc::c_float = 0.0f64 as libc::c_float;
    let mut embolden: libc::c_float = 0.0f64 as libc::c_float;
    let mut attributes: CFDictionaryRef = 0 as CFDictionaryRef;
    if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32 {
        let mut font: CTFontRef = 0 as *const __CTFont;
        let mut color: CGColorRef = 0 as *mut CGColor;
        let mut t: CGAffineTransform = CGAffineTransform {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            tx: 0.,
            ty: 0.,
        };
        let mut emboldenNumber: CFNumberRef = 0 as *const __CFNumber;
        let mut fSize: CGFloat = 0.;
        attributes = *font_layout_engine.offset(f as isize) as CFDictionaryRef;
        font = CFDictionaryGetValue(attributes, kCTFontAttributeName as *const libc::c_void)
            as CTFontRef;
        filename = getFileNameFromCTFont(font, &mut index);
        if filename.is_null() as libc::c_int as libc::c_long != 0 {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"make_font_def\x00"))
                    .as_ptr(),
                b"tectonic/xetex-ext.c\x00" as *const u8 as *const libc::c_char,
                1163i32,
                b"filename\x00" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        if !CFDictionaryGetValue(
            attributes,
            kCTVerticalFormsAttributeName as *const libc::c_void,
        )
        .is_null()
        {
            flags = (flags as libc::c_int | 0x100i32) as uint16_t
        }
        color = CFDictionaryGetValue(
            attributes,
            kCTForegroundColorAttributeName as *const libc::c_void,
        ) as CGColorRef;
        if !color.is_null() {
            rgba = cgColorToRGBA32(color)
        }
        t = CTFontGetMatrix(font);
        extend = t.a as libc::c_float;
        slant = t.c as libc::c_float;
        emboldenNumber = CFDictionaryGetValue(
            attributes,
            getkXeTeXEmboldenAttributeName() as *const libc::c_void,
        ) as CFNumberRef;
        if !emboldenNumber.is_null() {
            CFNumberGetValue(
                emboldenNumber,
                kCFNumberFloatType as libc::c_int as CFNumberType,
                &mut embolden as *mut libc::c_float as *mut libc::c_void,
            );
        }
        fSize = CTFontGetSize(font);
        size = D2Fix(fSize)
    } else if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32 {
        let mut engine: XeTeXLayoutEngine = 0 as *mut XeTeXLayoutEngine_rec;
        engine = *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
        /* fontRef = */
        getFontRef(engine);
        filename = getFontFilename(engine, &mut index);
        if filename.is_null() as libc::c_int as libc::c_long != 0 {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"make_font_def\x00"))
                    .as_ptr(),
                b"tectonic/xetex-ext.c\x00" as *const u8 as *const libc::c_char,
                1190i32,
                b"filename\x00" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        rgba = getRgbValue(engine);
        if *font_flags.offset(f as isize) as libc::c_int & 0x2i32 != 0i32 {
            flags = (flags as libc::c_int | 0x100i32) as uint16_t
        }
        extend = getExtendFactor(engine);
        slant = getSlantFactor(engine);
        embolden = getEmboldenFactor(engine);
        size = D2Fix(getPointSize(engine) as libc::c_double)
    } else {
        _tt_abort(
            b"bad native font flag in `make_font_def`\x00" as *const u8 as *const libc::c_char,
        );
    }
    filenameLen = strlen(filename) as uint8_t;
    /* parameters after internal font ID:
    //  size[4]
    //  flags[2]
    //  l[1] n[l]
    //  if flags & COLORED:
    //      c[4]
     */
    fontDefLength = 4i32 + 2i32 + 1i32 + filenameLen as libc::c_int + 4i32; /* face index */
    if *font_flags.offset(f as isize) as libc::c_int & 0x1i32 != 0i32 {
        fontDefLength += 4i32; /* 32-bit RGBA value */
        flags = (flags as libc::c_int | 0x200i32) as uint16_t
    }
    if extend as libc::c_double != 1.0f64 {
        fontDefLength += 4i32;
        flags = (flags as libc::c_int | 0x1000i32) as uint16_t
    }
    if slant as libc::c_double != 0.0f64 {
        fontDefLength += 4i32;
        flags = (flags as libc::c_int | 0x2000i32) as uint16_t
    }
    if embolden as libc::c_double != 0.0f64 {
        fontDefLength += 4i32;
        flags = (flags as libc::c_int | 0x4000i32) as uint16_t
    }
    if fontDefLength > xdvBufSize {
        free(xdv_buffer as *mut libc::c_void);
        xdvBufSize = (fontDefLength / 1024i32 + 1i32) * 1024i32;
        xdv_buffer = xmalloc(xdvBufSize as size_t) as *mut libc::c_char
    }
    cp = xdv_buffer;
    *(cp as *mut Fixed) = SWAP32(size as uint32_t) as Fixed;
    cp = cp.offset(4);
    *(cp as *mut uint16_t) = SWAP16(flags);
    cp = cp.offset(2);
    *(cp as *mut uint8_t) = filenameLen;
    cp = cp.offset(1);
    memcpy(
        cp as *mut libc::c_void,
        filename as *const libc::c_void,
        filenameLen as libc::c_ulong,
    );
    cp = cp.offset(filenameLen as libc::c_int as isize);
    *(cp as *mut uint32_t) = SWAP32(index);
    cp = cp.offset(4);
    if *font_flags.offset(f as isize) as libc::c_int & 0x1i32 != 0i32 {
        *(cp as *mut uint32_t) = SWAP32(rgba);
        cp = cp.offset(4)
    }
    if flags as libc::c_int & 0x1000i32 != 0 {
        let mut f_0: Fixed = D2Fix(extend as libc::c_double);
        *(cp as *mut uint32_t) = SWAP32(f_0 as uint32_t);
        cp = cp.offset(4)
    }
    if flags as libc::c_int & 0x2000i32 != 0 {
        let mut f_1: Fixed = D2Fix(slant as libc::c_double);
        *(cp as *mut uint32_t) = SWAP32(f_1 as uint32_t);
        cp = cp.offset(4)
    }
    if flags as libc::c_int & 0x4000i32 != 0 {
        let mut f_2: Fixed = D2Fix(embolden as libc::c_double);
        *(cp as *mut uint32_t) = SWAP32(f_2 as uint32_t);
        cp = cp.offset(4)
    }
    free(filename as *mut libc::c_void);
    return fontDefLength;
}
#[no_mangle]
pub unsafe extern "C" fn apply_mapping(
    mut pCnv: *mut libc::c_void,
    mut txtPtr: *mut uint16_t,
    mut txtLen: libc::c_int,
) -> libc::c_int {
    let mut cnv: TECkit_Converter = pCnv as TECkit_Converter;
    let mut inUsed: UInt32 = 0;
    let mut outUsed: UInt32 = 0;
    let mut status: TECkit_Status = 0;
    static mut outLength: UInt32 = 0i32 as UInt32;
    /* allocate outBuffer if not big enough */
    if (outLength as libc::c_ulong)
        < (txtLen as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<UniChar>() as libc::c_ulong)
            .wrapping_add(32i32 as libc::c_ulong)
    {
        free(mapped_text as *mut libc::c_void);
        outLength = (txtLen as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<UniChar>() as libc::c_ulong)
            .wrapping_add(32i32 as libc::c_ulong) as UInt32;
        mapped_text = xmalloc(outLength as size_t) as *mut UTF16_code
    }
    loop
    /* try the mapping */
    {
        status = TECkit_ConvertBuffer(
            cnv,
            txtPtr as *mut Byte,
            (txtLen as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<UniChar>() as libc::c_ulong)
                as UInt32,
            &mut inUsed,
            mapped_text as *mut Byte,
            outLength,
            &mut outUsed,
            1i32 as Byte,
        );
        match status {
            0 => {
                txtPtr = mapped_text as *mut UniChar;
                return (outUsed as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<UniChar>() as libc::c_ulong)
                    as libc::c_int;
            }
            1 => {
                outLength = (outLength as libc::c_ulong).wrapping_add(
                    (txtLen as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<UniChar>() as libc::c_ulong)
                        .wrapping_add(32i32 as libc::c_ulong),
                ) as UInt32 as UInt32;
                free(mapped_text as *mut libc::c_void);
                mapped_text = xmalloc(outLength as size_t) as *mut UTF16_code
            }
            _ => return 0i32,
        }
    }
}
unsafe extern "C" fn snap_zone(
    mut value: *mut scaled_t,
    mut snap_value: scaled_t,
    mut fuzz: scaled_t,
) {
    let mut difference: scaled_t = *value - snap_value;
    if difference <= fuzz && difference >= -fuzz {
        *value = snap_value
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_native_char_height_depth(
    mut font: int32_t,
    mut ch: int32_t,
    mut height: *mut scaled_t,
    mut depth: *mut scaled_t,
) {
    let mut ht: libc::c_float = 0.0f64 as libc::c_float;
    let mut dp: libc::c_float = 0.0f64 as libc::c_float;
    let mut fuzz: Fixed = 0;
    if *font_area.offset(font as isize) as libc::c_uint == 0xffffu32 {
        let mut attributes: CFDictionaryRef =
            *font_layout_engine.offset(font as isize) as CFDictionaryRef;
        let mut gid: libc::c_int = MapCharToGlyph_AAT(attributes, ch as UInt32);
        GetGlyphHeightDepth_AAT(attributes, gid as uint16_t, &mut ht, &mut dp);
    } else if *font_area.offset(font as isize) as libc::c_uint == 0xfffeu32 {
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine;
        let mut gid_0: libc::c_int = mapCharToGlyph(engine, ch as uint32_t) as libc::c_int;
        getGlyphHeightDepth(engine, gid_0 as uint32_t, &mut ht, &mut dp);
    } else {
        _tt_abort(
            b"bad native font flag in `get_native_char_height_depth`\x00" as *const u8
                as *const libc::c_char,
        );
    }
    *height = D2Fix(ht as libc::c_double);
    *depth = D2Fix(dp as libc::c_double);
    /* snap to "known" zones for baseline, x-height, cap-height if within 4% of em-size */
    fuzz = (*font_info.offset((6i32 + *param_base.offset(font as isize)) as isize))
        .b32
        .s1
        / 25i32;
    snap_zone(depth, 0i32, fuzz);
    snap_zone(height, 0i32, fuzz);
    snap_zone(
        height,
        (*font_info.offset((5i32 + *param_base.offset(font as isize)) as isize))
            .b32
            .s1,
        fuzz,
    );
    snap_zone(
        height,
        (*font_info.offset((8i32 + *param_base.offset(font as isize)) as isize))
            .b32
            .s1,
        fuzz,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getnativecharht(mut f: int32_t, mut c: int32_t) -> scaled_t {
    let mut h: scaled_t = 0;
    let mut d: scaled_t = 0;
    get_native_char_height_depth(f, c, &mut h, &mut d);
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn getnativechardp(mut f: int32_t, mut c: int32_t) -> scaled_t {
    let mut h: scaled_t = 0;
    let mut d: scaled_t = 0;
    get_native_char_height_depth(f, c, &mut h, &mut d);
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn get_native_char_sidebearings(
    mut font: int32_t,
    mut ch: int32_t,
    mut lsb: *mut scaled_t,
    mut rsb: *mut scaled_t,
) {
    let mut l: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    if *font_area.offset(font as isize) as libc::c_uint == 0xffffu32 {
        let mut attributes: CFDictionaryRef =
            *font_layout_engine.offset(font as isize) as CFDictionaryRef;
        let mut gid: libc::c_int = MapCharToGlyph_AAT(attributes, ch as UInt32);
        GetGlyphSidebearings_AAT(attributes, gid as uint16_t, &mut l, &mut r);
    } else if *font_area.offset(font as isize) as libc::c_uint == 0xfffeu32 {
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine;
        let mut gid_0: libc::c_int = mapCharToGlyph(engine, ch as uint32_t) as libc::c_int;
        getGlyphSidebearings(engine, gid_0 as uint32_t, &mut l, &mut r);
    } else {
        _tt_abort(
            b"bad native font flag in `get_native_char_side_bearings`\x00" as *const u8
                as *const libc::c_char,
        );
    }
    *lsb = D2Fix(l as libc::c_double);
    *rsb = D2Fix(r as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn get_glyph_bounds(
    mut font: int32_t,
    mut edge: int32_t,
    mut gid: int32_t,
) -> scaled_t {
    /* edge codes 1,2,3,4 => L T R B */
    let mut a: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    if *font_area.offset(font as isize) as libc::c_uint == 0xffffu32 {
        let mut attributes: CFDictionaryRef =
            *font_layout_engine.offset(font as isize) as CFDictionaryRef;
        if edge & 1i32 != 0 {
            GetGlyphSidebearings_AAT(attributes, gid as uint16_t, &mut a, &mut b);
        } else {
            GetGlyphHeightDepth_AAT(attributes, gid as uint16_t, &mut a, &mut b);
        }
    } else if *font_area.offset(font as isize) as libc::c_uint == 0xfffeu32 {
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine;
        if edge & 1i32 != 0 {
            getGlyphSidebearings(engine, gid as uint32_t, &mut a, &mut b);
        } else {
            getGlyphHeightDepth(engine, gid as uint32_t, &mut a, &mut b);
        }
    } else {
        _tt_abort(
            b"bad native font flag in `get_glyph_bounds`\x00" as *const u8 as *const libc::c_char,
        );
    }
    return D2Fix((if edge <= 2i32 { a } else { b }) as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn getnativecharic(mut f: int32_t, mut c: int32_t) -> scaled_t {
    let mut lsb: scaled_t = 0;
    let mut rsb: scaled_t = 0;
    get_native_char_sidebearings(f, c, &mut lsb, &mut rsb);
    if rsb < 0i32 {
        return *font_letter_space.offset(f as isize) - rsb;
    } else {
        return *font_letter_space.offset(f as isize);
    };
}
/* single-purpose metrics accessors */
#[no_mangle]
pub unsafe extern "C" fn getnativecharwd(mut f: int32_t, mut c: int32_t) -> scaled_t {
    let mut wd: scaled_t = 0i32;
    if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32 {
        let mut attributes: CFDictionaryRef =
            *font_layout_engine.offset(f as isize) as CFDictionaryRef;
        let mut gid: libc::c_int = MapCharToGlyph_AAT(attributes, c as UInt32);
        wd = D2Fix(GetGlyphWidth_AAT(attributes, gid as uint16_t))
    } else if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32 {
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
        let mut gid_0: libc::c_int = mapCharToGlyph(engine, c as uint32_t) as libc::c_int;
        wd = D2Fix(getGlyphWidthFromEngine(engine, gid_0 as uint32_t) as libc::c_double)
    } else {
        _tt_abort(
            b"bad native font flag in `get_native_char_wd`\x00" as *const u8 as *const libc::c_char,
        );
    }
    return wd;
}
#[no_mangle]
pub unsafe extern "C" fn real_get_native_glyph(
    mut pNode: *mut libc::c_void,
    mut index: libc::c_uint,
) -> uint16_t {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut locations: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint;
    let mut glyphIDs: *mut uint16_t =
        locations.offset((*node.offset(4)).b16.s0 as libc::c_int as isize) as *mut uint16_t;
    if index >= (*node.offset(4)).b16.s0 as libc::c_uint {
        return 0i32 as uint16_t;
    } else {
        return *glyphIDs.offset(index as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn store_justified_native_glyphs(mut pNode: *mut libc::c_void) {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut f: libc::c_uint = (*node.offset(4)).b16.s2 as libc::c_uint;
    /* separate Mac-only codepath for AAT fonts */
    if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32 {
        DoAATLayout(node as *mut libc::c_void, 1i32);
        return;
    }
    /* save desired width */
    let mut savedWidth: libc::c_int = (*node.offset(1)).b32.s1;
    measure_native_node(node as *mut libc::c_void, 0i32);
    if (*node.offset(1)).b32.s1 != savedWidth {
        /* see how much adjustment is needed overall */
        let mut justAmount: libc::c_double = Fix2D(savedWidth - (*node.offset(1)).b32.s1);
        /* apply justification to spaces (or if there are none, distribute it to all glyphs as a last resort) */
        let mut locations: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint;
        let mut glyphIDs: *mut uint16_t =
            locations.offset((*node.offset(4)).b16.s0 as libc::c_int as isize) as *mut uint16_t;
        let mut glyphCount: libc::c_int = (*node.offset(4)).b16.s0 as libc::c_int;
        let mut spaceCount: libc::c_int = 0i32;
        let mut i: libc::c_int = 0;
        let mut spaceGlyph: libc::c_int = map_char_to_glyph(f as int32_t, ' ' as i32);
        i = 0i32;
        while i < glyphCount {
            if *glyphIDs.offset(i as isize) as libc::c_int == spaceGlyph {
                spaceCount += 1
            }
            i += 1
        }
        if spaceCount > 0i32 {
            let mut adjustment: libc::c_double = 0i32 as libc::c_double;
            let mut spaceIndex: libc::c_int = 0i32;
            i = 0i32;
            while i < glyphCount {
                (*locations.offset(i as isize)).x =
                    D2Fix(Fix2D((*locations.offset(i as isize)).x) + adjustment);
                if *glyphIDs.offset(i as isize) as libc::c_int == spaceGlyph {
                    spaceIndex += 1;
                    adjustment =
                        justAmount * spaceIndex as libc::c_double / spaceCount as libc::c_double
                }
                i += 1
            }
        } else {
            i = 1i32;
            while i < glyphCount {
                (*locations.offset(i as isize)).x = D2Fix(
                    Fix2D((*locations.offset(i as isize)).x)
                        + justAmount * i as libc::c_double / (glyphCount - 1i32) as libc::c_double,
                );
                i += 1
            }
        }
        (*node.offset(1)).b32.s1 = savedWidth
    };
}
#[no_mangle]
pub unsafe extern "C" fn measure_native_node(
    mut pNode: *mut libc::c_void,
    mut use_glyph_metrics: libc::c_int,
) {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut txtLen: libc::c_int = (*node.offset(4)).b16.s1 as libc::c_int;
    let mut txtPtr: *mut uint16_t = node.offset(6) as *mut uint16_t;
    let mut f: libc::c_uint = (*node.offset(4)).b16.s2 as libc::c_uint;
    if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32 {
        /* we're using this font in AAT mode, so font_layout_engine[f] is actually a CFDictionaryRef */
        DoAATLayout(node as *mut libc::c_void, 0i32);
    } else if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32 {
        /* using this font in OT Layout mode, so font_layout_engine[f] is actually a XeTeXLayoutEngine */
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
        let mut locations: *mut FixedPoint = 0 as *mut FixedPoint;
        let mut glyphIDs: *mut uint16_t = 0 as *mut uint16_t;
        let mut glyphAdvances: *mut Fixed = 0 as *mut Fixed;
        let mut totalGlyphCount: libc::c_int = 0i32;
        /* need to find direction runs within the text, and call layoutChars separately for each */
        let mut dir: UBiDiDirection = UBIDI_LTR;
        let mut glyph_info: *mut libc::c_void = 0 as *mut libc::c_void;
        static mut positions: *mut FloatPoint = 0 as *const FloatPoint as *mut FloatPoint;
        static mut advances: *mut libc::c_float = 0 as *const libc::c_float as *mut libc::c_float;
        static mut glyphs: *mut uint32_t = 0 as *const uint32_t as *mut uint32_t;
        let mut pBiDi: *mut UBiDi = ubidi_open_64();
        let mut errorCode: UErrorCode = U_ZERO_ERROR;
        ubidi_setPara_64(
            pBiDi,
            txtPtr as *const UChar,
            txtLen,
            getDefaultDirection(engine) as UBiDiLevel,
            0 as *mut UBiDiLevel,
            &mut errorCode,
        );
        dir = ubidi_getDirection_64(pBiDi);
        if dir as libc::c_uint == UBIDI_MIXED as libc::c_int as libc::c_uint {
            /* we actually do the layout twice here, once to count glyphs and then again to get them;
               which is inefficient, but i figure that MIXED is a relatively rare occurrence, so i can't be
               bothered to deal with the memory reallocation headache of doing it differently
            */
            let mut nRuns: libc::c_int = ubidi_countRuns_64(pBiDi, &mut errorCode);
            let mut width: libc::c_double = 0i32 as libc::c_double;
            let mut i: libc::c_int = 0;
            let mut runIndex: libc::c_int = 0;
            let mut logicalStart: int32_t = 0;
            let mut length: int32_t = 0;
            runIndex = 0i32;
            while runIndex < nRuns {
                dir = ubidi_getVisualRun_64(pBiDi, runIndex, &mut logicalStart, &mut length);
                totalGlyphCount += layoutChars(
                    engine,
                    txtPtr,
                    logicalStart,
                    length,
                    txtLen,
                    dir as libc::c_uint == UBIDI_RTL as libc::c_int as libc::c_uint,
                );
                runIndex += 1
            }
            if totalGlyphCount > 0i32 {
                let mut x: libc::c_double = 0.;
                let mut y: libc::c_double = 0.;
                glyph_info = xcalloc(totalGlyphCount as size_t, 10i32 as size_t);
                locations = glyph_info as *mut FixedPoint;
                glyphIDs = locations.offset(totalGlyphCount as isize) as *mut uint16_t;
                glyphAdvances = xcalloc(
                    totalGlyphCount as size_t,
                    ::std::mem::size_of::<Fixed>() as libc::c_ulong,
                ) as *mut Fixed;
                totalGlyphCount = 0i32;
                y = 0.0f64;
                x = y;
                runIndex = 0i32;
                while runIndex < nRuns {
                    let mut nGlyphs: libc::c_int = 0;
                    dir = ubidi_getVisualRun_64(pBiDi, runIndex, &mut logicalStart, &mut length);
                    nGlyphs = layoutChars(
                        engine,
                        txtPtr,
                        logicalStart,
                        length,
                        txtLen,
                        dir as libc::c_uint == UBIDI_RTL as libc::c_int as libc::c_uint,
                    );
                    glyphs = xcalloc(
                        nGlyphs as size_t,
                        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                    ) as *mut uint32_t;
                    positions = xcalloc(
                        (nGlyphs + 1i32) as size_t,
                        ::std::mem::size_of::<FloatPoint>() as libc::c_ulong,
                    ) as *mut FloatPoint;
                    advances = xcalloc(
                        nGlyphs as size_t,
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ) as *mut libc::c_float;
                    getGlyphs(engine, glyphs);
                    getGlyphAdvances(engine, advances);
                    getGlyphPositions(engine, positions);
                    i = 0i32;
                    while i < nGlyphs {
                        *glyphIDs.offset(totalGlyphCount as isize) =
                            *glyphs.offset(i as isize) as uint16_t;
                        (*locations.offset(totalGlyphCount as isize)).x =
                            D2Fix((*positions.offset(i as isize)).x as libc::c_double + x);
                        (*locations.offset(totalGlyphCount as isize)).y =
                            D2Fix((*positions.offset(i as isize)).y as libc::c_double + y);
                        *glyphAdvances.offset(totalGlyphCount as isize) =
                            D2Fix(*advances.offset(i as isize) as libc::c_double);
                        totalGlyphCount += 1;
                        i += 1
                    }
                    x += (*positions.offset(nGlyphs as isize)).x as libc::c_double;
                    y += (*positions.offset(nGlyphs as isize)).y as libc::c_double;
                    free(glyphs as *mut libc::c_void);
                    free(positions as *mut libc::c_void);
                    free(advances as *mut libc::c_void);
                    runIndex += 1
                }
                width = x
            }
            (*node.offset(1)).b32.s1 = D2Fix(width);
            (*node.offset(4)).b16.s0 = totalGlyphCount as uint16_t;
            let ref mut fresh29 = (*node.offset(5)).ptr;
            *fresh29 = glyph_info
        } else {
            let mut width_0: libc::c_double = 0i32 as libc::c_double;
            totalGlyphCount = layoutChars(
                engine,
                txtPtr,
                0i32,
                txtLen,
                txtLen,
                dir as libc::c_uint == UBIDI_RTL as libc::c_int as libc::c_uint,
            );
            glyphs = xcalloc(
                totalGlyphCount as size_t,
                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
            ) as *mut uint32_t;
            positions = xcalloc(
                (totalGlyphCount + 1i32) as size_t,
                ::std::mem::size_of::<FloatPoint>() as libc::c_ulong,
            ) as *mut FloatPoint;
            advances = xcalloc(
                totalGlyphCount as size_t,
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
            ) as *mut libc::c_float;
            getGlyphs(engine, glyphs);
            getGlyphAdvances(engine, advances);
            getGlyphPositions(engine, positions);
            if totalGlyphCount > 0i32 {
                let mut i_0: libc::c_int = 0;
                glyph_info = xcalloc(totalGlyphCount as size_t, 10i32 as size_t);
                locations = glyph_info as *mut FixedPoint;
                glyphIDs = locations.offset(totalGlyphCount as isize) as *mut uint16_t;
                glyphAdvances = xcalloc(
                    totalGlyphCount as size_t,
                    ::std::mem::size_of::<Fixed>() as libc::c_ulong,
                ) as *mut Fixed;
                i_0 = 0i32;
                while i_0 < totalGlyphCount {
                    *glyphIDs.offset(i_0 as isize) = *glyphs.offset(i_0 as isize) as uint16_t;
                    *glyphAdvances.offset(i_0 as isize) =
                        D2Fix(*advances.offset(i_0 as isize) as libc::c_double);
                    (*locations.offset(i_0 as isize)).x =
                        D2Fix((*positions.offset(i_0 as isize)).x as libc::c_double);
                    (*locations.offset(i_0 as isize)).y =
                        D2Fix((*positions.offset(i_0 as isize)).y as libc::c_double);
                    i_0 += 1
                }
                width_0 = (*positions.offset(totalGlyphCount as isize)).x as libc::c_double
            }
            (*node.offset(1)).b32.s1 = D2Fix(width_0);
            (*node.offset(4)).b16.s0 = totalGlyphCount as uint16_t;
            let ref mut fresh30 = (*node.offset(5)).ptr;
            *fresh30 = glyph_info;
            free(glyphs as *mut libc::c_void);
            free(positions as *mut libc::c_void);
            free(advances as *mut libc::c_void);
        }
        ubidi_close_64(pBiDi);
        if *font_letter_space.offset(f as isize) != 0i32 {
            let mut lsDelta: Fixed = 0i32;
            let mut lsUnit: Fixed = *font_letter_space.offset(f as isize);
            let mut i_1: libc::c_int = 0;
            i_1 = 0i32;
            while i_1 < totalGlyphCount {
                if *glyphAdvances.offset(i_1 as isize) == 0i32 && lsDelta != 0i32 {
                    lsDelta -= lsUnit
                }
                let ref mut fresh31 = (*locations.offset(i_1 as isize)).x;
                *fresh31 += lsDelta;
                lsDelta += lsUnit;
                i_1 += 1
            }
            if lsDelta != 0i32 {
                lsDelta -= lsUnit;
                let ref mut fresh32 = (*node.offset(1)).b32.s1;
                *fresh32 += lsDelta
            }
        }
        free(glyphAdvances as *mut libc::c_void);
    } else {
        _tt_abort(
            b"bad native font flag in `measure_native_node`\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if use_glyph_metrics == 0i32 || (*node.offset(4)).b16.s0 as libc::c_int == 0i32 {
        /* for efficiency, height and depth are the font's ascent/descent,
        not true values based on the actual content of the word,
        unless use_glyph_metrics is non-zero */
        (*node.offset(3)).b32.s1 = *height_base.offset(f as isize);
        (*node.offset(2)).b32.s1 = *depth_base.offset(f as isize)
    } else {
        /* this iterates over the glyph data whether it comes from AAT or OT layout */
        let mut locations_0: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint; /* NB negative is upwards in locations[].y! */
        let mut glyphIDs_0: *mut uint16_t =
            locations_0.offset((*node.offset(4)).b16.s0 as libc::c_int as isize) as *mut uint16_t;
        let mut yMin: libc::c_float = 65536.0f64 as libc::c_float;
        let mut yMax: libc::c_float = -65536.0f64 as libc::c_float;
        let mut i_2: libc::c_int = 0;
        i_2 = 0i32;
        while i_2 < (*node.offset(4)).b16.s0 as libc::c_int {
            let mut ht: libc::c_float = 0.;
            let mut dp: libc::c_float = 0.;
            let mut y_0: libc::c_float =
                Fix2D(-(*locations_0.offset(i_2 as isize)).y) as libc::c_float;
            let mut bbox: GlyphBBox = GlyphBBox {
                xMin: 0.,
                yMin: 0.,
                xMax: 0.,
                yMax: 0.,
            };
            if getCachedGlyphBBox(f as uint16_t, *glyphIDs_0.offset(i_2 as isize), &mut bbox)
                == 0i32
            {
                if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32 {
                    GetGlyphBBox_AAT(
                        *font_layout_engine.offset(f as isize) as CFDictionaryRef,
                        *glyphIDs_0.offset(i_2 as isize),
                        &mut bbox,
                    );
                } else if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32 {
                    getGlyphBounds(
                        *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine,
                        *glyphIDs_0.offset(i_2 as isize) as uint32_t,
                        &mut bbox,
                    );
                }
                cacheGlyphBBox(f as uint16_t, *glyphIDs_0.offset(i_2 as isize), &mut bbox);
            }
            ht = bbox.yMax;
            dp = -bbox.yMin;
            if y_0 + ht > yMax {
                yMax = y_0 + ht
            }
            if y_0 - dp < yMin {
                yMin = y_0 - dp
            }
            i_2 += 1
        }
        (*node.offset(3)).b32.s1 = D2Fix(yMax as libc::c_double);
        (*node.offset(2)).b32.s1 = -D2Fix(yMin as libc::c_double)
    };
}
#[no_mangle]
pub unsafe extern "C" fn real_get_native_italic_correction(mut pNode: *mut libc::c_void) -> Fixed {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut f: libc::c_uint = (*node.offset(4)).b16.s2 as libc::c_uint;
    let mut n: libc::c_uint = (*node.offset(4)).b16.s0 as libc::c_uint;
    if n > 0i32 as libc::c_uint {
        let mut locations: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint;
        let mut glyphIDs: *mut uint16_t = locations.offset(n as isize) as *mut uint16_t;
        if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32 {
            return D2Fix(GetGlyphItalCorr_AAT(
                *font_layout_engine.offset(f as isize) as CFDictionaryRef,
                *glyphIDs.offset(n.wrapping_sub(1i32 as libc::c_uint) as isize),
            )) + *font_letter_space.offset(f as isize);
        }
        if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32 {
            return D2Fix(getGlyphItalCorr(
                *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine,
                *glyphIDs.offset(n.wrapping_sub(1i32 as libc::c_uint) as isize) as uint32_t,
            ) as libc::c_double)
                + *font_letter_space.offset(f as isize);
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn real_get_native_glyph_italic_correction(
    mut pNode: *mut libc::c_void,
) -> Fixed {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut gid: uint16_t = (*node.offset(4)).b16.s1;
    let mut f: libc::c_uint = (*node.offset(4)).b16.s2 as libc::c_uint;
    if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32 {
        return D2Fix(GetGlyphItalCorr_AAT(
            *font_layout_engine.offset(f as isize) as CFDictionaryRef,
            gid,
        ));
    }
    if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32 {
        return D2Fix(getGlyphItalCorr(
            *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine,
            gid as uint32_t,
        ) as libc::c_double);
    }
    return 0i32;
    /* can't actually happen */
}
#[no_mangle]
pub unsafe extern "C" fn measure_native_glyph(
    mut pNode: *mut libc::c_void,
    mut use_glyph_metrics: libc::c_int,
) {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut gid: uint16_t = (*node.offset(4)).b16.s1;
    let mut f: libc::c_uint = (*node.offset(4)).b16.s2 as libc::c_uint;
    let mut ht: libc::c_float = 0.0f64 as libc::c_float;
    let mut dp: libc::c_float = 0.0f64 as libc::c_float;
    if *font_area.offset(f as isize) as libc::c_uint == 0xffffu32 {
        let mut attributes: CFDictionaryRef =
            *font_layout_engine.offset(f as isize) as CFDictionaryRef;
        (*node.offset(1)).b32.s1 = D2Fix(GetGlyphWidth_AAT(attributes, gid));
        if use_glyph_metrics != 0 {
            GetGlyphHeightDepth_AAT(attributes, gid, &mut ht, &mut dp);
        }
    } else if *font_area.offset(f as isize) as libc::c_uint == 0xfffeu32 {
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
        let mut fontInst: XeTeXFont = getFont(engine);
        (*node.offset(1)).b32.s1 =
            D2Fix(getGlyphWidth(fontInst, gid as uint32_t) as libc::c_double);
        if use_glyph_metrics != 0 {
            getGlyphHeightDepth(engine, gid as uint32_t, &mut ht, &mut dp);
        }
    } else {
        _tt_abort(
            b"bad native font flag in `measure_native_glyph`\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if use_glyph_metrics != 0 {
        (*node.offset(3)).b32.s1 = D2Fix(ht as libc::c_double);
        (*node.offset(2)).b32.s1 = D2Fix(dp as libc::c_double)
    } else {
        (*node.offset(3)).b32.s1 = *height_base.offset(f as isize);
        (*node.offset(2)).b32.s1 = *depth_base.offset(f as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn map_char_to_glyph(mut font: int32_t, mut ch: int32_t) -> int32_t {
    if ch > 0x10ffffi32 || ch >= 0xd800i32 && ch <= 0xdfffi32 {
        return 0i32;
    }
    if *font_area.offset(font as isize) as libc::c_uint == 0xffffu32 {
        return MapCharToGlyph_AAT(
            *font_layout_engine.offset(font as isize) as CFDictionaryRef,
            ch as UInt32,
        );
    } else if *font_area.offset(font as isize) as libc::c_uint == 0xfffeu32 {
        return mapCharToGlyph(
            *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine,
            ch as uint32_t,
        ) as int32_t;
    } else {
        _tt_abort(
            b"bad native font flag in `map_char_to_glyph`\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn map_glyph_to_index(mut font: int32_t) -> int32_t
/* glyph name is at name_of_file */ {
    if *font_area.offset(font as isize) as libc::c_uint == 0xffffu32 {
        return MapGlyphToIndex_AAT(
            *font_layout_engine.offset(font as isize) as CFDictionaryRef,
            name_of_file,
        );
    } else if *font_area.offset(font as isize) as libc::c_uint == 0xfffeu32 {
        return mapGlyphToIndex(
            *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine,
            name_of_file,
        );
    } else {
        _tt_abort(
            b"bad native font flag in `map_glyph_to_index`\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_font_char_range(mut font: int32_t, mut first: libc::c_int) -> int32_t {
    if *font_area.offset(font as isize) as libc::c_uint == 0xffffu32 {
        return GetFontCharRange_AAT(
            *font_layout_engine.offset(font as isize) as CFDictionaryRef,
            first,
        );
    } else if *font_area.offset(font as isize) as libc::c_uint == 0xfffeu32 {
        return getFontCharRange(
            *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine,
            first,
        );
    } else {
        _tt_abort(
            b"bad native font flag in `get_font_char_range\'`\x00" as *const u8
                as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn D2Fix(mut d: libc::c_double) -> Fixed {
    let mut rval: Fixed = (d * 65536.0f64 + 0.5f64) as libc::c_int;
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn Fix2D(mut f: Fixed) -> libc::c_double {
    let mut rval: libc::c_double = f as libc::c_double / 65536.0f64;
    return rval;
}
/* the metrics params here are really TeX 'scaled' (or MacOS 'Fixed') values, but that typedef isn't available every place this is included */
/* these are here, not XeTeX_mac.c, because we need stubs on other platforms */
#[no_mangle]
pub unsafe extern "C" fn aat_get_font_metrics(
    mut attributes: CFDictionaryRef,
    mut ascent: *mut int32_t,
    mut descent: *mut int32_t,
    mut xheight: *mut int32_t,
    mut capheight: *mut int32_t,
    mut slant: *mut int32_t,
) {
    let mut font: CTFontRef = fontFromAttributes(attributes);
    *ascent = D2Fix(CTFontGetAscent(font));
    *descent = D2Fix(CTFontGetDescent(font));
    *xheight = D2Fix(CTFontGetXHeight(font));
    *capheight = D2Fix(CTFontGetCapHeight(font));
    *slant = D2Fix(tan(-CTFontGetSlantAngle(font)
        * 3.14159265358979323846264338327950288f64
        / 180.0f64));
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get(
    mut what: libc::c_int,
    mut attributes: CFDictionaryRef,
) -> libc::c_int {
    let mut rval: libc::c_int = -1i32;
    let mut font: CTFontRef = fontFromAttributes(attributes);
    let mut list: CFArrayRef = 0 as *const __CFArray;
    match what {
        1 => rval = CTFontGetGlyphCount(font) as libc::c_int,
        8 => {
            list = CTFontCopyFeatures(font);
            if !list.is_null() {
                rval = CFArrayGetCount(list) as libc::c_int;
                CFRelease(list as CFTypeRef);
            }
        }
        _ => {}
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get_1(
    mut what: libc::c_int,
    mut attributes: CFDictionaryRef,
    mut param: libc::c_int,
) -> libc::c_int {
    let mut rval: libc::c_int = -1i32;
    let mut font: CTFontRef = fontFromAttributes(attributes);
    match what {
        9 => {
            let mut features: CFArrayRef = CTFontCopyFeatures(font);
            if !features.is_null() {
                if CFArrayGetCount(features) > param as libc::c_long {
                    let mut feature: CFDictionaryRef =
                        CFArrayGetValueAtIndex(features, param as CFIndex) as CFDictionaryRef;
                    let mut identifier: CFNumberRef = CFDictionaryGetValue(
                        feature,
                        kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                    ) as CFNumberRef;
                    if !identifier.is_null() {
                        CFNumberGetValue(
                            identifier,
                            kCFNumberIntType as libc::c_int as CFNumberType,
                            &mut rval as *mut libc::c_int as *mut libc::c_void,
                        );
                    }
                }
                CFRelease(features as CFTypeRef);
            }
        }
        11 => {
            let mut features_0: CFArrayRef = CTFontCopyFeatures(font);
            if !features_0.is_null() {
                let mut value: CFBooleanRef = 0 as *const __CFBoolean;
                let mut feature_0: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                    features_0,
                    kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                    param,
                );
                let mut found: Boolean = CFDictionaryGetValueIfPresent(
                    feature_0,
                    kCTFontFeatureTypeExclusiveKey as *const libc::c_void,
                    &mut value as *mut CFBooleanRef as *mut *const libc::c_void,
                );
                if found != 0 {
                    rval = CFBooleanGetValue(value) as libc::c_int
                }
                CFRelease(features_0 as CFTypeRef);
            }
        }
        12 => {
            let mut features_1: CFArrayRef = CTFontCopyFeatures(font);
            if !features_1.is_null() {
                let mut feature_1: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                    features_1,
                    kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                    param,
                );
                if !feature_1.is_null() {
                    let mut selectors: CFArrayRef = CFDictionaryGetValue(
                        feature_1,
                        kCTFontFeatureTypeSelectorsKey as *const libc::c_void,
                    ) as CFArrayRef;
                    if !selectors.is_null() {
                        rval = CFArrayGetCount(selectors) as libc::c_int
                    }
                }
                CFRelease(features_1 as CFTypeRef);
            }
        }
        _ => {}
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get_2(
    mut what: libc::c_int,
    mut attributes: CFDictionaryRef,
    mut param1: libc::c_int,
    mut param2: libc::c_int,
) -> libc::c_int {
    let mut rval: libc::c_int = -1i32;
    let mut font: CTFontRef = fontFromAttributes(attributes);
    let mut features: CFArrayRef = CTFontCopyFeatures(font);
    if !features.is_null() {
        let mut feature: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
            features,
            kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
            param1,
        );
        if !feature.is_null() {
            let mut selectors: CFArrayRef = CFDictionaryGetValue(
                feature,
                kCTFontFeatureTypeSelectorsKey as *const libc::c_void,
            ) as CFArrayRef;
            if !selectors.is_null() {
                let mut selector: CFDictionaryRef = 0 as *const __CFDictionary;
                match what {
                    13 => {
                        if CFArrayGetCount(selectors) > param2 as libc::c_long {
                            let mut identifier: CFNumberRef = 0 as *const __CFNumber;
                            selector = CFArrayGetValueAtIndex(selectors, param2 as CFIndex)
                                as CFDictionaryRef;
                            identifier = CFDictionaryGetValue(
                                selector,
                                kCTFontFeatureSelectorIdentifierKey as *const libc::c_void,
                            ) as CFNumberRef;
                            if !identifier.is_null() {
                                CFNumberGetValue(
                                    identifier,
                                    kCFNumberIntType as libc::c_int as CFNumberType,
                                    &mut rval as *mut libc::c_int as *mut libc::c_void,
                                );
                            }
                        }
                    }
                    15 => {
                        selector = findDictionaryInArrayWithIdentifier(
                            selectors,
                            kCTFontFeatureSelectorIdentifierKey as *const libc::c_void,
                            param2,
                        );
                        if !selector.is_null() {
                            let mut isDefault: CFBooleanRef = 0 as *const __CFBoolean;
                            let mut found: Boolean = CFDictionaryGetValueIfPresent(
                                selector,
                                kCTFontFeatureSelectorDefaultKey as *const libc::c_void,
                                &mut isDefault as *mut CFBooleanRef as *mut *const libc::c_void,
                            );
                            if found != 0 {
                                rval = CFBooleanGetValue(isDefault) as libc::c_int
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        CFRelease(features as CFTypeRef);
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get_named(
    mut what: libc::c_int,
    mut attributes: CFDictionaryRef,
) -> libc::c_int {
    let mut rval: libc::c_int = -1i32;
    if what == 10i32 {
        let mut font: CTFontRef = fontFromAttributes(attributes);
        let mut features: CFArrayRef = CTFontCopyFeatures(font);
        if !features.is_null() {
            let mut feature: CFDictionaryRef = findDictionaryInArray(
                features,
                kCTFontFeatureTypeNameKey as *const libc::c_void,
                name_of_file,
                name_length,
            );
            if !feature.is_null() {
                let mut identifier: CFNumberRef = CFDictionaryGetValue(
                    feature,
                    kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                ) as CFNumberRef;
                CFNumberGetValue(
                    identifier,
                    kCFNumberIntType as libc::c_int as CFNumberType,
                    &mut rval as *mut libc::c_int as *mut libc::c_void,
                );
            }
            CFRelease(features as CFTypeRef);
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get_named_1(
    mut what: libc::c_int,
    mut attributes: CFDictionaryRef,
    mut param: libc::c_int,
) -> libc::c_int {
    let mut rval: libc::c_int = -1i32;
    let mut font: CTFontRef = fontFromAttributes(attributes);
    if what == 14i32 {
        let mut features: CFArrayRef = CTFontCopyFeatures(font);
        if !features.is_null() {
            let mut feature: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                features,
                kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                param,
            );
            if !feature.is_null() {
                let mut selector: CFNumberRef =
                    findSelectorByName(feature, name_of_file, name_length);
                if !selector.is_null() {
                    CFNumberGetValue(
                        selector,
                        kCFNumberIntType as libc::c_int as CFNumberType,
                        &mut rval as *mut libc::c_int as *mut libc::c_void,
                    );
                }
            }
            CFRelease(features as CFTypeRef);
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_print_font_name(
    mut what: libc::c_int,
    mut attributes: CFDictionaryRef,
    mut param1: libc::c_int,
    mut param2: libc::c_int,
) {
    let mut name: CFStringRef = 0 as CFStringRef;
    if what == 8i32 || what == 9i32 {
        let mut font: CTFontRef = fontFromAttributes(attributes);
        let mut features: CFArrayRef = CTFontCopyFeatures(font);
        if !features.is_null() {
            let mut feature: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                features,
                kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                param1,
            );
            if !feature.is_null() {
                if what == 8i32 {
                    name = CFDictionaryGetValue(
                        feature,
                        kCTFontFeatureTypeNameKey as *const libc::c_void,
                    ) as CFStringRef
                } else {
                    let mut selectors: CFArrayRef = CFDictionaryGetValue(
                        feature,
                        kCTFontFeatureTypeSelectorsKey as *const libc::c_void,
                    ) as CFArrayRef;
                    let mut selector: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                        selectors,
                        kCTFontFeatureSelectorIdentifierKey as *const libc::c_void,
                        param2,
                    );
                    if !selector.is_null() {
                        name = CFDictionaryGetValue(
                            selector,
                            kCTFontFeatureSelectorNameKey as *const libc::c_void,
                        ) as CFStringRef
                    }
                }
            }
            CFRelease(features as CFTypeRef);
        }
    }
    if !name.is_null() {
        let mut len: CFIndex = CFStringGetLength(name);
        let mut buf: *mut UniChar = xcalloc(
            len as size_t,
            ::std::mem::size_of::<UniChar>() as libc::c_ulong,
        ) as *mut UniChar;
        CFStringGetCharacters(name, CFRangeMake(0i32 as CFIndex, len), buf);
        print_chars(buf, len as libc::c_int);
        free(buf as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_glyph_name(mut font: int32_t, mut gid: int32_t) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0i32;
    if *font_area.offset(font as isize) as libc::c_uint == 0xffffu32 {
        s = GetGlyphNameFromCTFont(fontFromInteger(font), gid as uint16_t, &mut len)
    } else if *font_area.offset(font as isize) as libc::c_uint == 0xfffeu32 {
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine;
        s = getGlyphName(getFont(engine), gid as uint16_t, &mut len)
    } else {
        _tt_abort(
            b"bad native font flag in `print_glyph_name`\x00" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        let fresh33 = len;
        len = len - 1;
        if !(fresh33 > 0i32) {
            break;
        }
        let fresh34 = s;
        s = s.offset(1);
        print_char(*fresh34 as int32_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn real_get_native_word_cp(
    mut pNode: *mut libc::c_void,
    mut side: libc::c_int,
) -> int32_t {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut locations: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint;
    let mut glyphIDs: *mut uint16_t =
        locations.offset((*node.offset(4)).b16.s0 as libc::c_int as isize) as *mut uint16_t;
    let mut glyphCount: uint16_t = (*node.offset(4)).b16.s0;
    let mut f: int32_t = (*node.offset(4)).b16.s2 as int32_t;
    let mut actual_glyph: uint16_t = 0;
    if glyphCount as libc::c_int == 0i32 {
        return 0i32;
    }
    match side {
        0 => {
            actual_glyph = *glyphIDs
            // we should not reach this point
        }
        1 => actual_glyph = *glyphIDs.offset((glyphCount as libc::c_int - 1i32) as isize),
        _ => {
            if (0i32 == 0) as libc::c_int as libc::c_long != 0 {
                __assert_rtn(
                    (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                        b"real_get_native_word_cp\x00",
                    ))
                    .as_ptr(),
                    b"tectonic/xetex-ext.c\x00" as *const u8 as *const libc::c_char,
                    2136i32,
                    b"0\x00" as *const u8 as *const libc::c_char,
                );
            } else {
            };
        }
    }
    return get_cp_code(f, actual_glyph as libc::c_uint, side);
}
