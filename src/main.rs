mod cli;
mod package_manager;
mod utils;

use crate::package_manager::PackageManager;
use clap::Parser;

use clap::CommandFactory;
use clap_mangen::Man;

use std::{fs::File, path::Path};

fn main() {
    let args = cli::Cli::parse();
    let app = cli::Cli::command();

    let file = Path::new("./sapm.example.1");
    let mut file = File::create(&file).unwrap();

    Man::new(app).render(&mut file).unwrap();

    let package_manager =
        PackageManager::from_name(&args.package_manager).expect("No such package manager");

    let command = PackageManager::generate_command(package_manager, args.sub_command);

    PackageManager::execute_command(command);
}
