complete -c sapm -n "__fish_use_subcommand" -l package-manager -d 'Use the specified package manager instead of the default' -r
complete -c sapm -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c sapm -n "__fish_use_subcommand" -f -a "find" -d 'Find the specified package'
complete -c sapm -n "__fish_use_subcommand" -f -a "info" -d 'Get information about the specified package'
complete -c sapm -n "__fish_use_subcommand" -f -a "install" -d 'Install the specified package(s)'
complete -c sapm -n "__fish_use_subcommand" -f -a "list" -d 'List all of the installed packages'
complete -c sapm -n "__fish_use_subcommand" -f -a "uninstall" -d 'Uninstall the specified package(s)'
complete -c sapm -n "__fish_use_subcommand" -f -a "update" -d 'Update all packages'
complete -c sapm -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c sapm -n "__fish_seen_subcommand_from find" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_seen_subcommand_from info" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_seen_subcommand_from install" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_seen_subcommand_from uninstall" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_seen_subcommand_from update" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from find info install list uninstall update help" -f -a "find" -d 'Find the specified package'
complete -c sapm -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from find info install list uninstall update help" -f -a "info" -d 'Get information about the specified package'
complete -c sapm -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from find info install list uninstall update help" -f -a "install" -d 'Install the specified package(s)'
complete -c sapm -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from find info install list uninstall update help" -f -a "list" -d 'List all of the installed packages'
complete -c sapm -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from find info install list uninstall update help" -f -a "uninstall" -d 'Uninstall the specified package(s)'
complete -c sapm -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from find info install list uninstall update help" -f -a "update" -d 'Update all packages'
complete -c sapm -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from find info install list uninstall update help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'