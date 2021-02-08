# Maintainer: Daniel Rose <danielrose@member.fsf.org>
pkgname=swq-bin
pkgver=0.1.1
pkgrel=1
pkgdesc="Fetch and add Star Wars quotes (and GIFs!) from the CLI."
url="https://github.com/thecatster/swq-rs"
license=("GPL-3.0")
arch=("x86_64")
provides=("swq")
options=("strip")
source=("https://github.com/thecatster/swq-rs/releases/download/v$pkgver/swq-$pkgver-x86_64.tar.gz")
sha256sums=("667bcda539649577a83883a048c57c3b7b43f5bb36eaf9f7ce05687a5b206595")

package() {
    install -Dm755 swq -t "$pkgdir/usr/bin/"
}
