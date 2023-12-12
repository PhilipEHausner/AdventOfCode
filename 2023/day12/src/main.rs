use core::panic;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day12.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Input {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                panic!("Malformed input.");
            }
            let springs: Vec<Spring> = parts[0]
                .chars()
                .into_iter()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => panic!("Malformed input. unknown spring symbol."),
                })
                .collect();
            let control: Vec<usize> = parts[1].split(",").map(|el| el.parse().unwrap()).collect();
            (springs, control)
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    input
        .iter()
        .map(|el| num_arrangements(el, &mut vec![], 0))
        .sum()
}

fn num_arrangements(
    input: &(Vec<Spring>, Vec<usize>),
    curr_path: &mut Vec<Spring>,
    step: usize,
) -> usize {
    if step >= input.0.len() {
        return if is_valid_configuration(curr_path, &input.1) {
            1
        } else {
            0
        };
    }

    match input.0[step] {
        Spring::Operational => {
            curr_path.push(Spring::Operational);
            num_arrangements(input, curr_path, step + 1)
        }
        Spring::Damaged => {
            curr_path.push(Spring::Damaged);
            num_arrangements(input, curr_path, step + 1)
        }
        Spring::Unknown => {
            let mut curr_path2: Vec<Spring> = curr_path.clone();
            curr_path.push(Spring::Operational);
            curr_path2.push(Spring::Damaged);
            num_arrangements(input, curr_path, step + 1)
                + num_arrangements(input, &mut curr_path2, step + 1)
        }
    }
}

fn is_valid_configuration(path: &Vec<Spring>, gt: &Vec<usize>) -> bool {
    let mut result: Vec<usize> = vec![];
    let mut last = Spring::Operational;

    for spring in path {
        match (spring, &last) {
            (Spring::Operational, Spring::Operational) => (),
            (Spring::Operational, Spring::Damaged) => last = Spring::Operational,
            (Spring::Damaged, Spring::Operational) => {
                result.push(1);
                last = Spring::Damaged;
            }
            (Spring::Damaged, Spring::Damaged) => {
                let last_index = result.len() - 1;
                result[last_index] += 1;
            }
            (_, Spring::Unknown) => panic!("Should not contain unknown spring anymore."),
            (Spring::Unknown, _) => panic!("Should not contain unknown spring anymore."),
        }
    }

    &result == gt
}

fn solve2(input: &Input) -> usize {
    1
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

type Input = Vec<(Vec<Spring>, Vec<usize>)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day12.txt");
        let result = solve1(&input);
        assert_eq!(result, 7163);
    }

    #[test]
    fn test_solve1_testdata1() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 21);
    }
}