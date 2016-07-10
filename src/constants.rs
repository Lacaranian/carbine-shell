pub const DIRECTORY_STACK_SIZE : usize = 10;

#[derive(PartialEq,Eq)]
pub enum ShellStatus {
    Run(usize),
    Quit(usize)
}

impl ShellStatus {
    pub fn exit_code(&self) -> usize {
        match self {
            &ShellStatus::Run(num) => num,
            &ShellStatus::Quit(num) => num
        }
    }
}
