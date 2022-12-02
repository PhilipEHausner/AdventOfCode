use crate::handsign::Handsign;

pub struct Round {
    pub player: Handsign,
    pub opponent: Handsign,
}

impl Round {
    pub fn points(&self) -> u64 {
        let outcome_points = match (self.player, self.opponent) {
            // win
            (Handsign::Rock, Handsign::Scissors) => 6,
            (Handsign::Paper, Handsign::Rock) => 6,
            (Handsign::Scissors, Handsign::Paper) => 6,

            // tie
            (Handsign::Rock, Handsign::Rock) => 3,
            (Handsign::Paper, Handsign::Paper) => 3,
            (Handsign::Scissors, Handsign::Scissors) => 3,

            // lose
            (Handsign::Rock, Handsign::Paper) => 0,
            (Handsign::Paper, Handsign::Scissors) => 0,
            (Handsign::Scissors, Handsign::Rock) => 0,
        };
        let handsign_points = match self.player {
            Handsign::Rock => 1,
            Handsign::Paper => 2,
            Handsign::Scissors => 3,
        };
        outcome_points + handsign_points
    }
}