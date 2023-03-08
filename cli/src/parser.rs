use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(name = "MayhemCTL")]
#[command(author = "RedstoneWizard08")]
#[command(version = "0.1.0")]
#[command(about = "A tool to manage a self-hosted instance of Mayhem.", long_about = None)]
pub struct Arguments {
    pub name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init {
        #[arg(short = 'f', long = "force-overwrite")]
        force_overwrite: bool,

        directory: Option<PathBuf>,
    },
}

pub struct InitArgs {
    pub force_overwrite: bool,

    pub directory: Option<PathBuf>,
}
