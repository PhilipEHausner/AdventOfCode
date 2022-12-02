use std::fmt;

use crate::outcome::Outcome;

#[derive(Clone, Copy)]
pub enum Handsign {
    Rock,
    Paper,
    Scissors,
}

impl Handsign {
    pub fn get_complement(&self, outcome: &Outcome) -> Handsign {
        match *outcome {
            Outcome::Win => match self {
                Handsign::Rock => Handsign::Paper,
                Handsign::Paper => Handsign::Scissors,
                Handsign::Scissors => Handsign::Rock,
            },
            Outcome::Tie => match self {
                Handsign::Rock => Handsign::Rock,
                Handsign::Paper => Handsign::Paper,
                Handsign::Scissors => Handsign::Scissors,
            },
            Outcome::Lose => match self {
                Handsign::Rock => Handsign::Scissors,
                Handsign::Paper => Handsign::Rock,
                Handsign::Scissors => Handsign::Paper,
            },
        }
    }
}

impl fmt::Display for Handsign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Handsign::Rock => write!(f, "Rock"),
            Handsign::Paper => write!(f, "Paper"),
            Handsign::Scissors => write!(f, "Scissors"),
        }
    }
}