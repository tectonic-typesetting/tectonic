#include <errno.h>

/* first pass to make c compiler happy */

int tt_errno(void);
void tt_set_errno(int newval);

/* stubs */

int tt_errno() { 
    return errno;    
}

void tt_set_errno(int newval) {
    errno = newval;
}