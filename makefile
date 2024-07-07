# https://gist.github.com/jlgerber/0f280236c2ee1b741dfe41a38d39a467
pkgname :=sapm

all: build install clean

build target/release/$(pkgname):
	cargo build --release

install: ./target/release/$(pkgname)
	cp -a "./target/release/$(pkgname)" "${XDG_DATA_HOME}/bin/$(pkgname)"
	cp -a "./data/config/"* "${XDG_CONFIG_HOME}/$(pkgname)"

	mkdir -p "${XDG_CONFIG_HOME}/zsh/oh-my-zsh/completions"
	cp -a ./data/local/completions/_$(pkgname) "${XDG_CONFIG_HOME}/zsh/oh-my-zsh/completions/_$(pkgname)"

	mkdir -p "${XDG_DATA_HOME}/man/man1"
	cp -a "./data/local/man/"*".1" "${XDG_DATA_HOME}/man/man1/"

uninstall:
	rm -f "${XDG_DATA_HOME}/bin/$(pkgname)"
	rm -rf "${XDG_CONFIG_HOME}/$(pkgname)"
	rm -f "${XDG_CONFIG_HOME}/zsh/oh-my-zsh/completions/_$(pkgname)"
	rm -f "${XDG_DATA_HOME}/man/man1/$(pkgname)"*".1"

clean:
	cargo clean

help:
	@echo "build - build the program"
	@echo "install - install the program"
	@echo "all - build and install the program"
	@echo "help - print this message"
