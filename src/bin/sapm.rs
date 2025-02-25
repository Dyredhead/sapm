use std::process::ExitCode;

use clap::Parser;

use sapm::cli::{Config, Label, Message, SubCommand};
use sapm::modules::cli;
use sapm::modules::package_manager;

fn main() -> ExitCode {
    let mut args = cli::Cli::parse();
    let config = Config::parse();

    if args.package_manager == "all" && args.sub_command == SubCommand::Update {
        let all_package_managers = (config.all_package_managers).expect(
            &Message::new(
                Label::Error,
                "Config field not found",
                "all_package_managers",
            )
            .to_string(),
        );

        for package_manager in all_package_managers {
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
    let package_manager = package_manager::PackageManager::from_name(&args.package_manager).expect(
        &Message::new(
            Label::Error,
            "No such package manager",
            &args.package_manager,
        )
        .to_string(),
    );

    let command_string =
        package_manager::PackageManager::match_sapm_subcommand_to_package_manager_command_string(
            package_manager,
            args.sub_command.clone(),
        );

    if args.verbose {
        Message::printmsg(Message::new(
            Label::Info,
            "SAPM will execute",
            &command_string,
        ));
    }

    let status = package_manager::PackageManager::execute_command(&command_string).unwrap();

    if args.verbose {
        if status.success() {
            Message::printmsg(Message::new(
                Label::Info,
                "SAPM Sucessfully executed",
                &command_string,
            ));
        } else {
            Message::printmsg(Message::new(
                Label::Error,
                "SAPM failed to execute",
                &command_string,
            ));
        }
    }
    if status.success() {
        return ExitCode::SUCCESS;
    } else {
        return ExitCode::FAILURE;
    }
}

fn print_cmd(args: &cli::Cli) {
    let package_manager = package_manager::PackageManager::from_name(&args.package_manager).expect(
        &Message::new(
            Label::Error,
            "No such package manager",
            &args.package_manager,
        )
        .to_string(),
    );

    let command_string =
        package_manager::PackageManager::match_sapm_subcommand_to_package_manager_command_string(
            package_manager,
            args.sub_command.clone(),
        );

    Message::printmsg(Message::new(
        Label::Info,
        "SAPM will execute",
        &command_string,
    ));
}
