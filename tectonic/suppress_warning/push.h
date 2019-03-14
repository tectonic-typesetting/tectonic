// suppress_warning.h
// This inclusion file has no header guard because it may be multiply included.

#if __GNUC__
#   pragma GCC diagnostic push
#   ifdef __has_warning
#       define S6_SUPPRESS_HAS_WARNING( X ) __has_warning( X )
#       if S6_SUPPRESS_HAS_WARNING( SUPPRESS_WARNING )

             // Use the _Pragma operator to access the macro flag from the directive.
#            define S6_SUPPRESS_LITERAL_PRAGMA( X ) _Pragma( # X )
#            define S6_SUPPRESS_EXPAND_PRAGMA( X ) S6_SUPPRESS_LITERAL_PRAGMA( X )
             S6_SUPPRESS_EXPAND_PRAGMA( GCC diagnostic ignored SUPPRESS_WARNING )
#       endif
#   endif
#endif

#undef SUPPRESS_WARNING
