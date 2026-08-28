use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "the auspice toolchain")]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    Build {},
    Check {},
    Run {}
}

fn main() {
    let cli = Cli::parse();
}