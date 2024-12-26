use std::collections::{HashSet, VecDeque};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day12.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn solve1(input: &Input) -> usize {
    let size = (input.len(), input[0].len());
    let mut prices: Vec<usize> = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if visited.contains(&(i, j)) {
                continue;
            }
            let area_price = add_area(input, &mut visited, (i, j), size);
            prices.push(area_price);
        }
    }

    prices.iter().sum()
}

fn add_area(
    input: &Input,
    visited: &mut HashSet<(usize, usize)>,
    start: (usize, usize),
    size: (usize, usize),
) -> usize {
    let color = input[start.0][start.1];
    let mut borders: Vec<usize> = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);
    while let Some((x, y)) = queue.pop_front() {
        let mut fence_size = 0;
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if nx < size.0 && ny < size.1 && input[nx][ny] == color {
                    if !visited.contains(&(nx, ny)) {
                        visited.insert((nx, ny));
                        queue.push_back((nx, ny));
                    }
                } else {
                    fence_size += 1;
                }
            } else {
                fence_size += 1;
            }
        }
        borders.push(fence_size);
    }

    borders.len() * borders.iter().sum::<usize>()
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
        let input = get_input("./files/day12.txt");
        let result = solve1(&input);
        // assert_eq!(result, );
    }

    #[test]
    fn test_solve1_small_testdata() {
        let input = get_input("./files/test_small.txt");
        let result = solve1(&input);
        assert_eq!(result, 772);
    }

    #[test]
    fn test_solve1_smallest_testdata() {
        let input = get_input("./files/test_smallest.txt");
        let result = solve1(&input);
        assert_eq!(result, 140);
    }

    #[test]
    fn test_solve1_large_testdata() {
        let input = get_input("./files/test_large.txt");
        let result = solve1(&input);
        assert_eq!(result, 1930);
    }
}
