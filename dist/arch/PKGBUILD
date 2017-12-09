# Maintainer: Daniel M. Capella <polycitizen@gmail.com>
# Maintainer: Jan Tojnar <jtojnar@gmail.com>

pkgname=tectonic
pkgver=0.1.7
pkgrel=1
arch=('x86_64')
pkgdesc='Modernized, complete, self-contained TeX/LaTeX engine, powered by XeTeX and TeXLive'
url=https://tectonic-typesetting.github.io/en-US/
license=('MIT')
depends=('fontconfig' 'harfbuzz-icu' 'openssl')
makedepends=('rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/tectonic-typesetting/$pkgname/archive/v$pkgver.tar.gz")
sha512sums=('4d9d942d6b2617a58b1a18d8ec91c374b3f3f4c808f6c365c58129194b0fa66d9552dee58c42e499b349ccb753b7378223eaeb75c87b9afbf417bee3585265ac')

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
