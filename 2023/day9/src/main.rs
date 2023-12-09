use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day9.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Vec<Vec<i64>> {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    lines
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|it| it.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve1(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().map(|it| predict_series(it)).sum()
}

fn solve2(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().map(|it| predict_series_inverse(it)).sum()
}

fn predict_series(series: &Vec<i64>) -> i64 {
    let differences = get_series_gradient(series);
    differences
        .iter()
        .rev()
        .skip(1)
        .fold(0, |acc, v: &Vec<i64>| acc + v.last().unwrap())
}

fn predict_series_inverse(series: &Vec<i64>) -> i64 {
    let differences = get_series_gradient(series);
    differences
        .iter()
        .rev()
        .skip(1)
        .fold(0, |acc, v: &Vec<i64>| v.first().unwrap() - acc)
}

fn get_series_gradient(series: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut differences = vec![series.clone()];

    loop {
        let last = &differences.last().unwrap();
        if last.iter().all(|el| el == &0) {
            break;
        }
        let next = last.windows(2).into_iter().map(|w| w[1] - w[0]).collect();
        differences.push(next);
    }

    differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day9.txt");
        let result = solve1(&input);
        assert_eq!(result, 1584748274);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day9.txt");
        let result = solve2(&input);
        assert_eq!(result, 1026);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test2.txt");
        let result = solve2(&input);
        assert_eq!(result, 5);
    }
}
