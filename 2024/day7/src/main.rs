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
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.replace(':', "")
                .split(' ')
                .map(|part| part.parse::<i64>().unwrap())
                .collect()
        })
        .map(|part: Vec<_>| Equation {
            result: part[0].clone(),
            numbers: part[1..].to_vec(),
        })
        .collect()
}

fn solve1(input: &Input) -> i64 {
    input
        .iter()
        .filter(|eq| is_possible(eq.result, eq.numbers.clone(), &vec![plus, mul]))
        .map(|eq| eq.result)
        .sum()
}

fn is_possible(result: i64, numbers: Vec<i64>, ops: &Vec<fn(i64, i64) -> i64>) -> bool {
    _is_possible(result, numbers[1..].to_vec(), numbers[0], ops)
}

fn _is_possible(
    result: i64,
    numbers: Vec<i64>,
    current: i64,
    ops: &Vec<fn(i64, i64) -> i64>,
) -> bool {
    if numbers.is_empty() {
        return current == result;
    }

    let n = numbers[0];
    let remaining = numbers[1..].to_vec();
    for op in ops {
        let c = op(current, n);
        if _is_possible(result, remaining.clone(), c, ops) {
            return true;
        }
    }

    false
}

fn solve2(input: &Input) -> usize {
    1
}

fn plus(p1: i64, p2: i64) -> i64 {
    p1 + p2
}

fn mul(p1: i64, p2: i64) -> i64 {
    p1 * p2
}

struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

type Input = Vec<Equation>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day7.txt");
        let result = solve1(&input);
        assert_eq!(result, 8401132154762);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 3749);
    }
}
