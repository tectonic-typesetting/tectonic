/* openclose.c: open and close files for TeX, Metafont, and BibTeX.

   Written 1995 Karl Berry.  Public domain.  */

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/TECkit_Engine.h>
#include <tectonic/XeTeXLayoutInterface.h>
#include <tectonic/XeTeXswap.h>
#include <tectonic/xetexd.h>
#include <tectonic/stubs.h>

#include <unicode/ubidi.h>
#include <unicode/ubrk.h>
#include <unicode/ucnv.h>

/* Define some variables. */
/* For "file:line:error" style error messages. */
string fullnameoffile;          /* Defaults to NULL.  */

/* For TeX and MetaPost.  See below.  Always defined so we don't have to
   #ifdef, and thus this file can be compiled once and go in lib.a.  */
int tfm_temp;
int ocptemp;
int tex_input_type;


/* Open an input file F, using the kpathsea format FILEFMT and passing
   FOPEN_MODE to fopen.  The filename is in `name_of_file+1'.  We return
   whether or not the open succeeded.  If it did, `name_of_file' is set to
   the full filename opened, and `name_length' to its length.  */

boolean
open_input(FILE ** f_ptr, int filefmt, const_string fopen_mode)
{
    string fname = NULL;

    /* We havent found anything yet. */
    *f_ptr = NULL;
    if (fullnameoffile)
        free(fullnameoffile);
    fullnameoffile = NULL;

    if (filefmt < 0) {
	/* A negative FILEFMT means don't use a path, for BibTeX .aux files
	 * and MetaPost things. */
	*f_ptr = fopen(name_of_file + 1, fopen_mode);
	/* FIXME... fullnameoffile = xstrdup(name_of_file + 1); */
    } else {
	/* The only exception to `must_exist' being true is \openin, for which
	   we set `tex_input_type' to 0 in the change file. According to the
	   pdfTeX people, pounding the disk for .vf files is overkill as well.
	   A more general solution would be nice. */

	boolean must_exist = (filefmt != kpse_tex_format || tex_input_type)
	    && (filefmt != kpse_vf_format);
	int fd;

	/* Begin nontrivial tectonic customizations: */

	fname = name_of_file + 1;
	fd = kpsezip_get_readable_fd (fname, (kpse_file_format_type) filefmt, must_exist);
	if (fd < 0)
	    return false;

	fullnameoffile = xstrdup(fname);
	name_length = strlen(fname);
	name_of_file = xmalloc(name_length + 2);
	strcpy(name_of_file + 1, fname);

	*f_ptr = fdopen(fd, fopen_mode);
	if (!*f_ptr)
	    _tt_abort("fdopen(%d) failed: %s", fd, strerror(errno));

	/* End tectonic customizations. */
    }

    if (*f_ptr) {
        /*recorder_record_input (name_of_file + 1); */

        /* If we just opened a TFM file, we have to read the first
           byte, to pretend we're Pascal.  See tex.ch and mp.ch.
           Ditto for the ocp/ofm Omega file formats.  */
        if (filefmt == kpse_tfm_format) {
            tfm_temp = getc(*f_ptr);
            /* We intentionally do not check for EOF here, i.e., an
               empty TFM file.  TeX will see the 255 byte and complain
               about a bad TFM file, which is what we want.  */
        } else if (filefmt == kpse_ocp_format) {
            ocptemp = getc(*f_ptr);
        } else if (filefmt == kpse_ofm_format) {
            tfm_temp = getc(*f_ptr);
        }
    }

    return *f_ptr != NULL;
}


/* Close F.  */

void
close_file(FILE * f)
{
    /* If F is null, just return.  bad_pool might close a file that has
       never been opened.  */
    if (!f)
        return;

    if (fclose(f) == EOF) {
        /* It's not always name_of_file, we might have opened something else
           in the meantime.  And it's not easy to extract the filenames out
           of the pool array.  So just punt on the filename.  Sigh.  This
           probably doesn't need to be a fatal error.  */
        perror("fclose");
    }
}


/* XeTeX I/O */

/* tables/values used in UTF-8 interpretation -
   code is based on ConvertUTF.[ch] sample code
   published by the Unicode consortium */
