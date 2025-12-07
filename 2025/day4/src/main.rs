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
    1
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

    //    println!("{}, {} -> {}", pos.0, pos.1, neighbours);

    neighbours
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
        // assert_eq!(result, );
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        // assert_eq!(result, );
    }
}
