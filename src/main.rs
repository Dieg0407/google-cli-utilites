use clap::Parser;
use gcli::Gcli;

fn main() {
    let cli = Gcli::parse();
    cli.process();
}
