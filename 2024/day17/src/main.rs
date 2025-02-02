use core::panic;
use std::ops::{BitXor, Div};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day17.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    let reg_a = lines[0].split(": ").last().unwrap().parse().unwrap();
    let reg_b = lines[1].split(": ").last().unwrap().parse().unwrap();
    let reg_c = lines[2].split(": ").last().unwrap().parse().unwrap();
    let program = lines[4]
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|el| el.parse().unwrap())
        .collect();

    Input {
        reg_a,
        reg_b,
        reg_c,
        program,
    }
}

fn solve1(input: &Input) -> String {
    let mut ip = 0;
    let mut computer = input.clone();
    let mut result: Vec<String> = vec![];
    while ip < computer.program.len() {
        let opcode = computer.program[ip];
        let operand = computer.program[ip + 1];

        match opcode {
            0 => {
                let numerator = computer.reg_a;
                let denominator = 2_usize.pow(combo_operand(operand, &computer) as u32);
                computer.reg_a = numerator.div(denominator);
            }
            1 => {
                computer.reg_b = operand.bitxor(computer.reg_b);
            }
            2 => {
                computer.reg_b = combo_operand(operand, &computer) % 8;
            }
            3 => {
                if computer.reg_a != 0 {
                    ip = operand;
                }
            }
            4 => {
                computer.reg_b = computer.reg_b.bitxor(computer.reg_c);
            }
            5 => {
                let value = combo_operand(operand, &computer) % 8_usize;
                result.push(value.to_string());
            }
            6 => {
                let numerator = computer.reg_a;
                let denominator = 2_usize.pow(combo_operand(operand, &computer) as u32);
                computer.reg_b = numerator.div(denominator);
            }
            7 => {
                let numerator = computer.reg_a;
                let denominator = 2_usize.pow(combo_operand(operand, &computer) as u32);
                computer.reg_c = numerator.div(denominator);
            }
            _ => panic!("Unknown opcode {}", opcode),
        }

        if opcode != 3 || computer.reg_a == 0 {
            ip += 2;
        }
    }

    result.join(",")
}

fn combo_operand(operand: usize, computer: &Input) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => computer.reg_a,
        5 => computer.reg_b,
        6 => computer.reg_c,
        7 => panic!("Reserved combo operand."),
        _ => panic!("Invalid operand {}", operand),
    }
}

fn solve2(input: &Input) -> usize {
    let mut candidates: Vec<u64> = vec![0];
    let mut computer = input.clone();
    let program_len = computer.program.len();

    for i in 1..=program_len {
        let expected_output = get_expected_output(&computer, i);
        let mut new_candidates = vec![];
        for candidate in candidates.iter() {
            for j in 0..8 {
                let new_candidate = candidate * 8 + j;
                computer.reg_a = new_candidate as usize;
                if solve1(&computer) == expected_output {
                    new_candidates.push(new_candidate);
                }
            }
        }
        candidates = new_candidates;
    }

    *candidates.iter().min().unwrap_or(&0_u64) as usize
}

fn get_expected_output(computer: &Input, i: usize) -> String {
    let program_len = computer.program.len();
    computer.program[program_len - i..program_len]
        .iter()
        .map(|&el| el.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[derive(Debug, Clone)]
struct Input {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day17.txt");
        let result = solve1(&input);
        assert_eq!(result, "1,7,2,1,4,1,5,4,0");
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day17.txt");
        let result = solve2(&input);
        assert_eq!(result, 37221261688308);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test2.txt");
        let result = solve2(&input);
        assert_eq!(result, 117440);
    }
}
