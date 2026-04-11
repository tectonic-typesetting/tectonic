# Bundle Specification

Every directory in this dir defines a Tectonic bundle.
The main configuration file is `bundle.toml`, its contents are documented below.

## Overview

```toml
[bundle]
# This bundle's name
name = "texlive2023"
# This bundle's expected final SHA256 hash. See the bundle specification to learn how it's generated.
# If this is left empty, the bundle hash will be printed once the bundle is built.
expected_hash = "c1bbb5f8498d2bb5471cc2b0790700ecb86fc992ec290f7eb718d8751f8ada2a"

# Bundle search order. This tells Tectonic how to search for files.
# The global `search_order` may have two kinds of values:
#   - plain strings (which follow the usual search order rules)
#   - `{ input = "" }` entries, which specify an input's search order.
# If an input is not listed here, it will NOT be searched!
# (unless one specifies "//", which is a bad idea.)
#
# This is used to generate SEARCH in the bundle.
search_order = ["/", { input = "include" }, { input = "texlive" }]

# Note the triple-quoted multiline strings:
# this is the best way to get raw strings in TOML.
ignore = [
    # Files and extensions we want to ignore.
    # These will be applied to ALL inputs.
    # If a file's relative path matches any of these patterns,
    # that file will be excluded from the bundle.
    '''.*/LICENSE\.md''',
    '''.*/Makefile''',
    '''.*/README''',
    '''.*/README.md''',
    '''.*/readme\.txt'''
]


# A simple directory input, with `path` relative to this toml file.
[inputs."include"]
source.dir.path = "include"


# A tarball input, usually used to add TeXlive files.
#
# Note that this MUST be a .tar file.
# You'll likely download a compressed tar file. Extract it.
#
# It's a good idea to add a comment with the TeXlive version
# and url of this file, so that others may find it.
[inputs."texlive"]

# Patch directory for this input. Optional.
# This should be a directory of `.diff` files in unified format,
# the first line of which specifies the path (relative to `root_dir` below)
# of the file that diff should be applied to.
#
#
# To make a patch file, you should...
# - Copy the original file and apply your changes.
# - Run `diff "original-file" "modified-file" > file.diff`. ORDER MATTERS!
# - Add **one** new line to the top of `file.diff` containing a path to the file this diff should be applied to. This path should be relative to the bundle's content dir, as shown below.
# - Place `file.diff` anywhere in your bundle's include dir. The file selection script should find and apply it.
#
# The line at the top is essential and must be added manually.
# We can't do without it, since we may have many files with # the same name.
#
# Also note that the brace decorations used in `search_order` may also be used in this first line.
# For example, a patch marked `tex/{latex,latex-dev}/base/latex.ltx` will be applied to `latex.ltx` in both
# `texlive/tex/latex` and `texlive/tex/latex-dev`. This will only work if those files are identical.
patch_dir = "patches/texlive"


# Path to the tarball, relative to this toml file's parent directory.
source.tarball.path = "texlive-20230313-texmf.tar"

# Compute this hash by running `sha256 -b file.tar`
source.tarball.hash = "ac1683d4abeb7fd534851ad7ff0ec891da7da4729603506efd0245259dcdcc67"

# The directory inside this tarball to add. Optional.
# All paths below are relative to this.
source.tarball.root_dir = "texlive-20230313-texmf/texmf-dist"

# Regex ignore pattens. Any file whose path matches any of these patterns will not be added to the bundle.
# These are relative to `root_dir` and do NOT start with a slash.
ignore = [
    '''tex/luatex/.*''',
    '''tex/lualatex/.*'''
]


# Search order of this input.
# This is optional, ommitting `search_order` is equivalent
# to setting `search_order = [ "//" ]`
#
# As always, these paths are relative to `root_dir` and do NOT start with a slash.
#
#
# Lines may be decorated with braces: `/a/{b,c}/` will become `/a/b` and `a/c`, in that order.
# - Brace decorations may not be nested.
# - Paths may not contain braces. Escaping with `\{` will not work.
# - Multiple brace decorations in one line are allowed:
#   `/{a,b}/{1,2}` expands to `/a/1`, `/a/2`, `/b/1`, `b/2`, in that order.
#
# Just like kpathsea search paths, each search pattern can end with one or two slashes.
# - If a line ends with two slashes (like `texlive/tex/latex//`), it will match all subdirectories of that path.
# - If a line ends with one slash (like `texlive/tex/latex/`), it will match only direct children of that path:
#   `texlive/tex/latex/a.tex` will be searched, `texlive/tex/latex/base/a.tex` will not.
#
# - If a line does not end with a slash, we pretend it ends with one.
# - If a line ends with three or more slashes, it won't be searched at all. Don't do that.
#
# This scheme lets us override the default "alphabetic depth-first search" by adding seach paths as follows,
# which will look for direct children of `latex` before descending into subdirectories:
# ```
# texlive/tex/latex/
# texlive/tex/latex//
# ```
search_order = [
    "tex/{xelatex,latex,xetex,plain,generic}//",
    "bibtex/{bib,bst,csf}//",
    "web2c//",
    "fonts//",
    "biber//",
    "mft//",
    "dvips//",
    "makeindex//",
    "{web,cweb}//",
    "ttf2pk//",
    "dvipdfmx/",
]

```


## Extra details: finding files

### Overview
Any TeX distribution needs a way to find files. This is necessary because files are usually included only by name: `\include{file}`, `\usepackage{package}`, etc. Where do we find `file.tex` and `package.sty`?

In a conventional TeXLive installation, kpathsea solves this problem. It defines an array of "search paths," and walks through them when you ask for a file. You can find an overview [here](https://www.overleaf.com/learn/latex/Articles/An_introduction_to_Kpathsea_and_how_TeX_engines_search_for_files) and more detailed information in the kpathsea docs.

Tectonic's supporting files are distributed in bundles, so we can't use the same approach.
Within tectonic's *bundles*[^1], we use FILELIST and SEARCH files to map a filename to an input path. Note that this logic is implemented in tectonic, not in the bundle build script.

[^1]: Tectonic searches for files on your disk seperately. The information in this file only applies to bundles. I won't document this fully here, you'll have to read the tectonic docs and source code.

- **Case 1:** tectonic looks for `file.tex` and finds one path in `FILELIST`\
  Nothing fancy here, we just use the file we found.

- **Case 2:** tectonic looks for `partial/path/to/file.tex`\
  This is an edge case caused by some packages (for example, `fithesis`). To handle this,
  we first find `file.tex` in `FILELIST` and look at its path. If its path ends with `partial/path/to/file.tex`, we use it,
  if it doesn't, we don't. If multiple files match, we print an error--that shouldn't ever happen.

- **Case 3:** tectonic looks for `file.tex` and finds multiple paths in `FILELIST`\
This where things get interesting. First, we match all paths against each line of the bundles's `SEARCH` file with a simple `starts_with`.
  - If *exactly one* path matches a certain line, we immediately stop checking and use that path. Search lines are ordered by priority, so if only one path matches the first line, it *must* be the right path to use.
  - If multiple paths match a certain line, we discard all others and resolve the conflict alphabetically.
  - If we've checked all lines of `SEARCH` and found no matches, we didn't find the file. Return an error.

"Resolving the conflict alphabetically" means we sort the paths in alphabetical order and pick the first. This emulates an alphabetically-ordered depth-first search on the file tree, which is a reasonable default.

Any filename conflicts which would be resolved alphabetically are listed in `search-report` after the `select` build step. These aren't errors, but we should look over that file to make sure everything is working as expected.