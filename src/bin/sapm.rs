use std::process::ExitCode;

use clap::Parser;

use colored::Colorize;
use sapm::modules::cli;
use sapm::modules::package_manager;

fn main() -> ExitCode {
    let args = cli::Cli::parse();

    // parse sapm.conf

    let package_manager = package_manager::PackageManager::from_name(&args.package_manager)
        .expect("No such package manager");

    let command_string =
        package_manager::PackageManager::match_sapm_subcommand_to_package_manager_command_string(
            package_manager,
            args.sub_command,
        );

    if args.verbose {
        println!(
            "{} {}: `{}`\n",
            "[INFO]".green().bold(),
            "SAPM will execute".white().bold(),
            &command_string.yellow(),
        );
    }

    let status = package_manager::PackageManager::execute_command(&command_string).unwrap();

    if args.verbose {
        if status.success() {
            println!(
                "{} {}: `{}`",
                "[INFO]".green().bold(),
                "SAPM Sucessfully executed".white().bold(),
                &command_string.yellow(),
            );
        } else {
            println!(
                "\n{} {}: `{}`",
                "[ERROR]".red().bold(),
                "SAPM failed to execute".white().bold(),
                &command_string.yellow(),
            );
        }
    }

    if status.success() {
        return ExitCode::SUCCESS;
    } else {
        return ExitCode::FAILURE;
    }
}
