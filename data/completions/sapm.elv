
use builtin;
use str;

set edit:completion:arg-completer[sapm] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'sapm'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'sapm'= {
            cand --package-manager 'Use the specified package manager instead of the default'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand find 'Find the specified package'
            cand info 'Get information about the specified package'
            cand install 'Install the specified package(s)'
            cand list 'List all of the installed packages'
            cand uninstall 'Uninstall the specified package(s)'
            cand update 'Update all packages'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'sapm;find'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'sapm;info'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'sapm;install'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'sapm;list'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'sapm;uninstall'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'sapm;update'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'sapm;help'= {
            cand find 'Find the specified package'
            cand info 'Get information about the specified package'
            cand install 'Install the specified package(s)'
            cand list 'List all of the installed packages'
            cand uninstall 'Uninstall the specified package(s)'
            cand update 'Update all packages'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'sapm;help;find'= {
        }
        &'sapm;help;info'= {
        }
        &'sapm;help;install'= {
        }
        &'sapm;help;list'= {
        }
        &'sapm;help;uninstall'= {
        }
        &'sapm;help;update'= {
        }
        &'sapm;help;help'= {
        }
    ]
    $completions[$command]
}
