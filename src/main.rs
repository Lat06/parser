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
        Commands::Credits => {
            println!("Tasklist Parser");
            println!("Made for Rust course.");
            println!("Source code available on GitHub.");
        }
        Commands::Parse { file } => {
            let content = std::fs::read_to_string(file)
                .with_context(|| format!("Could not read file `{:?}`", file))?;

            let tasks = parse_tasks(&content)?;

            println!("--- Task Breakdown ---");
            for (index, task) in tasks.iter().enumerate() {
                let status_char = if task.status == TaskStatus::Done {
                    'x'
                } else {
                    ' '
                };
                let priority_info = task
                    .priority
                    .as_ref()
                    .map(|p| format!(" ({})", p))
                    .unwrap_or_default();
                let tags_info = if task.tags.is_empty() {
                    String::new()
                } else {
                    format!(" #{}", task.tags.join(" #"))
                };

                println!(
                    "Task {}: [{}]{} {}{}",
                    index + 1,
                    status_char,
                    priority_info,
                    task.description,
                    tags_info
                );
            }

            let total = tasks.len();
            let done = tasks
                .iter()
                .filter(|t| t.status == TaskStatus::Done)
                .count();

            println!("\n--- Task Summary ---");
            println!("Total:   {}", total);
            println!("Done:    {}", done);
            println!("Pending: {}", total - done);
        }
    }

    Ok(())
}
