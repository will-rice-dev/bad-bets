pub mod teams;

use std::{cmp::Ordering, collections::BinaryHeap};
use chrono::{Local, NaiveDate};
use serde::{Serialize, Deserialize};

pub use teams::Team;

pub const SECONDS_IN_DAY: i64 = 86400; // Want to give enough time for 

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub bets_settled: BinaryHeap<Bet>,
    pub bets_outstanding: BinaryHeap<Bet>,
    pub name: String,
}

impl User {
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
        if dif.num_days() > 0 {
            return Ordering::Less
        } else if dif.num_days() < 0 {
            return Ordering::Greater
        }
        Ordering::Equal
    }
}


#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, collections::BinaryHeap};
    use chrono::NaiveDate;

    use crate::{Bet, Team, User};

    #[test]
    fn sorting_bets_by_settle_date_works() {
        let mut user = get_new_user();
        let date1 = NaiveDate::from_ymd_opt(2024, 2, 14).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2024, 2, 16).unwrap();

        user.bets_outstanding.push(get_new_bet_of_date(&date2));
        user.bets_outstanding.push(get_new_bet_of_date(&date1));
        user.bets_outstanding.push(get_new_bet_of_date(&date3));

        for bet in user.bets_outstanding.iter() {
            assert_eq!(bet.date_settled, date1);
            break;
        }        
    }

    #[test]
    fn sort_bets() {
        let date1 = NaiveDate::from_ymd_opt(2024, 2, 14).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap();

        let bet1 = get_new_bet_of_date(&date1);
        let bet2 = get_new_bet_of_date(&date2);
        let bet3 = get_new_bet_of_date(&date1);

        assert_eq!(bet1.cmp(&bet2), Ordering::Greater);
        assert_eq!(bet2.cmp(&bet1), Ordering::Less);
        assert_eq!(bet1.cmp(&bet3), Ordering::Equal);
    }
    
    fn get_new_user() -> User {
        User {
            name: "Test User".to_string(),
            bets_outstanding: BinaryHeap::new(),
            bets_settled: BinaryHeap::new(),
        }
    }

    fn get_new_bet_of_date(settle_date: &NaiveDate) -> Bet {
        Bet {
            bet_type: crate::BetType::FutureUnder,
            team_for: None,
            team_against: Team::from_str("lions"),
            odds: -120,
            bet_amount: 10.0,
            date_placed: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            date_settled: *settle_date,
            won: None,
        }
    }
}