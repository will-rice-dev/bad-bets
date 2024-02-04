mod teams;

use std::{error::Error, io};
use chrono::{Local, NaiveDate};
use serde::{Serialize, Deserialize};

use teams::{validate_teams, Team};

const SECONDS_IN_DAY: i64 = 86400; // Want to give enough time for 

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub bets_settled: Vec<Bet>,
    pub bets_outstanding: Vec<Bet>,
    pub name: String,
}

impl Profile {
    pub fn new_from_cli() -> Result<Profile, Box<dyn Error>> {
        let mut name = String::new();
        println!("Welcome newcomer! What is your name?");
        io::stdin().read_line(&mut name)?;
        Ok(Profile { name, bets_outstanding: vec![], bets_settled: vec![]})

    }

    pub fn get_outstanding_bets(&self) -> Option<Vec<Bet>> {
        let current_date: NaiveDate = Local::now().date_naive();
        let mut outstanding_bets = vec![];
        for bet in &self.bets_outstanding {
            if bet.won.is_some() { continue; }
            let dif = current_date - bet.date_settled;
            if dif.num_seconds() > SECONDS_IN_DAY {
                outstanding_bets.push(bet);
            }
        }
        
        None
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub enum BetType {
    HeadToHead,
    FutureOver,
    FutureUnder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bet {
    pub bet_type: BetType,
    pub team_for: Option<Team>,
    pub team_against: Option<Team>,
    pub odds: i32,
    pub bet_amount: f64,
    pub date_placed: NaiveDate,
    pub date_settled: NaiveDate,
    pub won: Option<bool>,
}

impl Bet {
    pub fn create_from_cli() -> Result<Bet, Box<dyn Error>>{
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
        println!("Give date in mm/dd/yyyy or mm/dd/yy format");
        let mut date = String::new();
        io::stdin().read_line(&mut date)?;
        let mut date_split = date.trim().split('/');
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

