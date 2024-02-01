use std::env;
use std::error::Error;
use std::process;
use std::io;

use bad_bets::Action;
use bad_bets::Bet;
use bad_bets::Config;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    println!("Welcome to Bad Bets!");
    let _config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut bets: Vec<Bet> = vec![]; // TODO: Populate bets with bets from file

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
        println!("{:?}", bets);
    }

    Ok(())

}
