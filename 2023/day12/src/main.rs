use core::panic;
use std::{collections::HashMap, time::Instant};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day12.txt");
    let now1 = Instant::now();
    println!("Solution part 1: {}", solve1(&input));
    let elapsed1 = now1.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
    let now2 = Instant::now();
    println!("Solution part 2: {}", solve2(&input));
    let elapsed2 = now2.elapsed();
    println!("Elapsed: {:.2?}", elapsed2);
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
    solve(input, 1)
}

fn solve2(input: &Input) -> usize {
    solve(input, 5)
}

fn solve(input: &Input, multiplier: usize) -> usize {
    let mut total = 0;
    for (springs, record) in input.iter() {
        let s = multiply_springs(springs, multiplier);
        let r = multiply_vec(record, multiplier);
        total += state_machine(&s, &r);
    }
    total
}

fn state_machine(springs: &Vec<Spring>, record: &Vec<usize>) -> usize {
    let num_states = get_num_states(record);
    let mut counts = init_count(num_states);
    let transitions = init_transitions(record, num_states);

    for spring in springs {
        let mut new_counts = vec![0; counts.len()];
        transition_step(&transitions, &counts, &mut new_counts, spring);
        counts = new_counts;
    }

    counts[counts.len() - 1]
}

fn get_num_states(record: &Vec<usize>) -> usize {
    record.len() + record.iter().sum::<usize>()
}

fn init_count(num_states: usize) -> Vec<usize> {
    let mut result = vec![0; num_states];
    result[0] = 1;
    result
}

fn init_transitions(record: &Vec<usize>, num_states: usize) -> Transitions {
    let mut result = HashMap::new();
    let mut last = num_states - 1;
    let mut current = num_states - 1;

    for entry in record.iter().rev() {
        result.insert((current, Spring::Operational), State::Node(last));
        result.insert((current, Spring::Damaged), State::Stop);
        current -= 1;
        last = current + 1;
        for i in (0..*entry).rev() {
            let op_state = if i == 0 {
                State::Node(current)
            } else {
                State::Stop
            };
            result.insert((current, Spring::Operational), op_state);
            result.insert((current, Spring::Damaged), State::Node(last));
            if current == 0 {
                break;
            }
            current -= 1;
            last -= 1;
        }
    }
    result
}

fn transition_step(
    transitions: &Transitions,
    counts: &Vec<usize>,
    new_counts: &mut Vec<usize>,
    spring: &Spring,
) {
    for i in 0..counts.len() {
        match spring {
            Spring::Operational => {
                let state = transitions.get(&(i, Spring::Operational)).unwrap();
                match state {
                    State::Node(j) => new_counts[*j] += counts[i],
                    State::Stop => (),
                }
            }
            Spring::Damaged => {
                let state = transitions.get(&(i, Spring::Damaged)).unwrap();
                match state {
                    State::Node(j) => new_counts[*j] += counts[i],
                    State::Stop => (),
                }
            }
            Spring::Unknown => {
                let state = transitions.get(&(i, Spring::Operational)).unwrap();
                match state {
                    State::Node(j) => new_counts[*j] += counts[i],
                    State::Stop => (),
                }
                let state2 = transitions.get(&(i, Spring::Damaged)).unwrap();
                match state2 {
                    State::Node(j) => new_counts[*j] += counts[i],
                    State::Stop => (),
                }
            }
        }
    }
}

fn multiply_vec<T: Clone>(input: &Vec<T>, multiplier: usize) -> Vec<T> {
    input
        .iter()
        .cycle()
        .take(input.len() * multiplier)
        .map(|el| el.clone())
        .collect()
}

fn multiply_springs(springs: &Vec<Spring>, multiplier: usize) -> Vec<Spring> {
    let mut result = springs.clone();
    for _ in 1..multiplier {
        result.push(Spring::Unknown);
        result.append(&mut springs.clone());
    }
    result
}

type Transitions = HashMap<(usize, Spring), State>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum State {
    Node(usize),
    Stop,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day12.txt");
        let result = solve2(&input);
        assert_eq!(result, 17788038834112);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 525152);
    }
}
