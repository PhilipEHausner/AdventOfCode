use core::panic;
use std::collections::HashSet;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day6.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    let field = lines
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => FieldType::EMPTY,
                    '#' => FieldType::OBSTACLE,
                    '^' => FieldType::EMPTY,
                    _ => panic!("Unknown char '{}'.", c),
                })
                .collect()
        })
        .collect();

    let pos = lines
        .iter()
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(i, line)| line.chars().position(|c| c == '^').map(|pos| (i, pos)))
        .filter(|line| line.is_some())
        .next()
        .unwrap()
        .unwrap();

    Area::new(field, pos, Direction::UP)
}

fn solve1(_input: &Input) -> usize {
    let mut input = _input.clone();
    loop {
        let npos = match next_pos(&input) {
            WithinArea::FALSE => return input.count_visited_fields(),
            WithinArea::TRUE(pos) => pos,
        };
        if is_obstacle(&input, &npos) {
            input.direction = input.direction.turn_right();
        } else {
            input.set_pos(npos);
        }
    }
}

fn next_pos(input: &Input) -> WithinArea {
    let pos = input.pos;
    match input.direction {
        Direction::UP => {
            if input.pos.0 == 0 {
                WithinArea::FALSE
            } else {
                WithinArea::TRUE((pos.0 - 1, pos.1))
            }
        }
        Direction::RIGHT => {
            if input.pos.1 + 1 == input.size.1 {
                WithinArea::FALSE
            } else {
                WithinArea::TRUE((pos.0, pos.1 + 1))
            }
        }
        Direction::DOWN => {
            if input.pos.0 + 1 == input.size.0 {
                WithinArea::FALSE
            } else {
                WithinArea::TRUE((pos.0 + 1, pos.1))
            }
        }
        Direction::LEFT => {
            if input.pos.1 == 0 {
                WithinArea::FALSE
            } else {
                WithinArea::TRUE((pos.0, pos.1 - 1))
            }
        }
    }
}

fn is_obstacle(input: &Input, pos: &(usize, usize)) -> bool {
    input.field[pos.0][pos.1] == FieldType::OBSTACLE
}

fn solve2(input: &Input) -> usize {
    let mut result = 0;
    for i in 0..input.size.0 {
        for j in 0..input.size.1 {
            if has_loop_with_additional_obstacle(input, (i, j)) {
                result += 1;
            }
        }
    }
    result
}

fn has_loop_with_additional_obstacle(_input: &Input, obstacle: (usize, usize)) -> bool {
    if _input.pos == obstacle {
        return false;
    }

    let mut input = _input.clone();
    input.field[obstacle.0][obstacle.1] = FieldType::OBSTACLE;

    loop {
        let npos = match next_pos(&input) {
            WithinArea::FALSE => return false,
            WithinArea::TRUE(pos) => pos,
        };
        if is_obstacle(&input, &npos) {
            let ndirection = input.direction.turn_right();
            if input.already_visited(&input.pos, &ndirection) {
                return true;
            }
            input.direction = ndirection;
        } else {
            if input.already_visited(&npos, &input.direction) {
                return true;
            }
            input.set_pos(npos);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldType {
    EMPTY,
    OBSTACLE,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

enum WithinArea {
    FALSE,
    TRUE((usize, usize)),
}

type Field = Vec<Vec<FieldType>>;

#[derive(Debug, Clone)]
struct Area {
    field: Field,
    pos: (usize, usize),
    direction: Direction,
    size: (usize, usize),
    visited: HashSet<(usize, usize, Direction)>,
}

impl Area {
    pub fn new(field: Field, pos: (usize, usize), direction: Direction) -> Area {
        let size = (field.len(), field[0].len());
        let visited = HashSet::from([(pos.0, pos.1, direction)]);
        Area {
            field,
            pos,
            direction,
            size,
            visited,
        }
    }

    fn set_pos(&mut self, pos: (usize, usize)) {
        self.visited.insert((pos.0, pos.1, self.direction));
        self.pos = pos;
    }

    fn count_visited_fields(&self) -> usize {
        self.visited
            .iter()
            .map(|v| (v.0, v.1))
            .collect::<HashSet<_>>()
            .len()
    }

    fn already_visited(&self, pos: &(usize, usize), direction: &Direction) -> bool {
        self.visited.contains(&(pos.0, pos.1, direction.clone()))
    }
}

type Input = Area;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day6.txt");
        let result = solve1(&input);
        assert_eq!(result, 4826);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day6.txt");
        let result = solve2(&input);
        assert_eq!(result, 1721);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 6);
    }
}
