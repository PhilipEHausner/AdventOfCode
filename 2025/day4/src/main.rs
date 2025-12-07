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
        .map(|line| line.chars().into_iter().map(|c| c == '@').collect())
        .collect()
}

fn solve1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, _)| {
                    if input[x][y] && num_neighbours(input, (x, y)) < 4 {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn solve2(input: &Input) -> usize {
    let mut input = input.clone();
    let mut removed_rolls = 0;

    loop {
        let removable_rolls = get_removable_rolls(&input);
        removed_rolls += removable_rolls.len();
        if removable_rolls.len() == 0 {
            break;
        }
        remove_rolls(&mut input, removable_rolls);
    }

    removed_rolls
}

fn num_neighbours(input: &Input, pos: (usize, usize)) -> usize {
    let mut neighbours = 0;

    if pos.0 > 0 {
        if pos.1 > 0 && input[pos.0 - 1][pos.1 - 1] {
            neighbours += 1;
        }
        if input[pos.0 - 1][pos.1] {
            neighbours += 1;
        }
        if pos.1 < (input.get(pos.0 - 1).unwrap().len() - 1) && input[pos.0 - 1][pos.1 + 1] {
            neighbours += 1;
        }
    }
    if pos.1 > 0 && input[pos.0][pos.1 - 1] {
        neighbours += 1;
    }
    if pos.1 < (input.get(pos.0).unwrap().len() - 1) && input[pos.0][pos.1 + 1] {
        neighbours += 1;
    }
    if pos.0 < input.len() - 1 {
        if pos.1 > 0 && input[pos.0 + 1][pos.1 - 1] {
            neighbours += 1;
        }
        if input[pos.0 + 1][pos.1] {
            neighbours += 1;
        }
        if pos.1 < (input.get(pos.0 + 1).unwrap().len() - 1) && input[pos.0 + 1][pos.1 + 1] {
            neighbours += 1;
        }
    }

    neighbours
}

fn get_removable_rolls(input: &Input) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, _)| {
                    if input[x][y] && num_neighbours(input, (x, y)) < 4 {
                        vec![(x, y)]
                    } else {
                        vec![]
                    }
                })
                .flatten()
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect()
}

fn remove_rolls(input: &mut Input, removable_rolls: Vec<(usize, usize)>) {
    for roll in removable_rolls {
        input[roll.0][roll.1] = false;
    }
}

type Input = Vec<Vec<bool>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day4.txt");
        let result = solve1(&input);
        assert_eq!(result, 1349);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day4.txt");
        let result = solve2(&input);
        assert_eq!(result, 8277);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 43);
    }
}
