use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day16.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn solve1(input: &Input) -> usize {
    let mut visited = HashSet::new();
    let start_pos = get_start_position(input).unwrap();
    let mut queue: HashMap<(usize, usize, Direction), usize> =
        HashMap::from([((start_pos.0, start_pos.1, Direction::East), 0)]);

    while let Some(next_node) = get_next_node(&mut queue) {
        let (x, y, direction) = next_node.0;
        if input[x][y] == 'E' {
            return next_node.1;
        }
        visited.insert((x, y, direction));
        for ((dx, dy, new_direction), penalty) in direction.vectors() {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if visited.contains(&(nx, ny, new_direction)) || input[nx][ny] == '#' {
                continue;
            }
            let new_value = next_node.1 + penalty + 1;
            insert_if_lower(&mut queue, (nx, ny, new_direction), new_value);
        }
    }

    panic!("End node was not reached.")
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

fn get_next_node(
    queue: &mut HashMap<(usize, usize, Direction), usize>,
) -> Option<((usize, usize, Direction), usize)> {
    let min_entry = queue.iter().min_by_key(|&(_, &value)| value);
    if let Some((&k, &v)) = min_entry {
        queue.remove(&k);
        Some((k, v))
    } else {
        None
    }
}

fn insert_if_lower<T>(queue: &mut HashMap<T, usize>, key: T, value: usize)
where
    T: Eq + Hash,
{
    queue
        .entry(key)
        .and_modify(|existing| {
            if value < *existing {
                *existing = value; // Update if the new value is lower
            }
        })
        .or_insert(value); // Insert if the key does not exist
}

fn solve2(input: &Input) -> usize {
    1
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn vectors(&self) -> Vec<((isize, isize, Direction), usize)> {
        match self {
            Direction::East => vec![
                ((0, 1, Direction::East), 0),
                ((1, 0, Direction::South), 1000),
                ((-1, 0, Direction::North), 1000),
            ],
            Direction::West => vec![
                ((0, -1, Direction::West), 0),
                ((1, 0, Direction::South), 1000),
                ((-1, 0, Direction::North), 1000),
            ],
            Direction::South => vec![
                ((1, 0, Direction::South), 0),
                ((0, 1, Direction::East), 1000),
                ((0, -1, Direction::West), 1000),
            ],
            Direction::North => vec![
                ((-1, 0, Direction::North), 0),
                ((0, 1, Direction::East), 1000),
                ((0, -1, Direction::West), 1000),
            ],
        }
    }
}

type Input = Vec<Vec<char>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day16.txt");
        let result = solve1(&input);
        assert_eq!(result, 91464);
    }

    #[test]
    fn test_solve1_test_1_data() {
        let input = get_input("./files/test_1.txt");
        let result = solve1(&input);
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_solve1_test_2_data() {
        let input = get_input("./files/test_2.txt");
        let result = solve1(&input);
        assert_eq!(result, 11048);
    }
}
