use util::read_files::read_file_as_vector;

fn main() {
    let races = parse_races("./files/day6.txt");
    println!("Solution part 1: {}", solve1(&races));
    let race_part2 = parse_races_part2("./files/day6.txt");
    println!("Solution part 2: {}", solve2(&vec![race_part2]));
}

fn solve1(races: &Vec<Race>) -> i64 {
    races
        .iter()
        .map(|it| upper_bound(it) - lower_bound(it) + 1)
        .fold(1, |acc, it| acc * it)
}

fn lower_bound(race: &Race) -> i64 {
    for x in 0..=race.time {
        if x * (race.time - x) > race.distance {
            return x;
        }
    }
    panic!("Could not find lower bound.");
}

fn upper_bound(race: &Race) -> i64 {
    for x in (0..=race.time).rev() {
        if x * (race.time - x) > race.distance {
            return x;
        }
    }
    panic!("Could not find upper bound.");
}

fn solve2(races: &Vec<Race>) -> i64 {
    solve1(races)
}

fn parse_races(file: &str) -> Vec<Race> {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    assert!(lines.len() >= 2);
    let times: Vec<i64> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|it| it.parse().unwrap())
        .collect();
    let dists: Vec<i64> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|it| it.parse().unwrap())
        .collect();
    assert!(times.len() == dists.len());
    times
        .into_iter()
        .zip(dists.into_iter())
        .map(|(t, d)| Race::new(t, d))
        .collect()
}

fn parse_races_part2(file: &str) -> Race {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    assert!(lines.len() >= 2);
    let time = lines[0]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let dist = lines[1]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    Race::new(time, dist)
}

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    pub fn new(time: i64, distance: i64) -> Race {
        Race { time, distance }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = parse_races("./files/day6.txt");
        let result = solve1(&input);
        assert_eq!(result, 1195150);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = parse_races("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_solve2() {
        let input = parse_races_part2("./files/day6.txt");
        let result = solve2(&vec![input]);
        assert_eq!(result, 42550411);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = parse_races_part2("./files/test.txt");
        let result = solve2(&vec![input]);
        assert_eq!(result, 71503);
    }
}
