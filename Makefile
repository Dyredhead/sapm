# https://gist.github.com/jlgerber/0f280236c2ee1b741dfe41a38d39a467
pkgname :=sapm

# all: build install

build ./target/release/$(pkgname):
	cargo build --release
	./target/release/man
	./target/release/completions

# install: ./target/release/$(pkgname)

# 	@if [ -f "${XDG_DATA_HOME}/bin/$(pkgname)" ]; then \
# 		echo "sapm is already installed, skipping installation"; \
# 		echo "if you wish to proceed with the installation please first run make uninstall"; \
# 		exit 1; \
# 	fi

# 	install -Dm755 "./target/release/$(pkgname)" "${XDG_DATA_HOME}/bin/$(pkgname)"

# 	@if ![ -d "${XDG_CONFIG_HOME}/$(pkgname)" ]; then \
# 		mkdir -p "${XDG_CONFIG_HOME}/$(pkgname)"
# 	else \
# 		echo "Directory: ${XDG_CONFIG_HOME}/$(pkgname) already exists, skipping configuration installation"; \
# 	fi

# 	@if [ -d "${XDG_DATA_HOME}/bash-completion/" ]; then \
# 		mkdir -p "${XDG_DATA_HOME}/bash-completion/completions/"; \
# 		cp -a ./data/completions/$(pkgname).bash "${XDG_DATA_HOME}/bash-completion/$(pkgname)"; \
# 	else \
# 		echo "Directory: ${XDG_DATA_HOME}/bash-completion/completions/ was not found, skipping Bash completion installation"; \
# 	fi

# 	@if [ -d "${XDG_DATA_HOME}/fish/vendor_completions.d/" ]; then \
# 		cp -a ./data/completions/$(pkgname).fish "${XDG_DATA_HOME}/fish/vendor_completions.d/$(pkgname).fish"; \
# 	else \
# 		echo "Directory: ${XDG_DATA_HOME}/fish/vendor_completions.d/ was not found, skipping Fish completion installation"; \
# 	fi

# 	@if [ -d "${XDG_CONFIG_HOME}/zsh/oh-my-zsh/" ]; then \
# 		mkdir -p "${XDG_CONFIG_HOME}/zsh/oh-my-zsh/completions"; \
# 		cp -a ./data/completions/_$(pkgname) "${XDG_CONFIG_HOME}/zsh/oh-my-zsh/completions/_$(pkgname)"; \
# 	else \
# 		echo "Directory: ${XDG_CONFIG_HOME}/zsh/oh-my-zsh/completions/ was not found, skipping Zsh completion installation"; \
# 	fi

# 	mkdir -p "${XDG_DATA_HOME}/man/man1"
# 	cp -a "./data/man/"*".1" "${XDG_DATA_HOME}/man/man1/"

# 	mkdir -p "${XDG_DATA_HOME}/$(pkgname)/package_managers"
# 	cp -a "./data/package_managers/"* "${XDG_DATA_HOME}/$(pkgname)/package_managers"

# uninstall:
# 	rm -f "${XDG_DATA_HOME}/bin/$(pkgname)"
# 	rm -rf "${XDG_DATA_HOME}/$(pkgname)/"*
# 	rm -f "${XDG_DATA_HOME}/bash-completion/completions/$(pkgname)"
# 	rm -f "${XDG_DATA_HOME}/fish/vendor_completions.d/$(pkgname).fish"
# 	rm -f "${XDG_CONFIG_HOME}/zsh/oh-my-zsh/completions/_$(pkgname)"
# 	rm -f "${XDG_DATA_HOME}/man/man1/$(pkgname)"*".1"

# clean:
# 	cargo clean

# purge: uninstall clean
# 	rm -rf "${XDG_CONFIG_HOME}/$(pkgname)/"*

# help:
# 	@echo "Usage: make [target]"
# 	@echo ""
# 	@echo "Targets:"
# 	@echo "  all        Build and install the program"
# 	@echo "  build      Build the program"
# 	@echo "  install    Install the program"
# 	@echo "  uninstall  Uninstall the program"
# 	@echo "  clean      Clean the build directory"
# 	@echo "  purge      Uninstall the program and its configuration and clean the build directory"
# 	@echo "  help       Display this help message"
