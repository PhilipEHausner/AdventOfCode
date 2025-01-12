use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day20.txt");
    println!("Solution part 1: {}", solve1(&input, 100));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn solve1(input: &Input, time_save: usize) -> usize {
    let base_distance = dijkstra(input);
    let mut input_ = input.clone();
    let mut cheat_distances = vec![];
    for x in 0..input.len() {
        for y in 0..input[0].len() {
            if input[x][y] == '#' {
                input_[x][y] = '.';
                cheat_distances.push(dijkstra(&input_));
                input_[x][y] = '#';
            }
        }
    }
    cheat_distances
        .iter()
        .map(|dist| base_distance - dist)
        .filter(|&s| s >= time_save)
        .count()
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

fn dijkstra(input: &Input) -> usize {
    let start_pos = get_start_position(input).unwrap();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start_pos)));
    let mut visited = HashSet::new();

    while let Some(Reverse((distance, pos))) = heap.pop() {
        if input[pos.0][pos.1] == 'E' {
            return distance;
        }
        visited.insert(pos);
        for pos in get_next_regular_positions(input, pos, &visited) {
            heap.push(Reverse((distance + 1, pos)));
        }
    }

    panic!("No path found.")
}

fn get_next_regular_positions(
    input: &Input,
    pos: (usize, usize),
    visited: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let mut positions = Vec::with_capacity(4);
    if x > 0 && (input[x - 1][y] == '.' || input[x - 1][y] == 'E') && !visited.contains(&(x - 1, y))
    {
        positions.push((x - 1, y));
    }
    if y > 0 && (input[x][y - 1] == '.' || input[x][y - 1] == 'E') && !visited.contains(&(x, y - 1))
    {
        positions.push((x, y - 1));
    }
    if x < input.len() - 1
        && (input[x + 1][y] == '.' || input[x + 1][y] == 'E')
        && !visited.contains(&(x + 1, y))
    {
        positions.push((x + 1, y));
    }
    if y < input[0].len() - 1
        && (input[x][y + 1] == '.' || input[x][y + 1] == 'E')
        && !visited.contains(&(x, y + 1))
    {
        positions.push((x, y + 1));
    }
    positions
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
}
