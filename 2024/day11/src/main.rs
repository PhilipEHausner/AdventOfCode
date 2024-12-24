use std::collections::LinkedList;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day11.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    assert!(lines.len() == 1);

    lines[0]
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut current = input.clone();

    for _ in 0..25 {
        current = step(&current);
    }

    current.len()
}

fn solve2(_input: &Input) -> usize {
    1
}

fn step(input: &Input) -> Input {
    let mut new = Vec::new();
    for &number in input {
        if number == 0 {
            new.push(1);
        } else if number.to_string().len() % 2 == 0 {
            let digits = number.to_string();
            let split = digits.len() / 2;
            let (first, last) = digits.split_at(split);

            new.push(first.parse().unwrap());
            new.push(last.parse().unwrap());
        } else {
            new.push(number * 2024);
        }
    }
    new
}

type Input = Vec<u64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day11.txt");
        let result = solve1(&input);
        assert_eq!(result, 175006);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 55312);
    }
}
