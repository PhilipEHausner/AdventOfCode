use std::collections::HashMap;

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
    solve(input, 25)
}

fn solve2(input: &Input) -> usize {
    solve(input, 75)
}

fn solve(input: &Input, steps: usize) -> usize {
    let mut current: HashMap<u64, usize> = HashMap::new();
    input
        .iter()
        .for_each(|&el| *current.entry(el).or_insert(0) += 1);

    for i in 0..steps {
        current = step(&current);
    }

    current.values().sum()
}

fn step(input: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut out = HashMap::new();

    for entry in input {
        let number = *entry.0;
        if number == 0 {
            *out.entry(1).or_insert(0) += entry.1;
        } else if number.to_string().len() % 2 == 0 {
            let digits = number.to_string();
            let split = digits.len() / 2;
            let (first, last) = digits.split_at(split);

            *out.entry(first.parse().unwrap()).or_insert(0) += entry.1;
            *out.entry(last.parse().unwrap()).or_insert(0) += entry.1;
        } else {
            *out.entry(number * 2024).or_insert(0) += entry.1;
        }
    }

    out
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

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day11.txt");
        let result = solve2(&input);
        assert_eq!(result, 207961583799296);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 65601038650482);
    }
}
