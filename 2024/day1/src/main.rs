use std::{collections::HashMap, ops::Add};

use lazy_static::lazy_static;
use regex::Regex;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day1.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lazy_static! {
        static ref re: Regex = Regex::new(r"(?<first>\d+)\s+(?<second>\d+)").unwrap();
    }
    lines
        .iter()
        .map(|line| {
            re.captures(line).map(|capture| {
                (
                    capture["first"].parse::<i64>().unwrap(),
                    capture["second"].parse::<i64>().unwrap(),
                )
            })
        })
        .map(|it| it.unwrap())
        .collect()
}

fn solve1(input: &Input) -> i64 {
    let mut first_list: Vec<i64> = input.iter().map(|it| it.0).collect();
    let mut second_list: Vec<i64> = input.iter().map(|it| it.1).collect();
    first_list.sort();
    second_list.sort();

    first_list
        .iter()
        .zip(second_list.iter())
        .map(|it| (it.1 - it.0).abs())
        .sum()
}

fn solve2(input: &Input) -> i64 {
    let first_list: Vec<i64> = input.iter().map(|it| it.0).collect();
    let second_list: Vec<i64> = input.iter().map(|it| it.1).collect();

    let mut first_map: HashMap<&i64, i64> = HashMap::new();
    first_list.iter().for_each(|it| {
        let val = first_map.entry(it).or_insert(0).add(1);
        first_map.insert(it, val);
    });

    let mut second_map: HashMap<&i64, i64> = HashMap::new();
    second_list.iter().for_each(|it| {
        let val = second_map.entry(it).or_insert(0).add(1);
        second_map.insert(it, val);
    });

    first_map.iter().map(|it| *it.0 * it.1 * second_map.get(it.0).unwrap_or(&0)).sum()
}

type Input = Vec<(i64, i64)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day1.txt");
        let result = solve1(&input);
        assert_eq!(result, 2264607);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day1.txt");
        let result = solve2(&input);
        assert_eq!(result, 19457120);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 31);
    }
}