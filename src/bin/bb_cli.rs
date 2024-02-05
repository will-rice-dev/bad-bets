use std::collections::BinaryHeap;
use std::{fs, process};
use std::{error::Error, io};

use bad_bets::{Action, Profile, Team};
use bad_bets::{Bet, BetType};
use chrono::{Local, NaiveDate};

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Bad Bets?");
    let args: Vec<String> = std::env::args().collect();
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
            let bets_str = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
                eprintln!("Problem reading file: {}", err.to_string() );
                println!("Create {} as new file? (Y/n)", config.file_path);
                let mut create_file = String::new();
                io::stdin().read_line(&mut create_file).unwrap();
                match create_file.to_lowercase().trim() {
                    "y" | "yes" => {
                        println!("Name?");
                        let mut new_name = String::new();
                        io::stdin().read_line(&mut new_name).unwrap();
                        let new_profile = Profile {
                            name: new_name,
                            bets_outstanding: BinaryHeap::new(),
                            bets_settled: BinaryHeap::new()
                        };
                        serde_json::to_string(&new_profile).unwrap()
                    }
                    _ => process::exit(1),
                }

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
                let new_bet = create_bet_from_cli()?;
                if new_bet.won.is_some() {
                    profile.bets_settled.push(new_bet);
                } else {
                    profile.bets_outstanding.push(new_bet);
                }
                
            },
            Action::SettleBets => settle_bets_cli(&mut profile)?,
            Action::ListBets => println!("{}", list_bets(&profile)),
        }
    }

    println!("Save? (Y/n)");
    let mut save = String::new();
    io::stdin().read_line(&mut save)?;
    match save.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            let json = serde_json::to_string(&profile)?;
            fs::write(&config.file_path, json)?;
        }
        _ => ()
    }
    Ok(())
}

pub struct Config {
    pub file_path: String,
    pub is_new: bool,
}

impl Config {
    pub fn new_from_cli(args: &[String]) -> Config {
        if args.len() == 2  {
            let file_path = args[1].clone();
            return Config { file_path, is_new: false }
        }
        println!("Creating new profile. Will save to temp.json");
        let file_path = String::from("temp.json");
        Config {file_path, is_new: true}
    }
}

fn list_bets(profile: &Profile) -> String {
    let mut out = String::from("Outstanding Bets\n");
    out.push_str(serde_json::to_string_pretty(&profile.bets_outstanding)
                                    .unwrap_or("None".to_string()).as_str());
    out.push_str("\nSettled Bets\n");
    out.push_str(serde_json::to_string_pretty(&profile.bets_settled)
                                    .unwrap_or("None".to_string()).as_str());
    out
}

fn settle_bets_cli(profile: &mut Profile) -> Result<(), Box<dyn Error>> {
    let current_date: NaiveDate = Local::now().date_naive();
    loop {
        let bet = profile.bets_outstanding.peek();
        if bet.is_none() {
            break;
        }
        let bet = bet.unwrap();
        let dif = current_date - bet.date_settled;
        let is_today;
        if dif.num_seconds() > bad_bets::SECONDS_IN_DAY {
            is_today = false;
        } else if dif.num_seconds() > 0 {
            is_today = true;
        } else {
            println!("No more bets to settle!\n");
            break;
        }
        if is_today {
            if yes_or_no(format!("Has this bet settled yet: {:?}", bet).as_str())? {
                if yes_or_no("Did the bet win?")? {
                    profile.bets_settled.push(Bet {
                        won: Some(true),
                        ..*bet
                    });
                } else {
                    profile.bets_settled.push(Bet {
                        won: Some(false),
                        ..*bet
                    });
                }
            } else {
                println!("No more bets to settle!"); // TODO: This is not always true so reformat
                return Ok(())
            }
        } else {
            if yes_or_no(format!("Did this bet win: {:?}", bet).as_str())? {
                profile.bets_settled.push(Bet {
                    won: Some(true),
                    ..*bet
                });
            } else {
                profile.bets_settled.push(Bet {
                    won: Some(false),
                    ..*bet
                });
            }
        }
        profile.bets_outstanding.pop();
    }
    Ok(())
}

fn yes_or_no(message: &str) -> Result<bool, Box<dyn Error>>{
    println!("{} (Y/n)", message);
    let mut ans = String::new();
    io::stdin().read_line(&mut ans)?;
    match ans.to_lowercase().trim() {
        "y" | "yes" => return Ok(true),
        _ => return Ok(false),
    }
}

