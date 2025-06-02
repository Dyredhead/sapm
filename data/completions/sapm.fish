# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_sapm_global_optspecs
	string join \n package-manager= n/dry-run v/verbose h/help V/version
end

function __fish_sapm_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_sapm_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_sapm_using_subcommand
	set -l cmd (__fish_sapm_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c sapm -n "__fish_sapm_needs_command" -l package-manager -l pm -d 'Use the specified package manager instead of the default' -r
complete -c sapm -n "__fish_sapm_needs_command" -s n -l dry-run
complete -c sapm -n "__fish_sapm_needs_command" -s v -l verbose -d 'Show the command that SAPM will execute'
complete -c sapm -n "__fish_sapm_needs_command" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_needs_command" -s V -l version -d 'Print version'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "info" -d 'Get information about the specified package'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "show" -d 'Get information about the specified package'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "install" -d 'Install the specified package(s)'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "add" -d 'Install the specified package(s)'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "list" -d 'List all of the installed packages'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "ls" -d 'List all of the installed packages'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "search" -d 'Search for the specified package'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "find" -d 'Search for the specified package'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "uninstall" -d 'Uninstall the specified package(s)'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "remove" -d 'Uninstall the specified package(s)'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "update" -d 'Update all packages'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "upgrade" -d 'Update all packages'
complete -c sapm -n "__fish_sapm_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c sapm -n "__fish_sapm_using_subcommand info" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand show" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand install" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand add" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand list" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand ls" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand search" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand find" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand uninstall" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand remove" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand update" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand upgrade" -s h -l help -d 'Print help'
complete -c sapm -n "__fish_sapm_using_subcommand help; and not __fish_seen_subcommand_from info install list search uninstall update help" -f -a "info" -d 'Get information about the specified package'
complete -c sapm -n "__fish_sapm_using_subcommand help; and not __fish_seen_subcommand_from info install list search uninstall update help" -f -a "install" -d 'Install the specified package(s)'
complete -c sapm -n "__fish_sapm_using_subcommand help; and not __fish_seen_subcommand_from info install list search uninstall update help" -f -a "list" -d 'List all of the installed packages'
complete -c sapm -n "__fish_sapm_using_subcommand help; and not __fish_seen_subcommand_from info install list search uninstall update help" -f -a "search" -d 'Search for the specified package'
complete -c sapm -n "__fish_sapm_using_subcommand help; and not __fish_seen_subcommand_from info install list search uninstall update help" -f -a "uninstall" -d 'Uninstall the specified package(s)'
complete -c sapm -n "__fish_sapm_using_subcommand help; and not __fish_seen_subcommand_from info install list search uninstall update help" -f -a "update" -d 'Update all packages'
complete -c sapm -n "__fish_sapm_using_subcommand help; and not __fish_seen_subcommand_from info install list search uninstall update help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
