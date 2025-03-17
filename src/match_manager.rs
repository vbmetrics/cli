use std::io::{self, Write};
use parser::parser::parse_line;

pub fn start_new_match() {
    println!("=== Rozpoczynamy nowy mecz ===");

    let home_team: String = choose_team("Wybierz gospodarzy (L): ");
    let away_team: String = choose_team("Wybierz gości (O): ");

    println!("Mecz rozpoczęty! {} vs {}", home_team, away_team);    // TODO: use translations, check teams, save teams info

    match_loop();
}

fn choose_team(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let team = input.trim().to_uppercase();

        if !team.is_empty() {
            return team;
        }
        println!("Nieprawidłowy wybór, spróbuj ponownie.");
    }
}

fn match_loop() {
    println!("Wpisz akcje lub 'exit' aby zakończyć mecz.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Zamykanie...");
            break;
        }
        
        let parsed: parser::parser::ParsedAction = parse_line(input);

        println!("{:?}", parsed.events[0]);    // TODO: use match instead (?)       
    }
}
