# Maintainer: Jan Tojnar <jtojnar@gmail.com>
pkgname=tectonic
pkgver=0.1.5
pkgrel=2
makedepends=('cargo')
arch=('i686' 'x86_64')
pkgdesc='modernized, complete, self-contained TeX/LaTeX engine, powered by XeTeX and TeXLive'
url='https://tectonic-typesetting.github.io/en-US/'
license=('MIT')
depends=('fontconfig' 'harfbuzz-icu' 'openssl')
source=("https://github.com/tectonic-typesetting/tectonic/archive/v${pkgver}/${pkgname}.tar.gz")
sha256sums=('a493d6685cf63bea389c290677a641786f0b899e3e77fc7e865c8d1dcabc7aac')

build() {
	cd "$srcdir/$pkgname-$pkgver"
	cargo build --release
}

package() {
	cd "$srcdir/$pkgname-$pkgver"
	mkdir -p "$pkgdir/usr/bin"
	install "target/release/tectonic" "$pkgdir/usr/bin/tectonic"
}
