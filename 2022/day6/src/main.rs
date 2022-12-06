use std::collections::HashMap;

use util::read_files::read_file_as_vector;

fn main() {
    let message = get_message("./files/day6.txt");
    let solution1 = solve1(&message);
    println!("Solution part 1: {}", solution1);
}

fn solve1(input: &str) -> u64 {
    let mut result = 0;
    let mut last_four_chars = HashMap::<char, u64>::new();

    for (i, c) in input.chars().enumerate() {
        update_last_four_chars(&mut last_four_chars, &c, i, input);
        if i > 3 && last_four_chars_were_unique(&last_four_chars) {
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

fn update_last_four_chars(
    last_four_chars: &mut HashMap<char, u64>,
    c: &char,
    index: usize,
    input: &str,
) {
    last_four_chars.insert(*c, last_four_chars.get(c).unwrap_or(&0) + 1);
    if index < 4 {
        return;
    }
    let char_to_remove = input.chars().nth(index - 4).unwrap();
    last_four_chars.insert(
        char_to_remove,
        last_four_chars
            .get(&char_to_remove)
            .expect("Bug: last_four_chars non-existing key is accessed.")
            - 1,
    );
    if last_four_chars[&char_to_remove] == 0 {
        last_four_chars.remove(&char_to_remove);
    }
}

fn last_four_chars_were_unique(last_four_chars: &HashMap<char, u64>) -> bool {
    for (_, value) in last_four_chars {
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
}
