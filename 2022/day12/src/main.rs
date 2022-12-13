use std::collections::VecDeque;

use util::read_files::read_file_as_vector;

struct PuzzleInput(Vec<Vec<u8>>, (usize, usize), (usize, usize));

fn main() {
    let lines = read_file_as_vector("./files/day12.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    let puzzle_input = parse_puzzle_input(lines);
    shortest_path(&puzzle_input.0, puzzle_input.1, puzzle_input.2)
}

fn solve2(lines: &Vec<String>) -> u64 {
    let PuzzleInput(grid, _, end_point) = parse_puzzle_input(lines);
    let start_points = get_all_zero_elevation_points(&grid);
    let mut distances = vec![];

    for start_point in start_points {
        distances.push(shortest_path(&grid, start_point, end_point));
    }

    *distances.iter().min().unwrap()
}

fn parse_puzzle_input(lines: &Vec<String>) -> PuzzleInput {
    let start_point = get_character_position_in_grid(lines, 'S');
    let end_point = get_character_position_in_grid(lines, 'E');
    let grid = parse_input_grid(lines);
    PuzzleInput(grid, start_point, end_point)
}

fn get_character_position_in_grid(lines: &Vec<String>, character: char) -> (usize, usize) {
    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            if line.contains(character) {
                (idx as i32, line.find(character).unwrap() as i32)
            } else {
                (-1, -1)
            }
        })
        .filter(|&(x, _)| x != -1)
        .map(|(x, y)| (x as usize, y as usize))
        .last()
        .unwrap()
}

fn parse_input_grid(lines: &Vec<String>) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|el| match el {
                    'S' => 0,
                    'E' => 25,
                    _ => el as u8 - 'a' as u8,
                })
                .collect()
        })
        .collect()
}

fn get_all_zero_elevation_points(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &val)| {
                    if val == 0 {
                        (i as i32, j as i32)
                    } else {
                        (-1, -1)
                    }
                })
                .filter(|&(x, _)| x != -1)
                .map(|(x, y)| (x as usize, y as usize))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect()
}

fn shortest_path(
    grid: &Vec<Vec<u8>>,
    start_point: (usize, usize),
    end_point: (usize, usize),
) -> u64 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut distances = vec![vec![u64::MAX; grid[0].len()]; grid.len()];
    let mut input_queue = VecDeque::from([(start_point, 0)]);

    let (height, width) = (grid.len(), grid[0].len());

    while input_queue.len() > 0 {
        let ((x, y), dist) = input_queue.pop_front().unwrap();

        if visited[x][y] {
            continue;
        }

        distances[x][y] = dist;
        visited[x][y] = true;

        if x > 0 && !visited[x - 1][y] && grid[x][y] + 1 >= grid[x - 1][y] {
            input_queue.push_back(((x - 1, y), dist + 1));
        }
        if x < height - 1 && !visited[x + 1][y] && grid[x][y] + 1 >= grid[x + 1][y] {
            input_queue.push_back(((x + 1, y), dist + 1));
        }
        if y > 0 && !visited[x][y - 1] && grid[x][y] + 1 >= grid[x][y - 1] {
            input_queue.push_back(((x, y - 1), dist + 1));
        }
        if y < width - 1 && !visited[x][y + 1] && grid[x][y] + 1 >= grid[x][y + 1] {
            input_queue.push_back(((x, y + 1), dist + 1));
        }
    }

    distances[end_point.0][end_point.1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 31);
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 29);
    }

    #[test]
    fn test_get_character_position_in_grid() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(get_character_position_in_grid(&lines, 'S'), (0, 0));
        assert_eq!(get_character_position_in_grid(&lines, 'E'), (2, 5));
    }
}
