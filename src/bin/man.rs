use clap::CommandFactory;
use clap_mangen::generate_to;

use sapm::modules::cli;

fn main() {
    let cmd = cli::Cli::command();
    let _ = generate_to(cmd, "./data/local/man");
}
