use std::usize;

use regex::Regex;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day13.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let mut input = vec![];
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    let in_re = Regex::new(r"X\+(?<X>\d+).*Y\+(?<Y>\d+)").unwrap();
    let out_re = Regex::new(r"X=(?<X>\d+).*Y=(?<Y>\d+)").unwrap();

    for i in (0..lines.len()).step_by(4) {
        let in_captures_l1 = in_re.captures(&lines[i]).unwrap();
        let in_captures_l2 = in_re.captures(&lines[i + 1]).unwrap();
        let out_captures = out_re.captures(&lines[i + 2]).unwrap();
        input.push(LES {
            x0: in_captures_l1["X"].parse().unwrap(),
            x1: in_captures_l2["X"].parse().unwrap(),
            y0: in_captures_l1["Y"].parse().unwrap(),
            y1: in_captures_l2["Y"].parse().unwrap(),
            xt: out_captures["X"].parse().unwrap(),
            yt: out_captures["Y"].parse().unwrap(),
        });
    }
    input
}

fn solve1(input: &Input) -> usize {
    input
        .iter()
        .map(|les| min_tokens(les))
        .map(|m| m.unwrap_or(0))
        .sum()
}

fn min_tokens(les: &LES) -> Result<usize, ()> {
    let mut min_tokens = usize::MAX;

    for a in 0..100 {
        for b in 0..100 {
            if les.x0 * a + les.x1 * b != les.xt {
                continue;
            }
            if les.y0 * a + les.y1 * b != les.yt {
                continue;
            }
            min_tokens = min_tokens.min(3 * a + b);
        }
    }

    match min_tokens {
        usize::MAX => Err(()),
        n => Ok(n),
    }
}

fn solve2(input: &Input) -> usize {
    1
}

#[derive(Debug)]
struct LES {
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
    xt: usize,
    yt: usize,
}

type Input = Vec<LES>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day13.txt");
        let result = solve1(&input);
        assert_eq!(result, 31589);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 480);
    }
}
