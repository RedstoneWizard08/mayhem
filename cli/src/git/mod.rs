pub mod util;
pub mod clone;

use std::path::PathBuf;

use git2::Progress;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(name = "url")]
    pub arg_url: String,
    #[structopt(name = "path")]
    pub arg_path: String,
}

pub struct State {
    pub progress: Option<Progress<'static>>,
    pub total: usize,
    pub current: usize,
    pub path: Option<PathBuf>,
    pub newline: bool,
}
