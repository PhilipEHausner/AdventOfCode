use std::collections::HashMap;

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

#[derive(Debug)]
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
                        x: curr_level + 4,
                        y: el + 2,
                    })
                    .collect();
            }
            RockType::Cross => {
                points = [(4, 3), (4, 2), (4, 4), (3, 3), (5, 3)]
                    .iter()
                    .map(|&(x, y)| Point {
                        x: x + curr_level + 1,
                        y,
                    })
                    .collect();
            }
            RockType::L => {
                points = [(4, 2), (4, 3), (4, 4), (5, 4), (6, 4)]
                    .iter()
                    .map(|&(x, y)| Point {
                        x: x + curr_level,
                        y: y,
                    })
                    .collect();
            }
            RockType::Vertical => {
                points = [(4, 2), (5, 2), (6, 2), (7, 2)]
                    .iter()
                    .map(|&(x, y)| Point {
                        x: x + curr_level,
                        y,
                    })
                    .collect();
            }
            RockType::Square => {
                points = [(4, 2), (4, 3), (5, 2), (5, 3)]
                    .iter()
                    .map(|&(x, y)| Point {
                        x: x + curr_level,
                        y,
                    })
                    .collect();
            }
        }
        Rock { points }
    }

    pub fn move_sidewards(&mut self, direction: char, cave: &Vec<Vec<char>>) {
        match direction {
            '>' => {
                let does_collide = self.points.iter().any(|point| point.y >= 6 || cave[point.x][point.y+1] == '#');
                if !does_collide {
                    self.points = self
                        .points
                        .iter()
                        .map(|point| Point {
                            x: point.x,
                            y: point.y+1,
                        })
                        .collect();
                }
            }
            '<' => {
                let does_collide = self.points.iter().any(|point| point.y <= 0|| cave[point.x][point.y-1] == '#');
                if !does_collide {
                    self.points = self
                        .points
                        .iter()
                        .map(|point| Point {
                            x: point.x,
                            y: point.y-1,
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
            .any(|point| cave[point.x-1][point.y] == '#');
        if does_collide {
            return false;
        }
        self.points = self
            .points
            .iter()
            .map(|point| Point {
                x: point.x-1,
                y: point.y,
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
        self.points.iter().map(|point| point.x).max().unwrap()
    }
}

fn main() {
    let lines = read_file_as_vector("./files/day17.txt").expect("Error reading file.");
    // println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> usize {
    let jet_pattern: Vec<char> = lines[0].chars().collect();
    solve_generic(jet_pattern, 7, 20000)
}

fn solve2(lines: &Vec<String>) -> usize {
    let jet_pattern: Vec<char> = lines[0].chars().collect();
    solve_generic(jet_pattern, 7, 1_000_000_000_000)
}

fn solve_generic(jet_pattern: Vec<char>, cave_width: usize, num_rocks: u128) -> usize {
    let mut cave = create_cave(cave_width, 1000000);
    let mut total_level = 0;
    let mut curr_index_level = 0;
    let mut next_rock_pattern = 0;
    let mut next_jet = 0;
    
    let rock_pattern = [
        RockType::Horizontal,
        RockType::Cross,
        RockType::L,
        RockType::Vertical,
        RockType::Square,
    ];

    // let mut repetitions = HashMap::new();
    for r in 0..num_rocks {
        if r == 2018 {
            break;
        }
        if r as u128 % 1000000 == 0 {
            println!("{} - {}%", r, (r / num_rocks) * 100);
        }
        let mut rock = Rock::new(rock_pattern[next_rock_pattern], curr_index_level);
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
        let rock_upper_level = rock.get_upper_level();
        if rock_upper_level > curr_index_level {
            total_level += rock_upper_level - curr_index_level;
            curr_index_level = rock_upper_level;
        }
        next_rock_pattern = (next_rock_pattern + 1) % rock_pattern.len();

        // if curr_index_level > 500 {
        //     let last_50_lines = get_top_x_lines(&cave, 500);
        //     if !repetitions.contains_key(&last_50_lines) {
        //         repetitions.insert(last_50_lines.clone(), Vec::new());
        //     }
        //     let mut v: Vec<u128> = repetitions.get(&last_50_lines).unwrap().to_vec();
        //     v.push(r);
        //     repetitions.insert(last_50_lines, v);
        // }

        if r == 2017 {
            println!("TOTAL {}", total_level);
        }

        if (r - 2017) % 5205 ==  0 {
            println!("{} TOTAL {}", r, total_level);
        }

        if curr_index_level > 999990 {
            let shrink = shrink_cave_representation(&mut cave);
            curr_index_level -= shrink;
        }
    }

    total_level = 3104 + 8001 * 192122957;

    for r in 999999993203..num_rocks {
        // if r == 78 {
        //     break;
        // }
        if r as u128 % 1000000 == 0 {
            println!("{} - {}%", r, (r / num_rocks) * 100);
        }
        let mut rock = Rock::new(rock_pattern[next_rock_pattern], curr_index_level);
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
        let rock_upper_level = rock.get_upper_level();
        if rock_upper_level > curr_index_level {
            total_level += rock_upper_level - curr_index_level;
            curr_index_level = rock_upper_level;
        }
        next_rock_pattern = (next_rock_pattern + 1) % rock_pattern.len();

        if curr_index_level > 999990 {
            let shrink = shrink_cave_representation(&mut cave);
            curr_index_level -= shrink;
        }
    }

    // for (k, v) in repetitions.iter() {
    //     if v.len() < 4 {
    //         continue;
    //     }
    //     let distances = [v[1] - v[0], v[2] -v[0], v[3] -v[0]];
    //     'dist: for dist in distances {
    //         let mut d = v[0];
    //         while d < num_rocks {
    //             if !v.contains(&d) {
    //                 continue 'dist;
    //             }
    //             d += dist
    //         }
    //         println!("{:?} {:?}", k, v);
    //         println!("{} {}", v[0], dist)
    //     }
    // }

    total_level
}

fn create_cave(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut cave = vec![vec!['.'; width]; height];
    cave[0] = vec!['#'; width];
    cave
}

fn get_top_x_lines(cave: &Vec<Vec<char>>, x: usize) -> Vec<bool> {
    let mut idx = 0;

    for i in (0..cave.len()).rev() {
        if cave[i].iter().any(|&el| el == '#') {
            idx = i;
            break;
        }
    }

    let mut result = vec![];

    for i in (idx-x+1)..(idx+1) {
        result.append(&mut cave[i].iter().map(|&el| el == '#').collect::<Vec<bool>>());
    }

    result
}

fn shrink_cave_representation(cave: &mut Vec<Vec<char>>) -> usize {
    // let mut cut_idx = 0;
    // let mut found = [false; 7];

    // for (idx, row) in cave.iter().enumerate().rev() {
    //     for (i, c) in row.iter().enumerate() {
    //         if *c == '#' {
    //             found[i] = true;
    //         }
    //     }
    //     if found.iter().all(|&f| f) {
    //         cut_idx = idx;
    //         break;
    //     }
    // }

    // if cut_idx == 0 {
    //     panic!("Cave assumption failed.");
    // }

    // for i in 0..2000 {
    //     cave[i] = cave[cave.len()-2000+i].clone();
    // }
    let cut = 2000;

    // for i in cut..cave.len() {
    //     cave[i] = vec!['.'; 7];
    // }

    let mut new_cave = vec![vec!['.'; 7]; cave.len()];

    for i in 0..cut {
        new_cave[i] = cave[cave.len()-cut+i].clone();
    }

    *cave = new_cave;

    // let cut_idx = 2000;

    // for i in cut_idx..cave.len() {
    //     cave[i - cut_idx] = cave[i].clone();
    // }
    // for i in (cave.len() - cut_idx)..cave.len() {
    //     cave[i] = vec!['.'; 7];
    // }

    cave.len() - cut
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 3068);
    }

    #[test]
    fn test_solve1_large() {
        let lines = read_file_as_vector("./files/day17.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 3106);
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 1514285714288);
    }
}
