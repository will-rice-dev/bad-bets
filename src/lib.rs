pub mod teams;

use std::{cmp::Ordering, collections::BinaryHeap, error::Error, io};
use chrono::{Local, NaiveDate};
use serde::{Serialize, Deserialize};

pub use teams::Team;

pub const SECONDS_IN_DAY: i64 = 86400; // Want to give enough time for 

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub bets_settled: BinaryHeap<Bet>,
    pub bets_outstanding: BinaryHeap<Bet>,
    pub name: String,
}

impl Profile {
    pub fn new_from_cli() -> Result<Profile, Box<dyn Error>> {
        let mut name = String::new();
        println!("Welcome newcomer! What is your name?");
        io::stdin().read_line(&mut name)?;
        Ok(Profile { name: name.trim().to_string(), bets_outstanding: BinaryHeap::new(), bets_settled: BinaryHeap::new()})

    }

    /**
     * Returns the bets that have been entered but not marked as won or lost
     * Indicates whether the bet is settling today with a bool per Bet.
     */
    pub fn get_settleable_bets(&self) -> Option<Vec<(&Bet, bool)>> {
        let current_date: NaiveDate = Local::now().date_naive();
        let mut settleable_bets: Vec<(&Bet, bool)> = vec![];
        for bet in &self.bets_outstanding {
            let dif = current_date - bet.date_settled;
            if dif.num_seconds() > SECONDS_IN_DAY {
                settleable_bets.push((bet, false));
            } else if dif.num_seconds() > 0 {
                settleable_bets.push((bet, true));
            } else {
                break;
            }
        }
        if settleable_bets.len() == 0 {
            return None
        }
        Some(settleable_bets)
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum BetType {
    HeadToHead,
    FutureOver,
    FutureUnder,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
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

impl Eq for Bet {}

impl Ord for Bet {
    fn cmp(&self, other: &Self) -> Ordering {
        let dif = self.date_settled - other.date_settled;
        if dif.num_seconds() < 0 {
            return Ordering::Less
        } else if dif.num_seconds() > 0 {
            return Ordering::Greater
        }
        Ordering::Equal
    }
}
