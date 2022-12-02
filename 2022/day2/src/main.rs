mod handsign;
mod outcome;
mod round;

use std::collections::HashMap;
use util::hashmap;
use util::read_files::read_file_as_vector;

use handsign::Handsign;
use outcome::Outcome;
use round::Round;


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
                [&round[0].chars().next().expect("Unknown opponent Handsign.")],
            player: player_handsign_code
                [&round[1].chars().next().expect("Unknown player Handsign.")],
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
            opponent_handsign_code[&round[0].chars().next().expect("Unknown opponent Handsign.")];
        let required_outcome =
            &player_outcome_code[&round[1].chars().next().expect("Unknown player Handsign.")];
        let player_handsign = opponent_handsign.get_complement(required_outcome);
        strategy_guide.push(Round {
            opponent: opponent_handsign,
            player: player_handsign,
        })
    }

    strategy_guide
}

fn calculate_final_points(strategy_guide: Vec<Round>) -> u64 {
    let mut result = 0;

    for round in strategy_guide {
        result += round.points();
    }

    result
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
