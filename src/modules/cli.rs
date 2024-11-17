use clap::{Parser, Subcommand};
use colored::Colorize;
use os_release;

#[derive(Parser, Debug)]
#[clap(
    author = "Dyredhead",
    version = "1.0.0",
    about = format!("{}\n{}", "A System Agnostic Package Manager (SAPM) which provides basic but useful functionality.".bold(), "Should NOT be run as root, as it will prompt for sudo password if needed"),
    long_about = None,
    arg_required_else_help = true,
)]

pub struct Cli {
    /// Use the specified package manager instead of the default
    #[clap(long = "package-manager", default_value_t = get_default_package_manager(), env = "SAPM_DEFAULT_PACKAGE_MANAGER", visible_alias = "pm")]
    pub package_manager: String,

    /// Show the command that SAPM will execute
    #[clap(long = "verbose", short = 'v')]
    pub verbose: bool,

    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

fn get_default_package_manager() -> String {
    let os_release = os_release::OsRelease::new().unwrap();
    let distro = os_release.id.as_str();

    // Based of off https://github.com/chef/os_release
    let default_package_manager = match distro {
        "cumulus-linux" | "debian" | "elementary" | "kali" | "linuxmint" | "pop" | "raspbian"
        | "ubuntu" => "apt",

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
        SubCommand::Search { package: _ } => vec!["search"],
        SubCommand::Uninstall { packages: _ } => vec!["remove"],
        SubCommand::Update {} => vec!["upgrade"],
    };
    return aliases;
}
