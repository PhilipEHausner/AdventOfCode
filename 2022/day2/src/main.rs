use std::collections::HashMap;
use std::fmt;
use util::hashmap;
use util::read_files::read_file_as_vector;

#[derive(Clone, Copy)]
enum Handsign {
    Rock,
    Paper,
    Scissors,
}

impl Handsign {
    fn get_complement(&self, outcome: &Outcome) -> Handsign {
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

struct Round {
    player: Handsign,
    opponent: Handsign,
}

enum Outcome {
    Win,
    Tie,
    Lose,
}

impl Round {
    fn points(&self) -> u64 {
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

fn main() {
    let result1 = solve1("./files/day2.txt");
    println!("Solution 1: {}", result1);
    let result1 = solve2("./files/day2.txt");
    println!("Solution 2: {}", result1);
}

fn solve1(filename: &str) -> u64 {
    let player_handsign_code =
        hashmap!['X' => Handsign::Rock, 'Y' => Handsign::Paper, 'Z' => Handsign::Scissors];
    let opponent_handsign_code =
        hashmap!['A' => Handsign::Rock, 'B' => Handsign::Paper, 'C' => Handsign::Scissors];
    let strategy_guide =
        get_strategy_guide_problem1(filename, &player_handsign_code, &opponent_handsign_code);

    calculate_final_points(strategy_guide)
}

fn calculate_final_points(strategy_guide: Vec<Round>) -> u64 {
    let mut result = 0;

    for round in strategy_guide {
        result += round.points();
    }

    result
}

fn get_strategy_guide_problem1(
    filename: &str,
    player_handsign_code: &HashMap<char, Handsign>,
    opponent_handsign_code: &HashMap<char, Handsign>,
) -> Vec<Round> {
    let lines = read_file_as_vector(filename).expect("Coult not read file.");
    let mut strategy_guide = Vec::new();

    for line in lines {
        let round: Vec<&str> = line.split(" ").collect();
        debug_assert_eq!(round.len(), 2);
        strategy_guide.push(Round {
            opponent: opponent_handsign_code
                [&round[0].chars().next().expect("Unknown opponent handsign.")],
            player: player_handsign_code
                [&round[1].chars().next().expect("Unknown player handsign.")],
        })
    }

    strategy_guide
}

fn solve2(filename: &str) -> u64 {
    let player_outcome_code =
        hashmap!['X' => Outcome::Lose, 'Y' => Outcome::Tie, 'Z' => Outcome::Win];
    let opponent_handsign_code =
        hashmap!['A' => Handsign::Rock, 'B' => Handsign::Paper, 'C' => Handsign::Scissors];
    let strategy_guide = get_strategy_guide_problem2(filename, &player_outcome_code, &opponent_handsign_code);
    calculate_final_points(strategy_guide)
}

fn get_strategy_guide_problem2(
    filename: &str,
    player_outcome_code: &HashMap<char, Outcome>,
    opponent_handsign_code: &HashMap<char, Handsign>,
) -> Vec<Round> {
    let lines = read_file_as_vector(filename).expect("Coult not read file.");
    let mut strategy_guide = Vec::new();

    for line in lines {
        let round: Vec<&str> = line.split(" ").collect();
        debug_assert_eq!(round.len(), 2);
        let opponent_handsign =
            opponent_handsign_code[&round[0].chars().next().expect("Unknown opponent handsign.")];
        let required_outcome =
            &player_outcome_code[&round[1].chars().next().expect("Unknown player handsign.")];
        let player_handsign = opponent_handsign.get_complement(required_outcome);
        strategy_guide.push(Round {
            opponent: opponent_handsign,
            player: player_handsign,
        })
    }

    strategy_guide
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let result = solve1("./files/test.txt");
        assert_eq!(result, 15);
    }

    #[test]
    fn test_solve2() {
        let result = solve2("./files/test.txt");
        assert_eq!(result, 12);
    }
}
