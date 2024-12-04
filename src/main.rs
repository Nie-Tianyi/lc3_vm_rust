use structopt::StructOpt;
use termios::*;

mod lc3_vm;
use lc3_vm::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf, // The path to the file to read
}

fn main() {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();

    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode

    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    let cli = Cli::from_args();

    let mut vm = LC3VM::new();
    vm.load(cli.path);
    vm.execute();

    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
