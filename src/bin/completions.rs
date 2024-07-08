use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};

use sapm::modules::cli;

fn main() {
    let _ = std::fs::create_dir_all("./data/local/man");

    let mut cmd = cli::Cli::command();
    for &shell in Shell::value_variants() {
        let _ = generate_to(shell, &mut cmd, "sapm", "./data/local/completions");
    }
}
