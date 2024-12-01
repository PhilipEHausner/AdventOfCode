use std::collections::HashMap;

use regex::Regex;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day1.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    let re: Regex = Regex::new(r"(?<first>\d+)\s+(?<second>\d+)").unwrap();

    lines.iter().map(|line| parse_line(&re, line)).collect()
}

fn parse_line(re: &Regex, line: &str) -> (i64, i64) {
    let captures = re.captures(line).unwrap();
    let first = captures["first"].parse::<i64>().unwrap();
    let second = captures["second"].parse::<i64>().unwrap();
    (first, second)
}

fn solve1(input: &Input) -> i64 {
    let mut first_list: Vec<_> = input.iter().map(|&(first, _)| first).collect();
    let mut second_list: Vec<_> = input.iter().map(|&(_, second)| second).collect();
    first_list.sort();
    second_list.sort();

    first_list
        .iter()
        .zip(second_list.iter())
        .map(|(&first, &second)| (second - first).abs())
        .sum()
}

fn solve2(input: &Input) -> i64 {
    let mut first_map: HashMap<i64, i64> = HashMap::new();
    let mut second_map: HashMap<i64, i64> = HashMap::new();

    for &(first, second) in input {
        *first_map.entry(first).or_insert(0) += 1;
        *second_map.entry(second).or_insert(0) += 1;
    }

    first_map
        .iter()
        .map(|(&key, &count1)| key * count1 * second_map.get(&key).unwrap_or(&0))
        .sum()
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