fn create_bet_from_cli() -> Result<Bet, Box<dyn Error>>{
    let bet_type;
    let team_for;
    let team_against;
    let odds;
    let bet_amount;
    let date_placed;
    let date_settled;
    let won;
    
    loop {
        println!("Game, Over team wins, or Under team wins? (g, o, or u)");
        let mut bet_type_str: String = String::new();
        io::stdin().read_line(&mut bet_type_str)?;
        match bet_type_str.trim().to_lowercase().as_str() {
            "o" | "over" => bet_type = BetType::FutureOver,
            "u" | "under" => bet_type = BetType::FutureUnder,
            "g" | "game" => bet_type = BetType::HeadToHead,
            _ => continue,
        };
        break;
    }
    loop {
        (team_for, team_against) = match bet_type {
            BetType::FutureOver => (Some(get_team_from_cli("Team over?").unwrap()), None),
            BetType::FutureUnder => (None, Some(get_team_from_cli("Team under?").unwrap())),
            BetType::HeadToHead => {
                let team1 = get_team_from_cli("Team betting on?").unwrap();
                let team2 = get_team_from_cli("Team betting against?").unwrap();
                if !bad_bets::teams::validate_teams(&team1, &team2) {
                    println!("Teams must be in same league for head to head games");
                    continue;
                }
                (Some(team1), Some(team2))
            },
        };
        break;
    }
    loop {
        println!("Odds? (Use American odds ie -110 or 200");
        let mut odds_str: String = String::new();
        io::stdin().read_line(&mut odds_str)?;
        odds = match odds_str.trim().parse() {
            Ok(num) =>  {
                if num >= -99 && num <= 99 {
                    eprintln!("Odds cannot be between -99 and 99");
                    continue;
                }
                num
            }
            Err(_) => {
                eprintln!("Odds must be interger");
                continue
            },
        };
        break;
    }
    loop {
        println!("Bet Amount? ");
        let mut amnt_str: String = String::new();
        io::stdin().read_line(&mut amnt_str)?;
        bet_amount = match amnt_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    date_placed = get_date_from_cli("Date placed?").unwrap();
    println!("");
    date_settled = get_date_from_cli("Date to be settled (or already settled)?").unwrap();
    println!("");
    /* loop {
        let current_date: NaiveDate = Local::now().date_naive();
        let dif: Duration = current_date - date_settled;
        if dif.num_seconds() > SECONDS_IN_DAY {
            println!("Did your bet win? (Y/n)");
            let mut won_str = String::new();
            io::stdin().read_line(&mut won_str)?;
            match won_str.trim().to_lowercase().as_str() {
                "y" | "yes" => won = Some(true),
                "n" | "no" => won = Some(false),
                _ => continue,
            }
        } else {
            won = None;
        }
        break;
    } */
    won = None;
    Ok(Bet {
        bet_type, team_for, team_against, odds, bet_amount, date_placed, date_settled, won
    })
}


fn get_team_from_cli(message: &str) -> Result<Team, Box<dyn Error>> {
    loop {
        println!("{}", message);
        let mut team: String = String::new();
        io::stdin().read_line(&mut team)?;
        match Team::from_str(&team.trim()) {
            Some(team) => return Ok(team),
            None => continue,
        }
    }
}

fn get_date_from_cli(message: &str) ->Result<NaiveDate, io::Error> {
    loop {
        println!("{}", message);
        println!("Give date in mm/dd/yyyy or mm/dd/yy format (or enter t for today's date)");
        let mut date = String::new();
        io::stdin().read_line(&mut date)?;
        date = date.trim().to_lowercase();
        if date == "t" || date == "today" {
            return Ok(Local::now().date_naive());
        }
        let mut date_split = date.split('/');
        let month: u32 = match date_split.next().ok_or(get_invalid_date_error())?.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error parsing month");
                continue;
            },
        };
        let day: u32 = match date_split.next().ok_or(get_invalid_date_error())?.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error parsing day");
                continue;
            },
        };
        let year: i32 = match date_split.next().ok_or(get_invalid_date_error())?.parse() {
            Ok(num) => match num {
                0..=99 => num + 2000,
                1900..=2100 => num,
                _ => {
                    eprintln!("Uhh year must be between 0 and 99 or 1900 and 2100. Honestly I probably shouldn't even let bets go to 2100");
                    num
                },
            },
            Err(_) => {
                eprintln!("Error parsing day");
                continue;
            },
        };
        if let Some(out) = NaiveDate::from_ymd_opt(year, month, day) {
            return Ok(out);
        } else {
            continue;
        }
    }
}

fn get_invalid_date_error() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Invalid date format")
}
