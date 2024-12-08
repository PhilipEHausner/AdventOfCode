use regex::Regex;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day3.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lines
}

fn solve1(input: &Input) -> i64 {
    input.iter().map(|line| process_line(line)).sum()
}

fn process_line(line: &str) -> i64 {
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();

    re.captures_iter(line)
        .map(|capture| {
            let left: i64 = capture["first"].parse().unwrap();
            let right: i64 = capture["second"].parse().unwrap();
            left * right
        })
        .sum()
}

fn solve2(input: &Input) -> usize {
    1
}

type Input = Vec<String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day3.txt");
        let result = solve1(&input);
        assert_eq!(result, 161289189);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 161);
    }
}
