use std::collections::HashMap;
use util::read_files::read_file_as_vector;

#[derive(Debug)]
enum MonkeyJob {
    Number(i64),
    Operation(String, Operator, String),
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn compute(&self, operand1: i64, operand2: i64) -> i64 {
    match *self {
        Operator::Add => operand1 + operand2,
        Operator::Subtract => operand1 - operand2,
        Operator::Multiply => operand1 * operand2,
        Operator::Divide => operand1 / operand2,
    }
    }
}

fn main() {
    let lines = read_file_as_vector("./files/day21.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    // println!("Solution part 2: {}", solve2(&lines));
}
fn solve1(lines: &Vec<String>) -> i64 {
    let jobs = parse_input(lines);
    compute_job_result(&jobs, &"root".to_string())
}

fn parse_input(lines: &Vec<String>) -> HashMap<String, MonkeyJob> {
    let mut jobs = HashMap::new();

    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let key = parts[0].replace(":", "");

        if parts.len() == 2 {
            let number = parts[1].parse().unwrap();
            jobs.insert(key, MonkeyJob::Number(number));
        } else if parts.len() == 4 {
            let operand1 = parts[1].to_string();
            let operator = parse_operator(parts[2]);
            let operand2 = parts[3].to_string();
            jobs.insert(key, MonkeyJob::Operation(operand1, operator, operand2)); 
        } else {
            panic!("Unexpected input length {}.", parts.len());
        }
    }

    jobs
}

fn parse_operator(operator: &str) -> Operator {
    match operator {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        _ => panic!("Unknown operator '{}'.", operator),
    }
}

fn compute_job_result(jobs: &HashMap<String, MonkeyJob>, node: &String) -> i64 {
    match &jobs.get(node).unwrap() {
        MonkeyJob::Number(x) => *x,
        MonkeyJob::Operation(operand1, operator, operand2) => {
            let x = compute_job_result(jobs, operand1);
            let y = compute_job_result(jobs, operand2);
            operator.compute(x, y)
    }
        }
        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 152);
    }
}
