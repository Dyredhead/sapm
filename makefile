# https://gist.github.com/jlgerber/0f280236c2ee1b741dfe41a38d39a467
prog :=sapm

build target/release/$(prog):
	cargo build --release

install: target/release/$(prog)
	cp --force -r target/release/$(prog) ~/.local/share/bin/$(prog)
	cp --force -r data/config/* ~/.config/sapm/
	sudo cp --force -r  data/config/* /etc/sapm/

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"
