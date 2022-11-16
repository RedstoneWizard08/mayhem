use std::{path::{PathBuf, Path}, process::exit};

use path_absolutize::Absolutize;
use question::{Question, Answer};

use crate::{parser::InitArgs, git::{clone::clone_repository, Args}};

fn check_ok(dir: &Path) -> bool {
    if !dir.exists() {
        return true;
    }
    
    if dir.is_file() {
        return false;
    }

    let contents = dir.read_dir().unwrap();

    if contents.count() == 0 {
        return true;
    }

    return false;
}

pub fn dispatch_init(args: InitArgs) {
    let binding = args.directory.unwrap_or(PathBuf::from(".")).absolutize().unwrap().into_owned();
    let dir = binding.as_path();

    let is_ok = check_ok(dir);

    if !is_ok {
        let q = Question::new("Directory not empty! Continue? (y/n)")
            .yes_no()
            .until_acceptable()
            .ask().unwrap();
        
        if q == Answer::NO {
            println!("Aborting!");
            exit(0);
        }
    }

    println!("Cloning RedstoneWizard08/mayhem...");

    let repo_url = "https://github.com/RedstoneWizard08/mayhem";

    match clone_repository(&Args {
        arg_path: dir.to_str().unwrap().to_string(),
        arg_url: repo_url.to_string(),
    }) {
        Ok(_) => println!("Done!"),
        Err(_) => panic!("Unable to clone repo!"),
    }
}
