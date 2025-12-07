use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day3.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut result = 0;

    for bank in input {
        result += bank_output_joltage(bank);
    }

    result
}

fn solve2(input: &Input) -> usize {
    let mut result = 0;

    for bank in input {
        result += bank_output_joltage_general(bank, 12);
    }

    result
}

fn bank_output_joltage(bank: &Vec<u8>) -> usize {
    let (mut first, mut second): (u8, u8) = (0, 0);

    for i in 0..(bank.len() - 1) {
        let element = *bank.get(i).unwrap();
        if element > first {
            first = element;
            second = 0;
        } else if element > second {
            second = element;
        }
    }

    if *bank.last().unwrap() > second {
        second = *bank.last().unwrap();
    }

    first as usize * 10 + second as usize
}

fn bank_output_joltage_general(bank: &Vec<u8>, num_elements: usize) -> usize {
    let mut elements = vec![];

    let mut start = 0;
    for i in 0..num_elements {
        let (idx, num) = bank[start..bank.len() - num_elements + i + 1]
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                if b > a {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .unwrap();
        start = start + idx + 1;
        elements.push(*num as usize * 10usize.pow(num_elements as u32 - i as u32 - 1));
    }

    elements.iter().sum()
}

type Input = Vec<Vec<u8>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day3.txt");
        let result = solve1(&input);
        assert_eq!(result, 17074);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day3.txt");
        let result = solve2(&input);
        assert_eq!(result, 169512729575727);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 3121910778619);
    }
}
