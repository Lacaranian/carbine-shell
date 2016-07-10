use std::path::PathBuf;
use std::env;

use constants::*;

pub fn exec(args: Vec<String>) -> ShellStatus {
    // Determine where the caller wants to go
    let opt_path: Option<PathBuf> = if args.len() > 1 {
        // Use the first argument as the destination
        match &*args[1] {
            "-"  => unimplemented!(),
            path => {
                // Check if the path contains characters to expand (ie. ~)
                //let key = "HOME";
                //match env::var(key) {
                //    Ok(val) => println!("{}: {:?}", key, val),
                //    Err(e) => println!("couldn't interpret {}: {}", key, e),
                //}
                Some(PathBuf::from(path))
            }
        }
    } else {
        // Assume no argument indicates to go to the home directory
        env::home_dir()
    };

    // Try to navigate to the new path
    let exit_code : usize = match opt_path {
        Some(new_path) => {
            match env::set_current_dir(new_path.as_path()) {
                Ok(_) => 0,
                Err(_) => {
                    println!("cd: {}: No such file or directory", new_path.to_str().unwrap_or("?"));
                    1
                }
            }
        },
        None => {
            println!("cd: Could not parse path");
            1
        }
    };

    ShellStatus::Run(exit_code)
}
