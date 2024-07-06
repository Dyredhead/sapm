# https://wiki.archlinux.org/title/Rust_package_guidelines
pkgname="sapm"
pkgver="1.0.0"
pkgrel=1
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

# package() {
#     install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

#     mkdir -p "$pkgdir/etc/$pkgname/package_managers/"
#     for file in "data/config/*"; do
#         install -Dm644 "$file" "$pkgdir/etc/$pkgname/$file"
#     done

#     mkdir $pkgdir/usr/share/$pkgname/
#     install -Dm644 "data/local/template.json" "$pkgdir/usr/share/$pkgname/"
    
#     ./target/release/man 
#     for file in "data/local/man/*.1"; do
#         install -Dm644 "$file" "$pkgdir/usr/share/man/man1/$file"
#     done

#     ./target/release/completions
#     install -Dm644 "data/local/completions/$pkgname.bash" "$pkgdir/usr/share/bash-completion/completions/"
#     install -Dm644 "data/local/completions/$pkgname.fish" "$pkgdir/usr/share/fish/completions/"
#     install -Dm644 "data/local/completions/_$pkgname" "$pkgdir/usr/share/zsh/vendor-completions/"
# }
