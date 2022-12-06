use std::collections::HashMap;

use util::read_files::read_file_as_vector;

fn main() {
    let message = get_message("./files/day6.txt");
    let solution1 = solve1(&message);
    println!("Solution part 1: {}", solution1);
    let solution2 = solve2(&message);
    println!("Solution part 2: {}", solution2);
}

fn solve1(input: &str) -> u64 {
    solve_generic(input, 4)
}

fn solve2(input: &str) -> u64 {
    solve_generic(input, 14)
}

fn solve_generic(input: &str, num_chars: u64) -> u64 {
    let mut result = 0;
    let mut last_x_chars = HashMap::<char, u64>::new();

    for (i, c) in input.chars().enumerate() {
        update_last_x_chars(&mut last_x_chars, &c, i, input, num_chars);
        if i > (num_chars - 1) as usize && last_x_chars_were_unique(&last_x_chars) {
            result = (i + 1) as u64;
            break;
        }
    }

    result
}

fn get_message(filename: &str) -> String {
    let lines = read_file_as_vector(filename).expect("Error reading file.");
    lines[0].clone()
}

fn update_last_x_chars(
    last_x_chars: &mut HashMap<char, u64>,
    c: &char,
    index: usize,
    input: &str,
    num_chars: u64,
) {
    last_x_chars.insert(*c, last_x_chars.get(c).unwrap_or(&0) + 1);
    if index < num_chars as usize {
        return;
    }
    let char_to_remove = input.chars().nth(index - num_chars as usize).unwrap();
    last_x_chars.insert(
        char_to_remove,
        last_x_chars
            .get(&char_to_remove)
            .expect("Bug: last_x_chars non-existing key is accessed.")
            - 1,
    );
    if last_x_chars[&char_to_remove] == 0 {
        last_x_chars.remove(&char_to_remove);
    }
}

fn last_x_chars_were_unique(last_x_chars: &HashMap<char, u64>) -> bool {
    for (_, value) in last_x_chars {
        if *value > 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
