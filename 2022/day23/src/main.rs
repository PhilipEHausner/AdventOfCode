use std::collections::{HashMap, VecDeque};

use util::read_files::read_file_as_vector;

pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn move_in_direction(&self, pos: Position) -> Position {
        match *self {
            Direction::North => Position {
                x: pos.x - 1,
                y: pos.y,
            },
            Direction::East => Position {
                x: pos.x,
                y: pos.y + 1,
            },
            Direction::South => Position {
                x: pos.x + 1,
                y: pos.y,
            },
            Direction::West => Position {
                x: pos.x,
                y: pos.y - 1,
            },
        }
    }

    pub fn is_valid_direction(&self, pos: &Position, grid: &ElveGrid) -> bool {
        match *self {
            Direction::North => Direction::_north_is_valid_direction(pos, grid),
            Direction::East => Direction::_east_is_valid_direction(pos, grid),
            Direction::South => Direction::_south_is_valid_direction(pos, grid),
            Direction::West => Direction::_west_is_valid_direction(pos, grid),
        }
    }

    fn _north_is_valid_direction(pos: &Position, grid: &ElveGrid) -> bool {
        grid[pos.x - 1][pos.y - 1] == false
            && grid[pos.x - 1][pos.y] == false
            && grid[pos.x - 1][pos.y + 1] == false
    }

    fn _east_is_valid_direction(pos: &Position, grid: &ElveGrid) -> bool {
        grid[pos.x - 1][pos.y + 1] == false
            && grid[pos.x][pos.y + 1] == false
            && grid[pos.x + 1][pos.y + 1] == false
    }

    fn _south_is_valid_direction(pos: &Position, grid: &ElveGrid) -> bool {
        grid[pos.x + 1][pos.y - 1] == false
            && grid[pos.x + 1][pos.y] == false
            && grid[pos.x + 1][pos.y + 1] == false
    }

    fn _west_is_valid_direction(pos: &Position, grid: &ElveGrid) -> bool {
        grid[pos.x - 1][pos.y - 1] == false
            && grid[pos.x][pos.y - 1] == false
            && grid[pos.x + 1][pos.y - 1] == false
    }
}

struct DirectionProposer {
    directions: VecDeque<Direction>,
}

impl DirectionProposer {
    pub fn new(directions: Vec<Direction>) -> DirectionProposer {
        DirectionProposer {
            directions: VecDeque::from_iter(directions),
        }
    }

    pub fn propose_next_position(
        &self,
        position: (usize, usize),
        grid: &ElveGrid,
    ) -> (usize, usize) {
        let pos = Position {
            x: position.0,
            y: position.1,
        };

        if DirectionProposer::_is_hermit(&pos, grid) {
            return position;
        }

        for direction in &self.directions {
            if direction.is_valid_direction(&pos, grid) {
                let next_pos = direction.move_in_direction(pos);
                return (next_pos.x, next_pos.y);
            }
        }

        position
    }

    fn _is_hermit(pos: &Position, grid: &ElveGrid) -> bool {
        grid[pos.x - 1][pos.y - 1] == false
            && grid[pos.x - 1][pos.y] == false
            && grid[pos.x - 1][pos.y + 1] == false
            && grid[pos.x][pos.y - 1] == false
            && grid[pos.x][pos.y + 1] == false
            && grid[pos.x + 1][pos.y - 1] == false
            && grid[pos.x + 1][pos.y] == false
            && grid[pos.x + 1][pos.y + 1] == false
    }

    pub fn shift_directions(&mut self) {
        let first_direction = self.directions.pop_front().unwrap();
        self.directions.push_back(first_direction);
    }
}

type ElveGrid = VecDeque<VecDeque<bool>>;

