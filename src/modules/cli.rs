#![allow(clippy::needless_return)]

use std::{
    env,
    fmt::{self, Display},
    path::PathBuf,
    vec,
};

use clap::{Parser, Subcommand};
use colored::Colorize;
use os_release;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(
    author = "Dyredhead",
    version = "1.0.0",
    about = format!("{}\n{}", "A System Agnostic Package Manager (SAPM) which provides basic but useful functionality.".bold(), "Should NOT be run as root, as it will prompt for sudo password if needed"),
    long_about = None,
    arg_required_else_help = true,
)]
#[derive(Clone)]
pub struct Cli {
    /// Use the specified package manager instead of the default
    #[clap(long = "package-manager", default_value_t = get_default_package_manager(), env = "SAPM_DEFAULT_PACKAGE_MANAGER", visible_alias = "pm")]
    pub package_manager: String,

    #[clap(long = "dry-run", short = 'n')]
    pub dry_run: bool,

    /// Show the command that SAPM will execute
    #[clap(long = "verbose", short = 'v')]
    pub verbose: bool,

    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

/// Attempts to get `default_package_manager` from:
/// 1. `env`
/// 2. `conf.toml`
/// 3. huersitics
fn get_default_package_manager() -> String {
    let config = Config::parse();
    if let Ok(default_package_manager) = env::var("SAPM_DEFAULT_PACKAGE_MANAGER") {
        return default_package_manager;
    } else if let Some(default_package_manager) = config.default_package_manager {
        return default_package_manager;
    } else {
        let os_release = os_release::OsRelease::new().unwrap();
        let distro = os_release.id.as_str();

        // Based of off https://github.com/chef/os_release
        let default_package_manager = match distro {
            "cumulus-linux" | "debian" | "elementary" | "kali" | "linuxmint" | "pop"
            | "raspbian" | "ubuntu" => "apt",

            "almalinux" | "amzn" | "centos" | "clearos" | "fedora" | "mageia" | "ol" | "rhel"
            | "rocky" | "scientific" | "virtuozzo" | "xenenterprise" => "dnf",

            "nixos" => "nix",

            "antergos" | "arcolinux" | "arch" | "archarm" | "endeavouros" | "manjaro"
            | "manjaro-arm" => "pacman",

            "gentoo" => "portage",

            "sled" | "suse" => "zypper",

            _ => "",
        };
        return String::from(default_package_manager);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum SubCommand {
    /// Get information about the specified package
    #[clap(visible_aliases = get_aliases_of(SubCommand::Info {package: String::default()}))]
    Info { package: String },

    /// Install the specified package(s)
    #[clap(visible_aliases = get_aliases_of(SubCommand::Install {packages: Vec::default()}))]
    Install { packages: Vec<String> },

    /// List all of the installed packages
    #[clap(visible_aliases = get_aliases_of(SubCommand::List {}))]
    List,

    /// Search for the specified package
    #[clap(visible_aliases = get_aliases_of(SubCommand::Search {package: String::default()}))]
    Search { package: String },

    /// Uninstall the specified package(s)
    #[clap(visible_aliases = get_aliases_of(SubCommand::Uninstall {packages: Vec::default()}))]
    Uninstall { packages: Vec<String> },

    /// Update all packages
    #[clap(visible_aliases = get_aliases_of(SubCommand::Update {}))]
    Update,
}

fn get_aliases_of(sub_command: SubCommand) -> Vec<&'static str> {
    let aliases = match sub_command {
        SubCommand::Info { package: _ } => vec!["show"],
        SubCommand::Install { packages: _ } => vec!["add"],
        SubCommand::List {} => vec!["ls"],
        SubCommand::Search { package: _ } => vec!["find"],
        SubCommand::Uninstall { packages: _ } => vec!["remove"],
        SubCommand::Update {} => vec!["upgrade"],
    };
    return aliases;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub default_package_manager: Option<String>,
    pub all_package_managers: Option<Vec<String>>,
}

impl Config {
    fn new() -> Config {
        return Config {
            default_package_manager: None,
            all_package_managers: None,
        };
    }

    /// Parses `conf.toml` from:
    /// 1. `env`
    /// 2. `$XDG_CONFIG_HOME/sapm/conf.toml`
    /// 3. `/etc/sapm/conf.toml`
    /// 4. `/usr/share/sapm/conf.toml` (it is assumed that this always exist)
    pub fn parse() -> Self {
        let config_files = [
            PathBuf::from(env::var("SAPM_CONFIG_PATH").unwrap_or_default()),
            dirs::config_dir().unwrap().join("sapm/conf.toml"),
            PathBuf::from("/etc/sapm/conf.toml"),
            PathBuf::from("/usr/share/sapm/conf.toml"),
        ];

        let mut config = Self::new();

        for file in config_files {
            if let Some(new_config) = Self::from_file(file) {
                config = config.merge(new_config);
            }
        }

        return config;
    }

    fn merge(self, other: Config) -> Self {
        Self {
            default_package_manager: self
                .default_package_manager
                .or(other.default_package_manager),
            all_package_managers: self.all_package_managers.or(other.all_package_managers),
        }
    }

    fn from_file(path: std::path::PathBuf) -> Option<Self> {
        if let Ok(file) = std::fs::read_to_string(path) {
            return Some(toml::from_str::<Config>(&file).unwrap());
        }
        return None;
    }
}

pub struct Message<'a> {
    label: Label,
    message: &'a str,
    offender: &'a str,
}

impl Message<'_> {
    pub fn new<'a>(label: Label, message: &'a str, offender: &'a str) -> Message<'a> {
        return Message {
            label,
            message,
            offender,
        };
    }

    pub fn printmsg(message: Message) {
        println!("{message}");
    }
}

impl Display for Message<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            self.label,
            self.message.white().bold(),
            self.offender.yellow()
        )
    }
}
pub enum Label {
    Info,
    Error,
}

impl Label {
    fn to_string(&self) -> colored::ColoredString {
        match self {
            Label::Info => "[INFO]".yellow().bold(),
            Label::Error => "[ERROR]".red().bold(),
        }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
