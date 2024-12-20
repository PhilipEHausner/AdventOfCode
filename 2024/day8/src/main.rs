use std::collections::{HashMap, HashSet};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day8.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    lines.iter().map(|line| line.chars().collect()).collect()
}

fn solve1(input: &Input) -> usize {
    let antennas = get_antenna_position_map(input);
    let mut antinodes = HashSet::new();
    let (height, width) = (input.len() as i64, input.first().unwrap().len() as i64);

    for positions in antennas.values() {
        for (i, &(x1, y1)) in positions.iter().enumerate() {
            for &(x2, y2) in positions.iter().skip(i + 1) {
                let (dx, dy) = (x2 - x1, y2 - y1);

                for &(xn, yn) in &[(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)] {
                    if (0..height).contains(&xn)
                        && (0..width).contains(&yn)
                        && !positions.contains(&(xn, yn))
                    {
                        antinodes.insert((xn, yn));
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn get_antenna_position_map(input: &Input) -> HashMap<char, Vec<(i64, i64)>> {
    let mut antennas = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((i as i64, j as i64));
            }
        }
    }

    antennas
}

fn solve2(input: &Input) -> usize {
    1
}

type Input = Vec<Vec<char>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day8.txt");
        let result = solve1(&input);
        assert_eq!(result, 323);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 14);
    }
}
