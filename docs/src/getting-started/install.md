# Getting Started: Install Tectonic

Let’s start out by making sure that Tectonic is installed on your system.

Even this step will be very different than what you might be used to with other
TeX systems. To install a normal TeX system such as [TeXLive], you normally need
to download gigabytes of support files and set them up in a complex directory
hierarchy. Tectonic, on the other hand, is distributed as a single executable
file. That one file not only combines the functionality of many standard TeX
programs, it also can download the many necessary support files on the fly. This
makes Tectonic super easy to install.

[TeXLive]: https://www.tug.org/texlive/acquire-netinstall.html

The quickest way to get started is to [download the latest release][gh-latest]
from GitHub, or to [install a packaged version][inst-packaged] if one is
available. If you’d like to see detailed instructions, go to the [Installation
Reference][inst-ref]. But the short version of the GitHub approach is that you
should:

1. Click through to the most recent non-preview release
2. Download the `.tar.gz` or `.zip` file that corresponds to your computer’s
   operating system and CPU type
3. Unpack that archive to obtain the `tectonic` executable file. (Or,
   `tectonic.exe` on Windows.)
4. Put that file in the appropriate location so that you can easily run it from
   your computer’s command prompt.

[gh-latest]: https://github.com/tectonic-typesetting/tectonic/releases/latest
[inst-packaged]: ../installation/index.md#pre-built-binary-packages
[inst-ref]: ../installation/index.md

That’s all there is to it! You’ll know that you’re set up when you can go to
your computer’s command prompt and run:

```sh
$ tectonic --help
```

and the result is that you get a printout of information about different options
and arguments that you can pass to the Tectonic program. (Here and elsewhere
we’ll use a convention of a leading `$` to indicate a command that you should
run at your computer’s command prompt. You don’t type the dollar sign itself.)

To be explicit, *Tectonic does not invoke an external `latex` program, and
Tectonic is not a “wrapper” for (La)TeX.* Tectonic *is* the LaTeX program. This
is essential. [The goals of the Tectonic project][goals] involve a fundamental
transformation of how we use TeX to create technical documents, and it is not
possible to achieve that without radical surgery to the heart of how TeX has
traditionally operated.

[goals]: ../introduction/index.md#the-goals-of-tectonic