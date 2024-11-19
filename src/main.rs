use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufRead, BufReader};
use tikh_email_parser::EmailAddress;

#[derive(Parser)]
#[command(
    name = "tikh_email_parser",
    about = "CLI-додаток для парсингу електронних адрес.",
    long_about = "tikh_email_parser — це утиліта для парсингу електронних адрес. Ви можете:\n\
        - Перевіряти правильність електронних адрес з файлу.\n\
        - Дізнатися більше про автора програми.\n\n\
        Приклади використання:\n\
        - Перевірка файлу з адресами: `tikh_email_parser parse addresses.txt`\n\
        - Показати автора програми: `tikh_email_parser credits`",
    help_template = "\
{name} {version}\n\
{author}\n\
{about}\n\n\
Використання:\n    {usage}\n\n\
Команди:\n{all-args}"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Парсинг файлу з електронними адресами для перевірки кожного рядка.
    #[command(
        about = "Перевірити файл, що містить електронні адреси.",
        long_about = "Команда `parse` дозволяє перевірити файл, що містить електронні адреси. \
                      Кожен рядок у файлі буде проаналізовано на відповідність стандартам формату електронної адреси."
    )]
    Parse {
        /// Шлях до файлу, що містить електронні адреси для парсингу.
        #[arg(help = "Шлях до файлу для перевірки.")]
        input_file: String,
    },
    /// Інформація про автора програми.
    #[command(
        about = "Дізнайтеся більше про автора.",
        long_about = "Команда `credits` показує інформацію про автора програми — Tikhon Ishchenko."
    )]
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
                        "Email правильний! Локальна частина: {} Домен: {}",
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
