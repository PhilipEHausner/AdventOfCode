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
    get_num_fresh_ids(&input.intervals)
}

fn is_fresh(intervals: &Vec<Interval>, candidate: u64) -> bool {
    for interval in intervals {
        if candidate >= interval.from && candidate <= interval.to {
            return true;
        }
    }

    false
}

fn get_num_fresh_ids(intervals: &Vec<Interval>) -> usize {
    let mut intervals = intervals.to_vec();

    loop {
        let mut fusion = false;
        let mut new_intervals = vec![];
        let mut fusioned_intervals = vec![];

        for i in 0..intervals.len() {
            for j in (i + 1)..intervals.len() {
                if fusioned_intervals.contains(&j) {
                    continue;
                }
                if overlaps(&intervals[i], &intervals[j]) {
                    fusion = true;
                    fusioned_intervals.push(i);
                    fusioned_intervals.push(j);
                    let new_interval = Interval {
                        from: u64::min(intervals[i].from, intervals[j].from),
                        to: u64::max(intervals[i].to, intervals[j].to),
                    };
                    new_intervals.push(new_interval);
                }
            }

            if !fusioned_intervals.contains(&i) {
                new_intervals.push(intervals[i]);
            }
        }

        intervals = new_intervals;

        if !fusion {
            break;
        }
    }

    intervals
        .iter()
        .map(|interval| (interval.to - interval.from + 1) as usize)
        .sum()
}

fn overlaps(interval1: &Interval, interval2: &Interval) -> bool {
    (interval1.from <= interval2.to && interval1.to >= interval2.from)
        || (interval2.from <= interval1.to && interval2.to >= interval1.from)
}

#[derive(Clone, Copy, Debug)]
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
        assert_eq!(result, 358155203664116);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = get_input("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 14);
    }
}
