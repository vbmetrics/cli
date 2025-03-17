use clap::{Parser, Subcommand};
use std::io::{self, Write};
mod match_manager;

#[derive(Parser)]
#[command(name = "Volleyball CLI")]
#[command(about = "CLI do zarządzania meczami siatkarskimi", long_about = None)] // TODO: use translations
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    NewMatch,
    Exit,
}



fn main() {
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
            "1" => match_manager::start_new_match(),
            "2" => {
                println!("Zamykanie...");
                break;
            }
            _ => println!("Nieprawidłowa opcja, spróbuj ponownie."),
        }
    }
}
