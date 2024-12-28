use ndarray::prelude::*;
use ndarray_linalg::Solve;

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

fn solve2(input: &Input) -> usize {
    input
        .iter()
        .map(|les| LES {
            x0: les.x0,
            x1: les.x1,
            y0: les.y0,
            y1: les.y1,
            xt: les.xt + 10000000000000,
            yt: les.yt + 10000000000000,
        })
        .map(|les| min_tokens(&les))
        .map(|m| m.unwrap_or(0))
        .sum()
}

fn min_tokens(les: &LES) -> Result<usize, ()> {
    let r: Array2<f64> = array![
        [les.x0 as f64, les.x1 as f64],
        [les.y0 as f64, les.y1 as f64]
    ];
    let t: Array1<f64> = array![les.xt as f64, les.yt as f64];
    let s = r.solve_into(t).unwrap();

    let (a, b) = (s[0].round() as usize, s[1].round() as usize);

    if a * les.x0 + b * les.x1 == les.xt && a * les.y0 + b * les.y1 == les.yt {
        Ok(3 * a + b)
    } else {
        Err(())
    }
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

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day13.txt");
        let result = solve2(&input);
        assert_eq!(result, 98080815200063);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 875318608908);
    }
}
