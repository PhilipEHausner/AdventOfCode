use std::collections::HashSet;

use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day9.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert((0, 0));

    for line in lines {
        process_move_head_tail(line, &mut visited, &mut head, &mut tail);
    }

    visited.len() as u64
}

fn solve2(lines: &Vec<String>) -> u64 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut knots = vec![(0, 0); 10];
    visited.insert((0, 0));

    for line in lines {
        process_move_multiple_knots(line, &mut visited, &mut knots);
    }

    visited.len() as u64
}

fn process_move_head_tail(
    line: &String,
    visited: &mut HashSet<(i32, i32)>,
    head: &mut (i32, i32),
    tail: &mut (i32, i32),
) {
    let mov = line.split(" ").collect::<Vec<&str>>();
    let direction = mov[0];
    let distance = mov[1]
        .parse::<u32>()
        .expect("Distance is not a valid number.");

    match direction {
        "U" => {
            for _ in 0..distance {
                head.1 -= 1;
                process_knot(tail, *head, visited, true);
            }
        }
        "D" => {
            for _ in 0..distance {
                head.1 += 1;
                process_knot(tail, *head, visited, true);
            }
        }
        "R" => {
            for _ in 0..distance {
                head.0 += 1;
                process_knot(tail, *head, visited, true);
            }
        }
        "L" => {
            for _ in 0..distance {
                head.0 -= 1;
                process_knot(tail, *head, visited, true);
            }
        }
        _ => panic!("Invalid movement"),
    }
}

fn process_move_multiple_knots(
    line: &String,
    visited: &mut HashSet<(i32, i32)>,
    knots: &mut Vec<(i32, i32)>,
) {
    let mov = line.split(" ").collect::<Vec<&str>>();
    let direction = mov[0];
    let distance = mov[1]
        .parse::<u32>()
        .expect("Distance is not a valid number.");
    let num_knots = knots.len();

    match direction {
        "U" => {
            for _ in 0..distance {
                knots[0].1 -= 1;
                for ind in 1..knots.len() {
                    let target = (knots[ind - 1].0, knots[ind - 1].1);
                    let is_tail_knot = is_tail(num_knots, ind);
                    process_knot(&mut knots[ind], target, visited, is_tail_knot);
                }
            }
        }
        "D" => {
            for _ in 0..distance {
                knots[0].1 += 1;
                for ind in 1..knots.len() {
                    let target = (knots[ind - 1].0, knots[ind - 1].1);
                    let is_tail_knot = is_tail(num_knots, ind);
                    process_knot(&mut knots[ind], target, visited, is_tail_knot);
                }
            }
        }
        "R" => {
            for _ in 0..distance {
                knots[0].0 += 1;
                for ind in 1..knots.len() {
                    let target = (knots[ind - 1].0, knots[ind - 1].1);
                    let is_tail_knot = is_tail(num_knots, ind);
                    process_knot(&mut knots[ind], target, visited, is_tail_knot);
                }
            }
        }
        "L" => {
            for _ in 0..distance {
                knots[0].0 -= 1;
                for ind in 1..knots.len() {
                    let target = (knots[ind - 1].0, knots[ind - 1].1);
                    let is_tail_knot = is_tail(num_knots, ind);
                    process_knot(&mut knots[ind], target, visited, is_tail_knot);
                }
            }
        }
        _ => panic!("Invalid movement"),
    }
}

fn is_tail(num_knots: usize, curr_index: usize) -> bool {
    num_knots - 1 == curr_index
}


fn process_knot(knot: &mut (i32, i32), target: (i32, i32), visited: &mut HashSet<(i32, i32)>, is_tail_knot: bool) {
    move_knot_if_necessary(knot, target);
    if is_tail_knot {
        visited.insert(*knot);
    }
}

fn move_knot_if_necessary(knot: &mut (i32, i32), target: (i32, i32)) {
    if knots_are_neighbours(knot, &target) {
        return;
    }
    if knot.0 - target.0 > 0 {
        knot.0 -= 1;
    } else if target.0 - knot.0 > 0 {
        knot.0 += 1;
    }
    if knot.1 - target.1 > 0 {
        knot.1 -= 1;
    } else if target.1 - knot.1 > 0 {
        knot.1 += 1;
    }
}

fn knots_are_neighbours(knot1: &(i32, i32), knot2: &(i32, i32)) -> bool {
    let x_diff = (knot1.0 - knot2.0).abs();
    let y_diff = (knot1.1 - knot2.1).abs();
    x_diff <= 1 && y_diff <= 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1_small() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 13);
    }

    #[test]
    fn test_solve1_dataset() {
        let lines = read_file_as_vector("./files/day9.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 6057);
    }

    #[test]
    fn test_solve2_small() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 1);
    }

    #[test]
    fn test_solve2_large() {
        let lines = read_file_as_vector("./files/large_test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 36);
    }

    #[test]
    fn test_solve2_dataset() {
        let lines = read_file_as_vector("./files/day9.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 2514);
    }
}
