# Maintainer: Az <zazaguichi@outlook.com>
pkgname=matrizal
pkgver=Release.1.1.r9.gc5a1c37
pkgrel=1
pkgdesc="A matrix calculator cli to do my homework"
url="https://github.com/Azakidev/matrizal"
license=("GNU General Public License version 3")
arch=("x86_64")
makedepends=("cargo")

pkgver() {
    (git describe --long --tags || echo "$pkgver") | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
    return 0
}

package() {
    cd ..
    usrdir="$pkgdir/usr"
    mkdir -p $usrdir
    cargo install --no-track --path . --root "$usrdir"
}

