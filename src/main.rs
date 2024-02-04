use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::io;

use bad_bets::Action;
use bad_bets::Bet;
use bad_bets::Config;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    println!("Welcome to Bad Bets!");
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let bets_str = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err.to_string() );
        process::exit(1);
    });
    let mut bets: Vec<Bet> = serde_json::from_str(&bets_str)?;

    loop {
        println!("Please input your action (Add, List, Settle, Quit):");
        let mut action = String::new();
        io::stdin().read_line(&mut action)?;

        let action: Action = Action::from_str(&action);
        match action {
            Action::Continue => continue,
            Action::Quit => break,
            Action::AddBet => {
                bets.push(Bet::create_from_cl()?);
            },
            _ => continue,
        }
    }

    println!("Save? (Y/n)");
    let mut save = String::new();
    io::stdin().read_line(&mut save)?;
    match save.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            let json = serde_json::to_string(&bets)?;
            fs::write("temp.json", json)?;
        }
        _ => ()
    }
    
    Ok(())

}
