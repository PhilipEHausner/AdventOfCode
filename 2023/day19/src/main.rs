use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day19.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    let workflow_lines = lines
        .iter()
        .filter(|line| !line.starts_with("{") && !line.is_empty());
    let rating_lines = lines
        .iter()
        .filter(|line| line.starts_with("{") && !line.is_empty());

    let workflows = parse_workflows(workflow_lines);
    let ratings = parse_ratings(rating_lines);

    Input { workflows, ratings }
}

fn parse_workflows<'a, I>(lines: I) -> Workflows
where
    I: Iterator<Item = &'a String>,
{
    let content_regex = Regex::new(r"(?<name>\w+)\{(?<content>[^\}]+)\}").unwrap();
    let mut workflows = HashMap::new();
    lines.for_each(|line| {
        content_regex.captures(&line).iter().for_each(|capture| {
            let name = capture["name"].to_string();
            let rules = parse_rules(&capture["content"]);
            workflows.insert(name, rules);
        })
    });
    workflows
}

fn parse_rules(line: &str) -> Vec<Rule> {
    lazy_static! {
        static ref ENTRY_REGEX: Regex =
            Regex::new(r"(?<part>\w+)(?<comp>[<>])(?<limit>\d+):(?<target>\w+)|(?<transition>\w+)")
                .unwrap();
    }
    line.split(",")
        .map(|entry| {
            let capture = ENTRY_REGEX.captures(entry).unwrap();

            match &capture.name("transition") {
                Some(s) => Rule {
                    condition: None,
                    target: Target::new(s.as_str()),
                },
                None => {
                    let part = Part::new(&capture["part"]);
                    let comparator = Comparator::new(&capture["comp"]).unwrap();
                    let value = capture["limit"].parse().unwrap();
                    let target = Target::new(&capture["target"]);
                    let condition = Some(Condition {
                        part,
                        comparator,
                        value,
                    });
                    Rule { condition, target }
                }
            }
        })
        .collect()
}

fn parse_ratings<'a, I>(lines: I) -> Vec<Rating>
where
    I: Iterator<Item = &'a String>,
{
    let xmas_regex = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
    lines
        .map(|line| {
            let capture = xmas_regex.captures(line).expect("Invalid rating line.");
            Rating {
                x: capture["x"].parse().unwrap(),
                m: capture["m"].parse().unwrap(),
                a: capture["a"].parse().unwrap(),
                s: capture["s"].parse().unwrap(),
            }
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    input
        .ratings
        .iter()
        .map(|rating| elve_sort(&input.workflows, rating))
        .sum()
}

fn elve_sort(workflows: &Workflows, rating: &Rating) -> usize {
    let mut next_workflow = "in";

    'o: loop {
        let workflow = &workflows[next_workflow];
        for rule in workflow {
            match rule.apply(rating) {
                Some(target) => {
                    match target {
                        Target::Accept => return rating.sum(),
                        Target::Reject => return 0,
                        Target::Next(n) => next_workflow = n.as_str(),
                    }
                    continue 'o;
                }
                None => (),
            }
        }
    }
}

fn solve2(input: &Input) -> usize {
    1
}

#[derive(Debug)]
enum Comparator {
    Lower,
    Higher,
}

impl Comparator {
    pub fn new(comparator: &str) -> Result<Comparator, &str> {
        return match comparator {
            "<" => Ok(Comparator::Lower),
            ">" => Ok(Comparator::Higher),
            _ => Err("Unknown comparator '{comparator}'."),
        };
    }
}

#[derive(Debug)]
enum Part {
    X,
    M,
    A,
    S,
}

impl Part {
    pub fn new(s: &str) -> Part {
        match s {
            "x" => Part::X,
            "m" => Part::M,
            "a" => Part::A,
            "s" => Part::S,
            _ => panic!("Unknown part '{s}'."),
        }
    }
}

#[derive(Debug)]
struct Condition {
    part: Part,
    comparator: Comparator,
    value: usize,
}

impl Condition {
    pub fn apply(&self, rating: &Rating) -> bool {
        let part = match self.part {
            Part::X => rating.x,
            Part::M => rating.m,
            Part::A => rating.a,
            Part::S => rating.s,
        };
        match self.comparator {
            Comparator::Lower => part < self.value,
            Comparator::Higher => part > self.value,
        }
    }
}

#[derive(Debug)]
enum Target {
    Accept,
    Reject,
    Next(String),
}

impl Target {
    pub fn new(s: &str) -> Target {
        match s {
            "A" => Target::Accept,
            "R" => Target::Reject,
            s => Target::Next(s.to_string()),
        }
    }
}

struct Rule {
    condition: Option<Condition>,
    target: Target,
}

impl Rule {
    pub fn apply(&self, rating: &Rating) -> Option<&Target> {
        match &self.condition {
            Some(cond) => match cond.apply(rating) {
                true => Some(&self.target),
                false => None,
            },
            None => Some(&self.target),
        }
    }
}

struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    pub fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

struct Input {
    workflows: Workflows,
    ratings: Vec<Rating>,
}

type Workflow = Vec<Rule>;
type Workflows = HashMap<String, Workflow>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day19.txt");
        let result = solve1(&input);
        assert_eq!(result, 19114);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 397643);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day19.txt");
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
