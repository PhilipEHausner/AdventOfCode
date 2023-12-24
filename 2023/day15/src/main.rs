use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day15.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Input {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    assert_eq!(lines.len(), 1);
    lines.get(0).unwrap().split(",").map(|el| el.to_string()).collect()
}

fn solve1(input: &Input) -> usize {
    input.iter().map(|line| hash_line(line)).sum()
}

fn solve2(input: &Input) -> usize {
    1
}

fn hash_line(line: &String) -> usize {
    line.chars().into_iter().fold(0, |acc, c| ((acc + ascii(&c)) * 17) % 256)
}

fn ascii(c: &char) -> usize {
    *c as usize
}

type Input = Vec<String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day15.txt");
        let result = solve1(&input);
        assert_eq!(result, 517015);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 1320);
    }
}