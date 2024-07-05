use crate::package_manager;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    /// Use the specified package manager instead of the default
    #[clap(long, default_value = "default", env = "SAPM_DEFAULT_PACKAGE_MANAGER")]
    pub package_manager: String,

    #[clap(subcommand)]
    pub sub_command: package_manager::SubCommand,
}
