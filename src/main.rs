use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufRead, BufReader};
use tikh_email_parser::EmailAddress;

#[derive(Parser)]
#[command(name = "tikh_email_parser")]
#[command(about = "CLI-додаток для парсингу електронних адрес.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        input_file: String,
    },
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { input_file } => {
            let file = File::open(input_file)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                match EmailAddress::parse(&line) {
                    Ok(email) => println!(
                        "Email Правильний! Локальна частина: {} Домен: {}",
                        email.local, email.domain_part
                    ),
                    Err(e) => eprintln!("Помилка! '{}': {}", line, e),
                }
            }
        }
        Commands::Credits => {
            println!("tikh_email_parser by Tikhon Ishchenko.");
        }
    }

    Ok(())
}
