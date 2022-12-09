use std::collections::HashSet;

use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day9.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert((0, 0));

    for line in lines {
        process_move(line, &mut visited, &mut head, &mut tail);
    }

    visited.len() as u64
}

fn process_move(
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
                if !is_tail_next_to_head(head, tail) {
                    move_tail(visited, tail, (head.0, head.1 + 1));
                }
            }
        }
        "D" => {
            for _ in 0..distance {
                head.1 += 1;
                if !is_tail_next_to_head(head, tail) {
                    move_tail(visited, tail, (head.0, head.1 - 1));
                }
            }
        }
        "R" => {
            for _ in 0..distance {
                head.0 += 1;
                if !is_tail_next_to_head(head, tail) {
                    move_tail(visited, tail, (head.0 - 1, head.1));
                }
            }
        }
        "L" => {
            for _ in 0..distance {
                head.0 -= 1;
                if !is_tail_next_to_head(head, tail) {
                    move_tail(visited, tail, (head.0 + 1, head.1));
                }
            }
        }
        _ => panic!("Invalid movement"),
    }
}

fn is_tail_next_to_head(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    let x_diff = (head.0 - tail.0).abs();
    let y_diff = (head.1 - tail.1).abs();
    x_diff <= 1 && y_diff <= 1
}

fn move_tail(visited: &mut HashSet<(i32, i32)>, tail: &mut (i32, i32), target: (i32, i32)) {
    visited.insert(target);
    tail.0 = target.0;
    tail.1 = target.1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 13);
    }
}
