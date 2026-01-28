use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

use crate::logging::LogLevel;

#[derive(Parser)]
#[command(name = "vnp4rs")]
#[command(version = "0.1.0")]
#[command(about = "Vitis Networking P4 Runtime Server CLI")]
#[command(long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long)]
    pub log_json: bool,

    #[arg(long, value_enum, default_value = "debug")]
    pub log_level: LogLevel,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "run-server")]
    #[command(alias = "r")]
    #[command(about = "Start the P4 runtime server")]
    RunServerCommand {
        #[arg(value_name = "CONFIG_FILE")]
        config_file: PathBuf,
    },

    #[command(name = "generate-target-config")]
    #[command(alias = "g")]
    #[command(about = "Generate target configuration from library")]
    GenerateTargetConfigCommand {
        #[arg(value_name = "LIBRARY_FILE")]
        library_file: PathBuf,
        #[arg(value_name = "PROGRAM_FILE")]
        program_file: PathBuf,
        #[arg(value_name = "TARGET_NAME")]
        target_name: String,
    },
}
