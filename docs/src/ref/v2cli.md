# “V2” (Prototype) Command-Line Interface

The “V2” command-line interface to Tectonic is a substantially new interface to
the Tectonic engine introduced in the 0.4.x release series.

In short, the V2 interface is “[cargo]-like”, with a variety of subcommands
anchored around a [Tectonic.toml] file defining a document to build. For
comparison, the original (“V1”) interface is more “[rustc]-like”, offering a
single primary workflow with lots of options controlling its behavior.

[cargo]: https://doc.rust-lang.org/cargo/
[Tectonic.toml]: ./tectonic-toml.md
[rustc]: https://doc.rust-lang.org/rustc/command-line-arguments.html

## Current status

The V2 interface is currently **available but not the default**. It can be
activated as described below.

## Activating the V2 Interface

Because the V2 interface is not the current default, it must be explicitly
activated. There are several ways to do this. One way is to use `-X` for the
very first command-line argument to the `tectonic` executable, like so:

```sh
tectonic -X build
```

It is anticipated that this form of invocation will be supported indefinitely.

Alternatively, if the Tectonic executable is run under a name containing the
string `nextonic`, the V2 interface will also be enabled. In this case, no `-X`
is needed (or allowed, at the moment):

```sh
nextonic build
```

You can use various Unix `argv[0]` hacks to achieve this, or you can just rename
or symlink the `tectonic` binary to `nextonic` manually.

## Migration plan

The plan is to eventually migrate to make the V2 interface the default. This
will be implemented as follows:

1. Under the current situation, the V2 interface can be explicitly activated
   with the leading `-X` flag. The V1 interface can be explicitly selected by
   using a leading `-Y` flag. The default is V1.
2. Once the V2 interface is sufficiently stable, the V1 UI will start
   recommending that people adopt it via the `-X` flag. Users that want to keep
   using the V1 interface will be instructed to explicitly select it with the
   `-Y` option.
3. After a sufficient amount of time, the V1 interface will be flagged as
   deprecated.
4. After more time has passed, the default will flip: if neither `-X` nor `-Y`
   is provided, the V2 interface will be assumed rather than V1.
5. Eventually, the V1 interface may be entirely removed.
