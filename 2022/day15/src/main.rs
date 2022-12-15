use std::cmp;

use regex::Regex;
use util::read_files::read_file_as_vector;

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Sensor {
    point: Point,
    range: u64,
}

impl Sensor {
    pub fn point_in_range(&self, point: &Point) -> bool {
        manhattan_distance(&self.point, &point) <= self.range
    }
}

#[derive(Debug)]
struct Beacon {
    point: Point,
}

fn main() {
    let lines = read_file_as_vector("./files/day15.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines, 2000000));
    println!("Solution part 2: {}", solve2(&lines, 4000000));
}

fn solve1(lines: &Vec<String>, y: i64) -> u64 {
    let sensors_and_beacons = get_sensors_and_beacons(lines);
    let (min_x, max_x) = get_min_and_max_x(&sensors_and_beacons);

    let mut result = 0;
    for x in min_x..max_x + 1 {
        if field_has_no_beacon(&sensors_and_beacons, &Point { x, y }) {
            result += 1;
        }
    }

    result
}

fn solve2(lines: &Vec<String>, max_coord: i64) -> i64 {
    let sensors_and_beacons = get_sensors_and_beacons(lines);

    let mut intervals = vec![vec![]; (max_coord + 1) as usize];

    for (sensor, _) in &sensors_and_beacons {
        for i in cmp::max(0, sensor.point.y - sensor.range as i64)
            ..cmp::min(max_coord + 1, sensor.point.y + sensor.range as i64)
        {
            let (min_x, max_x) = (
                sensor.point.x - (sensor.range as i64 - (sensor.point.y - i).abs()),
                sensor.point.x + (sensor.range as i64 - (sensor.point.y - i).abs()),
            );
            intervals[i as usize].push((min_x, max_x));
        }
    }

    let mut result = (0, 0);
    'outer: for (x, interval) in intervals.iter_mut().enumerate() {
        interval.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut curr_ind = interval[0].1;
        for int in interval[1..].iter() {
            if curr_ind + 1 < int.0 {
                result = (x as i64, curr_ind + 1);
                break 'outer;
            }
            curr_ind = cmp::max(curr_ind, int.1);
        }
    }

    result.1 * 4_000_000 + result.0
}

fn get_sensors_and_beacons(lines: &Vec<String>) -> Vec<(Sensor, Beacon)> {
    let mut result = vec![];

    for line in lines {
        result.push(parse_line(line));
    }

    result
}

fn parse_line(line: &str) -> (Sensor, Beacon) {
    let re = Regex::new(r"^.*x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)$").unwrap();
    let caps = re.captures(line).unwrap();
    let sensor_point = Point {
        x: caps.get(1).unwrap().as_str().parse().unwrap(),
        y: caps.get(2).unwrap().as_str().parse().unwrap(),
    };
    let beacon_point = Point {
        x: caps.get(3).unwrap().as_str().parse().unwrap(),
        y: caps.get(4).unwrap().as_str().parse().unwrap(),
    };
    let sensor_range = manhattan_distance(&sensor_point, &beacon_point);
    (
        Sensor {
            point: sensor_point,
            range: sensor_range,
        },
        Beacon {
            point: beacon_point,
        },
    )
}

fn get_min_and_max_x(sensors_and_beacons: &Vec<(Sensor, Beacon)>) -> (i64, i64) {
    let min = sensors_and_beacons
        .iter()
        .map(|(s, p)| s.point.y - manhattan_distance(&s.point, &p.point) as i64)
        .min()
        .unwrap();
    let max = sensors_and_beacons
        .iter()
        .map(|(s, p)| s.point.y + manhattan_distance(&s.point, &p.point) as i64)
        .max()
        .unwrap();
    (min, max)
}

fn manhattan_distance(point1: &Point, point2: &Point) -> u64 {
    (point1.x - point2.x).abs() as u64 + (point1.y - point2.y).abs() as u64
}

fn field_has_no_beacon(sensors_and_beacons: &Vec<(Sensor, Beacon)>, point: &Point) -> bool {
    for (sensor, _) in sensors_and_beacons {
        if sensor.point_in_range(point) {
            if !field_contains_beacon(sensors_and_beacons, point) {
                return true;
            }
        }
    }
    false
}

fn field_contains_beacon(sensors_and_beacons: &Vec<(Sensor, Beacon)>, point: &Point) -> bool {
    for (_, beacon) in sensors_and_beacons {
        if beacon.point == *point {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines, 10), 26);
    }

    #[test]
    fn test_solve1_large() {
        let lines = read_file_as_vector("./files/day15.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines, 2000000), 5176944);
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines, 20), 56000011);
    }

    #[test]
    fn test_solve2_large() {
        let lines = read_file_as_vector("./files/day15.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines, 4_000_000), 13350458933732);
    }
}
