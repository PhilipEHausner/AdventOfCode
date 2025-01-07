use core::panic;
use std::{
    ops::{BitXor, Div},
    usize,
};

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
    1
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
        // assert_eq!(result, );
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        // assert_eq!(result, );
    }
}