const uint32_t
offsetsFromUTF8[6] =    {
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

const int halfShift                 = 10;
const uint32_t halfBase             = 0x0010000UL;
const uint32_t halfMask             = 0x3FFUL;
const uint32_t kSurrogateHighStart  = 0xD800UL;
const uint32_t kSurrogateHighEnd        = 0xDBFFUL;
const uint32_t kSurrogateLowStart       = 0xDC00UL;
const uint32_t kSurrogateLowEnd     = 0xDFFFUL;
const uint32_t byteMask             = 0x000000BFUL;
const uint32_t byteMark             = 0x00000080UL;


void
set_input_file_encoding(UFILE* f, integer mode, integer encodingData)
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
u_open_in(UFILE **f, integer filefmt, const_string fopen_mode, integer mode, integer encodingData)
{
    boolean rval;
    *f = (UFILE *) xmalloc(sizeof(UFILE));
    (*f)->encodingMode = 0;
    (*f)->conversionData = 0;
    (*f)->savedChar = -1;
    (*f)->skipNextLF = 0;
    rval = open_input (&((*f)->f), filefmt, fopen_mode);
    if (rval) {
        int B1, B2;
        if (mode == AUTO) {
            /* sniff encoding form */
            B1 = getc_unlocked((*f)->f);
            B2 = getc_unlocked((*f)->f);
            if (B1 == 0xfe && B2 == 0xff)
                mode = UTF16BE;
            else if (B2 == 0xfe && B1 == 0xff)
                mode = UTF16LE;
            else if (B1 == 0 && B2 != 0) {
                mode = UTF16BE;
                rewind((*f)->f);
            } else if (B2 == 0 && B1 != 0) {
                mode = UTF16LE;
                rewind((*f)->f);
            } else if (B1 == 0xef && B2 == 0xbb) {
                int B3 = getc_unlocked((*f)->f);
                if (B3 == 0xbf)
                    mode = UTF8;
            }
            if (mode == AUTO) {
                rewind((*f)->f);
                mode = UTF8;
            }
        }

        set_input_file_encoding(*f, mode, encodingData);
    }
    return rval;
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

    last = first;

    if (f->encodingMode == ICUMAPPING) {
        uint32_t bytesRead = 0;
        UConverter* cnv;
        int outLen;
        UErrorCode errorCode = U_ZERO_ERROR;

        if (byteBuffer == NULL)
            byteBuffer = (char*) xmalloc(buf_size + 1);

        /* Recognize either LF or CR as a line terminator; skip initial LF if prev line ended with CR.  */
        i = getc_unlocked(f->f);
        if (f->skipNextLF) {
            f->skipNextLF = 0;
            if (i == '\n')
                i = getc_unlocked(f->f);
        }

        if (i != EOF && i != '\n' && i != '\r')
            byteBuffer[bytesRead++] = i;
        if (i != EOF && i != '\n' && i != '\r')
            while (bytesRead < buf_size && (i = getc_unlocked(f->f)) != EOF && i != '\n' && i != '\r')
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
                    utf32Buf = (uint32_t*) xcalloc(buf_size, sizeof(uint32_t));
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
                    utf32Buf = (uint32_t*) xcalloc(buf_size, sizeof(uint32_t));
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

    /* Trim trailing whitespace.  */
    while (last > first && ISBLANK(buffer[last - 1]))
        --last;

    return true;
}


void
u_close(UFILE* f)
{
    if (f != 0) {
        fclose(f->f);
        if ((f->encodingMode == ICUMAPPING) && (f->conversionData != NULL))
            ucnv_close((UConverter*)(f->conversionData));
        free((void*)f);
    }
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
            c = rval = getc_unlocked(f->f);
            if (rval != EOF) {
                uint16_t extraBytes = bytesFromUTF8[rval];
                switch (extraBytes) {   /* note: code falls through cases! */
                    case 3: c = getc_unlocked(f->f);
                        if (c < 0x80 || c >= 0xc0) goto bad_utf8;
                        rval <<= 6; rval += c;
                    case 2: c = getc_unlocked(f->f);
                        if (c < 0x80 || c >= 0xc0) goto bad_utf8;
                        rval <<= 6; rval += c;
                    case 1: c = getc_unlocked(f->f);
                        if (c < 0x80 || c >= 0xc0) goto bad_utf8;
                        rval <<= 6; rval += c;
                    case 0:
                        break;

                    bad_utf8:
                        if (c != EOF)
                            ungetc(c, f->f);
                    case 5:
                    case 4:
                        bad_utf8_warning();
                        return 0xfffd;      /* return without adjusting by offsetsFromUTF8 */
                };
                rval -= offsetsFromUTF8[extraBytes];
            }
            break;

        case UTF16BE:
            rval = getc_unlocked(f->f);
            if (rval != EOF) {
                rval <<= 8;
                rval += getc_unlocked(f->f);
                if (rval >= 0xd800 && rval <= 0xdbff) {
                    int lo = getc_unlocked(f->f);
                    lo <<= 8;
                    lo += getc_unlocked(f->f);
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
            rval = getc_unlocked(f->f);
            if (rval != EOF) {
                rval += (getc_unlocked(f->f) << 8);
                if (rval >= 0xd800 && rval <= 0xdbff) {
                    int lo = getc_unlocked(f->f);
                    lo += (getc_unlocked(f->f) << 8);
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
            rval = getc_unlocked(f->f);
            break;

        default:
	    _tt_abort("internal error; file input mode=%d", f->encodingMode);
    }

    return rval;
}


void
make_utf16_name(void)
{
    unsigned char* s = name_of_file + 1;
    uint32_t rval;
    uint16_t* t;
    static int name16len = 0;
    if (name16len <= name_length) {
        if (name_of_file16 != 0)
            free(name_of_file16);
        name16len = name_length + 10;
        name_of_file16 = (uint16_t*) xcalloc(name16len, sizeof(uint16_t));
    }
    t = name_of_file16;
    while (s <= name_of_file + name_length) {
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
    memory_word *eqtb = zeqtb;
    unsigned char c, n;
    integer k;

    c = cur_chr;
    scan_four_bit_int();
    n = cur_val;

    if (read_open[n] != 2 /*closed */ ) {
        u_close(read_file[n]);
        read_open[n] = 2 /*closed */ ;
    }

    if (c != 0) {
        scan_optional_equals();
        scan_file_name();
        pack_file_name(cur_name, cur_area, cur_ext);
        tex_input_type = 0;

        if (u_open_in(&read_file[n], kpse_tex_format, "rb", eqtb[8938817L /*eTeX_state_base 6 */ ].cint,
		      eqtb[8938818L /*eTeX_state_base 7 */ ].cint)) {
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
            read_open[n] = 1 /*just_open */ ;
        }
    }
}
