#!/bin/env bash
cd "$TRAVIS_BUILD_DIR/dist/arch"

# Obtain mksrcinfo
wget https://www.archlinux.org/packages/community/any/pkgbuild-introspection/download/ -O pkgbuild-introspection.tar.xz
tar -Jxf pkgbuild-introspection.tar.xz usr/bin/mksrcinfo
PATH="$PATH:$(pwd)/usr/bin"

# Prepare directory for deploy
git clone ssh://aur@aur.archlinux.org/tectonic.git aur
cp PKGBUILD aur
cd aur
mksrcinfo

git add PKGBUILD .SRCINFO
git config user.email "tectonic-deploy@example.com"
git config user.name "tectonic-deploy"
git commit -m "Release $TRAVIS_TAG"

# Deploy to AUR
git push origin master
