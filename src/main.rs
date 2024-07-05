mod cli;
mod package_manager;
mod utils;

use crate::package_manager::PackageManager;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    let package_manager =
        PackageManager::from_name(&args.package_manager).expect("No such package manager");

    let command = PackageManager::generate_command(package_manager, args.sub_command);

    println!("{:?}", command);
    PackageManager::execute_command(command);
}
