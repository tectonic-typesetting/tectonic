#ifndef __ulong_chartraits_h__
#define __ulong_chartraits_h__

#include <iostream>
#include <string>
#include <cstring>
#include <locale>

/*
	This used to be defined for unsigned long; changed to unsigned int for 64-bit architecture.
	Note that this will now fail if int is not 4 bytes (let's hope nobody is using such old compilers any more).
	The real solution would be a proper autoconf setup for TECkit, but that's left for another day.
*/

// Provide std::char_traits specialization for unsigned int.

// BEWARE:
// This is adequate for the TECkit compiler but would not work for generic stream i/o, for example.

namespace std
{
  template<>
    struct char_traits<unsigned int>
    {
      typedef unsigned int		char_type;

      // NB: this type should be bigger than char_type, so as to
      // properly hold EOF values in addition to the full range of
      // char_type values.
      typedef unsigned int     int_type;

      typedef streampos         pos_type;
      typedef streamoff         off_type;
      typedef mbstate_t         state_type;
      
      static void 
      assign(char_type& __c1, const char_type& __c2)
      { __c1 = __c2; }

      static bool 
      eq(const char_type& __c1, const char_type& __c2)
      { return __c1 == __c2; }

      static bool 
      lt(const char_type& __c1, const char_type& __c2)
      { return __c1 < __c2; }

      static int 
      compare(const char_type* __s1, const char_type* __s2, size_t __n)
      { 
        for (size_t __i = 0; __i < __n; ++__i)
          if (!eq(__s1[__i], __s2[__i]))
            return lt(__s1[__i], __s2[__i]) ? -1 : 1;
        return 0; 
      }

      static size_t
      length(const char_type* __s)
      { 
        const char_type* __p = __s; 
        while (*__p) ++__p; 
        return (__p - __s); 
      }

      static const char_type* 
      find(const char_type* __s, size_t __n, const char_type& __a)
      { 
        for (const char_type* __p = __s; size_t(__p - __s) < __n; ++__p)
          if (*__p == __a) return __p;
        return 0;
      }

      static char_type* 
      move(char_type* __s1, const char_type* __s2, size_t __n)
      { return (char_type*) memmove(__s1, __s2, __n * sizeof(char_type)); }

      static char_type* 
      copy(char_type* __s1, const char_type* __s2, size_t __n)
      { return (char_type*) memcpy(__s1, __s2, __n * sizeof(char_type)); }

      static char_type* 
      assign(char_type* __s, size_t __n, char_type __a)
      { 
        for (char_type* __p = __s; __p < __s + __n; ++__p) 
          assign(*__p, __a);
        return __s; 
      }

      static char_type 
      to_char_type(const int_type& __c)
      { return char_type(__c); }

      static int_type 
      to_int_type(const char_type& __c) { return int_type(__c); }

      static bool 
      eq_int_type(const int_type& __c1, const int_type& __c2)
      { return __c1 == __c2; }

      static int_type 
      eof() { return static_cast<int_type>(-1); }

      static int_type 
      not_eof(const int_type& __c)
      { return eq_int_type(__c, eof()) ? int_type(0) : __c; }
    };
}

#endif
