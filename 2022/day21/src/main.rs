use std::collections::HashMap;
use util::read_files::read_file_as_vector;

#[derive(Debug)]
enum MonkeyJob {
    Number(i128),
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
    fn compute(&self, operand1: i128, operand2: i128) -> i128 {
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
    println!("Solution part 2: {}", solve2(&lines));
}
fn solve1(lines: &Vec<String>) -> i128 {
    let jobs = parse_input(lines);
    compute_job_result(&jobs, &"root".to_string())
}

fn solve2(lines: &Vec<String>) -> i128 {
    let jobs = parse_input(lines);
    compute_human_number(&jobs)
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

fn compute_job_result(jobs: &HashMap<String, MonkeyJob>, node: &String) -> i128 {
    match &jobs.get(node).unwrap() {
        MonkeyJob::Number(x) => *x,
        MonkeyJob::Operation(operand1, operator, operand2) => {
            let x = compute_job_result(jobs, operand1);
            let y = compute_job_result(jobs, operand2);
            operator.compute(x, y)
        }
    }
}

fn compute_human_number(jobs: &HashMap<String, MonkeyJob>) -> i128 {
    // let mut minval: i128 = -5000000000000;
    // let mut maxval: i128 = 5000000000000;

    try_human_number(jobs, 3296135418819);
    try_human_number(jobs, 3296135418820);
    try_human_number(jobs, 3296135418821);
    try_human_number(jobs, 3296135418822);
    try_human_number(jobs, 3296135418823);

    // Harcoded binary search to find possible solution (which was one off for some reason).
    // while minval != maxval {
    //     let attempt = (minval + maxval) / 2;
    //     let result = try_human_number(jobs, attempt);

    //     match result {
    //         std::cmp::Ordering::Less => minval = attempt,
    //         std::cmp::Ordering::Equal => return attempt,
    //         std::cmp::Ordering::Greater => maxval = attempt,
    //     }
    // }
    panic!("Did not find result.");
}

fn try_human_number(jobs: &HashMap<String, MonkeyJob>, human_number: i128) -> std::cmp::Ordering {
    let root = jobs.get("root").unwrap();

    match root {
        MonkeyJob::Operation(operand1, _, operand2) => {
            let res1 = compute_job_result_with_human_number(jobs, operand1, human_number);
            let res2 = compute_job_result_with_human_number(jobs, operand2, human_number);
            println!("RES {} - {}", human_number, res1);
            println!("RES {} - {}", human_number, res2);
            println!("-----------");
            if res1 < res2 {
                std::cmp::Ordering::Less
            } else if res1 == res2 {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Greater
            }
        }
        _ => panic!("Root node needs to be an operation."),
    }
}

fn compute_job_result_with_human_number(
    jobs: &HashMap<String, MonkeyJob>,
    node: &String,
    human_number: i128,
) -> i128 {
    if node == "humn" {
        return human_number;
    }

    match &jobs.get(node).unwrap() {
        MonkeyJob::Number(x) => *x,
        MonkeyJob::Operation(operand1, operator, operand2) => {
            let x = compute_job_result_with_human_number(jobs, operand1, human_number);
            let y = compute_job_result_with_human_number(jobs, operand2, human_number);
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
