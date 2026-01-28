use clap::Parser;
use std::process::exit;
use vnp4rs::cli::Cli;
use vnp4rs::cli::Commands;
use vnp4rs::logging;
use vnp4rs::server::process::RunServerProcess;
use vnp4rs::target::process::GenerateTargetConfigProcess;

fn main() {
    let cli = Cli::parse();
    logging::init_logging(cli.log_json, cli.log_level);
    match &cli.command {
        Commands::RunServerCommand { config_file } => {
            if let Err(err) = RunServerProcess::new(config_file.clone()).execute() {
                logging::chain_error("Failed to run server ", &err);
                exit(1);
            }
        }
        Commands::GenerateTargetConfigCommand {
            library_file,
            program_file,
            target_name,
        } => {
            if let Err(err) = GenerateTargetConfigProcess::new(library_file.clone(), program_file.clone(), target_name.clone()).execute() {
                logging::chain_error("Failed to generate target", &err);
                exit(1);
            }
        }
    }
}
