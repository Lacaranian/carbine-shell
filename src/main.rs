#![feature(io)]
extern crate libc;
extern crate shlex;

mod builtins;
mod constants;

use std::ptr;
use std::str;
use std::io::*;
use std::path::*;
use std::env;
use std::ffi::CString;

use libc::{c_char, c_int, execvp, fork, waitpid, WIFEXITED, WIFSIGNALED, WIFSTOPPED, WEXITSTATUS, WTERMSIG, WSTOPSIG};

use constants::*;

fn main() {
    // Establish the initial directory of the shell
    if env::current_dir().is_err() {
        let init_dir = match env::home_dir() {
            Some(dir) => dir,
            None      => PathBuf::from("/")
        };
        if env::set_current_dir(init_dir).is_err() {
            panic!("Couldn't establish initial working directory!");
        }
    };

    // Use that initial directory to begin the directory stack
    let init_dir = env::current_dir().unwrap();
    let mut directory_stack : Vec<PathBuf> = Vec::with_capacity(DIRECTORY_STACK_SIZE);
    directory_stack.push(init_dir);

    let mut status : ShellStatus = ShellStatus::Run(0);

    loop {
        print_prompt();

        let (cmd, args) = read_input();
        //println!("CMD: {}, ARGS: {:?}", cmd, args);

        status = match cmd.as_ref() {
            // Built-in commands
            "cd"   => builtins::cd::exec(args),
            "exit" => builtins::exit::exec(args, status),
            // Other commands found in the PATH environment variable
            _      => run_command(cmd, args)
        };

        if let ShellStatus::Quit(_) = status {
            break;
        }
    }

    std::process::exit(status.exit_code() as i32)
}

fn print_prompt() {
    let directory : PathBuf = env::current_dir().unwrap();

    print!("{} $ ", directory.as_path().to_string_lossy());
}

fn read_input() -> (String, Vec<String>) {
    // Flush to make sure stdout is printed immediately
    let mut stdout = stdout();
    stdout.flush().unwrap();

    let mut stdin = stdin();
    let reader = BufReader::new(&mut stdin);

    let mut line: String = "".to_string();
    for opt_char in reader.chars() {
        let character = match opt_char {
            Ok(character) => character,
            Err(e)   => panic!("Couldn't read character: {}", e)
        };
        // Special characters
        match character {
            '\n' => break,
            _    => line.push(character)
        }
    }

    // Parse paramaters shell style
    let params: Vec<String> = shlex::split(&line).unwrap_or(vec![]);
    let cmd : String = match params.first() {
        Some(cmd) => cmd.clone(),
        None      => String::from("")
    };

    (cmd, params)
}

fn run_command(command: String, args: Vec<String>) -> ShellStatus {
    let exit_code : usize;
    unsafe {
        let pid = fork();
        match pid {
            0 => { // This is the child process, replace the shell code with the specified program using execvp
                let cmd : *const c_char = CString::new(command).unwrap().as_ptr();
                let mut cmd_arg_vec : Vec<*const c_char>  = args.into_iter()
                    .map(|string| CString::new(string).unwrap().as_ptr())
                    .collect();
                cmd_arg_vec.push(ptr::null());
                let cmd_args : *const *const c_char = cmd_arg_vec.as_ptr();
                let err_code = execvp(cmd, cmd_args);
                libc::exit(err_code); // We only reach this point if execvp failed, in which case exit with the error
            },
            child_pid => { // This is the parent process - wait for the child to finish
                loop {
                    let status : *mut c_int = &mut 0;
                    let _ = waitpid(child_pid, status, 0);
                    // Handle each exit code type
                    if WIFEXITED(*status) {
                        exit_code = WEXITSTATUS(*status) as usize;
                        break;
                    } else if WIFSIGNALED(*status) {
                        exit_code = WTERMSIG(*status) as usize;
                        break;
                    } else if WIFSTOPPED(*status) {
                        exit_code = WSTOPSIG(*status) as usize;
                        break;
                    }
                }
            }
        };
    }
    // TODO: Use the real exit code
    ShellStatus::Run(exit_code)
}
