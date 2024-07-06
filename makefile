# https://gist.github.com/jlgerber/0f280236c2ee1b741dfe41a38d39a467
prog :=sapm

build target/release/$(prog):
	cargo build --release

install: target/release/$(prog)
	cp --force target/release/$(prog) ~/.local/share/bin/$(prog)

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"
