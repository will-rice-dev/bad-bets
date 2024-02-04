use std::collections::BinaryHeap;
use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::io;

use bad_bets::Action;
use bad_bets::Bet;
use bad_bets::Config;
use bad_bets::Profile;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    println!("Welcome to Bad Bets!");
    let config = Config::new_from_cli(&args);

    let mut profile: Profile = match config.is_new {
        true => Profile::new_from_cli().unwrap_or(
            Profile {
                                name: "Error creating name".to_string(),
                                bets_outstanding: BinaryHeap::new(),
                                bets_settled: BinaryHeap::new()
                            }
        ),
        false => {
            let bets_str = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
                eprintln!("Problem reading file: {}", err.to_string() );
                process::exit(1);
            });
            serde_json::from_str(&bets_str)?
        }
    };

    loop {
        println!("Please input your action (Add, List, Settle, Quit):");
        let mut action = String::new();
        io::stdin().read_line(&mut action)?;

        let action: Action = Action::from_str(&action);
        match action {
            Action::Continue => continue,
            Action::Quit => break,
            Action::AddBet => {
                let new_bet = Bet::create_from_cli()?;
                if new_bet.won.is_some() {
                    profile.bets_settled.push(new_bet);
                } else {
                    profile.bets_outstanding.push(new_bet);
                }
                
            },
            _ => continue,
        }
    }

    println!("Save? (Y/n)");
    let mut save = String::new();
    io::stdin().read_line(&mut save)?;
    match save.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            let json = serde_json::to_string(&profile)?;
            fs::write("temp.json", json)?;
        }
        _ => ()
    }
    
    Ok(())

}
