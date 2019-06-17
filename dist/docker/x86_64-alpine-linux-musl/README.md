# Statically-linked Tectonic cross-compilation environment

This Docker recipe creates a container that can be used to build a
statically-linked Tectonic executable. The process is pretty complex!

We used to build Tectonic inside a pure Alpine Linux container. Everything in
Alpine uses musl, the static-linking-friendly libc, and it provides a
toolchain that can create static binaries, and Rust generally supports musl.
So far so good.

*However*, Tectonic needs “procedural macro” crates to compile, and these
are not compatible with the Alpine static target. Procedural macros are
implemented as dynamic modules loaded into the Rust compiler executable.
On a static target, you ... can't do that.

It took me a while to figure this out, but the solution is cross-compilation.
The Rust compiler executable is running on the "build" architecture, whereas
the output program will run on the "target" architecture. Procedural macros
should work for a static target architecture *if* the build architecture is
dynamic — a cross-compilation situation.

Cross-compilation is gnarly for Tectonic because we depend on several system
libraries, like Harfbuzz and Freetype. If we're going to cross-compile, we
need static versions of these libraries available on the build machine *in the
target architecture* — which is usually difficult to set up. In particular,
our build machine will need some C/C++ cross-compilation toolchain, and our
target libraries need to be guaranteed to be compatible with that toolchain.
Copying binaries willy-nilly off the interent *might* work but is probably
asking for trouble.

How can we conveniently get cross-compiled versions of our dependencies?

It would be nice to leverage Alpine Linux — it has up-to-date, pre-built,
static binaries of all of the libraries that Tectonic depends upon. You can’t
just install the Alpine toolchain and libraries in an Ubunutu container, but
you *can* install [Alpine in a chroot]. Inside the chroot, the Alpine
toolchain is accessible.

So our Docker container sets up the Alpine tools, then creates some wrappers
so that *outside* of the chroot we can pretend that the tools are normal
programs. The wrappers strip off `/alpine` prefixes and then run things inside
the chroot.

[Alpine in a chroot]: https://wiki.alpinelinux.org/wiki/Installing_Alpine_Linux_in_a_chroot

Once we actually get cross-compiling, it turns out that statically linking
Rust with C++ is a bit of a pain. After some voodoo hacks I got it working,
specifically by using a [linker wrapper script] suggested by GitHub user
`@dl00`.

[linker wrapper script]: https://github.com/rust-lang/rust/issues/36710#issuecomment-364623950

**Note**: Our CI runs this docker container inside its own Ubuntu Xenial VM.
The way that we do things now, we could just skip Docker altogether and set up
the Alpine chroot inside the VM directly. But it seems helpful to capture the
magic in a Docker setup and shouldn't slow things down much, so we'll stick
with the container.
