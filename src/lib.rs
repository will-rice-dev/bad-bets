mod teams;

use std::{error::Error, io};
use chrono::NaiveDate;

use teams::{validate_teams, Team};

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2  {
            return Err("No file path given");
        }
        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

pub enum Action {
    AddBet,
    ListBets,
    SettleBets,
    Continue,
    Quit,
}

impl Action {
    pub fn from_str(action_str: &str) -> Action {
        match action_str.trim().to_lowercase().as_str() {
            "add" | "a" => Action::AddBet,
            "list" | "ls" | "l" => Action::ListBets,
            "settle" | "s" => Action::SettleBets,
            "quit" | "q" => Action::Quit,
            _ => Action::Continue,
        }
    }
}

#[derive(Debug)]
pub enum BetType {
    HeadToHead,
    FutureOver,
    FutureUnder,
}

#[derive(Debug)]
pub struct Bet {
    pub bet_type: BetType,
    pub team_for: Option<Team>,
    pub team_against: Option<Team>,
    pub odds: i32,
    pub bet_amount: f64,
    pub date_placed: NaiveDate,
    pub date_settled: NaiveDate,
}

impl Bet {
    pub fn create_from_cl() -> Result<Bet, Box<dyn Error>>{
        let bet_type;
        let team_for;
        let team_against;
        let odds;
        let bet_amount;
        let date_placed;
        let date_settled;
        
        loop {
            println!("Game, Over team wins, or Under team wins? (g, o, or u");
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
                BetType::FutureOver => (Some(get_team_from_cl("Team over?").unwrap()), None),
                BetType::FutureUnder => (None, Some(get_team_from_cl("Team under?").unwrap())),
                BetType::HeadToHead => {
                    let team1 = get_team_from_cl("Team betting on?").unwrap();
                    let team2 = get_team_from_cl("Team betting against?").unwrap();
                    if !validate_teams(&team1, &team2) {
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
                Ok(num) => num,
                Err(_) => continue,
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
        date_placed = get_date_from_cl("Date placed?").unwrap();
        date_settled = get_date_from_cl("Date to be settled (or already settled)?").unwrap();
        Ok(Bet {
            bet_type, team_for, team_against, odds, bet_amount, date_placed, date_settled,
        })
    }

    
}

fn get_team_from_cl(message: &str) -> Result<Team, Box<dyn Error>> {
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

fn get_date_from_cl(message: &str) ->Result<NaiveDate, io::Error> {
    loop {
        println!("{}", message);
        println!("Give date in mm/dd/yyyy format");
        let mut date = String::new();
        io::stdin().read_line(&mut date)?;
        let mut date_split = date.trim().split('/');
        let month: u32 = match date_split.next().ok_or(get_invalid_date_error())?.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing month");
                continue;
            },
        };
        let day: u32 = match date_split.next().ok_or(get_invalid_date_error())?.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing day");
                continue;
            },
        };
        let year: i32 = match date_split.next().ok_or(get_invalid_date_error())?.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing day");
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

