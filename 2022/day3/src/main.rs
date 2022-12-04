mod item_priorities;

use std::collections::HashSet;

use item_priorities::get_item_priority;
use util::read_files::read_file_as_vector;

fn main() {
    let result1 = solve1("./files/day3.txt");
    println!("Solution problem 1: {}", result1);
    let result2 = solve2("./files/day3.txt");
    println!("Solution problem 2: {}", result2);
}

fn solve1(filename: &str) -> u64 {
    let backpacks = read_file_as_vector(filename).expect("Error reading file.");

    backpacks
        .into_iter()
        .map(|backpack| priority_of_double_item(&backpack))
        .sum()
}

fn priority_of_double_item(backpack: &str) -> u64 {
    let compartment1 = &backpack[..backpack.len() / 2];
    let compartment2 = &backpack[backpack.len() / 2..];

    for item in compartment1.chars() {
        if compartment2.contains(item) {
            return get_item_priority(item);
        }
    }

    0
}

fn solve2(filename: &str) -> u64 {
    let backpacks = read_file_as_vector(filename).expect("Error reading file.");
    debug_assert_eq!(backpacks.len() % 3, 0);

    let mut result = 0;
    for i in (0..backpacks.len()).step_by(3) {
        result += priority_of_group_badge(&backpacks[i], &backpacks[i + 1], &backpacks[i + 2]);
    }

    result
}

fn priority_of_group_badge(backpack1: &str, backpack2: &str, backpack3: &str) -> u64 {
    let items2: HashSet<char> = HashSet::from_iter(backpack2.chars());
    let items3: HashSet<char> = HashSet::from_iter(backpack3.chars());

    for item in backpack1.chars() {
        if items2.contains(&item) && items3.contains(&item) {
            return get_item_priority(item);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("./files/test.txt"), 157);
    }

    #[test]
    fn test_priority_of_double_item() {
        assert_eq!(priority_of_double_item("abcgbf"), 2);
        assert_eq!(priority_of_double_item("aXpoqcgbXf"), 50);
        assert_eq!(priority_of_double_item("uhgfylkpoy"), 25);
        assert_eq!(
            priority_of_double_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            38
        );
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2("./files/test.txt"), 70);
    }

    #[test]
    fn test_priority_of_group_badge() {
        assert_eq!(
            priority_of_group_badge(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            18
        );
        assert_eq!(priority_of_group_badge("abcdef", "poejkfqwe", "mnbwe"), 5);
    }
}
