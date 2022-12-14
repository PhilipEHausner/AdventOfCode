use std::{cmp, vec};

use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day14.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    let mut cave = create_cave(lines);
    how_much_sand_flows(&mut cave)
}

fn solve2(lines: &Vec<String>) -> u64 {
    let mut cave = create_cave(lines);
    create_floor(&mut cave);
    how_much_sand_flows(&mut cave)
}

fn how_much_sand_flows(cave: &mut Vec<Vec<char>>) -> u64 {
    let mut units_of_sand = 0;
    while throw_sand(cave) {
        units_of_sand += 1;
    }

    units_of_sand
}

fn create_cave(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut cave = vec![vec!['.'; 1000]; 1000];

    for line in lines {
        let points = line
            .split("->")
            .map(|el| {
                (
                    el.trim()
                        .split(",")
                        .nth(1)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                    el.trim()
                        .split(",")
                        .nth(0)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                )
            })
            .collect::<Vec<(usize, usize)>>();

        for i in 1..points.len() {
            let start_point = points[i - 1];
            let end_point = points[i];
            cave[start_point.0][start_point.1] = '#';
            cave[end_point.0][end_point.1] = '#';
            if start_point.0 == end_point.0 {
                for j in cmp::min(start_point.1, end_point.1)..cmp::max(start_point.1, end_point.1)
                {
                    cave[start_point.0][j] = '#';
                }
            } else if start_point.1 == end_point.1 {
                for j in cmp::min(start_point.0, end_point.0)..cmp::max(start_point.0, end_point.0)
                {
                    cave[j][start_point.1] = '#';
                }
            }
        }
    }

    cave
}

fn throw_sand(cave: &mut Vec<Vec<char>>) -> bool {
    let mut sand_pos = (0, 500);

    if cave[sand_pos.0][sand_pos.1] != '.' {
        return false;
    }

    loop {
        if sand_pos.0 >= cave.len() - 1 {
            return false;
        }
        if cave[sand_pos.0 + 1][sand_pos.1] == '.' {
            sand_pos.0 = sand_pos.0 + 1;
            continue;
        }
        if cave[sand_pos.0 + 1][sand_pos.1 - 1] == '.' {
            sand_pos = (sand_pos.0 + 1, sand_pos.1 - 1);
            continue;
        }
        if cave[sand_pos.0 + 1][sand_pos.1 + 1] == '.' {
            sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            continue;
        }
        break;
    }

    cave[sand_pos.0][sand_pos.1] = 'o';
    true
}

fn create_floor(cave: &mut Vec<Vec<char>>) {
    let floor_level = cave
        .iter()
        .enumerate()
        .map(|(i, row)| if row.contains(&'#') { i } else { 0 })
        .max()
        .unwrap()
        + 2;

    cave[floor_level] = vec!['#'; 1000];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 24);
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 93);
    }
}
