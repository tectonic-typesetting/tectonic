/* tectonic/xetex-io.c: low-level input/output functions tied to the XeTeX engine
   Copyright 2016-2019 The Tectonic Project
   Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "teckit-c-Engine.h"
#include "xetex-swap.h"
#include "xetex-xetexd.h"

#include <stdio.h>
#include <unicode/ucnv.h>

char *name_of_input_file = NULL;


rust_input_handle_t
tt_xetex_open_input (int filefmt)
{
    rust_input_handle_t handle;

    if (filefmt == TTBC_FILE_FORMAT_TECTONIC_PRIMARY)
        handle = ttstub_input_open_primary ();
    else
        handle = ttstub_input_open (name_of_file, (ttbc_file_format) filefmt, 0);

    if (handle == NULL)
        return NULL;

    name_length = strlen(name_of_file);
    free(name_of_input_file);
    name_of_input_file = xstrdup(name_of_file);
    return handle;
}

/* tables/values used in UTF-8 interpretation -
   code is based on ConvertUTF.[ch] sample code
   published by the Unicode consortium */
const uint32_t
offsetsFromUTF8[6] = {
    0x00000000UL,
    0x00003080UL,
    0x000E2080UL,
    0x03C82080UL,
    0xFA082080UL,
    0x82082080UL
};

const uint8_t
bytesFromUTF8[256] = {
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, 3,3,3,3,3,3,3,3,4,4,4,4,5,5,5,5
};

const uint8_t
firstByteMark[7] = {
    0x00, 0x00, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC
};

void
set_input_file_encoding(UFILE* f, int32_t mode, int32_t encodingData)
{
    if ((f->encodingMode == ICUMAPPING) && (f->conversionData != NULL))
        ucnv_close((UConverter*)(f->conversionData));
    f->conversionData = 0;

    switch (mode) {
        case UTF8:
        case UTF16BE:
        case UTF16LE:
        case RAW:
            f->encodingMode = mode;
            break;

        case ICUMAPPING:
            {
                char* name = gettexstring(encodingData);
                UErrorCode err = U_ZERO_ERROR;
                UConverter* cnv = ucnv_open(name, &err);
                if (cnv == NULL) {
                    begin_diagnostic();
                    print_nl('E');
                    print_c_string("rror ");
                    print_int(err);
                    print_c_string(" creating Unicode converter for `");
                    print_c_string(name);
                    print_c_string("'; reading as raw bytes");
                    end_diagnostic(1);
                    f->encodingMode = RAW;
                } else {
                    f->encodingMode = ICUMAPPING;
                    f->conversionData = cnv;
                }
                free(name);
            }
            break;
    }
}


int
u_open_in(UFILE **f, int32_t filefmt, const char *fopen_mode, int32_t mode, int32_t encodingData)
{
    rust_input_handle_t handle;
    int B1, B2;

    handle = tt_xetex_open_input (filefmt);
    if (handle == NULL)
        return 0;

    *f = xmalloc(sizeof(UFILE));
    (*f)->encodingMode = 0;
    (*f)->conversionData = 0;
    (*f)->savedChar = -1;
    (*f)->skipNextLF = 0;
    (*f)->handle = handle;

    if (mode == AUTO) {
        /* sniff encoding form */
        B1 = ttstub_input_getc ((*f)->handle);
        B2 = ttstub_input_getc ((*f)->handle);

        if (B1 == 0xfe && B2 == 0xff)
            mode = UTF16BE;
        else if (B2 == 0xfe && B1 == 0xff)
            mode = UTF16LE;
        else if (B1 == 0 && B2 != 0) {
            mode = UTF16BE;
            ttstub_input_seek ((*f)->handle, 0, SEEK_SET);
        } else if (B2 == 0 && B1 != 0) {
            mode = UTF16LE;
            ttstub_input_seek ((*f)->handle, 0, SEEK_SET);
        } else if (B1 == 0xEF && B2 == 0xBB) {
            int B3 = ttstub_input_getc((*f)->handle);
            if (B3 == 0xBF)
                mode = UTF8;
        }

        if (mode == AUTO) {
            ttstub_input_seek ((*f)->handle, 0, SEEK_SET);
            mode = UTF8;
        }
    }

    set_input_file_encoding(*f, mode, encodingData);
    return 1;
}


