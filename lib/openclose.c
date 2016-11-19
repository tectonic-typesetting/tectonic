/* openclose.c: open and close files for TeX, Metafont, and BibTeX.

   Written 1995 Karl Berry.  Public domain.  */

#include <w2c/config.h>
#include "lib.h"
#include <tidy_kpathutil/public.h>
#include <kpsezip/public.h>

/* The globals we use to communicate.  */
extern string name_of_file;
extern unsigned name_length;

/* Define some variables. */
/* For "file:line:error" style error messages. */
string fullnameoffile;          /* Defaults to NULL.  */
/* For the output-dir option. */
string output_directory;        /* Defaults to NULL.  */

/* For TeX and MetaPost.  See below.  Always defined so we don't have to
   #ifdef, and thus this file can be compiled once and go in lib.a.  */
int tfm_temp;
int ocptemp;
int tex_input_type;


/* Open an input file F, using the kpathsea format FILEFMT and passing
   FOPEN_MODE to fopen.  The filename is in `name_of_file+1'.  We return
   whether or not the open succeeded.  If it did, `name_of_file' is set to
   the full filename opened, and `name_length' to its length.  */

boolean open_input(FILE ** f_ptr, int filefmt, const_string fopen_mode)
{
    string fname = NULL;

    /* We havent found anything yet. */
    *f_ptr = NULL;
    if (fullnameoffile)
        free(fullnameoffile);
    fullnameoffile = NULL;

    /* Look in -output-directory first, if the filename is not
       absolute.  This is because .aux and other such files will get
       written to the output directory, and we have to be able to read
       them from there.  We only look for the name as-is.  */
    if (output_directory && !kpse_absolute_p(name_of_file + 1, false)) {
        fname = concat3(output_directory, DIR_SEP_STRING, name_of_file + 1);
        *f_ptr = fopen(fname, fopen_mode);
        if (*f_ptr) {
            free(name_of_file);
            name_length = strlen(fname);
            name_of_file = xmalloc(name_length + 2);
            strcpy(name_of_file + 1, fname);
            fullnameoffile = fname;
        } else {
            free(fname);
        }
    }

    /* No file means do the normal search. */
    if (*f_ptr == NULL) {
        /* A negative FILEFMT means don't use a path.  */
        if (filefmt < 0) {
            /* no_file_path, for BibTeX .aux files and MetaPost things.  */
            *f_ptr = fopen(name_of_file + 1, fopen_mode);
            /* FIXME... fullnameoffile = xstrdup(name_of_file + 1); */
        } else {
            /* The only exception to `must_exist' being true is \openin, for
               which we set `tex_input_type' to 0 in the change file.  */
            /* According to the pdfTeX people, pounding the disk for .vf files
               is overkill as well.  A more general solution would be nice. */
            boolean must_exist = (filefmt != kpse_tex_format || tex_input_type)
                && (filefmt != kpse_vf_format);
            fname = kpse_find_file(name_of_file + 1, (kpse_file_format_type) filefmt, must_exist);
            if (fname) {
                fullnameoffile = xstrdup(fname);
                /* If we found the file in the current directory, don't leave
                   the `./' at the beginning of `name_of_file', since it looks
                   dumb when `tex foo' says `(./foo.tex ... )'.  On the other
                   hand, if the user said `tex ./foo', and that's what we
                   opened, then keep it -- the user specified it, so we
                   shouldn't remove it.  */
                if (fname[0] == '.' && IS_DIR_SEP(fname[1])
                    && (name_of_file[1] != '.' || !IS_DIR_SEP(name_of_file[2]))) {
                    unsigned i = 0;
                    while (fname[i + 2] != 0) {
                        fname[i] = fname[i + 2];
                        i++;
                    }
                    fname[i] = 0;
                }

                /* kpse_find_file always returns a new string. */
                free(name_of_file);
                name_length = strlen(fname);
                name_of_file = xmalloc(name_length + 2);
                strcpy(name_of_file + 1, fname);
                free(fname);

                /* This fopen is not allowed to fail. */
		*f_ptr = xfopen(name_of_file + 1, fopen_mode);
            }
        }
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


/* Open an output file F either in the current directory or in
   $TEXMFOUTPUT/F, if the environment variable `TEXMFOUTPUT' exists.
   (Actually, this also applies to the BibTeX and MetaPost output files,
   but `TEXMFMPBIBOUTPUT' was just too long.)  The filename is in the
   global `name_of_file' + 1.  We return whether or not the open
   succeeded.  If it did, `name_of_file' is reset to the name opened if
   necessary, and `name_length' to its length.  */

boolean open_output(FILE ** f_ptr, const_string fopen_mode)
{
    string fname;
    boolean absolute = kpse_absolute_p(name_of_file + 1, false);

    /* If we have an explicit output directory, use it. */
    if (output_directory && !absolute) {
        fname = concat3(output_directory, DIR_SEP_STRING, name_of_file + 1);
    } else {
        fname = name_of_file + 1;
    }

    /* Is the filename openable as given?  */
    *f_ptr = fopen(fname, fopen_mode);

    /* If this succeeded, change name_of_file accordingly.  */
    if (*f_ptr) {
        if (fname != name_of_file + 1) {
            free(name_of_file);
            name_length = strlen(fname);
            name_of_file = xmalloc(name_length + 2);
            strcpy(name_of_file + 1, fname);
        }
        /*recorder_record_output (fname); */
    }
    if (fname != name_of_file + 1)
        free(fname);
    return *f_ptr != NULL;
}


/* Close F.  */

void close_file(FILE * f)
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
