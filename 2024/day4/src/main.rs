use std::i64;

use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day4.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    lines
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect()
}

fn solve1(input: &Input) -> usize {
    let height = input.len();
    let width = input.first().unwrap().len();
    let mut count = 0;

    for x in 0..height {
        for y in 0..width {
            if x < height - 3 {
                if find(input, x, y, (1, 0)) {
                    count += 1;
                }
                if y < width - 3 {
                    if find(input, x, y, (1, 1)) {
                        count += 1;
                    }
                }
                if y > 2 {
                    if find(input, x, y, (1, -1)) {
                        count += 1;
                    }
                }
            }
            if x > 2 {
                if find(input, x, y, (-1, 0)) {
                    count += 1;
                }
                if y < width - 3 {
                    if find(input, x, y, (-1, 1)) {
                        count += 1;
                    }
                }
                if y > 2 {
                    if find(input, x, y, (-1, -1)) {
                        count += 1;
                    }
                }
            }
            if y < width - 3 {
                if find(input, x, y, (0, 1)) {
                    count += 1;
                }
            }
            if y > 2 {
                if find(input, x, y, (0, -1)) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn find(input: &Input, x: usize, y: usize, dir: (i64, i64)) -> bool {
    input[x][y] == 'X'
        && input[(x as i64 + 1 * dir.0) as usize][(y as i64 + 1 * dir.1) as usize] == 'M'
        && input[(x as i64 + 2 * dir.0) as usize][(y as i64 + 2 * dir.1) as usize] == 'A'
        && input[(x as i64 + 3 * dir.0) as usize][(y as i64 + 3 * dir.1) as usize] == 'S'
}

fn solve2(input: &Input) -> usize {
    1
}

type Input = Vec<Vec<char>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day4.txt");
        let result = solve1(&input);
        // assert_eq!(result, );
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 18);
    }
}