static void
buffer_overflow(void)
{
    _tt_abort("unable to read an entire line (buf_size=%u)", (unsigned) buf_size);
}


static void
conversion_error(int errcode)
{
    begin_diagnostic();
    print_nl('U');
    print_c_string("nicode conversion failed (ICU error code = ");
    print_int(errcode);
    print_c_string(") discarding any remaining text");
    end_diagnostic(1);
}


static void
apply_normalization(uint32_t* buf, int len, int norm)
{
    static TECkit_Converter normalizers[2] = { NULL, NULL };

    TECkit_Status status;
    UInt32 inUsed, outUsed;
    TECkit_Converter *normPtr = &normalizers[norm - 1];
    if (*normPtr == NULL) {
        status = TECkit_CreateConverter(NULL, 0, 1,
            NATIVE_UTF32, NATIVE_UTF32 | (norm == 1 ? kForm_NFC : kForm_NFD),
            &*normPtr);
        if (status != kStatus_NoError)
            _tt_abort ("failed to create normalizer: error code = %d", (int)status);
    }

    status = TECkit_ConvertBuffer(*normPtr, (Byte*)buf, len * sizeof(UInt32), &inUsed,
                (Byte*)&buffer[first], sizeof(*buffer) * (buf_size - first), &outUsed, 1);
    if (status != kStatus_NoError)
        buffer_overflow();
    last = first + outUsed / sizeof(*buffer);
}


int
input_line(UFILE* f)
{
    static char* byteBuffer = NULL;
    static uint32_t *utf32Buf = NULL;
    int i, tmpLen;
    int norm = get_input_normalization_state();

    if (f->handle == NULL)
        /* NULL 'handle' means this: */
        _tt_abort ("reads from synthetic \"terminal\" file #0 should never happen");

    last = first;

    if (f->encodingMode == ICUMAPPING) {
        uint32_t bytesRead = 0;
        UConverter* cnv;
        int outLen;
        UErrorCode errorCode = U_ZERO_ERROR;

        if (byteBuffer == NULL)
            byteBuffer = xmalloc(buf_size + 1);

        /* Recognize either LF or CR as a line terminator; skip initial LF if prev line ended with CR.  */
        i = ttstub_input_getc (f->handle);
        if (f->skipNextLF) {
            f->skipNextLF = 0;
            if (i == '\n')
                i = ttstub_input_getc (f->handle);
        }

        if (i != EOF && i != '\n' && i != '\r')
            byteBuffer[bytesRead++] = i;
        if (i != EOF && i != '\n' && i != '\r')
            while (bytesRead < buf_size && (i = ttstub_input_getc(f->handle)) != EOF && i != '\n' && i != '\r')
                byteBuffer[bytesRead++] = i;

        if (i == EOF && errno != EINTR && bytesRead == 0)
            return false;

        if (i != EOF && i != '\n' && i != '\r')
            buffer_overflow();

        /* now apply the mapping to turn external bytes into Unicode characters in buffer */
        cnv = (UConverter*)(f->conversionData);
        switch (norm) {
            case 1: // NFC
            case 2: // NFD
                if (utf32Buf == NULL)
                    utf32Buf = xcalloc(buf_size, sizeof(uint32_t));
                tmpLen = ucnv_toAlgorithmic(UCNV_UTF32_NativeEndian, cnv,
                                            (char*)utf32Buf, buf_size * sizeof(*utf32Buf),
                                            byteBuffer, bytesRead, &errorCode);
                if (errorCode != 0) {
                    conversion_error((int)errorCode);
                    return false;
                }
                apply_normalization(utf32Buf, tmpLen / sizeof(*utf32Buf), norm); // sets 'last' correctly
                break;

            default: // none
                outLen = ucnv_toAlgorithmic(UCNV_UTF32_NativeEndian, cnv,
                                            (char*)&buffer[first], sizeof(*buffer) * (buf_size - first),
                                            byteBuffer, bytesRead, &errorCode);
                if (errorCode != 0) {
                    conversion_error((int)errorCode);
                    return false;
                }
                outLen /= sizeof(*buffer);
                last = first + outLen;
                break;
        }
    } else {
        /* Recognize either LF or CR as a line terminator; skip initial LF if prev line ended with CR.  */
        i = get_uni_c(f);
        if (f->skipNextLF) {
            f->skipNextLF = 0;
            if (i == '\n')
                i = get_uni_c(f);
        }

        switch (norm) {
            case 1: // NFC
            case 2: // NFD
                // read Unicode chars into utf32Buf as UTF32
                if (utf32Buf == NULL)
                    utf32Buf = xcalloc(buf_size, sizeof(uint32_t));
                tmpLen = 0;
                if (i != EOF && i != '\n' && i != '\r')
                    utf32Buf[tmpLen++] = i;
                if (i != EOF && i != '\n' && i != '\r')
                    while (tmpLen < buf_size && (i = get_uni_c(f)) != EOF && i != '\n' && i != '\r')
                        utf32Buf[tmpLen++] = i;

                if (i == EOF && errno != EINTR && tmpLen == 0)
                    return false;

                /* We didn't get the whole line because our buffer was too small.  */
                if (i != EOF && i != '\n' && i != '\r')
                    buffer_overflow();
                apply_normalization(utf32Buf, tmpLen, norm);
                break;

            default: // none
                if (last < buf_size && i != EOF && i != '\n' && i != '\r')
                    buffer[last++] = i;
                if (i != EOF && i != '\n' && i != '\r')
                    while (last < buf_size && (i = get_uni_c(f)) != EOF && i != '\n' && i != '\r')
                        buffer[last++] = i;

                if (i == EOF && errno != EINTR && last == first)
                    return false;

                /* We didn't get the whole line because our buffer was too small.  */
                if (i != EOF && i != '\n' && i != '\r')
                    buffer_overflow();
                break;
        }
    }

    /* If line ended with CR, remember to skip following LF. */
    if (i == '\r')
        f->skipNextLF = 1;

    buffer[last] = ' ';
    if (last >= max_buf_stack)
        max_buf_stack = last;

    /* Trim trailing space or EOL characters.  */
#define IS_SPC_OR_EOL(c) ((c) == ' ' || (c) == '\r' || (c) == '\n')
    while (last > first && IS_SPC_OR_EOL(buffer[last - 1]))
        --last;

    return true;
}


