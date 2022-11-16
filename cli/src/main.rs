pub mod git;
pub mod parser;
pub mod commands;

use clap::Parser;
use commands::init::dispatch_init;
use parser::{Arguments, Commands, InitArgs};
use path_absolutize::Absolutize;

pub static CONFIG_FILE: &str = "Mayhem.toml";
pub static REPO_URL: &str = "https://github.com/RedstoneWizard08/mayhem.git";

pub fn main() {
    let cli = Arguments::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = cli.config.as_deref() {
        let path = config_path.absolutize().unwrap();
        println!("Value for config: {}", path.to_str().unwrap());
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Init { force_overwrite, directory }) => {
            dispatch_init(InitArgs {
                force_overwrite: force_overwrite.clone(),
                directory: directory.clone(),
            });
        }
        None => {},
    }
}
