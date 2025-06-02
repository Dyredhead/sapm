export pkgname := "sapm"

@default:
    just --list

# Build main binary as well as man page and shell completions
build:
    #!/usr/bin/env sh
    cargo build --release
    ./target/release/man
    ./target/release/completions

# Install all project related files
install:
    #!/usr/bin/env fish

    if test -f "$XDG_DATA_HOME/bin/$pkgname"
        echo "sapm is already installed, skipping installation"
        echo "if you wish to proceed with the installation please first run make uninstall"
        exit 1
    end

    # Binary
    install -Dm755 "./target/release/$pkgname" "$XDG_DATA_HOME/bin/$pkgname"
    
    mkdir -p "$XDG_DATA_HOME/$pkgname"
    mkdir -p "$XDG_DATA_HOME/$pkgname/package_managers"
    
    # data
    cp -a "./data/package_managers/"* "$XDG_DATA_HOME/$pkgname/package_managers"
    cp -a "./data/conf.toml" "$XDG_DATA_HOME/$pkgname/conf.toml"
    cp -a "./data/template.toml" "$XDG_DATA_HOME/$pkgname/template.toml"

    mkdir -p "$XDG_CONFIG_HOME/$pkgname"

    # Bash completions
    if test -d "$XDG_DATA_HOME/bash-completion/"
        mkdir -p "$XDG_DATA_HOME/bash-completion/completions/"
        cp -a "./data/completions/$pkgname.bash" "$XDG_DATA_HOME/bash-completion/completions/$pkgname"
    else 
        echo "Directory `$XDG_DATA_HOME/bash-completion/completions` was not found, skipping Bash completion installation"
    end

    # Zsh completions
    if test -d "$XDG_CONFIG_HOME/zsh/oh-my-zsh/"
        mkdir -p "$XDG_CONFIG_HOME/zsh/oh-my-zsh/completions"
        cp -a "./data/completions/_$pkgname" "$XDG_CONFIG_HOME/zsh/oh-my-zsh/completions/_$pkgname"
    else
        echo "Directory `$XDG_CONFIG_HOME/zsh/oh-my-zsh/completions/` was not found, skipping Zsh completion installation"
    end

    # Fish completions
    if test -d "$XDG_DATA_HOME/fish/vendor_completions.d/"
        cp -a "./data/completions/$pkgname.fish" "$XDG_DATA_HOME/fish/vendor_completions.d/$pkgname.fish"
    else 
        echo "Directory: `$XDG_DATA_HOME/fish/vendor_completions.d/` was not found, skipping Fish completion installation"
    end

    # Man page	
    mkdir -p "$XDG_DATA_HOME/man/man1"
    cp -a "./data/man/"*".1" "$XDG_DATA_HOME/man/man1/"

# Uninstall all project related files
uninstall:
    #!/usr/bin/env fish

    rm -f "$XDG_DATA_HOME/bin/$pkgname"
    rm -rf "$XDG_DATA_HOME/$pkgname/"
    rm -f "$XDG_DATA_HOME/bash-completion/completions/$pkgname"
    rm -f "$XDG_CONFIG_HOME/zsh/oh-my-zsh/completions/_$pkgname"
    rm -f "$XDG_DATA_HOME/fish/vendor_completions.d/$pkgname.fish"
    rm -f "$XDG_DATA_HOME/man/man1/$pkgname*"

clean:
	cargo clean