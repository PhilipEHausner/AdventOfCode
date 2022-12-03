mod item_priorities;

use item_priorities::get_item_priority;
use util::read_files::read_file_as_vector;

fn main() {
    let result1 = solve1("./files/day3.txt");
    println!("Solution problem 1: {}", result1);
}

fn solve1(filename: &str) -> u64 {
    let backpacks = read_file_as_vector(filename).expect("Error reading file.");

    backpacks.into_iter()
        .map(|backpack| priority_of_double_item(&backpack))
        .sum()
}

fn priority_of_double_item(backpack: &String) -> u64 {
    let compartment1 = &backpack[..backpack.len() / 2];
    let compartment2 = &backpack[backpack.len() / 2..];

    for c in compartment1.chars() {
        if compartment2.contains(c) { return get_item_priority(c); }
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
        assert_eq!(priority_of_double_item(&"abcgbf".to_string()), 2);
        assert_eq!(priority_of_double_item(&"aXpoqcgbXf".to_string()), 50);
        assert_eq!(priority_of_double_item(&"uhgfylkpoy".to_string()), 25);
        assert_eq!(priority_of_double_item(&"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()), 38);
    }
}

