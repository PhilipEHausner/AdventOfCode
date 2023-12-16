use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day13.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Vec<Pattern> {
    let mut result = vec![];
    let lines = read_file_as_vector(file).expect("Could not read file.");
    let slices: Vec<_> = lines.split(|line| line.is_empty()).collect();

    for slice in slices {
        let pattern = slice
            .iter()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|c| match c {
                        '.' => Ground::Ash,
                        '#' => Ground::Rock,
                        _ => panic!("Unknown ground '{c}'."),
                    })
                    .collect::<Vec<Ground>>()
            })
            .collect::<Pattern>();
        result.push(pattern);
    }

    result
}

fn solve1(patterns: &Vec<Pattern>) -> usize {
    patterns.iter().map(|p| match_line(p)).sum()
}

fn match_line(pattern: &Pattern) -> usize {
    match match_horizontal_line(pattern) {
        Some(x) => x * 100,
        None => match match_vertical_line(pattern) {
            Some(x) => x,
            None => panic!("No symmetry found."),
        }
    }
}

fn match_horizontal_line(pattern: &Pattern) -> Option<usize> {
    for i in 1..(pattern.len()) {
        let lines = i.min(pattern.len() - i);
        let top = &pattern[(i - lines)..i];
        let bot = &pattern[i..(i + lines)];
        if is_symmetric(top, bot) {
            return Some(i)
        }
    }
    None
}

fn match_vertical_line(pattern: &Pattern) -> Option<usize> {
    let transposed = transpose(pattern);
    match_horizontal_line(&transposed)
}

fn is_symmetric(top: &[Vec<Ground>], bot: &[Vec<Ground>]) -> bool {
    assert_eq!(top.len(), bot.len());
    top.iter().zip(bot.iter().rev()).all(|(v1, v2)| v1 == v2)
}

fn solve2(patterns: &Vec<Pattern>) -> usize {
    1
}

fn transpose<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

type Pattern = Vec<Vec<Ground>>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Ground {
    Ash,
    Rock,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day13.txt");
        let result = solve1(&input);
        assert_eq!(result, 32035);
    }

    #[test]
    fn test_solve1_testdata1() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 405);
    }
}
