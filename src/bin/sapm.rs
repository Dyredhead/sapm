use clap::Parser;

use sapm::modules::cli;
use sapm::modules::package_manager;

fn main() {
    let args = cli::Cli::parse();

    let package_manager = package_manager::PackageManager::from_name(&args.package_manager)
        .expect("No such package manager");

    let command =
        package_manager::PackageManager::generate_command(package_manager, args.sub_command);

    package_manager::PackageManager::execute_command(command);
}
