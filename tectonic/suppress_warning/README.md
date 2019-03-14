`suppress_warning` library
==========================

A small C and C++ preprocessor library to suppress warnings without compromising backward compatibility.

*by David Krauss (potatoswatter)*
<!-- language: lang-c lang-cxx -->

Summary
=======

Compilers, particularly Clang, implement ever-more warnings as friendly reminders to follow good coding practices.
Sometimes warnings are spurious and they must be silenced in particular instances.

There is a `#pragma` directive to do so, but it generates another warning if the compiler simply doesn't implement the warning.
Clang offers a `__has_warning` macro to avoid this issue, but then you also need to check that `__has_warning` exists… it gets complicated.

With this library, if you wanted to write `char q = { 800 };` in C++, you can suppress the two resulting diagnostic messages like so:

    // Specify the warning flag by defining the SUPPRESS_WARNING macro.
    #define SUPPRESS_WARNING "-Wnarrowing"
    // Include the header to suppress the warning and un-define the SUPPRESS_WARNING parameter.
    #include "suppress_warning/push.h"

    // Warning suppressions may be nested.
    #define SUPPRESS_WARNING "-Wconstant-conversion"
    #include "suppress_warning/push.h"

    char q = { 800 }; // Do dastardly deeds without reprimand.

    // Re-enable warnings when finished.
    #include "suppress_warning/pop.h"
    #include "suppress_warning/pop.h"

Compatibility
=============

Unfortunately, GCC does not yet implement `__has_warning`. Anticipating that it will, this library is coded for forward compatibility.
Until then, only Clang is supported and warnings are not suppressed on GCC.

Tectonic Notes
==============

* Original source: <https://github.com/potswa/suppress_warning>
* Commit: 85b6451fd02916d4c8c94caf9d7f6c96e88826ca
* Discovered at <https://stackoverflow.com/a/42979988/545794>
* Changes: The header file names were modified from the original. Everything
  else is the same.
