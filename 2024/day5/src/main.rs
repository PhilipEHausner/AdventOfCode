use std::{collections::HashSet, ops::Div, usize};

use regex::Regex;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day5.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    let orderings_re = Regex::new(r"(?<first>\d+)\|(?<second>\d+)").unwrap();
    let orderings = lines
        .iter()
        .filter(|line| line.contains('|'))
        .map(|line| {
            let capture = orderings_re.captures(line).unwrap();
            (
                capture["first"].parse().unwrap(),
                capture["second"].parse().unwrap(),
            )
        })
        .collect();

    let updates = lines
        .iter()
        .filter(|line| line.contains(','))
        .map(|line| line.split(',').map(|it| it.parse().unwrap()).collect())
        .collect();

    Input { orderings, updates }
}

fn solve1(input: &Input) -> usize {
    input
        .updates
        .iter()
        .filter(|update| update_is_correctly_ordered(&input.orderings, &update))
        .map(|update| get_center_page(&update))
        .sum()
}

fn update_is_correctly_ordered(orderings: &HashSet<(usize, usize)>, update: &Vec<usize>) -> bool {
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            if orderings.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }
    true
}

fn get_center_page(update: &Vec<usize>) -> usize {
    update[update.len().div(2)]
}

fn solve2(input: &Input) -> usize {
    1
}

struct Input {
    orderings: HashSet<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day5.txt");
        let result = solve1(&input);
        assert_eq!(result, 4924);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 143);
    }
}
