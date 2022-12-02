use std::fs::File;
use std::io;
use util::read_files::read_lines;

fn main() {
    let elve_inventories = build_elve_inventories_from_file("./src/day1.txt");
    let result1 = solve1(&elve_inventories);
    println!("Solution problem 1: {}", result1);

    let result2 = solve2(&elve_inventories);
    println!("Solution problem 2: {}", result2);
}

fn build_elve_inventories_from_file(filename: &str) -> Vec<Vec<u64>> {
    let lines = read_lines(filename).expect("File cannot be loaded.");
    build_elve_inventories_from_lines(lines)
}

fn build_elve_inventories_from_lines(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<u64>> {
    let mut inventories = Vec::new();
    let mut next_elve_inventory = Vec::new();

    for line in lines {
        let content = line.expect("Line is malformed.");
        match content.as_str() {
            "" if next_elve_inventory.len() > 0 => {
                inventories.push(next_elve_inventory);
                next_elve_inventory = Vec::<u64>::new();
            },
            _ => {
                let calories = content.parse::<u64>().expect("Malformed number.");
                next_elve_inventory.push(calories);
            },
        }
    }

    if next_elve_inventory.len() > 0 {
        inventories.push(next_elve_inventory);
    }
    

    inventories
}

fn solve1(elve_inventories: &Vec<Vec<u64>>) -> u64 {
    elve_inventories.iter()
        .map(|inventory| inventory.iter().sum())
        .max()
        .expect("max cannot be computed.")
}

fn solve2(elve_inventories: &Vec<Vec<u64>>) -> u64 {
    let mut calories_carried = elve_inventories.iter()
        .map(|inventory| inventory.iter().sum())
        .collect::<Vec<u64>>();
    calories_carried.sort_by(|a, b| b.cmp(a));
    calories_carried[..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let elve_inventories = [[1, 4, 5].to_vec(), [2, 1].to_vec(), [20, 2, 4].to_vec()].to_vec();
        assert_eq!(solve1(&elve_inventories), 26);
    }

    #[test]
    fn test_solve2() {
        let elve_inventories = [[1, 4, 5].to_vec(), [2, 1].to_vec(), [20, 2, 4].to_vec(), [1, 2, 4].to_vec(), [4, 5, 4].to_vec()].to_vec();
        assert_eq!(solve2(&elve_inventories), 49);
    }
}
