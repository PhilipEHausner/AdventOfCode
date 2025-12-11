use core::panic;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day7.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    'S' => Space::Start,
                    '^' => Space::Split,
                    _ => panic!("Unknown char {}", c),
                })
                .collect()
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut splits = 0;

    for tick_num in 0..(grid.len() - 1) {
        splits += tick1(&mut grid, tick_num);
    }

    splits
}

fn tick1(grid: &mut Input, tick_num: usize) -> usize {
    let mut splits = 0;
    if tick_num == grid.len() - 1 {
        return splits;
    }
    for column in 0..grid.get(tick_num).unwrap().len() {
        match grid[tick_num][column] {
            Space::Beam | Space::Start => {
                if grid[tick_num + 1][column] == Space::Split {
                    splits += 1;
                    if column > 0 && grid[tick_num + 1][column - 1] == Space::Empty {
                        grid[tick_num + 1][column - 1] = Space::Beam;
                    }
                    if column < grid[tick_num + 1].len() - 1
                        && grid[tick_num + 1][column + 1] == Space::Empty
                    {
                        grid[tick_num + 1][column + 1] = Space::Beam;
                    }
                } else if grid[tick_num + 1][column] == Space::Empty {
                    grid[tick_num + 1][column] = Space::Beam;
                }
            }
            Space::Empty | Space::Split => {}
        }
    }
    splits
}

fn solve2(input: &Input) -> usize {
    1
}

#[allow(dead_code)]
fn print_grid(input: &Input) {
    for line in input.iter() {
        line.iter()
            .map(|el| el.to_char())
            .for_each(|c| print!("{}", c));
        println!();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Space {
    Beam,
    Empty,
    Split,
    Start,
}

impl Space {
    fn to_char(self) -> char {
        match self {
            Space::Beam => '|',
            Space::Empty => '.',
            Space::Split => '^',
            Space::Start => 'S',
        }
    }
}

type Input = Vec<Vec<Space>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day7.txt");
        let result = solve1(&input);
        assert_eq!(result, 1598);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day7.txt");
        let result = solve2(&input);
        // assert_eq!(result, );
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        // assert_eq!(result, );
    }
}
