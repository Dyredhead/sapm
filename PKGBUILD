# https://wiki.archlinux.org/title/Rust_package_guidelines
pkgname="sapm"
pkgver="1.0.0"
pkgdesc="A System Agnostic Package Manager (SAPM) for simple, general use cases."
arch=("any")
url="https://github.com/Dyredhead/sapm"
license=("MIT")
makedepends=("cargo")

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/"
    
    mkdir $pkgdir/usr/share/$pkgname/
    install -Dm644 "data/local/*" "$pkgdir/usr/share/$pkgname/"

    mdkir -p "$pkgdir/etc/$pkgname/package_managers/"
    install -Dm644 "data/config/*" "$pkgdir/etc/$pkgname/"
}
