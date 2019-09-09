/* tectonic/stub_icu.c: Binding stub for ICU functions.
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/

#include <unicode/ubidi.h>
#include <unicode/ubrk.h>
#include <unicode/ucnv.h>

UBiDi * tt_ubidi_open(void) {
    return ubidi_open();
}
void tt_ubidi_close(UBiDi *pBiDi) {
    return ubidi_close(pBiDi);
}
void tt_ubidi_setPara(UBiDi *pBiDi, const UChar *text, int32_t length,
              UBiDiLevel paraLevel, UBiDiLevel *embeddingLevels,
              UErrorCode *pErrorCode) {
    return ubidi_setPara(pBiDi, text, length,
              paraLevel, embeddingLevels,
              pErrorCode);
}
UBiDiDirection tt_ubidi_getDirection(const UBiDi *pBiDi) {
    return ubidi_getDirection(pBiDi);
}
UBiDiDirection tt_ubidi_getVisualRun(UBiDi *pBiDi, int32_t runIndex,
                   int32_t *pLogicalStart, int32_t *pLength) {
    return ubidi_getVisualRun(pBiDi, runIndex, pLogicalStart, pLength);
}

int32_t tt_ubidi_countRuns(UBiDi *pBiDi, UErrorCode *pErrorCode) {
    return ubidi_countRuns(pBiDi, pErrorCode);
}

UBreakIterator* tt_ubrk_open(UBreakIteratorType type,
      const char *locale,
      const UChar *text,
      int32_t textLength,
      UErrorCode *status) {
    return ubrk_open(type,
      locale,
      text,
      textLength,
      status);
}

int32_t tt_ubrk_next(UBreakIterator *bi) {
    return ubrk_next(bi);
}

void tt_ubrk_close(UBreakIterator *bi) {
    return ubrk_close(bi);
}

void tt_ubrk_setText(UBreakIterator* bi,
             const UChar*    text,
             int32_t         textLength,
             UErrorCode*     status) {
    return ubrk_setText(bi, text, textLength, status);
}

UConverter* tt_ucnv_open(const char *converterName, UErrorCode *err) {
    return ucnv_open(converterName, err);
}

void tt_ucnv_close(UConverter * converter) {
    return ucnv_close(converter);
}

int32_t tt_ucnv_toAlgorithmic(UConverterType algorithmicType,
                   UConverter *cnv,
                   char *target, int32_t targetCapacity,
                   const char *source, int32_t sourceLength,
                   UErrorCode *pErrorCode) {
    return ucnv_toAlgorithmic(algorithmicType,
                   cnv, target, targetCapacity,
                   source, sourceLength, pErrorCode);
}
