use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    usize,
};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day18.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {:?}", solve2(&input));
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
    let mut grid = vec![vec![false; input.size.0]; input.size.1];

    for &byte in &input.bytes[0..input.num_bytes] {
        grid[byte.1][byte.0] = true;
    }

    get_path_length(&grid, input.size).unwrap()
}

fn solve2(input: &Input) -> (usize, usize) {
    let mut grid = vec![vec![false; input.size.0]; input.size.1];

    for &byte in &input.bytes {
        grid[byte.1][byte.0] = true;
        if get_path_length(&grid, input.size).is_none() {
            return (byte.0, byte.1);
        }
    }

    panic!("Path is never blocked.")
}

fn get_path_length(grid: &Vec<Vec<bool>>, size: (usize, usize)) -> Option<usize> {
    let (width, height) = size;
    let end = (width - 1, height - 1);
    let mut visited = vec![false; width * height];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0, 0)));

    while let Some(Reverse((distance, x, y))) = heap.pop() {
        if (x, y) == end {
            return Some(distance);
        }

        let index = y * width + x;
        if visited[index] {
            continue;
        };
        visited[index] = true;

        for (nx, ny) in get_neighbours(x, y, size) {
            let n_index = ny * width + nx;
            if !grid[nx][ny] && !visited[n_index] {
                heap.push(Reverse((distance + 1, nx, ny)));
            }
        }
    }

    None
}

fn get_neighbours(x: usize, y: usize, size: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::with_capacity(4);
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if x < size.0 - 1 {
        neighbours.push((x + 1, y));
    }
    if y < size.1 - 1 {
        neighbours.push((x, y + 1));
    }
    neighbours
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

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day18.txt");
        let result = solve2(&input);
        assert_eq!(result, (8, 51));
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, (6, 1));
    }
}
