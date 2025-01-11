use std::collections::{HashMap, HashSet};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day18.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    let temp = lines
        .first()
        .unwrap()
        .split(",")
        .map(|el| el.parse::<usize>().unwrap())
        .collect();
    let size = to_tuple(temp);
    let num_bytes = lines[1].parse().unwrap();
    let bytes = lines
        .iter()
        .skip(2)
        .map(|line| {
            line.split(",")
                .map(|el| el.parse::<usize>().unwrap())
                .collect()
        })
        .map(|pair| to_tuple(pair))
        .collect();
    GridInfo {
        size,
        num_bytes,
        bytes,
    }
}

fn to_tuple(pair: Vec<usize>) -> (usize, usize) {
    assert!(pair.len() == 2);
    (pair[0], pair[1])
}

fn solve1(input: &Input) -> usize {
    let size = input.size;
    let mut grid = vec![vec![false; size.0]; size.1];

    for &byte in &input.bytes[0..input.num_bytes] {
        grid[byte.1][byte.0] = true;
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = PrioQueue {
        queue: HashMap::from([((0, 0), 0)]),
    };
    let end = (size.1 - 1, size.0 - 1);

    while let Some(((x, y), distance)) = queue.next() {
        if (x, y) == end {
            return distance;
        }
        visited.insert((x, y));

        for (dx, dy) in get_directions(x, y, size) {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if !grid[nx][ny] && !visited.contains(&(nx, ny)) {
                queue.update((nx, ny), distance + 1);
            }
        }
    }

    panic!("Cannot find path.");
}

fn get_directions(x: usize, y: usize, size: (usize, usize)) -> Vec<(isize, isize)> {
    let mut directions = Vec::with_capacity(4);
    if x > 0 {
        directions.push((-1, 0));
    }
    if y > 0 {
        directions.push((0, -1));
    }
    if x < size.0 - 1 {
        directions.push((1, 0));
    }
    if y < size.1 - 1 {
        directions.push((0, 1));
    }
    directions
}

fn solve2(input: &Input) -> usize {
    1
}

fn print_grid(grid: &Vec<Vec<bool>>, visited: &HashSet<(usize, usize)>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] {
                print!("#");
            } else if visited.contains(&(i, j)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

struct PrioQueue {
    queue: HashMap<(usize, usize), usize>,
}

impl PrioQueue {
    pub fn next(&mut self) -> Option<((usize, usize), usize)> {
        let min_entry = self.queue.iter().min_by_key(|&(_, value)| value);
        if let Some((&k, &v)) = min_entry {
            self.queue.remove(&k);
            Some((k, v))
        } else {
            None
        }
    }

    pub fn update(&mut self, pos: (usize, usize), distance: usize) {
        self.queue
            .entry(pos)
            .and_modify(|existing| {
                if distance < *existing {
                    *existing = distance;
                }
            })
            .or_insert(distance);
    }
}

struct GridInfo {
    size: (usize, usize),
    num_bytes: usize,
    bytes: Vec<(usize, usize)>,
}

type Input = GridInfo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day18.txt");
        let result = solve1(&input);
        assert_eq!(result, 454);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 22);
    }
}
