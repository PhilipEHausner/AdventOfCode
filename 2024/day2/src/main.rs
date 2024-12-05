use std::alloc::alloc;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day2.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(' ').map(|el| el.parse().unwrap()).collect())
        .collect()
}

fn solve1(input: &Input) -> usize {
    input
        .iter()
        .map(|report| is_safe(report))
        .filter(|safe| *safe)
        .count()
}

fn is_safe(report: &Vec<i64>) -> bool {
    (is_strictly_rising(report) || is_strictly_decreasing(report))
        && differences_lower_than_three(report)
}

fn is_strictly_rising(report: &Vec<i64>) -> bool {
    report.windows(2).all(|w| w[0] < w[1])
}

fn is_strictly_decreasing(report: &Vec<i64>) -> bool {
    report.windows(2).all(|w| w[0] > w[1])
}

fn differences_lower_than_three(report: &Vec<i64>) -> bool {
    report
        .windows(2)
        .map(|w| (w[0] - w[1]).abs())
        .all(|el| el <= 3)
}

fn solve2(input: &Input) -> usize {
    input
        .iter()
        .map(|report| is_roughly_safe(report))
        .filter(|safe| *safe)
        .count()
}

fn is_roughly_safe(report: &Vec<i64>) -> bool {
    is_safe(report) || is_safe_with_one_exception(report)
}

fn is_safe_with_one_exception(report: &Vec<i64>) -> bool {
    for i in 0..report.len() {
        let temp = [&report[0..i], &report[i + 1..report.len()]].concat();
        if is_safe(&temp) {
            return true;
        }
    }
    false
}

type Input = Vec<Vec<i64>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day2.txt");
        let result = solve1(&input);
        assert_eq!(result, 421);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day2.txt");
        let result = solve2(&input);
        assert_eq!(result, 476);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 4);
    }
}
