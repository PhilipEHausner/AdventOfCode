use std::collections::HashSet;

use util::read_files::read_file_as_vector;

struct ScratchCard {
    winning_numbers: HashSet<i64>,
    numbers: HashSet<i64>,
}

impl ScratchCard {
    pub fn new(winning_numbers: HashSet<i64>, numbers: HashSet<i64>) -> ScratchCard {
        ScratchCard {
            winning_numbers,
            numbers,
        }
    }

    pub fn from_line(line: &str) -> ScratchCard {
        let parts: Vec<&str> = line.split("|").collect();
        if parts.len() != 2 {
            panic!("Invalid input: {}", line);
        }
        let winning_numbers = ScratchCard::parse_number_line(parts.get(0).unwrap(), 2);
        let numbers = ScratchCard::parse_number_line(parts.get(1).unwrap(), 0);

        ScratchCard::new(winning_numbers, numbers)
    }

    fn parse_number_line(line: &str, skip: usize) -> HashSet<i64> {
        line.split(" ")
            .filter(|it| it.len() > 0)
            .skip(skip)
            .map(|it| it)
            .map(|it: &str| it.parse::<i64>().unwrap())
            .collect::<HashSet<i64>>()
    }

    pub fn points(&self) -> i64 {
        let matches = self
            .numbers
            .iter()
            .filter(|it| self.winning_numbers.contains(it))
            .count();
        if matches == 0 {
            0
        } else {
            2_i64.pow((matches - 1) as u32)
        }
    }
}

fn main() {
    let scratch_cards = parse_scratchcards("./files/day4.txt");
    println!("Solution part 1: {}", solve1(&scratch_cards));
    // println!("Solution part 2: {}", solve2(&scratch_cards));
}

fn parse_scratchcards(file: &str) -> Vec<ScratchCard> {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    lines
        .into_iter()
        .map(|line| ScratchCard::from_line(&line))
        .collect()
}

fn solve1(scratch_cards: &Vec<ScratchCard>) -> i64 {
    scratch_cards.into_iter().map(|it| it.points()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = parse_scratchcards("./files/day4.txt");
        let result = solve1(&input);
        assert_eq!(result, 27059);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = parse_scratchcards("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 13);
    }
}
