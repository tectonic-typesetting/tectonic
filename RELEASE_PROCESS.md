# Releasing New Versions of Tectonic

1. Announce intention to make a release.
2. Review outstanding pull requests and issues for showstoppers or easy wins.
   Address as many as possible.
3. Fetch from `tectonic-typesetting` and create a new branch `release-X.Y.Z`
   from `master`.
4. Update deps: `cargo update && cargo test --all`, then commit changes.
5. Check Arch Linux PKGBUILD file for updates and merge locally if necessary:
   ```
   curl 'https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=tectonic' >dist/arch/PKGBUILD
   ```
6. Consider updating the mdbook version used in `dist/build-mdbook.sh`;
   [releases listed here](https://github.com/rust-lang/mdBook/releases).
7. Backfill `CHANGELOG.md` from the Git history (which will include GitHub PR
   numbers, etc.) To view the history while skipping dependabot commits:
   ```
   git log --perl-regexp --author='^((?!dependabot).*)$' v${CURRENT_VERSION}..
   ```
8. Update version number(s):
   - Main version in `Cargo.toml`
   - Un-bump versions of crates like `tectonic_xdv` if they haven't updated.
   - `cargo build --release` to update `Cargo.lock` as well.
   Commit with message `Tentative version $version`
9. Push to GitHub and create a pull request.
10. Examine the CI output. Fix any failures, committing any changes and
    re-pushing. No need to rewrite history at this juncture.
11. `mkdir ~/tectonictmp; mv .envrc target $other_files ~/tectonictmp ; git clean -fxd`
12. Attempt to `cargo publish`. Fix anything that needs fixing, committing
    changes.
13. Do a `git rebase -i` to rewrite the history to make the version-bump
    commit the most recent one, rewording its commit message to remove the
    "Tentative". Re-push and re-CI if anything changed that might affect the
    build.
14. Download the new package from
    <https://crates.io/api/v1/crates/tectonic/$version/download>. Put its
    sha512 sum into `dist/arch/PKGBULD` and update the package version and
    release in that file. Rewrite the version-bump commit to include the
    changes (`git commit --amend -C HEAD`).
15. `git tag v${version}` and, if necessary, `git tag submodule-v${version}`.
16. Back to development:
    - Bump main version in `Cargo.toml` and crates like `tectonic_xdv`.
    - `cargo build` to update the lockfile
    - Commit with message `Back to development, version $version`.
17. Re-push to the release PR branch and wait for CI.
18. Merge, assuming nothing bad happened with the CI.
19. Pull the new `master` locally to get GitHub’s merge commit.
20. `git push --tags`, which should trigger some automated release processes.
    Monitor them.
21. Create new release on GitHub, filling with contents of `CHANGELOG.md`.
22. Update
    [conda-forge package](https://github.com/conda-forge/tectonic-feedstock) —
    the bot should notice the new version on Crates.io and automatically file
    a pull request quickly.
23. Announce on forum.
24. Update website.


### To check

- Do we need to take any further steps to release the AppImage?
- Uploading artifacts like static binaries to the GitHub release
- `cargo publish` from CI, not manually (need some way to test-publish first).
