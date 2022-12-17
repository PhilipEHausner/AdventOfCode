use util::read_files::read_file_as_vector;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
enum RockType {
    Horizontal,
    Cross,
    L,
    Vertical,
    Square,
}

struct Rock {
    points: Vec<Point>,
}

impl Rock {
    pub fn new(rock_type: RockType, curr_level: usize) -> Rock {
        let points;

        match rock_type {
            RockType::Horizontal => {
                points = (0..4)
                    .map(|el| Point {
                        x: el + 2,
                        y: curr_level + 4,
                    })
                    .collect();
            }
            RockType::Cross => {
                points = [(3, 4), (2, 4), (4, 4), (3, 3), (3, 5)]
                    .iter()
                    .map(|&(x, y)| Point {
                        x,
                        y: (curr_level + y + 1) as usize,
                    })
                    .collect();
            }
            RockType::L => {
                points = [(2, 4), (3, 4), (4, 4), (4, 5), (4, 6)]
                    .iter()
                    .map(|&(x, y)| Point {
                        x,
                        y: y + curr_level,
                    })
                    .collect();
            }
            RockType::Vertical => {
                points = [(2, 4), (2, 5), (2, 6), (2, 7)]
                    .iter()
                    .map(|&(x, y)| Point {
                        x,
                        y: y + curr_level,
                    })
                    .collect();
            }
            RockType::Square => {
                points = [(2, 4), (3, 4), (2, 5), (3, 5)]
                    .iter()
                    .map(|&(x, y)| Point {
                        x,
                        y: y + curr_level,
                    })
                    .collect();
            }
        }

        Rock { points }
    }

    pub fn move_sidewards(&mut self, direction: char, cave: &Vec<Vec<char>>) {
        match direction {
            '>' => {
                let does_collide = self.points.iter().any(|point| point.x >= 6 || cave[point.x+1][point.y] == '#');
                if !does_collide {
                    self.points = self
                        .points
                        .iter()
                        .map(|point| Point {
                            x: point.x + 1,
                            y: point.y,
                        })
                        .collect();
                }
            }
            '<' => {
                let does_collide = self.points.iter().any(|point| point.x <= 0|| cave[point.x-1][point.y] == '#');
                if !does_collide {
                    self.points = self
                        .points
                        .iter()
                        .map(|point| Point {
                            x: point.x - 1,
                            y: point.y,
                        })
                        .collect();
                }
            }
            _ => panic!("Invalid direction!"),
        }
    }

    pub fn move_downwards(&mut self, cave: &Vec<Vec<char>>) -> bool {
        let does_collide = self
            .points
            .iter()
            .any(|point| cave[point.x][point.y - 1] == '#');
        if does_collide {
            return false;
        }
        self.points = self
            .points
            .iter()
            .map(|point| Point {
                x: point.x,
                y: point.y - 1,
            })
            .collect();
        true
    }

    pub fn draw_rock(&self, cave: &mut Vec<Vec<char>>) {
        for point in &self.points {
            cave[point.x][point.y] = '#';
        }
    }

    pub fn get_upper_level(&self) -> usize {
        self.points.iter().map(|point| point.y).max().unwrap()
    }
}

fn main() {
    let lines = read_file_as_vector("./files/day17.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
}

fn solve1(lines: &Vec<String>) -> usize {
    let mut cave = create_cave(7, 10000);
    let jet_pattern: Vec<char> = lines[0].chars().collect();
    let rock_patterns = [
        RockType::Horizontal,
        RockType::Cross,
        RockType::L,
        RockType::Vertical,
        RockType::Square,
    ];
    let mut curr_level = 0;
    let mut next_rock_pattern = 0;
    let mut next_jet = 0;

    for pattern in rock_patterns {
        Rock::new(pattern, 3);
    }

    for _ in 0..2022 {
        let mut rock = Rock::new(rock_patterns[next_rock_pattern], curr_level);
        loop {
            let jet = jet_pattern[next_jet];
            next_jet = (next_jet + 1) % jet_pattern.len();

            rock.move_sidewards(jet, &cave);
            let can_move = rock.move_downwards(&cave);
            if !can_move {
                break;
            }
        }
        rock.draw_rock(&mut cave);
        curr_level = std::cmp::max(curr_level, rock.get_upper_level());
        next_rock_pattern = (next_rock_pattern + 1) % rock_patterns.len();
    }

    curr_level
}

fn create_cave(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut cave = vec![vec!['.'; height]; width];
    for i in 0..width {
        cave[i][0] = '#';
    }
    cave
}

fn print_cave(cave: &Vec<Vec<char>>, rock: &Rock) {
    let mut c = transpose(&cave);
    for p in &rock.points {
        c[p.y][p.x] = '@';
    }
    for line in c.iter().rev() {
        println!("{:?}", line);
    }
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 3068);
    }
}
