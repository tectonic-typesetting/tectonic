# Getting Started: Install Tectonic

Let’s start out by making sure that Tectonic is installed on your system.

Even this step will be very different than what you might be used to with other
TeX systems. To install a normal TeX system such as [TeXLive], you normally need
to download gigabytes of support files and set them up in a complex directory
hierarchy. Tectonic, on the other hand, is distributed as a single executable
file. That one file not only combines the functionality of many standard TeX
programs, but it also can download the many necessary support files on the fly.
This makes Tectonic super easy to install.

[TeXLive]: https://www.tug.org/texlive/acquire-netinstall.html

The quickest way to get started is to use your terminal. On a computer running a
Unix-like operating system, including macOS, just run the following command in
your terminal:

```sh
curl --proto '=https' --tlsv1.2 -fsSL https://drop-sh.fullyjustified.net |sh
```

This will download the `tectonic` program and place it into the directory where
you ran the command.

On Windows, copy-paste this into a PowerShell window, which will unpack
`tectonic.exe` for you:

```ps1
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://drop-ps1.fullyjustified.net'))
```

No matter your operating system, you should move the unpacked file into a
directory in your executable search path so that you can run Tectonic from any
working directory. For the time being, the download script doesn’t do this
because it can be tricky to automatically determine what the best installation
destination would be.

Alternatively, you can [install a packaged version of Tectonic][inst-packaged]
if one is available. For detailed instructions and additional installation
options, go to the [How To Install Tectonic][inst-ref] guide.

[inst-packaged]: ../installation/index.md#pre-built-binary-packages
[inst-ref]: ../installation/index.md

You’ll know that you’re set up when you can go to your computer’s command prompt
and run:

```sh
$ tectonic --help
```

and the result is that you get a printout of information about different options
and arguments that you can pass to the Tectonic program. (From now on we’ll use
a convention of a leading `$` to indicate a command that you should run at your
computer’s command prompt. You don’t type the dollar sign itself.)

To be explicit, *Tectonic does not invoke an external `latex` program, and
Tectonic is not a “wrapper” for (La)TeX.* Tectonic *is* the LaTeX program. This
is essential. [The goals of the Tectonic project][goals] involve a fundamental
transformation of how we use TeX to create technical documents, and it is not
possible to achieve that without radical surgery to the heart of how TeX has
traditionally operated.

[goals]: ../introduction/index.md#the-goals-of-tectonic