void
u_close(UFILE* f)
{
    if (f == NULL || f->handle == NULL)
        /* NULL handle is stdin/terminal file. Shouldn't happen but meh. */
        return;

    ttstub_input_close (f->handle);

    if (f->encodingMode == ICUMAPPING && f->conversionData != NULL)
        ucnv_close ((UConverter*) f->conversionData);

    free (f);
}


int
get_uni_c(UFILE* f)
{
    int rval;
    int c;

    if (f->savedChar != -1) {
        rval = f->savedChar;
        f->savedChar = -1;
        return rval;
    }

    switch (f->encodingMode) {
        case UTF8:
            c = rval = ttstub_input_getc(f->handle);
            if (rval != EOF) {
                uint16_t extraBytes = bytesFromUTF8[rval];
                switch (extraBytes) {
                /* note: code falls through cases! */
                case 3:
                    c = ttstub_input_getc(f->handle);
                    if (c < 0x80 || c >= 0xC0)
                        goto bad_utf8;
                    rval <<= 6;
                    rval += c;
                case 2:
                    c = ttstub_input_getc(f->handle);
                    if (c < 0x80 || c >= 0xC0)
                        goto bad_utf8;
                    rval <<= 6;
                    rval += c;
                case 1:
                    c = ttstub_input_getc(f->handle);
                    if (c < 0x80 || c >= 0xC0)
                        goto bad_utf8;
                    rval <<= 6;
                    rval += c;
                case 0:
                    break;

                bad_utf8:
                    if (c != EOF)
                        ttstub_input_ungetc(f->handle, c);
                case 5:
                case 4:
                    bad_utf8_warning();
                    return 0xFFFD; /* return without adjusting by offsetsFromUTF8 */
                };

                rval -= offsetsFromUTF8[extraBytes];

                if (rval < 0 || rval > 0x10ffff) {
                    bad_utf8_warning();
                    return 0xfffd;
                }
            }
            break;

        case UTF16BE:
            rval = ttstub_input_getc(f->handle);
            if (rval != EOF) {
                rval <<= 8;
                rval += ttstub_input_getc(f->handle);
                if (rval >= 0xd800 && rval <= 0xdbff) {
                    int lo = ttstub_input_getc(f->handle);
                    lo <<= 8;
                    lo += ttstub_input_getc(f->handle);
                    if (lo >= 0xdc00 && lo <= 0xdfff)
                        rval = 0x10000 + (rval - 0xd800) * 0x400 + (lo - 0xdc00);
                    else {
                        rval = 0xfffd;
                        f->savedChar = lo;
                    }
                } else if (rval >= 0xdc00 && rval <= 0xdfff)
                    rval = 0xfffd;
            }
            break;

        case UTF16LE:
            rval = ttstub_input_getc(f->handle);
            if (rval != EOF) {
                rval += (ttstub_input_getc(f->handle) << 8);
                if (rval >= 0xd800 && rval <= 0xdbff) {
                    int lo = ttstub_input_getc(f->handle);
                    lo += (ttstub_input_getc(f->handle) << 8);
                    if (lo >= 0xdc00 && lo <= 0xdfff)
                        rval = 0x10000 + (rval - 0xd800) * 0x400 + (lo - 0xdc00);
                    else {
                        rval = 0xfffd;
                        f->savedChar = lo;
                    }
                } else if (rval >= 0xdc00 && rval <= 0xdfff)
                    rval = 0xfffd;
            }
            break;

        case RAW:
            rval = ttstub_input_getc(f->handle);
            break;

        default:
            _tt_abort("internal error; file input mode=%d", f->encodingMode);
    }

    return rval;
}


