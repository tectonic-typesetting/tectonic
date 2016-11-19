/* openclose.c: open and close files for TeX, Metafont, and BibTeX.

   Written 1995 Karl Berry.  Public domain.  */

#include <w2c/config.h>
#include "lib.h"
#include <tidy_kpathutil/public.h>
#include <tidy_kpathsea/public.h>
#ifdef PTEX
#include <ptexenc/ptexenc.h>
#endif

#ifdef WIN32
#undef fopen
#undef xfopen
#define fopen fsyscp_fopen
#define xfopen fsyscp_xfopen
#endif

/* The globals we use to communicate.  */
extern string name_of_file;
extern unsigned name_length;

/* Define some variables. */
/* For "file:line:error" style error messages. */
string fullnameoffile;       /* Defaults to NULL.  */
static string recorder_name; /* Defaults to NULL.  */
static FILE *recorder_file;  /* Defaults to NULL.  */
/* For the filename recorder. */
boolean recorder_enabled;    /* Defaults to false. */
/* For the output-dir option. */
string output_directory;     /* Defaults to NULL.  */

/* For TeX and MetaPost.  See below.  Always defined so we don't have to
   #ifdef, and thus this file can be compiled once and go in lib.a.  */
int tfm_temp;
int ocptemp;
int tex_input_type;

/* Helpers for the filename recorder... */
/* Start the recorder */
static void
recorder_start(void)
{
    /* Alas, while we'd like to use mkstemp it is not portable,
       and doing the autoconfiscation (and providing fallbacks) is more
       than we want to cope with.  So we have to be content with using a
       default name.  Throw in the pid so at least parallel builds might
       work (Debian bug 575731).  */
    string cwd;
    char pid_str[MAX_INT_LENGTH];

    /* Windows (MSVC) seems to have no pid_t, so instead of storing the
       value returned by getpid() we immediately consume it.  */
    sprintf (pid_str, "%ld", (long) getpid());
    recorder_name = concat3(kpse_program_name, pid_str, ".fls");
    
    /* If an output directory was specified, use it instead of cwd.  */
    if (output_directory) {
      string temp = concat3(output_directory, DIR_SEP_STRING, recorder_name);
      free(recorder_name);
      recorder_name = temp;
    }
    
    recorder_file = xfopen(recorder_name, FOPEN_W_MODE);
    
    cwd = xgetcwd();
    fprintf(recorder_file, "PWD %s\n", cwd);
    free(cwd);
}

/* Change the name of the recorder file after we know the log file to
   the usual thing -- no pid integer and the document file name instead
   of the program name.  Unfortunately, we have to explicitly take
   -output-directory into account (again), since the NEW_NAME we are
   called with does not; it is just the log file name with .log replaced
   by .fls.  */

void
recorder_change_filename (string new_name)
{
   string temp = NULL;
   
   if (!recorder_file)
     return;

   /* On windows, an opened file cannot be renamed. */
#if defined(WIN32)
   fclose (recorder_file);
#endif

   /* If an output directory was specified, use it.  */
   if (output_directory) {
     temp = concat3(output_directory, DIR_SEP_STRING, new_name);
     new_name = temp;
   }

   /* On windows, renaming fails if a file with new_name exists. */
#if defined(WIN32)
   remove (new_name);
#endif

   rename(recorder_name, new_name);
   free(recorder_name);
   recorder_name = xstrdup(new_name);

   /* reopen the recorder file by FOPEN_A_MODE. */
#if defined(WIN32)
   recorder_file = fsyscp_xfopen (recorder_name, FOPEN_A_MODE);
#endif

   if (temp)
     free (temp);
}

/* helper for recorder_record_* */
static void
recorder_record_name (const_string prefix, const_string name)
{
    if (recorder_enabled) {
        if (!recorder_file)
            recorder_start();
        fprintf(recorder_file, "%s %s\n", prefix, name);
        fflush(recorder_file);
    }
}

/* record an input file name */
void
recorder_record_input (const_string name)
{
    recorder_record_name ("INPUT", name);
}

/* record an output file name */
void
recorder_record_output (const_string name)
{
    recorder_record_name ("OUTPUT", name);
}

/* Open an input file F, using the kpathsea format FILEFMT and passing
   FOPEN_MODE to fopen.  The filename is in `name_of_file+1'.  We return
   whether or not the open succeeded.  If it did, `name_of_file' is set to
   the full filename opened, and `name_length' to its length.  */

