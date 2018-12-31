# Maintainer: tectonic-deploy <sasha+tectonic@hackafe.net>
# Maintainer: Daniel M. Capella <polyzen@archlinux.org>
# Contributor: Jan Tojnar <jtojnar@gmail.com>

# The master version of this file is maintained here:
#
#   https://github.com/tectonic-typesetting/tectonic/blob/master/dist/arch/PKGBUILD
#
# The version on aur.archlinux.org is updated automatically through a Travis
# CI deploy script that's invoked when new tags are pushed to the GitHub
# tectonic repository.

pkgname=tectonic
pkgver=0.1.11
pkgrel=0
arch=('x86_64')
pkgdesc='Modernized, complete, self-contained TeX/LaTeX engine, powered by XeTeX and TeXLive'
url=https://tectonic-typesetting.github.io/
license=('MIT')
depends=('fontconfig' 'harfbuzz-icu' 'openssl')
makedepends=('rust')
source=("$pkgname-$pkgver.tar.gz::https://crates.io/api/v1/crates/$pkgname/$pkgver/download")
sha512sums=('103b22129e3daa28dbb6e68cbc256c711bc6cd3b51e3b356db67d421c6b547eef1d600da2750a5312426ad9db132f4f77d452abeb77062d5394e0f887af8ff81')

build() {
	cd $pkgname-$pkgver
	cargo build --release
}

check() {
	cd $pkgname-$pkgver
	cargo test --release
}

package() {
	cd $pkgname-$pkgver
	install -Dm755 target/release/tectonic "$pkgdir"/usr/bin/tectonic
	install -Dm644 LICENSE "$pkgdir"/usr/share/licenses/$pkgname/LICENSE
}
