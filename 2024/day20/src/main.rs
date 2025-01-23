use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day20.txt");
    println!("Solution part 1: {}", solve1(&input, 10));
    println!("Solution part 2: {}", solve2(&input, 100));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn solve1(input: &Input, time_save: usize) -> usize {
    solve(input, time_save, 2)
}

fn solve2(input: &Input, time_save: usize) -> usize {
    solve(input, time_save, 20)
}

fn solve(input: &Input, time_save: usize, cheat_distance: usize) -> usize {
    let mut cheat_distances = vec![];
    let (from_start, to_end) = compute_distances(input);
    let start_position = get_start_position(input).unwrap();
    let base_distance = to_end[&start_position];
    for x in 0..input.len() {
        for y in 0..input[0].len() {
            if input[x][y] != '#' && input[x][y] != 'E' {
                let cheat_targets = get_cheat_targets(input, (x, y), cheat_distance);
                for target in cheat_targets {
                    let d =
                        from_start[&(x, y)] + manhattan_distance((x, y), target) + to_end[&target];
                    cheat_distances.push(d);
                }
            }
        }
    }

    cheat_distances
        .iter()
        .filter(|&dist| *dist < base_distance)
        .map(|dist| base_distance - dist)
        .filter(|&s| s >= time_save)
        .count()
}

fn compute_distances(
    input: &Input,
) -> (
    HashMap<(usize, usize), usize>,
    HashMap<(usize, usize), usize>,
) {
    let start_pos = get_start_position(input).unwrap();
    let mut heap = BinaryHeap::new();
    let mut distances_from_start = HashMap::new();
    heap.push(Reverse((0, start_pos)));
    let mut visited = HashSet::new();

    while let Some(Reverse((distance, pos))) = heap.pop() {
        if !distances_from_start.contains_key(&pos) {
            distances_from_start.insert(pos, distance);
        }
        if input[pos.0][pos.1] == 'E' {
            let distances_to_end = distances_from_start
                .iter()
                .map(|(&(x, y), v)| ((x, y), distance - v))
                .collect();
            return (distances_from_start, distances_to_end);
        }
        visited.insert(pos);
        for next_pos in get_next_regular_positions(input, pos, &visited) {
            heap.push(Reverse((distance + 1, next_pos)));
        }
    }

    panic!("No path found.")
}

fn get_cheat_targets(
    input: &Input,
    pos: (usize, usize),
    max_distance: usize,
) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for x in 0..input.len() {
        for y in 0..input[0].len() {
            if input[x][y] == '#' || input[x][y] == 'S' {
                continue;
            }
            let dist = manhattan_distance((x, y), pos);
            if dist <= max_distance && dist > 0 {
                result.push((x, y));
            }
        }
    }
    result
}

fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn get_start_position(input: &Input) -> Result<(usize, usize), ()> {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, el)| if *el == 'S' { Some((i, j)) } else { None })
        })
        .next()
        .ok_or(())
}

fn get_next_regular_positions(
    input: &Input,
    pos: (usize, usize),
    visited: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    get_next_positions(input, pos, visited, HashSet::from(['.', 'E']))
}

fn get_next_positions(
    input: &Input,
    pos: (usize, usize),
    visited: &HashSet<(usize, usize)>,
    valid_characters: HashSet<char>,
) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let mut positions = Vec::with_capacity(4);
    if x > 0 && valid_characters.contains(&input[x - 1][y]) && !visited.contains(&(x - 1, y)) {
        positions.push((x - 1, y));
    }
    if y > 0 && valid_characters.contains(&input[x][y - 1]) && !visited.contains(&(x, y - 1)) {
        positions.push((x, y - 1));
    }
    if x < input.len() - 1
        && valid_characters.contains(&input[x + 1][y])
        && !visited.contains(&(x + 1, y))
    {
        positions.push((x + 1, y));
    }
    if y < input[0].len() - 1
        && valid_characters.contains(&input[x][y + 1])
        && !visited.contains(&(x, y + 1))
    {
        positions.push((x, y + 1));
    }
    positions
}

type Input = Vec<Vec<char>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day20.txt");
        let result = solve1(&input, 100);
        assert_eq!(result, 1327);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input, 10);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day20.txt");
        let result = solve2(&input, 100);
        assert_eq!(result, 985737);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input, 76);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_same_point() {
        let p1 = (0, 0);
        let p2 = (0, 0);
        assert_eq!(manhattan_distance(p1, p2), 0);
    }

    #[test]
    fn test_horizontal_distance() {
        let p1 = (2, 3);
        let p2 = (5, 3);
        assert_eq!(manhattan_distance(p1, p2), 3);
    }

    #[test]
    fn test_vertical_distance() {
        let p1 = (1, 1);
        let p2 = (1, 6);
        assert_eq!(manhattan_distance(p1, p2), 5);
    }

    #[test]
    fn test_diagonal_distance() {
        let p1 = (1, 1);
        let p2 = (4, 5);
        assert_eq!(manhattan_distance(p1, p2), 7);
    }

    #[test]
    fn test_reverse_diagonal_distance() {
        let p1 = (4, 5);
        let p2 = (1, 1);
        assert_eq!(manhattan_distance(p1, p2), 7);
    }

    #[test]
    fn test_large_coordinates() {
        let p1 = (1000, 2000);
        let p2 = (3000, 4000);
        assert_eq!(manhattan_distance(p1, p2), 4000);
    }
}
