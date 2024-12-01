use lazy_static::lazy_static;
use regex::Regex;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day1.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lazy_static! {
        static ref re: Regex = Regex::new(r"(?<first>\d+)\s+(?<second>\d+)").unwrap();
    }
    lines
        .iter()
        .map(|line| {
            re.captures(line).map(|capture| {
                (
                    capture["first"].parse::<i64>().unwrap(),
                    capture["second"].parse::<i64>().unwrap(),
                )
            })
        })
        .map(|it| it.unwrap())
        .collect()
}

fn solve1(input: &Input) -> i64 {
    let mut first_list: Vec<i64> = input.iter().map(|it| it.0).collect();
    let mut second_list: Vec<i64> = input.iter().map(|it| it.1).collect();
    first_list.sort();
    second_list.sort();

    first_list.iter().zip(second_list.iter()).map(|it| (it.1 - it.0).abs()).sum()
}

fn solve2(input: &Input) -> usize {
    1
}

type Input = Vec<(i64, i64)>;
