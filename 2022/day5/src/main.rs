type Stack = Vec<char>;

#[derive(Debug)]
struct Move {
    pub num_elements: usize,
    pub from: usize,
    pub to: usize,
}

use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day5.txt").expect("Error reading file.");
    let solution1 = solve1(&lines);
    println!("Solution part1: {}", solution1);
    let solution2 = solve2(&lines);
    println!("Solution part2: {}", solution2);
}

fn solve1(lines: &Vec<String>) -> String {
    solve_generic(lines, &crane_solve1)
}

fn solve2(lines: &Vec<String>) -> String {
    solve_generic(lines, &crane_solve2)
}

fn solve_generic(
    lines: &Vec<String>,
    crane_version: &dyn Fn(&mut Vec<Stack>, Vec<Move>),
) -> String {
    let mut stacks = initialize_stacks(lines);
    let moves = create_moves(lines);

    crane_version(&mut stacks, moves);

    build_result_string(stacks)
}

fn initialize_stacks(lines: &Vec<String>) -> Vec<Stack> {
    let first_empty_line = get_first_empty_line(lines);
    let num_stacks = compute_number_of_stacks(lines, first_empty_line);
    let mut stacks = vec![Stack::new(); num_stacks];
    fill_stacks(lines, &mut stacks, first_empty_line, num_stacks);
    stacks
}

fn get_first_empty_line(lines: &Vec<String>) -> usize {
    let mut first_empty_line = 0;
    for line in lines {
        if line.len() == 0 {
            debug_assert!(
                first_empty_line > 1,
                "File malformed: Empty line found too soon."
            );
            return first_empty_line;
        }
        first_empty_line += 1;
    }
    panic!("No empty line found.")
}

fn fill_stacks(
    lines: &Vec<String>,
    stacks: &mut Vec<Stack>,
    first_empty_line: usize,
    num_stacks: usize,
) {
    for i in (0..(first_empty_line - 1)).rev() {
        let current_line = &lines[i];
        fill_stacks_by_line(&current_line, stacks, num_stacks);
    }
}

fn fill_stacks_by_line(line: &String, stacks: &mut Vec<Stack>, num_stacks: usize) {
    for stack_idx in 0..num_stacks {
        let element = line.chars().nth(stack_idx * 4 + 1).unwrap();
        if element != ' ' {
            stacks[stack_idx].push(element);
        }
    }
}

fn compute_number_of_stacks(lines: &Vec<String>, first_empty_line: usize) -> usize {
    lines[first_empty_line - 1]
        .chars()
        .filter(|c| if c.is_whitespace() { false } else { true })
        .collect::<Vec<char>>()
        .len()
}

fn create_moves(lines: &Vec<String>) -> Vec<Move> {
    let mut moves = Vec::new();

    let first_empty_line = get_first_empty_line(lines);
    for line in lines[first_empty_line + 1..].iter() {
        let split = line.split(" ").collect::<Vec<&str>>();
        let new_move = Move {
            num_elements: split[1].parse().expect("Error parsing Move num_elements."),
            from: split[3].parse::<usize>().expect("Error parsing Move from.") - 1,
            to: split[5].parse::<usize>().expect("Error parsing Move to.") - 1,
        };
        moves.push(new_move);
    }

    moves
}

fn crane_solve1(stacks: &mut Vec<Stack>, moves: Vec<Move>) {
    for mov in moves {
        for _ in 0..mov.num_elements {
            if stacks[mov.from].is_empty() {
                break;
            };

            let moved_element = stacks[mov.from].pop().unwrap();
            stacks[mov.to].push(moved_element);
        }
    }
}

fn crane_solve2(stacks: &mut Vec<Stack>, moves: Vec<Move>) {
    for mov in moves {
        let mut moved_elements = vec![];
        for _ in 0..mov.num_elements {
            if stacks[mov.from].is_empty() {
                break;
            };

            moved_elements.push(stacks[mov.from].pop().unwrap());
        }
        moved_elements.reverse();
        stacks[mov.to].append(&mut moved_elements);
    }
}

fn build_result_string(stacks: Vec<Stack>) -> String {
    stacks
        .iter()
        .map(|v| *v.last().unwrap())
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_empty_line() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(get_first_empty_line(&lines), 4);
    }

    #[test]
    fn test_initialize_stacks() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(
            initialize_stacks(&lines),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), "CMZ", "solve1 failed.");
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), "MCD", "solve2 failed.");
    }
}
