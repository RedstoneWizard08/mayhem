use std::path::PathBuf;

use path_absolutize::Absolutize;

use crate::parser::{Commands, InitArgs};

pub fn dispatch_init(args: InitArgs) {
    let dir = args.directory.unwrap_or(PathBuf::from(".")).absolutize().unwrap().into_owned().as_path();

    if dir.is_file() {
        panic!("Path is not a directory!");
    }

    let contents = dir.read_dir().unwrap();
    
    println!("{}", contents.count());
}
