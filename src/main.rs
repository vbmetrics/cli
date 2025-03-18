use clap::{Parser, Subcommand};
use std::io::{self, Write};
use sqlx::postgres::PgPoolOptions;
use tokio;

mod match_manager;

#[derive(Parser)]
#[command(name = "Volleyball CLI")]
#[command(about = "CLI do zarządzania meczami siatkarskimi")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    NewMatch,
    Exit,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url: &str = "postgres://vbm:password@localhost:5432/vbmetrics-test";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    loop {
        println!("\n=== Volleyball CLI ===");
        println!("1. Nowy mecz");
        println!("2. Wyjdź");
        print!("Wybierz opcję: ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: &str = input.trim();

        match input {
            "1" => {
                match_manager::start_new_match(&pool).await;
            }
            "2" => {
                println!("Zamykanie...");
                break;
            }
            _ => println!("Nieprawidłowa opcja, spróbuj ponownie."),
        }
    }

    Ok(())
}
