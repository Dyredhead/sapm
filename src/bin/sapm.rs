use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use colored::Colorize;
use sapm::cli::{Config, Label, Message, SubCommand};
use sapm::modules::cli;
use sapm::modules::package_manager;

fn main() -> ExitCode {
    let mut args = cli::Cli::parse();

    let config = toml::from_str::<Config>(
        &fs::read_to_string(PathBuf::from("/etc/sapm/conf.toml")).unwrap(),
    )
    .unwrap();

    if args.package_manager == "all" && args.sub_command == SubCommand::Update {
        if let Some(all_package_managers) {

        } else {
            
        }
        for package_manager in config.all_package_managers {
            args.package_manager = package_manager.to_string();
            if args.dry_run {
                print_cmd(&args)
            } else {
                run_cmd(&args);
            }
        }
    } else {
        if args.dry_run {
            print_cmd(&args)
        } else {
            run_cmd(&args);
        }
    }
    return ExitCode::SUCCESS;
}

fn run_cmd(args: &cli::Cli) -> ExitCode {
    let package_manager =
        package_manager::PackageManager::from_name(&args.package_manager).expect(&format!(
            "{} {}: {}",
            "[ERROR]".red().bold(),
            "No such package manager".white().bold(),
            &args.package_manager.yellow(),
        ));

    let command_string =
        package_manager::PackageManager::match_sapm_subcommand_to_package_manager_command_string(
            package_manager,
            args.sub_command.clone(),
        );

    if args.verbose {
        println!(
            "{} {}: `{}`\n",
            "[INFO]".green().bold(),
            "SAPM will execute".white().bold(),
            &command_string.yellow(),
        );
    }

    let status = package_manager::PackageManager::execute_command(&command_string).unwrap();

    if args.verbose {
        if status.success() {
            println!(
                "{} {}: `{}`",
                "[INFO]".green().bold(),
                "SAPM Sucessfully executed".white().bold(),
                &command_string.yellow(),
            );
        } else {
            println!(
                "\n{} {}: `{}`",
                "[ERROR]".red().bold(),
                "SAPM failed to execute".white().bold(),
                &command_string.yellow(),
            );
        }
    }
    if status.success() {
        return ExitCode::SUCCESS;
    } else {
        return ExitCode::FAILURE;
    }
}

fn print_cmd(args: &cli::Cli) {
    Message::Error("No such package manager", &args.package_manager);
    let package_manager =
        package_manager::PackageManager::from_name(&args.package_manager).expect(&format!(
            "{} {}: {}",
            "[ERROR]".red().bold(),
            "No such package manager".white().bold(),
            &args.package_manager.yellow(),
        ));

    let command_string =
        package_manager::PackageManager::match_sapm_subcommand_to_package_manager_command_string(
            package_manager,
            args.sub_command.clone(),
        );
    
    let str = Message(Label::Info, "SAPM will execute", &command_string);
    println!("{}", );
}
