use util::read_files::read_file_as_vector;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = get_input("./files/day15.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Input {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    assert_eq!(lines.len(), 1);
    lines
        .get(0)
        .unwrap()
        .split(",")
        .map(|el| el.to_string())
        .collect()
}

fn solve1(input: &Input) -> usize {
    input.iter().map(|line| hash_line(line)).sum()
}

fn solve2(input: &Input) -> usize {
    lazy_static! {
        static ref OPERATION_REGEX: Regex =
            Regex::new(r"(?<box>\w+)(?<op>-|=)(?<focus>\d*)").unwrap();
    }

    let mut boxes = vec![vec![]; 256];

    input.iter().for_each(|line| {
        OPERATION_REGEX.captures(line).map(|capture| {
            let box_id = &capture["box"].to_string();
            let box_num = hash_line(box_id);
            match &capture["op"] {
                "-" => boxes[box_num].retain(|lens: &Lens| &lens.id != box_id),
                "=" => {
                    let focus = capture["focus"].parse().unwrap();
                    match boxes[box_num]
                        .iter()
                        .filter(|lens| &lens.id == box_id)
                        .count()
                    {
                        0 => boxes[box_num].push(Lens {
                            id: box_id.clone(),
                            value: focus,
                        }),
                        1 => boxes[box_num]
                            .iter_mut()
                            .filter(|lens| &lens.id == box_id)
                            .for_each(|lens| lens.value = focus),
                        _ => panic!(),
                    }
                }
                x => panic!("Invalid operator '{x}'."),
            };
        });
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_num, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_num, lens)| (1 + box_num) * (lens_num + 1) * lens.value)
                .sum::<usize>()
        })
        .sum()
}

fn hash_line(line: &String) -> usize {
    line.chars()
        .into_iter()
        .fold(0, |acc, c| ((acc + ascii(&c)) * 17) % 256)
}

fn ascii(c: &char) -> usize {
    *c as usize
}

type Input = Vec<String>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    id: String,
    value: usize,
}

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

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day15.txt");
        let result = solve2(&input);
        assert_eq!(result, 286104);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 145);
    }
}
