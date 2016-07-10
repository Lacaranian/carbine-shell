use constants::*;

pub fn exec(args: Vec<String>, status: ShellStatus) -> ShellStatus {
    let mut good_exit = true;

    let exit_code: usize = if args.len() > 1 {
        match args[1].parse() {
            Ok(num)  => num,
            Err(_) => {
                println!("Argument needs to be a numeric exit status");
                good_exit = false;
                1
            }
        }
    } else {
        status.exit_code()
    };

    if good_exit {
        ShellStatus::Quit(exit_code)
    } else {
        ShellStatus::Run(exit_code)
    }
}
