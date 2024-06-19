use clap::Parser;
mod cli;
mod package_manager;
use package_manager::PackageManager;

fn main() {
    let args = cli::Args::parse();

    let package_manager =
        PackageManager::from_name(&args.package_manager).expect("No such package manager");

    let command = PackageManager::generate_command(package_manager, args.sub_command);

    println!("{:?}", command);
    PackageManager::execute_command(command);
}
