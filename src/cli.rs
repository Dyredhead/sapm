use crate::package_manager;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    #[clap(long, default_value = "default", env = "SAPM_DEFAULT_PACKAGE_MANAGER")]
    pub package_manager: String,

    #[clap(subcommand)]
    pub sub_command: package_manager::SubCommand,
    // #[clap(
    //     requires = "sub_command",
    //     default_value = "",
    //     trailing_var_arg = true,
    //     allow_hyphen_values = true,
    //     hide = true
    // )]
    // pub packages: Vec<String>,
}
