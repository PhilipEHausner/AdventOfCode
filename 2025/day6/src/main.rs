use core::panic;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input_part1("./files/day6.txt");
    println!("Solution part 1: {}", solve1(&input));
    let input2 = get_input_part2("./files/day6.txt");
    println!("Solution part 2: {}", solve2(&input2));
}

fn get_input_part1(filename: &str) -> Input {
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

fn get_input_part2(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

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

    let number_lines: Vec<&String> = lines.iter().take(lines.len() - 1).collect();

    let mut number_vecs = vec![];
    let mut number_vec = vec![];
    for idx in 0..number_lines.get(0).unwrap().len() {
        if all_columns_empty(&number_lines, idx) {
            number_vecs.push(number_vec);
            number_vec = vec![];
            continue;
        }
        let number = compute_column_number(&number_lines, idx);
        number_vec.push(number);
    }
    if !number_vec.is_empty() {
        number_vecs.push(number_vec);
    }

    assert!(number_vecs.len() == operations.len());

    operations
        .iter()
        .zip(number_vecs)
        .map(|(op, nvec)| Problem {
            numbers: nvec,
            operation: *op,
        })
        .collect()
}

fn all_columns_empty(lines: &Vec<&String>, idx: usize) -> bool {
    lines
        .iter()
        .map(|line| line.chars().nth(idx).unwrap())
        .all(|el| el == ' ')
}

fn compute_column_number(lines: &Vec<&String>, idx: usize) -> u64 {
    let digits: Vec<u64> = lines
        .iter()
        .map(|line| line.chars().nth(idx).unwrap())
        .filter(|c| *c != ' ')
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let mut result = 0;
    let mut order = 1;
    for digit in digits.iter().rev() {
        result += order * digit;
        order *= 10;
    }

    result
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
    solve1(input)
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
        let input = get_input_part1("./files/day6.txt");
        let result = solve1(&input);
        assert_eq!(result, 7229350537438);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input_part1("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_solve2() {
        let input = get_input_part2("./files/day6.txt");
        let result = solve2(&input);
        assert_eq!(result, 11479269003550);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input_part2("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 3263827);
    }
}
