use termios::*;
use clap::Parser;
mod lc3_vm;
use lc3_vm::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[arg(short, long, help = "path to the assembly file")]
    path: String
}

fn main() {
    let stdin = 0;
    let mut termios = Termios::from_fd(stdin).unwrap();

    termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode

    tcsetattr(stdin, TCSANOW, &termios).unwrap();

    let cli = Cli::parse();

    let mut vm = LC3VM::new();
    vm.load(cli.path);
    vm.execute();

    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
