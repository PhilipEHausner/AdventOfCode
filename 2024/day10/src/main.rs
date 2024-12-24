use std::collections::{HashSet, VecDeque};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day10.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut result = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 0 {
                result += dijkstra_adaptive((i, j), input);
            }
        }
    }

    result
}

fn dijkstra_adaptive(start: (usize, usize), input: &Input) -> usize {
    let mut result = 0;
    let (height, width) = (input.len(), input[0].len());
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some((x, y)) = queue.pop_front() {
        let value = input[x][y];
        if value == 9 {
            result += 1;
            continue;
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if nx < height
                    && ny < width
                    && !visited.contains(&(nx, ny))
                    && input[nx][ny] == value + 1
                {
                    visited.insert((nx, ny));
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    result
}

fn solve2(input: &Input) -> usize {
    let mut result = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 0 {
                result += dijkstra_adaptive_part2((i, j), input);
            }
        }
    }

    result
}

fn dijkstra_adaptive_part2(start: (usize, usize), input: &Input) -> usize {
    let mut result = 0;
    let (height, width) = (input.len(), input[0].len());
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some((x, y)) = queue.pop_front() {
        visited.insert((x, y));
        let value = input[x][y];
        if value == 9 {
            result += 1;
            continue;
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if nx < height && ny < width && input[nx][ny] == value + 1 {
                    visited.insert((nx, ny));
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    result
}

type Input = Vec<Vec<i8>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day10.txt");
        let result = solve1(&input);
        assert_eq!(result, 611);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 36);
    }
    #[test]
    fn test_solve2() {
        let input = get_input("./files/day10.txt");
        let result = solve2(&input);
        assert_eq!(result, 1380);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 81);
    }
}
