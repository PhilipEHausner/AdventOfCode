use std::cmp::Reverse;

use priority_queue::PriorityQueue;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day17.txt");
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
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    let xsize = input.len();
    let ysize = input[0].len();
    let goal = (xsize - 1, ysize - 1);

    let mut priority_queue: PriorityQueue<Crucible, Reverse<usize>> = PriorityQueue::new();
    priority_queue.push(Crucible::new(0, 0, Direction::South, 3), Reverse(0));
    priority_queue.push(Crucible::new(0, 0, Direction::East, 3), Reverse(0));

    while let Some(item) = priority_queue.pop() {
        let crucible = item.0;
        let distance = item.1 .0;

        if (crucible.x, crucible.y) == goal {
            return distance;
        }

        match crucible.dir {
            Direction::North => {
                if crucible.moves > 0 && crucible.x > 0 {
                    let new_distance = distance + input[crucible.x - 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x - 1, crucible.y, crucible.dir, crucible.moves - 1),
                        Reverse(new_distance),
                    );
                }
                if crucible.y > 0 {
                    let new_distance = distance + input[crucible.x][crucible.y - 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y - 1, Direction::West, 2),
                        Reverse(new_distance),
                    );
                }
                if crucible.y < ysize - 1 {
                    let new_distance = distance + input[crucible.x][crucible.y + 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y + 1, Direction::East, 2),
                        Reverse(new_distance),
                    );
                }
            }
            Direction::South => {
                if crucible.moves > 0 && crucible.x < xsize - 1 {
                    let new_distance = distance + input[crucible.x + 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x + 1, crucible.y, crucible.dir, crucible.moves - 1),
                        Reverse(new_distance),
                    );
                }
                if crucible.y > 0 {
                    let new_distance = distance + input[crucible.x][crucible.y - 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y - 1, Direction::West, 2),
                        Reverse(new_distance),
                    );
                }
                if crucible.y < ysize - 1 {
                    let new_distance = distance + input[crucible.x][crucible.y + 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y + 1, Direction::East, 2),
                        Reverse(new_distance),
                    );
                }
            }
            Direction::West => {
                if crucible.moves > 0 && crucible.y > 0 {
                    let new_distance = distance + input[crucible.x][crucible.y - 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y - 1, crucible.dir, crucible.moves - 1),
                        Reverse(new_distance),
                    );
                }
                if crucible.x > 0 {
                    let new_distance = distance + input[crucible.x - 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x - 1, crucible.y, Direction::North, 2),
                        Reverse(new_distance),
                    );
                }
                if crucible.x < xsize - 1 {
                    let new_distance = distance + input[crucible.x + 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x + 1, crucible.y, Direction::South, 2),
                        Reverse(new_distance),
                    );
                }
            }
            Direction::East => {
                if crucible.moves > 0 && crucible.y < ysize - 1 {
                    let new_distance = distance + input[crucible.x][crucible.y + 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y + 1, crucible.dir, crucible.moves - 1),
                        Reverse(new_distance),
                    );
                }
                if crucible.x > 0 {
                    let new_distance = distance + input[crucible.x - 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x - 1, crucible.y, Direction::North, 2),
                        Reverse(new_distance),
                    );
                }
                if crucible.x < xsize - 1 {
                    let new_distance = distance + input[crucible.x + 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x + 1, crucible.y, Direction::South, 2),
                        Reverse(new_distance),
                    );
                }
            }
        }
    }
    panic!("End");
}

fn solve2(input: &Input) -> usize {
    let xsize = input.len();
    let ysize = input[0].len();
    let goal = (xsize - 1, ysize - 1);

    let mut priority_queue: PriorityQueue<Crucible, Reverse<usize>> = PriorityQueue::new();
    priority_queue.push(Crucible::new(0, 0, Direction::South, 10), Reverse(0));
    priority_queue.push(Crucible::new(0, 0, Direction::East, 10), Reverse(0));

    while let Some(item) = priority_queue.pop() {
        let crucible = item.0;
        let distance = item.1 .0;

        if (crucible.x, crucible.y) == goal {
            return distance;
        }

        match crucible.dir {
            Direction::North => {
                if crucible.moves > 0 && crucible.x > 0 {
                    let new_distance = distance + input[crucible.x - 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x - 1, crucible.y, crucible.dir, crucible.moves - 1),
                        Reverse(new_distance),
                    );
                }
                if crucible.y > 0 && crucible.moves <= 6 {
                    let new_distance = distance + input[crucible.x][crucible.y - 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y - 1, Direction::West, 9),
                        Reverse(new_distance),
                    );
                }
                if crucible.y < ysize - 1 && crucible.moves <= 6 {
                    let new_distance = distance + input[crucible.x][crucible.y + 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y + 1, Direction::East, 9),
                        Reverse(new_distance),
                    );
                }
            }
            Direction::South => {
                if crucible.moves > 0 && crucible.x < xsize - 1 {
                    let new_distance = distance + input[crucible.x + 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x + 1, crucible.y, crucible.dir, crucible.moves - 1),
                        Reverse(new_distance),
                    );
                }
                if crucible.y > 0 && crucible.moves <= 6 {
                    let new_distance = distance + input[crucible.x][crucible.y - 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y - 1, Direction::West, 9),
                        Reverse(new_distance),
                    );
                }
                if crucible.y < ysize - 1 && crucible.moves <= 6 {
                    let new_distance = distance + input[crucible.x][crucible.y + 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y + 1, Direction::East, 9),
                        Reverse(new_distance),
                    );
                }
            }
            Direction::West => {
                if crucible.moves > 0 && crucible.y > 0 {
                    let new_distance = distance + input[crucible.x][crucible.y - 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y - 1, crucible.dir, crucible.moves - 1),
                        Reverse(new_distance),
                    );
                }
                if crucible.x > 0 && crucible.moves <= 6 {
                    let new_distance = distance + input[crucible.x - 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x - 1, crucible.y, Direction::North, 9),
                        Reverse(new_distance),
                    );
                }
                if crucible.x < xsize - 1 && crucible.moves <= 6 {
                    let new_distance = distance + input[crucible.x + 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x + 1, crucible.y, Direction::South, 9),
                        Reverse(new_distance),
                    );
                }
            }
            Direction::East => {
                if crucible.moves > 0 && crucible.y < ysize - 1 {
                    let new_distance = distance + input[crucible.x][crucible.y + 1];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x, crucible.y + 1, crucible.dir, crucible.moves - 1),
                        Reverse(new_distance),
                    );
                }
                if crucible.x > 0 && crucible.moves <= 6 {
                    let new_distance = distance + input[crucible.x - 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x - 1, crucible.y, Direction::North, 9),
                        Reverse(new_distance),
                    );
                }
                if crucible.x < xsize - 1 && crucible.moves <= 6 {
                    let new_distance = distance + input[crucible.x + 1][crucible.y];
                    priority_queue.push_increase(
                        Crucible::new(crucible.x + 1, crucible.y, Direction::South, 9),
                        Reverse(new_distance),
                    );
                }
            }
        }
    }
    panic!("End");
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Crucible {
    x: usize,
    y: usize,
    dir: Direction,
    moves: u8,
}

impl Crucible {
    fn new(x: usize, y: usize, dir: Direction, moves: u8) -> Crucible {
        Crucible { x, y, dir, moves }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

type Input = Vec<Vec<usize>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day17.txt");
        let result = solve1(&input);
        assert_eq!(result, 953);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 102);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day17.txt");
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
