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
    input.iter().map(|line| process_line_part1(line)).sum()
}

fn process_line_part1(line: &str) -> i64 {
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();

    re.captures_iter(line)
        .map(|capture| {
            let left: i64 = capture["first"].parse().unwrap();
            let right: i64 = capture["second"].parse().unwrap();
            left * right
        })
        .sum()
}

fn solve2(input: &Input) -> i64 {
    process_line_part2(&input.join(""))
}

fn process_line_part2(line: &str) -> i64 {
    let re = Regex::new(r"(^|do\(\)).*?(don't\(\)|$)").unwrap();
    re.find_iter(line)
        .map(|capture| process_line_part1(capture.as_str()))
        .sum()
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

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day3.txt");
        let result = solve2(&input);
        assert_eq!(result, 83595109);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test2.txt");
        let result = solve2(&input);
        assert_eq!(result, 48);
    }
}
