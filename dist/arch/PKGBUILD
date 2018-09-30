# Maintainer: Daniel M. Capella <polycitizen@gmail.com>
# Maintainer: Jan Tojnar <jtojnar@gmail.com>

pkgname=tectonic
pkgver=0.1.10
pkgrel=1
arch=('x86_64')
pkgdesc='Modernized, complete, self-contained TeX/LaTeX engine, powered by XeTeX and TeXLive'
url=https://tectonic-typesetting.github.io/
license=('MIT')
depends=('fontconfig' 'harfbuzz-icu' 'openssl')
makedepends=('rust')
source=("$pkgname-$pkgver.tar.gz::https://crates.io/api/v1/crates/$pkgname/$pkgver/download")
sha512sums=('44d75487a8543e765730751879fab44a78fe0fdc8cd92943be74e6e4c12df3ef0a002e1632991090e853d4944e95d3c1075d6e9a011fd432a8e8501adc52ebb9')

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
