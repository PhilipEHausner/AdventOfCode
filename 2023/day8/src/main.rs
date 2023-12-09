use std::collections::HashMap;

use lazy_static::lazy_static;
use num_integer::lcm;
use regex::Regex;
use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day8.txt").expect("Could not read file.");
    println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i64 {
    let mut steps = Steps::new(&lines[0]);
    let mut nodes = Nodes::new(&lines);
    let mut result = 0;
    loop {
        if nodes.current_node == "ZZZ" {
            break;
        }
        nodes.step(&mut steps);
        result += 1;
    }
    result
}

fn solve2(lines: &Vec<String>) -> i64 {
    let nodes = Nodes::new(&lines);
    let keys: Vec<&String> = nodes
        .elements
        .keys()
        .filter(|it| it.ends_with('A'))
        .collect();
    let results: Vec<i64> = keys
        .iter()
        .map(|it| {
            let mut subnodes = nodes.clone();
            subnodes.current_node = it.to_string();
            let mut steps = Steps::new(&lines[0]);
            let mut result = 0;
            loop {
                if subnodes.current_node.ends_with('Z') {
                    break;
                }
                subnodes.step(&mut steps);
                result += 1;
            }
            result
        })
        .collect();
    results.iter().fold(1, |acc, it| lcm(acc, *it))
}

enum Direction {
    Left,
    Right,
}

struct Steps {
    steps: Vec<Direction>,
    current_step: usize,
}

impl Steps {
    pub fn new(line: &str) -> Steps {
        let steps = line
            .chars()
            .into_iter()
            .map(|it| {
                if it == 'R' {
                    Direction::Right
                } else {
                    Direction::Left
                }
            })
            .collect();
        Steps {
            steps,
            current_step: 0,
        }
    }

    pub fn next_step(&mut self) -> &Direction {
        let result = &self.steps[self.current_step];
        self.current_step += 1;
        if self.current_step == self.steps.len() {
            self.current_step = 0;
        }
        result
    }
}

#[derive(Clone)]
struct Nodes {
    pub elements: HashMap<String, Node>,
    current_node: String,
}

impl Nodes {
    pub fn new(lines: &Vec<String>) -> Nodes {
        lazy_static! {
            static ref NODE_REGEX: Regex =
                Regex::new(r"(?<name>\w+)\s+=\s+\((?<left>\w+),\s+(?<right>\w+)\)").unwrap();
        }
        let nodes: Vec<Node> = lines
            .iter()
            .skip(2)
            .filter(|it| it.len() > 0)
            .into_iter()
            .map(|it| {
                NODE_REGEX
                    .captures(it)
                    .map(|capture| Node {
                        name: capture["name"].to_string(),
                        left: capture["left"].to_string(),
                        right: capture["right"].to_string(),
                    })
                    .unwrap()
            })
            .collect::<Vec<Node>>();
        let elements = nodes
            .into_iter()
            .map(|it| (it.name.clone(), it))
            .collect::<HashMap<String, Node>>();
        Nodes {
            elements,
            current_node: "AAA".to_string(),
        }
    }

    pub fn step(&mut self, steps: &mut Steps) {
        let node = &self.elements[&self.current_node];
        let next_node = match steps.next_step() {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        };
        self.current_node = next_node.clone();
    }
}

#[derive(Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = read_file_as_vector("./files/day8.txt").expect("Cannot read file.");
        let result = solve1(&input);
        assert_eq!(result, 14893);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = read_file_as_vector("./files/test.txt").expect("Cannot read file.");
        let result = solve1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve1_testdata2() {
        let input = read_file_as_vector("./files/test2.txt").expect("Cannot read file.");
        let result = solve1(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_solve2() {
        let input = read_file_as_vector("./files/day8.txt").expect("Cannot read file.");
        let result = solve2(&input);
        assert_eq!(result, 10241191004509);
    }

    #[test]
    fn test_solve2_testdata3() {
        let input = read_file_as_vector("./files/test3.txt").expect("Cannot read file.");
        let result = solve2(&input);
        assert_eq!(result, 6);
    }
}
