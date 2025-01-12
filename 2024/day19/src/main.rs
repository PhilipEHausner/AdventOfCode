use cached::proc_macro::cached;
use cached::SizedCache;
use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day19.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    let patterns = lines
        .first()
        .unwrap()
        .split(",")
        .map(|pattern| pattern.trim().to_string())
        .collect();
    let designs = lines.iter().skip(2).map(|d| d.to_string()).collect();

    Input { patterns, designs }
}

fn solve1(input: &Input) -> usize {
    let patterns = &input.patterns;

    let mut result = 0;
    for design in input.designs.iter() {
        if design_is_valid(design, patterns) {
            result += 1;
        }
    }
    result
}

#[cached(
    ty = "SizedCache<String,bool>",
    create = "{ SizedCache::with_size(10000) }",
    convert = r#"{ format!("{:?}{:?}", design, patterns) }"#
)]
fn design_is_valid(design: &str, patterns: &Vec<String>) -> bool {
    if design.len() == 0 {
        return true;
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            if design_is_valid(&design[pattern.len()..], patterns) {
                return true;
            }
        }
    }

    false
}

fn solve2(input: &Input) -> usize {
    1
}

struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day19.txt");
        let result = solve1(&input);
        assert_eq!(result, 350);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 6);
    }
}