fn main() {
    let lines = read_file_as_vector("./files/day23.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines, 10));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>, rounds: usize) -> u64 {
    let mut elve_grid = create_elve_grid(lines);
    let mut direction_proposer = create_direction_proposer();

    for _ in 0..rounds {
        process_round(&mut elve_grid, &mut direction_proposer);
    }

    get_empty_ground_tiles(&elve_grid)
}

fn solve2(lines: &Vec<String>) -> u64 {
    let mut elve_grid = create_elve_grid(lines);
    let mut direction_proposer = create_direction_proposer();

    let mut rounds = 0;
    loop {
        rounds += 1;
        let no_moves = process_round(&mut elve_grid, &mut direction_proposer);
        if no_moves {
            break;
        }
    }

    rounds
}

#[allow(dead_code)]
fn print_elve_grid(elve_grid: &ElveGrid) {
    for row in elve_grid {
        for col in row {
            if *col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn create_elve_grid(lines: &Vec<String>) -> ElveGrid {
    let mut grid = VecDeque::from_iter(vec![
        VecDeque::from_iter(vec![false; lines[0].len()]);
        lines.len()
    ]);

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                grid[row][col] = true;
            }
        }
    }

    grid
}

fn create_direction_proposer() -> DirectionProposer {
    DirectionProposer::new(vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ])
}

fn process_round(elve_grid: &mut ElveGrid, direction_proposer: &mut DirectionProposer) -> bool {
    pad_grid(elve_grid);
    let mut move_to_hashmap: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for row in 1..(elve_grid.len() - 1) {
        for col in 1..(elve_grid[row].len() - 1) {
            if !elve_grid[row][col] {
                continue;
            }
            let move_to = direction_proposer.propose_next_position((row, col), elve_grid);
            let origins = move_to_hashmap.get_mut(&move_to);
            match origins {
                Some(value) => value.push((row, col)),
                None => {
                    move_to_hashmap.insert(move_to, vec![(row, col)]);
                }
            }
        }
    }

    *elve_grid = VecDeque::from_iter(vec![
        VecDeque::from_iter(vec![false; elve_grid[0].len()]);
        elve_grid.len()
    ]);

    let mut no_moves = true;
    for (target, origins) in move_to_hashmap.iter() {
        if origins.len() == 1 {
            elve_grid[target.0][target.1] = true;
            if target.0 != origins[0].0 || target.1 != origins[0].1 {
                no_moves = false;
            }
        } else {
            for origin in origins {
                elve_grid[origin.0][origin.1] = true;
            }
        }
    }

    direction_proposer.shift_directions();
    no_moves
}

fn pad_grid(elve_grid: &mut ElveGrid) {
    if elve_grid[0].contains(&true) {
        elve_grid.push_front(VecDeque::from_iter(vec![false; elve_grid[0].len()]));
    }

    if elve_grid[elve_grid.len() - 1].contains(&true) {
        elve_grid.push_back(VecDeque::from_iter(vec![false; elve_grid[0].len()]));
    }

    if elve_grid.iter().map(|el| el[0]).any(|el| el) {
        for row in elve_grid.iter_mut() {
            row.push_front(false);
        }
    }

    if elve_grid.iter().map(|el| el[el.len() - 1]).any(|el| el) {
        for row in elve_grid.iter_mut() {
            row.push_back(false);
        }
    }
}

fn get_empty_ground_tiles(elve_grid: &ElveGrid) -> u64 {
    let mut min_row = 0;
    for i in 0..elve_grid.len() {
        if elve_grid[i].contains(&true) {
            min_row = i;
            break;
        }
    }

    let mut max_row = 0;
    for i in (0..elve_grid.len()).rev() {
        if elve_grid[i].contains(&true) {
            max_row = i;
            break;
        }
    }

    let mut min_col = elve_grid[0].len();
    let mut max_col = 0;

    for row in elve_grid {
        min_col = std::cmp::min(
            min_col,
            row.iter().position(|&el| el).unwrap_or(elve_grid[0].len()),
        );
        max_col = std::cmp::max(max_col, row.iter().rposition(|&el| el).unwrap_or(0));
    }

    let num_elves: usize = elve_grid
        .iter()
        .map(|row| row.iter().filter(|&&el| el).count())
        .sum::<usize>();

    ((max_row - min_row + 1) * (max_col - min_col + 1) - num_elves) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines, 10), 110);
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 20);
    }
}
