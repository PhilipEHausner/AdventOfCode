use core::panic;
use std::usize;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input_part1("./files/day9.txt");
    println!("Solution part 1: {}", solve1(&input));
    let input2 = get_input_part2("./files/day9.txt");
    println!("Solution part 2: {}", solve2(&input2));
}

fn get_input_part1(filename: &str) -> Input {
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

fn get_input_part2(filename: &str) -> Input2 {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    let mut file_num = 0;
    let mut is_file = true;
    let mut result = vec![];

    for c in lines.first().unwrap().chars() {
        let len: usize = c.to_string().parse().unwrap();
        if is_file {
            result.push(FileEntry::File(len, file_num));
            file_num += 1;
        } else {
            result.push(FileEntry::Empty(len));
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

fn solve2(input: &Input2) -> usize {
    let mut file_desc = input.clone();

    let mut end = input.len().saturating_sub(1);

    while end > 0 {
        while end > 0 && matches!(file_desc[end], FileEntry::Empty(_)) {
            end -= 1;
        }
        let (cn, cnum) = match file_desc[end] {
            FileEntry::Empty(_) => panic!(),
            FileEntry::File(n, num) => (n, num),
        };
        let mut start = 0;
        while start < end {
            if let FileEntry::Empty(n) = file_desc[start] {
                if n < cn {
                    start += 1;
                    continue;
                }
                file_desc.remove(start);
                file_desc.remove(end - 1);
                file_desc.insert(end - 1, FileEntry::Empty(cn));
                file_desc.insert(start, FileEntry::File(cn, cnum));
                if n > cn {
                    file_desc.insert(start + 1, FileEntry::Empty(n - cn));
                }
                break;
            }
            start += 1;
        }
        end -= 1;
    }

    serialize(&file_desc)
        .iter()
        .enumerate()
        .fold(0, |result, (i, entry)| result + i * entry)
}

fn serialize(file_desc: &Vec<FileEntry>) -> Vec<usize> {
    file_desc
        .iter()
        .flat_map(|f| match f {
            FileEntry::Empty(n) => vec![0; *n],
            FileEntry::File(n, num) => vec![*num; *n],
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Empty,
    File(usize),
}

type Input = Vec<Entry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileEntry {
    Empty(usize),
    File(usize, usize),
}
type Input2 = Vec<FileEntry>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input_part1("./files/day9.txt");
        let result = solve1(&input);
        assert_eq!(result, 6262891638328);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input_part1("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_solve2() {
        let input = get_input_part2("./files/day9.txt");
        let result = solve2(&input);
        assert_eq!(result, 6287317016845);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input_part2("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 2858);
    }
}
