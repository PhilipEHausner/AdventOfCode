use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day2.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    let line = lines.first().unwrap();
    line.split(',')
        .map(|interval| {
            let parts: Vec<&str> = interval.split('-').collect();
            Interval {
                from: parts.get(0).unwrap().parse().unwrap(),
                to: parts.get(1).unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut result = 0;
    for interval in input {
        for number in interval.from..=interval.to {
            if is_illegal_number(number) {
                result += number as usize;
            }
        }
    }
    result
}

fn solve2(input: &Input) -> usize {
    let mut result = 0;
    for interval in input {
        for number in interval.from..=interval.to {
            if is_illegal_number_part2(number) {
                result += number as usize;
            }
        }
    }
    result
}

fn is_illegal_number(number: u64) -> bool {
    let str = number.to_string();
    if str.len() % 2 != 0 {
        return false;
    }

    if str[..str.len() / 2] == str[str.len() / 2..] {
        return true;
    }

    false
}

fn is_illegal_number_part2(number: u64) -> bool {
    let str_num = number.to_string();

    for l in 1..=(str_num.len() / 2) {
        if str_num.len() % l != 0 {
            continue;
        }

        let mut parts: Vec<&str> = vec![];
        for x in 0..(str_num.len() / l) {
            parts.push(&str_num[x * l..x * l + l]);
        }

        let first_part = parts.first().unwrap();
        if parts.iter().all(|part| part == first_part) {
            return true;
        }
    }

    false
}

struct Interval {
    from: u64,
    to: u64,
}

type Input = Vec<Interval>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day2.txt");
        let result = solve1(&input);
        assert_eq!(result, 54641809925);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day2.txt");
        let result = solve2(&input);
        assert_eq!(result, 73694270688);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 4174379265);
    }
}
