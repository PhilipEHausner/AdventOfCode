use core::fmt;
use std::{thread::current, usize};

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day15.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    let (first, second) = split_on_empty_string(&lines);

    let fields = first
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Field::Wall,
                    '.' => Field::Empty,
                    'O' => Field::Box,
                    '@' => Field::Robot,
                    _ => panic!("Unexpected char {}", c),
                })
                .collect()
        })
        .collect();

    let directions = second
        .iter()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => panic!("Unexpected char {}", c),
            })
        })
        .collect();

    (fields, directions)
}

fn split_on_empty_string(vec: &Vec<String>) -> (Vec<String>, Vec<String>) {
    if let Some(pos) = vec.iter().position(|line| line.is_empty()) {
        let before = vec[..pos].to_vec();
        let after = vec[pos + 1..].to_vec();
        (before, after)
    } else {
        (vec.to_vec(), Vec::new())
    }
}

fn solve1(input: &Input) -> usize {
    let (mut fields, directions) = (input.0.to_vec(), input.1.to_vec());

    let mut robot_pos = get_robot_position(&fields).unwrap();
    for direction in directions {
        robot_pos = move_field(&robot_pos, &mut fields, &direction).0;
    }

    sum_box_gps_coordinates(&fields)
}

fn get_robot_position(fields: &Fields) -> Result<(usize, usize), ()> {
    fields
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter().enumerate().find_map(|(j, el)| {
                if *el == Field::Robot {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .next()
        .ok_or(())
}

fn move_field(
    pos: &(usize, usize),
    fields: &mut Fields,
    direction: &Direction,
) -> ((usize, usize), bool) {
    let current = fields[pos.0][pos.1];
    let (height, width) = (fields.len() as isize, fields[0].len() as isize);
    let (dx, dy) = direction.to_vec();
    let (nx, ny) = (pos.0 as isize + dx, pos.1 as isize + dy);

    if nx < 0 || ny < 0 || nx >= height || ny >= width {
        return (*pos, false);
    }

    let (nx, ny) = (nx as usize, ny as usize);

    let success = match fields[nx][ny] {
        Field::Wall => false,
        Field::Empty => true,
        Field::Box => move_field(&(nx, ny), fields, direction).1,
        Field::Robot => panic!("Cannot happen."),
    };

    if success {
        fields[nx][ny] = current;
        fields[pos.0][pos.1] = Field::Empty;
    }
    let new_pos = if success { (nx, ny) } else { (pos.0, pos.1) };

    (new_pos, success)
}

fn sum_box_gps_coordinates(fields: &Fields) -> usize {
    fields
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(move |(j, f)| if *f == Field::Box { 100 * i + j } else { 0 })
        })
        .sum()
}

fn solve2(input: &Input) -> usize {
    1
}

fn print_fields(fields: &Fields) {
    fields.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Wall,
    Empty,
    Box,
    Robot,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Self::Wall => "#",
            Self::Empty => ".",
            Self::Box => "O",
            Self::Robot => "@",
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vec(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
        }
    }
}

type Fields = Vec<Vec<Field>>;
type Input = (Fields, Vec<Direction>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day15.txt");
        let result = solve1(&input);
        assert_eq!(result, 1465152);
    }

    #[test]
    fn test_solve1_small_testdata() {
        let input = get_input("./files/test_small.txt");
        let result = solve1(&input);
        assert_eq!(result, 2028);
    }

    #[test]
    fn test_solve1_large_testdata() {
        let input = get_input("./files/test_large.txt");
        let result = solve1(&input);
        assert_eq!(result, 10092);
    }
}
