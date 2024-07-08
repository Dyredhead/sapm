use clap::Parser;

use colored::Colorize;
use sapm::modules::cli;
use sapm::modules::package_manager;

fn main() {
    let args = cli::Cli::parse();

    // parse sapm.conf

    let package_manager = package_manager::PackageManager::from_name(&args.package_manager)
        .expect("No such package manager");

    let command_string = package_manager::PackageManager::match_subcommand_to_package_manager(
        package_manager,
        args.sub_command,
    );

    if args.verbose {
        println!(
            "{} {} `{}`\n",
            "[INFO]".green().bold(),
            "SAPM will execute:".white().bold(),
            &command_string.yellow(),
        );
    }
    package_manager::PackageManager::execute_command(&command_string);
}
