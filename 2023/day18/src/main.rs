use core::panic;
use std::ops;

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
    let vertices = get_polygon_vertices(input);
    solve(&vertices)
}

fn solve2(input: &Input) -> usize {
    let vertices = get_polygon_vertices_part2(input);
    solve(&vertices)
}

fn solve(vertices: &Vec<Coords>) -> usize {
    assert!(vertices[0] == vertices[vertices.len() - 1]);
    let area = get_area(vertices) as usize;
    let boundary = get_boundary_length(vertices) as usize;
    let inner = area - boundary / 2 + 1;
    inner + boundary
}

fn get_polygon_vertices(input: &Input) -> Vec<Coords> {
    let mut vertices = vec![];
    let mut current = Coords::new(0, 0);
    vertices.push(current);

    input.iter().for_each(|step| {
        let s = match step.dir {
            Direction::Up => Coords::up() * step.steps as i64,
            Direction::Left => Coords::left() * step.steps as i64,
            Direction::Down => Coords::down() * step.steps as i64,
            Direction::Right => Coords::right() * step.steps as i64,
        };
        current += s;
        vertices.push(current);
    });
    vertices
}

fn get_polygon_vertices_part2(input: &Input) -> Vec<Coords> {
    let mut vertices = vec![];
    let mut current = Coords::new(0, 0);
    vertices.push(current);

    input.iter().for_each(|step| {
        let steps = i64::from_str_radix(&step.code[0..5], 16).unwrap();
        let s = match step.code.chars().nth(5).unwrap() {
            '0' => Coords::right() * steps,
            '1' => Coords::down() * steps,
            '2' => Coords::left() * steps,
            '3' => Coords::up() * steps,
            _ => panic!("Unknown direction."),
        };
        current += s;
        vertices.push(current);
    });
    vertices
}

fn get_area(vertices: &Vec<Coords>) -> i64 {
    let mut area = 0;

    for i in 0..(vertices.len() - 1) {
        let v1 = vertices[i];
        let v2 = vertices[i + 1];
        area += v1.x * v2.y - v1.y * v2.x;
    }
    area /= 2;

    area.abs()
}

fn get_boundary_length(vertices: &Vec<Coords>) -> i64 {
    let mut boundary = 0;

    for i in 0..(vertices.len() - 1) {
        let v1 = vertices[i];
        let v2 = vertices[i + 1];
        boundary += ((v1.x - v2.x) + (v1.y - v2.y)).abs();
    }

    boundary
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    pub fn new(x: i64, y: i64) -> Coords {
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

impl ops::Mul<i64> for Coords {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Coords {
            x: self.x * rhs,
            y: self.y * rhs,
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
    fn test_solve1_testdata2() {
        let input = get_input("./files/test2.txt");
        let result = solve1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day18.txt");
        let result = solve2(&input);
        assert_eq!(result, 63806916814808);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 952408144115);
    }
}
