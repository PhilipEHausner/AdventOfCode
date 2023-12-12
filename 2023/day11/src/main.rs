use itertools::Itertools;
use std::collections::HashMap;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day11.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Input {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    lines
        .iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    _ => panic!("Unknown space '{c}'."),
                })
                .collect()
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    solve(input, 2)
}

fn solve2(input: &Input) -> usize {
    solve(input, 1_000_000)
}

fn solve(input: &Input, expansion_factor: usize) -> usize {
    let se_x: HashMap<usize, usize> = get_space_expansion_x(input, expansion_factor);
    let se_y: HashMap<usize, usize> = get_space_expansion_y(input, expansion_factor);
    let galaxy_positions = get_galaxy_positions(input);
    galaxy_positions
        .iter()
        .combinations(2)
        .map(|combination| {
            if combination.len() != 2 {
                panic!("Bug");
            }
            let el1 = combination[0];
            let el2 = combination[1];
            se_x.get(&el1.0)
                .unwrap()
                .abs_diff(*se_x.get(&el2.0).unwrap())
                + se_y
                    .get(&el1.1)
                    .unwrap()
                    .abs_diff(*se_y.get(&el2.1).unwrap())
        })
        .sum()
}

fn get_space_expansion_x(input: &Input, expansion_factor: usize) -> HashMap<usize, usize> {
    let mut curr = 0;
    let mut result = HashMap::new();
    for i in 0..input.len() {
        if input[i].iter().all(|el| el == &Space::Empty) {
            curr += expansion_factor;
        } else {
            curr += 1;
        }
        result.insert(i, curr);
    }
    result
}

fn get_space_expansion_y(input: &Input, expansion_factor: usize) -> HashMap<usize, usize> {
    let mut curr = 0;
    let mut result = HashMap::new();

    let rows = input.len();
    let cols = input[0].len();

    let col_iter = (0..cols).map(|row_idx| input.iter().flatten().skip(row_idx).step_by(rows));

    col_iter.enumerate().for_each(|(i, col)| {
        if col.into_iter().all(|el| el == &Space::Empty) {
            curr += expansion_factor;
        } else {
            curr += 1;
        }
        result.insert(i, curr);
    });
    result
}

fn get_galaxy_positions(input: &Input) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .map(|(i, row)| {
            Some(
                row.iter()
                    .enumerate()
                    .filter_map(|(j, el)| {
                        if el == &Space::Galaxy {
                            Some((i, j))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(usize, usize)>>(),
            )
            .unwrap()
        })
        .flatten()
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Space {
    Empty,
    Galaxy,
}

type Input = Vec<Vec<Space>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day11.txt");
        let result = solve1(&input);
        assert_eq!(result, 9563821);
    }

    #[test]
    fn test_solve1_testdata1() {
        let input = get_input("./files/test1.txt");
        let result = solve1(&input);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_solve_10_times_testdata() {
        let input = get_input("./files/test1.txt");
        let result = solve(&input, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_solve_100_times_testdata() {
        let input = get_input("./files/test1.txt");
        let result = solve(&input, 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day11.txt");
        let result = solve2(&input);
        assert_eq!(result, 827009909817);
    }
}