void
make_utf16_name(void)
{
    unsigned char* s = (unsigned char *) name_of_file;
    uint32_t rval;
    uint16_t* t;
    static int name16len = 0;
    if (name16len <= name_length) {
        free(name_of_file16);
        name16len = name_length + 10;
        name_of_file16 = xcalloc(name16len, sizeof(uint16_t));
    }
    t = name_of_file16;

    while (s < (unsigned char *) name_of_file + name_length) {
        uint16_t extraBytes;
        rval = *(s++);
        extraBytes = bytesFromUTF8[rval];
        switch (extraBytes) {   /* note: code falls through cases! */
            case 5: rval <<= 6; if (*s) rval += *(s++);
            case 4: rval <<= 6; if (*s) rval += *(s++);
            case 3: rval <<= 6; if (*s) rval += *(s++);
            case 2: rval <<= 6; if (*s) rval += *(s++);
            case 1: rval <<= 6; if (*s) rval += *(s++);
            case 0: ;
        };
        rval -= offsetsFromUTF8[extraBytes];
        if (rval > 0xffff) {
            rval -= 0x10000;
            *(t++) = 0xd800 + rval / 0x0400;
            *(t++) = 0xdc00 + rval % 0x0400;
        } else {
            *(t++) = rval;
        }
    }
    name_length16 = t - name_of_file16;
}


void
open_or_close_in(void)
{
    unsigned char c, n;
    int32_t k;

    c = cur_chr;
    scan_four_bit_int();
    n = cur_val;

    if (read_open[n] != CLOSED) {
        u_close(read_file[n]);
        read_open[n] = CLOSED;
    }

    if (c != 0) {
        scan_optional_equals();
        scan_file_name();
        pack_file_name(cur_name, cur_area, cur_ext);

        if (u_open_in(&read_file[n], TTBC_FILE_FORMAT_TEX, "rb", INTPAR(xetex_default_input_mode),
                      INTPAR(xetex_default_input_encoding))) {
            make_utf16_name();
            name_in_progress = true;
            begin_name();
            stop_at_space = false;
            k = 0;
            while ((k < name_length16) && (more_name(name_of_file16[k])))
                k++;
            stop_at_space = true;
            end_name();
            name_in_progress = false;
            read_open[n] = JUST_OPEN;
        }
    }
}
