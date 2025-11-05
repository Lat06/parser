use clap::Parser;
use std::path::PathBuf;
use tasklist_parser::{TaskStatus, parse_tasks};

use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Parse {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            let content = std::fs::read_to_string(file)
                .with_context(|| format!("Could not read file `{:?}`", file))?;

            let tasks = parse_tasks(&content)?;

            let total = tasks.len();
            let done = tasks
                .iter()
                .filter(|t| t.status == TaskStatus::Done)
                .count();
            let pending = total - done;

            println!("--- Task Summary ---");
            println!("Total:   {}", total);
            println!("Done:    {}", done);
            println!("Pending: {}", pending);
        }

        Commands::Credits => {
            println!("Tasklist Parser");
            println!("Made by [Ваше Ім'я тут]");
            println!("A simple parser for a Rust course.");
        }
    }

    Ok(())
}
