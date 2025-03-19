use std::fs::OpenOptions;
use std::io::{self, Write};
use sqlx::postgres::PgPool;
use parser::parser::parse_line;

pub async fn start_new_match(pool: &PgPool) {
    println!("=== Rozpoczynamy nowy mecz ===");

    let teams = sqlx::query!(
        "SELECT id, name FROM teams;"
    )
    .fetch_all(pool)
    .await
    .expect("Błąd pobierania danych z bazy.");

    for team in &teams {
        println!("- {} (ID: {})", team.name, team.id)
    }

    let home_team: String = match choose_team_from_db(pool, "Wybierz gospodarzy (L): ").await {
        Some(team) => team,
        None => {
            println!("Nie można znaleźć drużyny. Kończenie meczu...");
            return;
        }
    };

    let away_team: String = match choose_team_from_db(pool, "Wybierz gości (O): ").await {
        Some(team) => team,
        None => {
            println!("Nie można znaleźć drużyny. Kończenie meczu...");
            return;
        }
    };

    println!("Mecz rozpoczęty! {} vs {}", home_team, away_team);

    match_loop(&home_team, &away_team);
}

pub async fn choose_team_from_db(pool: &PgPool, prompt: &str) -> Option<String> {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let team_id: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Nieprawidłowy wybór, spróbuj ponownie.");
                continue;
            }
        };

        let team = sqlx::query!(
            "SELECT name FROM teams WHERE id = $1;",
            team_id
        )
        .fetch_optional(pool)
        .await
        .unwrap_or_else(|_| {
            println!("Błąd pobierania danych z bazy.");
            None
        });

        match team {
            Some(t) => return Some(t.name),
            None => println!("Nie znaleziono drużyny o podanym ID. Spróbuj ponownie."),
        }
    }
}


pub fn match_loop(home_team: &str, away_team: &str) {
    println!("Wpisz akcje lub 'exit' aby zakończyć mecz.");

    let mut file: std::fs::File = OpenOptions::new()
        .create(true)
        .append(true)
        .open("match_actions.log")
        .unwrap();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: &str = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Zamykanie meczu...");
            break;
        }

        let _parsed: parser::parser::ParsedAction = parse_line(input);

        // Zapisz akcję do pliku
        if let Err(e) = writeln!(file, "{} vs {}: {}", home_team, away_team, input) {
            eprintln!("Błąd podczas zapisywania do pliku: {}", e);
        }
        
        println!("Akcja zapisana: {}", input);       
    }
}
