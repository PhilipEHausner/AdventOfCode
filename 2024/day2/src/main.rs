use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day2.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(' ').map(|el| el.parse().unwrap()).collect())
        .collect()
}

fn solve1(input: &Input) -> usize {
    input
        .iter()
        .map(|report| is_safe(report))
        .filter(|safe| *safe)
        .count()
}

fn is_safe(report: &Vec<i64>) -> bool {
    (is_strictly_rising(report) || is_strictly_decreasing(report))
        && differences_lower_than_three(report)
}

fn is_strictly_rising(report: &Vec<i64>) -> bool {
    (0..report.len() - 1)
        .map(|i| report[i] < report[i + 1])
        .all(|el| el)
}

fn is_strictly_decreasing(report: &Vec<i64>) -> bool {
    (0..report.len() - 1)
        .map(|i| report[i] > report[i + 1])
        .all(|el| el)
}

fn differences_lower_than_three(report: &Vec<i64>) -> bool {
    (0..report.len() - 1)
        .map(|i| (report[i] - report[i + 1]).abs())
        .map(|el| el <= 3)
        .all(|el| el)
}

fn solve2(input: &Input) -> usize {
    1
}

type Input = Vec<Vec<i64>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day2.txt");
        let result = solve1(&input);
        assert_eq!(result, 421);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 2);
    }

    //  #[test]
    //  fn test_solve2() {
    //      let input = get_input("./files/day1.txt");
    //      let result = solve2(&input);
    //      assert_eq!(result, 19457120);
    //  }

    //  #[test]
    //  fn test_solve2_testdata() {
    //      let input = get_input("./files/test.txt");
    //      let result = solve2(&input);
    //      assert_eq!(result, 31);
    //  }
}
