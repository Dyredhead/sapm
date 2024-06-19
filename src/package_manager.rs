use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use SAPM::combine_vectors;
#[derive(Deserialize, Serialize)]
pub struct PackageManager {
    path: std::path::PathBuf,

    auto_remove: Vec<String>,
    clean: Vec<String>,
    install: Vec<String>,
    list: Vec<String>,
    purge: Vec<String>,
    uninstall: Vec<String>,
    find: Vec<String>,
    info: Vec<String>,
    update: Vec<String>,
    upgrade: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum SubCommand {
    ///
    AutoRemove,
    ///
    Clean,
    /// Install specified package(s)
    Install { packages: Vec<String> },
    /// List installed packages
    List,
    ///
    Purge,
    /// Uninstall specified package(s)
    Uninstall { packages: Vec<String> },
    /// Find a package
    Find { package: String },
    /// Get information about a package
    Info { package: String },
    /// Updates packages
    Update,
    /// Upgrade specific package(s)
    Upgrade { packages: Vec<String> },
}

impl PackageManager {
    pub fn execute_command(mut command: Command) {
        command.status().expect("failed to execute process");
    }

    pub fn generate_command(package_manager: PackageManager, subcommand: SubCommand) -> Command {
        let mut command = Command::new(&package_manager.path);
        let args =
            self::PackageManager::map_command_to_package_manager(package_manager, subcommand);
        for arg in args {
            command.arg(arg);
        }
        return command;
    }

    // function for mapping the command to the package manager
    fn map_command_to_package_manager(
        package_manager: PackageManager,
        sub_command: SubCommand,
    ) -> Vec<String> {
        match sub_command {
            SubCommand::AutoRemove => package_manager.auto_remove,
            SubCommand::Clean => package_manager.clean,
            SubCommand::Install { packages } => combine_vectors(package_manager.install, packages),
            SubCommand::List => package_manager.list,
            SubCommand::Purge => package_manager.purge,
            SubCommand::Uninstall { packages } => {
                combine_vectors(package_manager.uninstall, packages)
            }
            SubCommand::Find { package } => combine_vectors(package_manager.find, vec![package]),
            SubCommand::Info { package } => combine_vectors(package_manager.info, vec![package]),
            SubCommand::Update => package_manager.update,
            SubCommand::Upgrade { packages } => combine_vectors(package_manager.upgrade, packages),
        }
    }

    // TODO: Add support for $XDG specifcation
    // TODO: maybe use another format for the package manager config that is not json
    pub fn from_name(name: &str) -> Option<Self> {
        const DATA_DIRS: [&str; 2] = [
            "/home/dyredhead/.local/share/sapm/package_managers/",
            "/usr/share/sapm/package_managers/",
        ];

        for data_dir in DATA_DIRS {
            let package_managers = std::fs::read_dir(PathBuf::from(data_dir)).unwrap();
            for package_manager in package_managers {
                let package_manager = package_manager.unwrap();
                if package_manager.file_name() == (name.to_string() + ".json").as_str() {
                    println!(
                        "Found package manager config: {}",
                        package_manager.path().display()
                    );
                    return Self::from_file(package_manager.path());
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
