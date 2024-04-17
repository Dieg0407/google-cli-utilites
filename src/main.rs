use std::process;
use clap::Parser;
use gcli::Gcli;

fn main() {
    let cli = Gcli::parse();
    let exit_code = cli.process();

    process::exit(exit_code);
}
