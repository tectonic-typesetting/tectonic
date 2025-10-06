# tectonic -X init

Initializes a new Tectonic workspace in the current directory.

**_This is a [V2 CLI][v2cli-ref] command. For information on the original ("V1"
CLI), see [its reference page][v1cli-ref]._**

[v2cli-ref]: ../ref/v2cli.md
[v1cli-ref]: ../ref/v1cli.md

#### Usage Synopsis

```sh
tectonic -X init
```

#### Remarks

This command will create a bare-bones [Tectonic.toml][tectonic-toml] file in the
target directory. The project’s name will be initialized to the name of the
workspace directory.

[tectonic-toml]: ../ref/tectonic-toml.md

It will also create a placeholder source file in `src/main.tex`.

#### See Also

- [`tectonic -X new`](./new.md)
