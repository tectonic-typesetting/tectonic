# Tectonic Bundles

This repository contains tools for building bundles for [Tectonic](https://tectonic-typesetting.github.io).
You should only need this if you're producing your own bundles. If you're just using Tectonic to compile LaTeX,
you should use the pre-generated bundles we distribute.


## Prerequisites
To use these tools, you will need:
- GNU `patch`. Patch is called by `tectonic bundle create `.
- A [TeXlive tarball](https://tug.org/texlive/acquire-tar.html).

The following bundles are available:
 - [`texlive2023`](./bundles/texlive2023): based on `texlive2023-20230313`.


## Build Process:
Before building any bundles, you'll need to download the prerequisite files.
Usually, this is a [TeXlive tarball](https://tug.org/texlive/acquire-tar.html) with a version that matches the bundle you want to build.
See `bundle.toml` in the bundle you want to build.


To build a bundle, run the following:
 - `cd bundles`
 - `tectonic -X bundle create --build-dir ./build texlive2023/bundle.toml v1`

This runs the following jobs, in order. Individual jobs may be run by specifying `--job <job name>`.
 - `select`
 - `pack`

The contents of `<build dir>/content` may be inspected and edited after running `select`. \
This should only be used to debug bundles.


## Extra Documentation
 - Each directory in [`./bundles`](./bundles/) is a bundle specification, documented [here](./bundles/README.md).
 - Only one bundle format is currently supported, it is described in [`./format-v1.md`](./format-v1.md).
 - This repository includes legacy bundle [tests](./tests/README.md), which may be broken.


## Output files

The files that `tectonic bundle create` produces are listed below:
 - `./build/output/<bundle>/content`: contains all bundle files. It is organized by source: files from the bundle's `include` dir will be under `./include`, texlive files will be under `./texlive`, and so on. See `builder/src/select.rs`.
 This directory also contains some metadata:
   - `content/FILELIST`: each line of this file is `<path> <hash>`, sorted by file name.\
   Files with identical names are included.\
   Files not in any search path are also included.\
   `<hash>` is either a hex sha256 of that file's contents, or `nohash` for a few special files.
   - `content/SHA256SUM`: The sha256sum of `content/FILES`. This string uniquely defines this bundle.
   - `content/SEARCH`: File search order for this bundle. See bundle spec documentation.
 - `search-report`: debug file. Lists all directories that will not be searched by the rules in `search-order`.\
  The entries in this file are non-recursive: If `search-report` contains a line with `/texlive`, this means that direct children of `/texlive` (like `/texlive/file.tex`) will not be found, but files in *subdirectories* (like `/texlive/tex/file.tex`) may be.

**Final output files are listed below:**
 - `<bundle>.ttb`: the bundle. Note that the ttb version is *not* included in the extension.
   - Index location and length are printed once this job completes.
   - You can extract files from this bundle by running `dd if=file.ttb ibs=1 skip=<start> count=<len> | gunzip`