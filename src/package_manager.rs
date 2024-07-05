use crate::utils::combine_vectors;

use std::process::Command;

use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PackageManager {
    path: std::path::PathBuf,

    find: Vec<String>,
    info: Vec<String>,
    install: Vec<String>,
    list: Vec<String>,
    uninstall: Vec<String>,
    update: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum SubCommand {
    /// Find the specified package
    Find { package: String },
    /// Get information about the specified package
    Info { package: String },
    /// Install the specified package(s)
    Install { packages: Vec<String> },
    /// List all of the installed packages
    List,
    /// Uninstall the specified package(s)
    Uninstall { packages: Vec<String> },
    /// Update all packages
    Update,
}

impl PackageManager {
    pub fn execute_command(mut command: Command) {
        command.status().expect("failed to execute process");
    }

    pub fn generate_command(package_manager: PackageManager, subcommand: SubCommand) -> Command {
        let mut command = Command::new(&package_manager.path);
        let args =
            self::PackageManager::match_subcommand_to_package_manager(package_manager, subcommand);
        for arg in args {
            command.arg(arg);
        }
        return command;
    }

    // function for mapping the command to the package manager
    fn match_subcommand_to_package_manager(
        package_manager: PackageManager,
        sub_command: SubCommand,
    ) -> Vec<String> {
        match sub_command {
            SubCommand::Find { package } => {
                return combine_vectors(package_manager.find, vec![package]);
            }
            SubCommand::Info { package } => {
                return combine_vectors(package_manager.info, vec![package]);
            }
            SubCommand::Install { packages } => {
                return combine_vectors(package_manager.install, packages);
            }
            SubCommand::List => {
                return package_manager.list;
            }
            SubCommand::Uninstall { packages } => {
                return combine_vectors(package_manager.uninstall, packages);
            }
            SubCommand::Update => {
                return package_manager.update;
            }
        }
    }

    // TODO: maybe use another format for the package manager config that is not json
    pub fn from_name(name: &str) -> Option<Self> {
        for data_dir in xdg::BaseDirectories::get_data_dirs(
            &xdg::BaseDirectories::with_prefix("sapm/package_managers").unwrap(),
        ) {
            if let Ok(package_managers) = std::fs::read_dir(data_dir) {
                for package_manager in package_managers {
                    let package_manager = package_manager.unwrap();
                    if package_manager.file_name() == (name.to_string() + ".json").as_str() {
                        return Self::from_file(package_manager.path());
                    }
                }
            }
        }
        return None;
    }
    fn from_file(path: std::path::PathBuf) -> Option<Self> {
        if path.is_file() {
            let json = std::fs::read_to_string(path).unwrap();
            return Self::from_json(&json);
        }
        return None;
    }
    fn from_json(json: &str) -> Option<Self> {
        let package_manager: PackageManager = serde_json::from_str(json).unwrap();
        return Some(package_manager);
    }
}
