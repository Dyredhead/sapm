
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'sapm' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'sapm'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'sapm' {
            [CompletionResult]::new('--package-manager', 'package-manager', [CompletionResultType]::ParameterName, 'Use the specified package manager instead of the default')
            [CompletionResult]::new('--pm', 'pm', [CompletionResultType]::ParameterName, 'Use the specified package manager instead of the default')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show the command that SAPM will execute')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show the command that SAPM will execute')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('info', 'info', [CompletionResultType]::ParameterValue, 'Get information about the specified package')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Get information about the specified package')
            [CompletionResult]::new('install', 'install', [CompletionResultType]::ParameterValue, 'Install the specified package(s)')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Install the specified package(s)')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all of the installed packages')
            [CompletionResult]::new('ls', 'ls', [CompletionResultType]::ParameterValue, 'List all of the installed packages')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search for the specified package')
            [CompletionResult]::new('find', 'find', [CompletionResultType]::ParameterValue, 'Search for the specified package')
            [CompletionResult]::new('uninstall', 'uninstall', [CompletionResultType]::ParameterValue, 'Uninstall the specified package(s)')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Uninstall the specified package(s)')
            [CompletionResult]::new('update', 'update', [CompletionResultType]::ParameterValue, 'Update all packages')
            [CompletionResult]::new('upgrade', 'upgrade', [CompletionResultType]::ParameterValue, 'Update all packages')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'sapm;info' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;show' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;install' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;add' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;list' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;ls' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;search' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;find' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;uninstall' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;remove' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;update' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;upgrade' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'sapm;help' {
            [CompletionResult]::new('info', 'info', [CompletionResultType]::ParameterValue, 'Get information about the specified package')
            [CompletionResult]::new('install', 'install', [CompletionResultType]::ParameterValue, 'Install the specified package(s)')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all of the installed packages')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search for the specified package')
            [CompletionResult]::new('uninstall', 'uninstall', [CompletionResultType]::ParameterValue, 'Uninstall the specified package(s)')
            [CompletionResult]::new('update', 'update', [CompletionResultType]::ParameterValue, 'Update all packages')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'sapm;help;info' {
            break
        }
        'sapm;help;install' {
            break
        }
        'sapm;help;list' {
            break
        }
        'sapm;help;search' {
            break
        }
        'sapm;help;uninstall' {
            break
        }
        'sapm;help;update' {
            break
        }
        'sapm;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