boolean
open_input (FILE **f_ptr, int filefmt, const_string fopen_mode)
{
    string fname = NULL;
#ifdef FUNNY_CORE_DUMP
    /* This only applies if a preloaded TeX/Metafont is being made;
       it allows automatic creation of the core dump (typing ^\ loses
       since that requires manual intervention).  */
    if ((filefmt == kpse_tex_format || filefmt == kpse_mf_format
         || filefmt == kpse_mp_format)
        && STREQ (name_of_file + 1, "HackyInputFileNameForCoreDump.tex"))
        funny_core_dump ();
#endif

    /* We havent found anything yet. */
    *f_ptr = NULL;
    if (fullnameoffile)
        free(fullnameoffile);
    fullnameoffile = NULL;
    
    /* Look in -output-directory first, if the filename is not
       absolute.  This is because .aux and other such files will get
       written to the output directory, and we have to be able to read
       them from there.  We only look for the name as-is.  */
    if (output_directory && !kpse_absolute_p (name_of_file+1, false)) {
        fname = concat3 (output_directory, DIR_SEP_STRING, name_of_file + 1);
        *f_ptr = fopen (fname, fopen_mode);
        if (*f_ptr) {
            free (name_of_file);
            name_length = strlen (fname);
            name_of_file = xmalloc (name_length + 2);
            strcpy (name_of_file + 1, fname);
            fullnameoffile = fname;
        } else {
            free (fname);
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
            fname = kpse_find_file (name_of_file + 1,
                                    (kpse_file_format_type)filefmt,
                                    must_exist);
            if (fname) {
                fullnameoffile = xstrdup(fname);
                /* If we found the file in the current directory, don't leave
                   the `./' at the beginning of `name_of_file', since it looks
                   dumb when `tex foo' says `(./foo.tex ... )'.  On the other
                   hand, if the user said `tex ./foo', and that's what we
                   opened, then keep it -- the user specified it, so we
                   shouldn't remove it.  */
                if (fname[0] == '.' && IS_DIR_SEP (fname[1])
                    && (name_of_file[1] != '.' || !IS_DIR_SEP (name_of_file[2])))
                {
                    unsigned i = 0;
                    while (fname[i + 2] != 0) {
                        fname[i] = fname[i + 2];
                        i++;
                    }
                    fname[i] = 0;
                }

                /* kpse_find_file always returns a new string. */
                free (name_of_file);
                name_length = strlen (fname);
                name_of_file = xmalloc (name_length + 2);
                strcpy (name_of_file + 1, fname);
                free (fname);

                /* This fopen is not allowed to fail. */
#if defined(PTEX) && !defined(WIN32)
                if (filefmt == kpse_tex_format ||
                    filefmt == kpse_bib_format) {
                    *f_ptr = nkf_open (name_of_file + 1, fopen_mode);
                } else
#endif
                *f_ptr = xfopen (name_of_file + 1, fopen_mode);
            }
        }
    }

    if (*f_ptr) {
        recorder_record_input (name_of_file + 1);

        /* If we just opened a TFM file, we have to read the first
           byte, to pretend we're Pascal.  See tex.ch and mp.ch.
           Ditto for the ocp/ofm Omega file formats.  */
        if (filefmt == kpse_tfm_format) {
            tfm_temp = getc (*f_ptr);
            /* We intentionally do not check for EOF here, i.e., an
               empty TFM file.  TeX will see the 255 byte and complain
               about a bad TFM file, which is what we want.  */
        } else if (filefmt == kpse_ocp_format) {
            ocptemp = getc (*f_ptr);
        } else if (filefmt == kpse_ofm_format) {
            tfm_temp = getc (*f_ptr);
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

boolean
open_output (FILE **f_ptr, const_string fopen_mode)
{
    string fname;
    boolean absolute = kpse_absolute_p(name_of_file+1, false);

    /* If we have an explicit output directory, use it. */
    if (output_directory && !absolute) {
        fname = concat3(output_directory, DIR_SEP_STRING, name_of_file + 1);
    } else {
        fname = name_of_file + 1;
    }

    /* Is the filename openable as given?  */
    *f_ptr = fopen (fname, fopen_mode);

    if (!*f_ptr) {
        /* Can't open as given.  Try the envvar.  */
        string texmfoutput = kpse_var_value("TEXMFOUTPUT");

        if (texmfoutput && *texmfoutput && !absolute) {
            if (fname != name_of_file + 1)
                free(fname);
            fname = concat3(texmfoutput, DIR_SEP_STRING, name_of_file+1);
            *f_ptr = fopen(fname, fopen_mode);
        }
    }
    /* If this succeeded, change name_of_file accordingly.  */
    if (*f_ptr) {
        if (fname != name_of_file + 1) {
            free (name_of_file);
            name_length = strlen (fname);
            name_of_file = xmalloc (name_length + 2);
            strcpy (name_of_file + 1, fname);
        }
        recorder_record_output (fname);
    }
    if (fname != name_of_file +1)
        free(fname);
    return *f_ptr != NULL;
}

/* Close F.  */

void
close_file (FILE *f)
{
  /* If F is null, just return.  bad_pool might close a file that has
     never been opened.  */
  if (!f)
    return;
    
#ifdef PTEX
#ifdef WIN32
  clear_infile_enc (f);
  if (fclose (f) == EOF) {
#else
  if (nkf_close (f) == EOF) {
#endif
#else
  if (fclose (f) == EOF) {
#endif
    /* It's not always name_of_file, we might have opened something else
       in the meantime.  And it's not easy to extract the filenames out
       of the pool array.  So just punt on the filename.  Sigh.  This
       probably doesn't need to be a fatal error.  */
    perror ("fclose");
  }
}
