use std::{
    io,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use serde::{Deserialize, Serialize};

use crate::cli::SubCommand;

#[derive(Deserialize, Serialize)]
pub struct PackageManager {
    info: String,
    install: String,
    list: String,
    search: String,
    uninstall: String,
    update: String,
}

impl PackageManager {
    pub fn execute_command(command_string: &str) -> io::Result<ExitStatus> {
        return Command::new("sh").arg("-c").arg(command_string).status();
    }

    pub fn match_sapm_subcommand_to_package_manager_command_string(
        package_manager: PackageManager,
        sub_command: SubCommand,
    ) -> String {
        match sub_command {
            SubCommand::Info { package } => {
                return package_manager.info + " " + &package;
            }
            SubCommand::Install { packages } => {
                return package_manager.install + " " + &(packages.join(" "));
            }
            SubCommand::List => {
                return package_manager.list;
            }
            SubCommand::Search { package } => {
                return package_manager.search + " " + &package;
            }
            SubCommand::Uninstall { packages } => {
                return package_manager.uninstall + " " + &(packages.join(" "));
            }
            SubCommand::Update => {
                return package_manager.update;
            }
        }
    }

    // TODO: maybe use another format for the package manager config that is not json
    pub fn from_name(name: &str) -> Option<Self> {
        let directories = [
            PathBuf::from("/etc/sapm/package_managers"),
            PathBuf::from("/usr/share/sapm/package_managers"),
            PathBuf::from("/usr/share/sapm/vendor_package_managers.d"),
        ];
        for directory in directories {
            if let Ok(package_managers) = std::fs::read_dir(directory) {
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
        let package_manager: PackageManager =
            serde_json::from_str(json).expect("Json is incorrectly formatted");
        return Some(package_manager);
    }
}
