/* coredump.c.  Public domain.

   This procedure is due to Chris Torek, chris@umd.edu.  It makes a core
   dump without any sort of error status (abort(2) does return an error
   status, so we don't want to use that).  It is used only when making a
   preloaded TeX from virtex, and is triggered by a magic file name
   requested as input (see `open_input', above).  Finding a way to
   reconstitute the core dump into a binary (i.e., with undump) is up to
   you.  Perl has some things to say about these days.  */

#include <w2c/config.h>

/* Do not try to compile this Unix-y unportable stuff unless it's needed.  */

#ifdef FUNNY_CORE_DUMP
#include <signal.h>
#include <sys/wait.h>

void
funny_core_dump (void)
{
#ifdef __EMX__
  {
    int handle = open ("core", O_WRONLY | O_CREAT | O_TRUNC | O_BINARY);
    if (handle >= 0 && _core (handle) == 0)
      exit (0);
    (void) write (2, "attempt to dump core failed\n", 28);
    exit (1);
  }
#else /* !__EMX__ */
  int pid, w;
  union wait status;

  switch (pid = fork ())
    {
    case -1:		/* failed */
      perror ("vfork");
      exit (-1);
      /*NOTREACHED*/

    case 0:             /* child */
       (void) signal (SIGQUIT, SIG_DFL);
       (void) kill (getpid (), SIGQUIT);
       (void) write (2, "how did we get here?\n", 21);
       _exit (1);
       /*NOTREACHED*/

    default:		/* parent */
      while ((w = wait (&status)) != pid && w != -1)
	;
      if (status.w_coredump)
	exit (0);
      (void) write (2, "attempt to dump core failed\n", 28);
      exit (1);
    }
#endif /* not __EMX__ */
}
#endif /* FUNNY_CORE_DUMP */
