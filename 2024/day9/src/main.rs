use std::result;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day9.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    let mut file_num = 0;
    let mut is_file = true;
    let mut result = vec![];

    for c in lines.first().unwrap().chars() {
        let len: usize = c.to_string().parse().unwrap();
        if is_file {
            result.extend(vec![Entry::File(file_num); len]);
            file_num += 1;
        } else {
            result.extend(vec![Entry::Empty; len]);
        }
        is_file = !is_file;
    }

    result
}

fn solve1(input: &Input) -> usize {
    let mut file_desc = input.clone();

    let (mut start, mut end) = (0, input.len().saturating_sub(1));

    while start < end {
        while start < end && file_desc[start] != Entry::Empty {
            start += 1;
        }
        while end > start && file_desc[end] == Entry::Empty {
            end -= 1;
        }
        if start < end {
            file_desc.swap(start, end);
        }
    }

    file_desc.iter().enumerate().fold(0, |result, (i, entry)| {
        if let Entry::File(n) = entry {
            result + i * n
        } else {
            result
        }
    })
}

fn solve2(input: &Input) -> usize {
    1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Empty,
    File(usize),
}

type Input = Vec<Entry>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day9.txt");
        let result = solve1(&input);
        assert_eq!(result, 6262891638328);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 1928);
    }
}
