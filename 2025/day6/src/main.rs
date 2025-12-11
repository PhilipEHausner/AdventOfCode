use core::panic;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day6.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    let number_lines: Vec<Vec<u64>> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|el| el.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();
    let operations: Vec<Operation> = lines
        .iter()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|el| {
            if el == "+" {
                Operation::Add
            } else if el == "*" {
                Operation::Multiply
            } else {
                panic!("Unknown operations '{}'", el)
            }
        })
        .collect();

    assert!(number_lines.iter().all(|el| el.len() == operations.len()));

    let mut problems = vec![];
    for i in 0..operations.len() {
        let numbers: Vec<u64> = number_lines.iter().map(|el| *el.get(i).unwrap()).collect();
        let operation = *operations.get(i).unwrap();
        problems.push(Problem { numbers, operation });
    }

    problems
}

fn solve1(input: &Input) -> usize {
    let mut result = 0;
    for problem in input {
        result += match problem.operation {
            Operation::Add => problem.numbers.iter().fold(0, |a, b| a + b),
            Operation::Multiply => problem.numbers.iter().fold(1, |a, b| a * b),
        }
    }
    result as usize
}

fn solve2(input: &Input) -> usize {
    1
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

type Input = Vec<Problem>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day6.txt");
        let result = solve1(&input);
        assert_eq!(result, 7229350537438);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day6.txt");
        let result = solve2(&input);
        // assert_eq!(result, );
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        // assert_eq!(result, );
    }
}
