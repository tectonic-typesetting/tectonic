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

## How to Build Your Very Own Static Binary

Because we use docker to create this environment you can easily build a static `tectonic` yourself.

### prerequisites
* You'll need to have [docker installed](https://docs.docker.com/install/).
* local copy of this repository. The steps below assume you have cloned the repository and are in its root.
* a modern shell. Note the below commands should work in most shells but may not in some. For example, if using [fishshell](https://fishshell.com/) you'll have to update the environment flag assignment and `pwd` invocation.


### steps

1. build an image that correctly sets up the needed environment
```
$ DOCKER_BUILDKIT=1 docker build -t tectonic-cross-compiler:v1 dist/docker/x86_64-alpine-linux-musl/
```
This will take a while. You should see output similar to below:
```
[+] Building 95.7s (15/15) FINISHED
 => [internal] load build definition from Dockerfile                                                                                                                         0.0s
 => => transferring dockerfile: 953B                                                                                                                                         0.0s
 => [internal] load .dockerignore                                                                                                                                            0.0s
 => => transferring context: 2B                                                                                                                                              0.0s
 => [internal] load metadata for docker.io/library/ubuntu:18.04                                                                                                              0.0s
 => [internal] load build context                                                                                                                                            0.0s
 => => transferring context: 4.33kB                                                                                                                                          0.0s
 => [1/10] FROM docker.io/library/ubuntu:18.04                                                                                                                               0.0s
 => => resolve docker.io/library/ubuntu:18.04                                                                                                                                0.0s
 => [2/10] ADD setup_priv.sh /                                                                                                                                               0.1s
 => [3/10] RUN sh /setup_priv.sh                                                                                                                                            41.6s
 => [4/10] ADD sudoers /etc/sudoers.d/nopasswd                                                                                                                               0.2s
 => [5/10] ADD setup_unpriv.sh /                                                                                                                                             0.1s
 => [6/10] ADD toolwrapper.sh /alpine/home/rust/                                                                                                                             0.1s
 => [7/10] ADD linkwrapper.sh /alpine/home/rust/                                                                                                                             0.1s
 => [8/10] RUN sh /setup_unpriv.sh                                                                                                                                          43.9s
 => [9/10] ADD cargo-config.toml /alpine/home/rust/.cargo/config                                                                                                             0.1s
 => [10/10] WORKDIR /alpine/home/rust/src                                                                                                                                    0.1s
 => exporting to image                                                                                                                                                       9.1s
 => => exporting layers                                                                                                                                                      9.1s
 => => writing image sha256:842075298248fc0c885d77e25ae244833551c08a400bd252445620763d031a82                                                                                 0.0s
 => => naming to docker.io/library/tectonic-cross-compiler:v1
```
[BuildKit](https://github.com/moby/buildkit) is an improved toolkit that [docker can take advantage](https://docs.docker.com/develop/develop-images/build_enhancements/) of by toggling the environment variable.

2. run a container that [invokes cargo](https://github.com/tectonic-typesetting/tectonic/blob/master/dist/docker/x86_64-alpine-linux-musl/Dockerfile#L31) to build the static binary
```
$ docker run --rm -v $(pwd):/alpine/home/rust/src tectonic-cross-compiler:v1
    Updating crates.io index
 Downloading crates ...
  Downloaded aho-corasick v0.7.6
  Downloaded lazy_static v1.3.0
  Downloaded error-chain v0.12.1
```
This will also take some time. You should see cargo's output from compiling crates.
Upon completing, it should create the static binary in your working directory's `target/x86_64-unknown-linux-musl/release/`:
```
 $ ls target/x86_64-unknown-linux-musl/release/tectonic
target/x86_64-unknown-linux-musl/release/tectonic*
```

Now you should be able to copy that `tectonic` binary to any Linux system it should *just work*:
```
some-box $ ./tectonic -V
Tectonic 0.1.12-dev
```

Three cheers for cross compilation!
