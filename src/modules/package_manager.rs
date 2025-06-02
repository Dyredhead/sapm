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
        Command::new("sh").arg("-c").arg(command_string).status()
    }

    pub fn match_sapm_subcommand_to_package_manager_command_string(
        package_manager: PackageManager,
        sub_command: SubCommand,
    ) -> String {
        match sub_command {
            SubCommand::Info { package } => package_manager.info + " " + &package,
            SubCommand::Install { packages } => {
                package_manager.install + " " + &(packages.join(" "))
            }
            SubCommand::List => package_manager.list,
            SubCommand::Search { package } => package_manager.search + " " + &package,
            SubCommand::Uninstall { packages } => {
                package_manager.uninstall + " " + &(packages.join(" "))
            }
            SubCommand::Update => package_manager.update,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let package_manager_directories = [
            dirs::config_dir().unwrap().join("sapm/package_managers"),
            dirs::data_dir().unwrap().join("sapm/package_managers"),
            PathBuf::from("/etc/sapm/package_managers"),
            PathBuf::from("/usr/share/sapm/package_managers"),
            PathBuf::from("/usr/share/sapm/vendor_package_managers.d"),
        ];
        for directory in package_manager_directories {
            if let Ok(package_managers) = std::fs::read_dir(directory) {
                for package_manager in package_managers {
                    let package_manager = package_manager.unwrap();
                    if package_manager.file_name() == (name.to_string() + ".toml").as_str() {
                        return Self::from_file(package_manager.path());
                    }
                }
            }
        }
        None
    }
    fn from_file(path: std::path::PathBuf) -> Option<Self> {
        if path.is_file() {
            let toml = std::fs::read_to_string(&path).unwrap();
            let package_manager: PackageManager = toml::from_str(&toml)
                .unwrap_or_else(|_| panic!("{:?} is incorrectly formatted", path));
            return Some(package_manager);
        }
        None
    }
}
