use std::usize;

use regex::Regex;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day14.txt");
    let test_board_size = (11, 7);
    let day_board_size = (101, 103);
    println!("Solution part 1: {}", solve1(&input, &day_board_size));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    let re = Regex::new(r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    lines.iter().map(|line| parse_robot(line, &re)).collect()
}

fn parse_robot(line: &str, re: &Regex) -> Robot {
    let captures = re.captures(line).unwrap();
    let pos = (
        captures["px"].parse().unwrap(),
        captures["py"].parse().unwrap(),
    );
    let direction = (
        captures["vx"].parse().unwrap(),
        captures["vy"].parse().unwrap(),
    );
    Robot::new(pos, direction)
}

fn solve1(input: &Input, board_size: &V2D) -> usize {
    let mut robots = input.clone();
    for _ in 0..100 {
        robots
            .iter_mut()
            .for_each(|robot| robot.teleport(board_size));
    }
    safety_factor(&robots, board_size)
}

fn safety_factor(robots: &Input, board_size: &V2D) -> usize {
    // Might only work for boards with uneven rows and columns
    let border_vertical = (board_size.0 as f32 / 2.).floor() as i32;
    let border_horizontal = (board_size.1 as f32 / 2.).floor() as i32;

    let mut quadrants = (0, 0, 0, 0);

    robots.iter().for_each(|robot| {
        if robot.pos.0 < border_vertical {
            if robot.pos.1 < border_horizontal {
                quadrants.0 += 1;
            }
            if robot.pos.1 > border_horizontal {
                quadrants.1 += 1;
            }
        } else if robot.pos.0 > border_vertical {
            if robot.pos.1 < border_horizontal {
                quadrants.2 += 1;
            }
            if robot.pos.1 > border_horizontal {
                quadrants.3 += 1;
            }
        }
    });

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn solve2(input: &Input) -> usize {
    1
}

type V2D = (i32, i32);

#[derive(Clone, Debug)]
struct Robot {
    pos: V2D,
    direction: V2D,
}

impl Robot {
    pub fn new(pos: V2D, direction: V2D) -> Robot {
        Robot { pos, direction }
    }

    pub fn teleport(&mut self, board_size: &V2D) {
        let x = (self.pos.0 + self.direction.0).rem_euclid(board_size.0);
        let y = (self.pos.1 + self.direction.1).rem_euclid(board_size.1);
        self.pos = (x, y);
    }
}

type Input = Vec<Robot>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day14.txt");
        let result = solve1(&input, &(101, 103));
        assert_eq!(result, 219512160);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input, &(11, 7));
        assert_eq!(result, 12);
    }
}
