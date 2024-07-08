use crate::package_manager;
use clap::Parser;
use os_release;

#[derive(Parser, Debug)]
#[clap(author = "Dyredhead", version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// Use the specified package manager instead of the default
    #[clap(long = "package-manager", default_value_t = get_default_package_manager(), env = "SAPM_DEFAULT_PACKAGE_MANAGER")]
    pub package_manager: String,

    #[clap(subcommand)]
    pub sub_command: package_manager::SubCommand,
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
