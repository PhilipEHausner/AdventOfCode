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
    pub fn point_in_range(&self, point: & Point) -> bool {
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
}

fn solve1(lines: &Vec<String>, y: i64) -> u64 {
    let sensors_and_beacons = get_sensors_and_beacons(lines);
    let (min_x, max_x) = get_min_and_max_x(&sensors_and_beacons);

    let mut result = 0;
    for x in min_x..max_x+1 {
        if field_has_no_beacon(&sensors_and_beacons, &Point { x, y}) {
            result += 1;
        }
    }

    result
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
}
