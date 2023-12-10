use core::panic;
use std::collections::HashSet;

use geo::{Contains, point};
use geo::{coord, Coord, LineString, Polygon};
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day10.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Input {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    lines
        .iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| match c {
                    '|' => Pipe::Vertical,
                    '-' => Pipe::Horizontal,
                    'L' => Pipe::NE,
                    'J' => Pipe::NW,
                    '7' => Pipe::SW,
                    'F' => Pipe::SE,
                    '.' => Pipe::Ground,
                    'S' => Pipe::Start,
                    _ => panic!("Unknown direction '{c}'."),
                })
                .collect()
        })
        .collect()
}

fn solve1(input: &Input) -> u64 {
    let start_pos = get_start_pos(input);
    let mut cycle = None;
    let candidates = get_start_candidates(input, start_pos);
    candidates.into_iter().for_each(|candidate| {
        let temp = get_cycle_steps(input, &candidate, &mut HashSet::from([start_pos]));
        cycle = match temp {
            Some(_) => temp,
            None => cycle,
        };
    });
    cycle.expect("No cycle found.") / 2
}

fn solve2(input: &Input) -> u64 {
    let start_pos = get_start_pos(input);
    let mut cycle = vec![];
    let candidates = get_start_candidates(input, start_pos);
    candidates.first().into_iter().for_each(|candidate| {
        cycle = get_nodes_on_cycle(input, candidate,  &mut HashSet::from([start_pos]));
    });
    get_enclosed_fields(input, &cycle)
}

fn get_start_pos(input: &Input) -> Pos {
    let line = input
        .iter()
        .position(|line| line.contains(&Pipe::Start))
        .unwrap();
    let col = input[line]
        .iter()
        .position(|el| el == &Pipe::Start)
        .unwrap();
    (line, col)
}

fn get_start_candidates(input: &Input, pos: Pos) -> Vec<Pos> {
    let mut candidates = vec![];
    if pos.0 > 0
        && HashSet::from([Pipe::Vertical, Pipe::SW, Pipe::SE]).contains(&input[pos.0 - 1][pos.1])
    {
        candidates.push((pos.0 - 1, pos.1))
    }
    if pos.0 < input.len() - 1
        && HashSet::from([Pipe::Vertical, Pipe::NW, Pipe::NE]).contains(&input[pos.0 + 1][pos.1])
    {
        candidates.push((pos.0 + 1, pos.1))
    }
    if pos.1 > 0
        && HashSet::from([Pipe::Horizontal, Pipe::NE, Pipe::SE]).contains(&input[pos.0][pos.1 - 1])
    {
        candidates.push((pos.0, pos.1 - 1))
    }
    if pos.1 < input[pos.0].len() - 1
        && HashSet::from([Pipe::Horizontal, Pipe::NW, Pipe::SW]).contains(&input[pos.0][pos.1 + 1])
    {
        candidates.push((pos.0, pos.1 + 1))
    }
    candidates
}

fn get_cycle_steps(input: &Input, pos: &Pos, visited: &mut HashSet<Pos>) -> Option<u64> {
    let mut next_pos = pos.clone();
    let mut result = 1;
    loop {
        visited.insert(next_pos);
        result += 1;
        let candidates = match input[next_pos.0][next_pos.1] {
            Pipe::Vertical => vec![(next_pos.0 - 1, next_pos.1), (next_pos.0 + 1, next_pos.1)],
            Pipe::Horizontal => vec![(next_pos.0, next_pos.1 - 1), (next_pos.0, next_pos.1 + 1)],
            Pipe::NE => vec![(next_pos.0 - 1, next_pos.1), (next_pos.0, next_pos.1 + 1)],
            Pipe::NW => vec![(next_pos.0 - 1, next_pos.1), (next_pos.0, next_pos.1 - 1)],
            Pipe::SW => vec![(next_pos.0 + 1, next_pos.1), (next_pos.0, next_pos.1 - 1)],
            Pipe::SE => vec![(next_pos.0 + 1, next_pos.1), (next_pos.0, next_pos.1 + 1)],
            Pipe::Ground => return None,
            Pipe::Start => return Some(result),
        };
        let next: Vec<&Pos> = candidates
            .iter()
            .filter(|p: &&(usize, usize)| !visited.contains(p))
            .collect();
        if next.len() == 0 {
            if candidates.iter().any(|p| input[p.0][p.1] == Pipe::Start) {
                return Some(result); // Include this step and the final step to S.
            }
            panic!("Pipe ended abruptly.");
        }
        if next.len() > 1 {
            panic!("Pipe should have unique direction.");
        }
        next_pos = *next[0];
    }
}

