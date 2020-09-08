# Releasing New Versions of Tectonic

The new goal is to automate as much of this process as possible with [Cranko].
Here are some notes that might still be helpful even in the Cranko world, though.

[Cranko]: https://pkgw.github.io/cranko/

1. Announce intention to make a release.
1. Review outstanding pull requests and issues for showstoppers or easy wins.
   Address as many as possible.
1. Check if worth bulk-updating deps: `cargo update`
1. Check Arch Linux PKGBUILD file for updates to keep:
   ```
   curl 'https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=tectonic' >dist/arch/PKGBUILD
   ```
1. Announce on forum.
