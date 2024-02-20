use serde::{Deserialize, Serialize};

// Define an enum for NBA teams
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NBATeam {
    Hawks, Celtics, Nets, Hornets, Bulls, Cavaliers, Mavericks, Nuggets, Pistons, Warriors,
    Rockets, Pacers, Clippers, Lakers, Grizzlies, Heat, Bucks, Timberwolves, Pelicans, Knicks,
    Thunder, Magic, Sixers, Suns, TrailBlazers, Kings, Spurs, Raptors, Jazz, Wizards,
}

// Implement a method to convert a &str to NBATeam
impl NBATeam {
    fn from_str(name: &str) -> Option<NBATeam> {
        match name.to_lowercase().as_str() {
            "hawks" => Some(NBATeam::Hawks),
            "celtics" => Some(NBATeam::Celtics),
            "nets" => Some(NBATeam::Nets),
            "hornets" => Some(NBATeam::Hornets),
            "bulls" => Some(NBATeam::Bulls),
            "cavaliers" => Some(NBATeam::Cavaliers),
            "mavericks" => Some(NBATeam::Mavericks),
            "nuggets" => Some(NBATeam::Nuggets),
            "pistons" => Some(NBATeam::Pistons),
            "warriors" => Some(NBATeam::Warriors),
            "rockets" => Some(NBATeam::Rockets),
            "pacers" => Some(NBATeam::Pacers),
            "clippers" => Some(NBATeam::Clippers),
            "lakers" => Some(NBATeam::Lakers),
            "grizzlies" => Some(NBATeam::Grizzlies),
            "heat" => Some(NBATeam::Heat),
            "bucks" => Some(NBATeam::Bucks),
            "timberwolves" => Some(NBATeam::Timberwolves),
            "pelicans" => Some(NBATeam::Pelicans),
            "knicks" => Some(NBATeam::Knicks),
            "thunder" => Some(NBATeam::Thunder),
            "magic" => Some(NBATeam::Magic),
            "76ers" => Some(NBATeam::Sixers),
            "suns" => Some(NBATeam::Suns),
            "trailblazers" => Some(NBATeam::TrailBlazers),
            "kings" => Some(NBATeam::Kings),
            "spurs" => Some(NBATeam::Spurs),
            "raptors" => Some(NBATeam::Raptors),
            "jazz" => Some(NBATeam::Jazz),
            "wizards" => Some(NBATeam::Wizards),
            _ => None,
        }
    }
}

// Define an enum for NFL teams
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NFLTeam {
    Cardinals, Falcons, Ravens, Bills, Panthers, Bears, Bengals, Browns, Cowboys, Broncos,
    Lions, Packers, Texans, Colts, Jaguars, Chiefs, Chargers, Rams, Dolphins, Vikings,
    Patriots, Saints, Giants, Jets, Raiders, Eagles, Steelers, Niners, Seahawks, Buccaneers,
    Titans, WashingtonFootballTeam,
}

// Implement a method to convert a &str to NFLTeam
impl NFLTeam {
    fn from_str(name: &str) -> Option<NFLTeam> {
        match name.to_lowercase().as_str() {
            "cardinals" => Some(NFLTeam::Cardinals),
            "falcons" => Some(NFLTeam::Falcons),
            "ravens" => Some(NFLTeam::Ravens),
            "bills" => Some(NFLTeam::Bills),
            "panthers" => Some(NFLTeam::Panthers),
            "bears" => Some(NFLTeam::Bears),
            "bengals" => Some(NFLTeam::Bengals),
            "browns" => Some(NFLTeam::Browns),
            "cowboys" => Some(NFLTeam::Cowboys),
            "broncos" => Some(NFLTeam::Broncos),
            "lions" => Some(NFLTeam::Lions),
            "packers" => Some(NFLTeam::Packers),
            "texans" => Some(NFLTeam::Texans),
            "colts" => Some(NFLTeam::Colts),
            "jaguars" => Some(NFLTeam::Jaguars),
            "chiefs" => Some(NFLTeam::Chiefs),
            "chargers" => Some(NFLTeam::Chargers),
            "rams" => Some(NFLTeam::Rams),
            "dolphins" => Some(NFLTeam::Dolphins),
            "vikings" => Some(NFLTeam::Vikings),
            "patriots" => Some(NFLTeam::Patriots),
            "saints" => Some(NFLTeam::Saints),
            "giants" => Some(NFLTeam::Giants),
            "jets" => Some(NFLTeam::Jets),
            "raiders" => Some(NFLTeam::Raiders),
            "eagles" => Some(NFLTeam::Eagles),
            "steelers" => Some(NFLTeam::Steelers),
            "niners" => Some(NFLTeam::Niners),
            "seahawks" => Some(NFLTeam::Seahawks),
            "buccaneers" => Some(NFLTeam::Buccaneers),
            "titans" => Some(NFLTeam::Titans),
            "washingtonfootballteam" => Some(NFLTeam::WashingtonFootballTeam),
            _ => None,
        }
    }
}

// Define a generic enum for sports teams
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Team {
    NBA(NBATeam),
    NFL(NFLTeam),
}

// Implement a method to convert a &str to Team
impl Team {
    pub fn from_str(name: &str) -> Option<Team> {
        if let Some(nba_team) = NBATeam::from_str(name) {
            return Some(Team::NBA(nba_team));
        } else if let Some(nfl_team) = NFLTeam::from_str(name) {
            return Some(Team::NFL(nfl_team));
        }
        None
    }
}

// Function to validate team choices
pub fn validate_teams(team1: &Team, team2: &Team) -> bool {
    match (team1, team2) {
        (Team::NBA(nba1), Team::NBA(nba2)) if nba1 != nba2 => true,
        (Team::NFL(nfl1), Team::NFL(nfl2)) if nfl1 != nfl2 => true,
        _ => false,
    }
}