fn get_nodes_on_cycle(input: &Input, pos: &Pos, visited: &mut HashSet<Pos>) -> Vec<(usize, usize)> {
    let mut next_pos = pos.clone();
    let mut result = visited.iter().map(|el| el.clone()).collect::<Vec<Pos>>();
    loop {
        visited.insert(next_pos);
        result.push(next_pos);
        let candidates = match input[next_pos.0][next_pos.1] {
            Pipe::Vertical => vec![(next_pos.0 - 1, next_pos.1), (next_pos.0 + 1, next_pos.1)],
            Pipe::Horizontal => vec![(next_pos.0, next_pos.1 - 1), (next_pos.0, next_pos.1 + 1)],
            Pipe::NE => vec![(next_pos.0 - 1, next_pos.1), (next_pos.0, next_pos.1 + 1)],
            Pipe::NW => vec![(next_pos.0 - 1, next_pos.1), (next_pos.0, next_pos.1 - 1)],
            Pipe::SW => vec![(next_pos.0 + 1, next_pos.1), (next_pos.0, next_pos.1 - 1)],
            Pipe::SE => vec![(next_pos.0 + 1, next_pos.1), (next_pos.0, next_pos.1 + 1)],
            Pipe::Ground => panic!(),
            Pipe::Start => panic!(),
        };
        let next: Vec<&Pos> = candidates
            .iter()
            .filter(|p: &&(usize, usize)| !visited.contains(p))
            .collect();
        if next.len() == 0 {
            if candidates.iter().any(|p| input[p.0][p.1] == Pipe::Start) {
                return result; // Include this step and the final step to S.
            }
            panic!("Pipe ended abruptly.");
        }
        if next.len() > 1 {
            panic!("Pipe should have unique direction.");
        }
        next_pos = *next[0];
    }
}

fn get_enclosed_fields(input: &Input, cycle: &Vec<Pos>) -> u64 {
    let mut result = 0;
    let coords: Vec<Coord<i32>> = cycle.iter().map(|el| {
        coord! {x: el.0 as i32 , y: el.1 as i32}
    }).collect();
    let line_string: LineString<i32> = coords.into();
    
    let polygon = Polygon::new(line_string.clone(), vec![]);
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if polygon.contains(&point!(x: i as i32, y: j as i32)) {
                result += 1;
            }
        }
    }
    
    result 
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

type Input = Vec<Vec<Pipe>>;
type Pos = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day10.txt");
        let result = solve1(&input);
        assert_eq!(result, 6800);
    }

    #[test]
    fn test_solve1_testdata1() {
        let input = get_input("./files/test1.txt");
        let result = solve1(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_solve1_testdata2() {
        let input = get_input("./files/test2.txt");
        let result = solve1(&input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day10.txt");
        let result = solve2(&input);
        assert_eq!(result, 483);
    }

    #[test]
    fn test_solve2_testdata3() {
        let input = get_input("./files/test3.txt");
        let result = solve2(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_solve2_testdata4() {
        let input = get_input("./files/test4.txt");
        let result = solve2(&input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_solve2_testdata5() {
        let input = get_input("./files/test5.txt");
        let result = solve2(&input);
        assert_eq!(result, 10);
    }
}
