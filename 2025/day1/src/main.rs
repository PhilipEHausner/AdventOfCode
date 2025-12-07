use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/test.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    lines
        .iter()
        .map(|line| {
            let magnitude = line[1..].parse().unwrap();
            if line.starts_with("L") {
                Rotation::L(magnitude)
            } else {
                Rotation::R(magnitude)
            }
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut dial = 50;
    let mut clicks = 0;

    input.iter().for_each(|rotation| {
        match rotation {
            Rotation::L(m) => dial -= m,
            Rotation::R(m) => dial += m,
        }
        dial = dial % 100;
        if dial ==0 {
            clicks+=1;
        }
    });

    clicks
}

fn solve2(input: &Input) -> usize {
    1
}

#[derive(Debug)]
enum Rotation {
    L(i64),
    R(i64),
}

type Input = Vec<Rotation>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day1.txt");
        let result = solve1(&input);
        assert_eq!(result, 1036);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 3);
    }
}
