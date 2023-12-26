use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    ops,
};

use util::read_files::read_file_as_vector;

use regex::Regex;

fn main() {
    let input = get_input("./files/day18.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Input {
    let lines = read_file_as_vector(file).expect("Cannot read file.");
    let input_regex =
        Regex::new(r"(?<dir>[ULDR])\s(?<steps>\d+)\s\(#(?<code>[0-9a-f]+)\)").unwrap();
    lines
        .iter()
        .map(|line| {
            input_regex
                .captures(line)
                .map(|capture| {
                    let dir = match &capture["dir"] {
                        "U" => Direction::Up,
                        "L" => Direction::Left,
                        "D" => Direction::Down,
                        "R" => Direction::Right,
                        _ => panic!("Unknown direction '{}'.", capture["dir"].to_string()),
                    };
                    let steps = capture["steps"].parse().unwrap();
                    let code = capture["code"].to_string();
                    Step::new(dir, steps, code)
                })
                .unwrap()
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut vertices = get_polygon_vertices(input);
    vertices = translate_vertices_to_pos_coords(&vertices);
    let enclosing = get_enclosing_fields(&vertices);
    let total = get_total_fields(&vertices);
    total - enclosing
}

fn solve2(input: &Input) -> usize {
    1
}

fn get_polygon_vertices(input: &Input) -> HashSet<Coords> {
    let mut vertices = HashSet::new();
    let mut current = Coords::new(0, 0);
    vertices.insert(current);

    input.iter().for_each(|step| {
        let s = match step.dir {
            Direction::Up => Coords::up(),
            Direction::Left => Coords::left(),
            Direction::Down => Coords::down(),
            Direction::Right => Coords::right(),
        };
        (0..step.steps).for_each(|_| {
            current += s;
            vertices.insert(current);
        });
    });
    vertices
}

fn translate_vertices_to_pos_coords(vertices: &HashSet<Coords>) -> HashSet<Coords> {
    let trans_x = vertices.iter().map(|coord| coord.x).min().unwrap() - 1;
    let trans_y = vertices.iter().map(|coord| coord.y).min().unwrap() - 1;
    let t = Coords::new(trans_x, trans_y);
    vertices.iter().map(|coord| *coord - t).collect()
}

fn get_enclosing_fields(vertices: &HashSet<Coords>) -> usize {
    let mut result = 0;
    let mc = get_max_coords(&vertices) + Coords::new(1, 1);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let options = vec![
        Coords::up(),
        Coords::down(),
        Coords::left(),
        Coords::right(),
    ];
    queue.push_back(Coords::new(0, 0));

    while let Some(next) = queue.pop_front() {
        if visited.contains(&next)
            || next.x < 0
            || next.y < 0
            || next.x > mc.x
            || next.y > mc.y
            || vertices.contains(&next)
        {
            continue;
        }
        result += 1;
        options.iter().for_each(|c| queue.push_back(next + *c));
        visited.insert(next);
    }

    result
}

fn get_total_fields(vertices: &HashSet<Coords>) -> usize {
    let mc = get_max_coords(&vertices) + Coords::new(2, 2);
    mc.x as usize * mc.y as usize
}

fn get_max_coords(vertices: &HashSet<Coords>) -> Coords {
    Coords::new(
        vertices.iter().map(|coord| coord.x).max().unwrap(),
        vertices.iter().map(|coord| coord.y).max().unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }

    pub fn up() -> Coords {
        Coords::new(-1, 0)
    }

    pub fn down() -> Coords {
        Coords::new(1, 0)
    }

    pub fn left() -> Coords {
        Coords::new(0, -1)
    }

    pub fn right() -> Coords {
        Coords::new(0, 1)
    }
}

impl ops::Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

struct Step {
    dir: Direction,
    steps: usize,
    code: String,
}

impl Step {
    pub fn new(dir: Direction, steps: usize, code: String) -> Step {
        Step { dir, steps, code }
    }
}

type Input = Vec<Step>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day18.txt");
        let result = solve1(&input);
        assert_eq!(result, 106459);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day18.txt");
        let result = solve2(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 1);
    }
}
