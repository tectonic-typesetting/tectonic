/* openclose.c: open and close files for TeX, Metafont, and BibTeX.

   Written 1995 Karl Berry.  Public domain.  */

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/stubs.h>

/* The globals we use to communicate.  */
extern string name_of_file;
extern unsigned name_length;

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
	    FATAL_PERROR(fname);

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


/* Open an output file F either in the current directory or in $TEXMFOUTPUT/F,
   if the environment variable `TEXMFOUTPUT' exists. (Actually, this also
   applies to the BibTeX and MetaPost output files, but `TEXMFMPBIBOUTPUT' was
   just too long.) The filename is in the global `name_of_file' + 1. We return
   whether or not the open succeeded. If it did, `name_of_file' is reset to
   the name opened if necessary, and `name_length' to its length. */

boolean
open_output(FILE ** f_ptr, const_string fopen_mode)
{
    string fname = name_of_file + 1;

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
