mod cli;
mod commands;
mod core;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use ui::banner::show_banner;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            show_banner();
            Ok(())
        }
        Some(cmd) => commands::execute(cmd),
    }
}
