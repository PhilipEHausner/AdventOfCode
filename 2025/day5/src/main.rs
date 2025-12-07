use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day5.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    let mut idx = 0;

    let mut intervals = vec![];
    loop {
        let line = &lines[idx];
        idx += 1;

        if line.len() == 0 {
            break;
        }

        let parts: Vec<&str> = line.split('-').collect();
        let interval = Interval {
            from: parts.get(0).unwrap().parse().unwrap(),
            to: parts.get(1).unwrap().parse().unwrap(),
        };
        intervals.push(interval);
    }

    let candidates = lines[idx..]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    Data {
        intervals,
        candidates,
    }
}

fn solve1(input: &Input) -> usize {
    input
        .candidates
        .iter()
        .filter(|candidate| is_fresh(&input.intervals, **candidate))
        .collect::<Vec<&u64>>()
        .len()
}

fn solve2(input: &Input) -> usize {
    1
}

fn is_fresh(intervals: &Vec<Interval>, candidate: u64) -> bool {
    for interval in intervals {
        if candidate >= interval.from && candidate <= interval.to {
            return true;
        }
    }

    false
}

struct Interval {
    from: u64,
    to: u64,
}

struct Data {
    intervals: Vec<Interval>,
    candidates: Vec<u64>,
}

type Input = Data;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day5.txt");
        let result = solve1(&input);
        assert_eq!(result, 679);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve2() {
        let input = get_input("./files/day5.txt");
        let result = solve2(&input);
        // assert_eq!(result, );
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        // assert_eq!(result, );
    }
}
