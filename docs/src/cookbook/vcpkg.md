# build tectonic with vcpkg on macOS


This tutorial will walk you through how to build a *mostly* staticly linked `tectonic` binary on macOS. Why does this matter? Static binaries are more portable and so free you from having to support complex user environments. Thanks to [mcgoo's work](https://github.com/tectonic-typesetting/tectonic/pull/420) we can use `vcpkg`. [vcpkg](https://vcpkg.readthedocs.io/en/latest/) is a C/C++ package manager from Microsoft.


## prerequisites

This guide assumes you have [git](https://git-scm.com/) installed and are comfortable using the command-line. You'll also need to install [homebrew](https://brew.sh/) to install for `vcpkg`'s gcc dependency. Package management is *fun*.

1. install `gcc`:
```sh
 brew install gcc
```

## setting up your environment

1. Install  vcpkg
	1. Clone the vcpkg repository:
		```sh
		git clone https://github.com/Microsoft/vcpkg
		```
	2. Install it for your system:
		```sh
		./bootstrap-vcpkg.sh
		```

2. Install `tectonic` dependencies using vcpkg.
	* Run the following command in your checkout of the `vcpkg` repository
	```sh
	./vcpkg install freetype harfbuzz\[icu,graphite2\]
	```
It should print something like the following:
```sh
$ ./vcpkg install freetype harfbuzz\[icu,graphite2\]
The following packages will be built and installed:
  * bzip2[core]:x64-osx
    freetype[core]:x64-osx
  * gettext[core]:x64-osx
  * graphite2[core]:x64-osx
    harfbuzz[core,graphite2,icu,ucdn]:x64-osx
  * icu[core]:x64-osx
  * libiconv[core]:x64-osx
  * libpng[core]:x64-osx
  * ragel[core]:x64-osx
  * zlib[core]:x64-osx
Additional packages (*) will be modified to complete this operation.
Starting package 1/10: graphite2:x64-osx
Building package graphite2[core]:x64-osx...
```

## build tectonic

Now we only need to configure tectonic so it knows we are using `vcpkg` to build the binary.
1. Run `cargo build` with the appropriate environment variables:
	```sh
	TECTONIC_DEP_BACKEND="vcpkg" VCPKG_ROOT=/Users/me/vcpkg/ cargo build --release
	```

You'll need to set `VCPKG_ROOT` to the full path of your vcpkg checkout.
`TECTONIC_DEP_BACKEND="vcpkg"` tells `tectonic` to use `vcpkg` instead of trying to resolve the libraries using `pkgconfig`. `VCPKG_ROOT=/Users/me/vcpkg/` is the root of the `vcpkg` tree where we just installed the required libraries.

This will take a couple minutes but should eventuall print something like:
```sh
    Finished release [optimized] target(s) in 3m 39s
```

Congratulations! You should now have a mostly staticly linked binary in `target/release` suitable for sharing with users or whatever your needs are.


## caveats

I say *mostly* staticly linked because the binary itself presumes system libraries.
You can observe those links by executing `otool` on a given binary:
```sh
otool -L target/release/tectonic

```

In my case it printed the following:
```sh
target/release/tectonic:
	/System/Library/Frameworks/Foundation.framework/Versions/C/Foundation (compatibility version 300.0.0, current version 1454.98.0)
	/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation (compatibility version 150.0.0, current version 1454.98.0)
	/System/Library/Frameworks/CoreGraphics.framework/Versions/A/CoreGraphics (compatibility version 64.0.0, current version 1161.21.2)
	/System/Library/Frameworks/CoreText.framework/Versions/A/CoreText (compatibility version 1.0.0, current version 1.0.0)
	/System/Library/Frameworks/AppKit.framework/Versions/C/AppKit (compatibility version 45.0.0, current version 1561.60.100)
	/usr/lib/libc++.1.dylib (compatibility version 1.0.0, current version 400.9.0)
	/System/Library/Frameworks/Security.framework/Versions/A/Security (compatibility version 1.0.0, current version 58286.70.14)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1252.50.4)
	/usr/lib/libresolv.9.dylib (compatibility version 1.0.0, current version 1.0.0)
	/usr/lib/libobjc.A.dylib (compatibility version 1.0.0, current version 228.0.0)
```


