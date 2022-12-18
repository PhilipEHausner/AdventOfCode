use std::collections::VecDeque;

use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day18.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    let grid = create_lava_grid(lines);
    solve_generic(&grid, &vec![])
}

fn solve2(lines: &Vec<String>) -> u64 {
    let grid = create_lava_grid(lines);
    let exterior_grid = create_exterior_indication_grid(&grid);
    solve_generic(&grid, &exterior_grid)
}

fn solve_generic(grid: &Vec<Vec<Vec<bool>>>, exterior_grid: &Vec<Vec<Vec<bool>>>) -> u64 {
    let mut result = 0;

    let (dim_x, dim_y, dim_z) = (grid.len(), grid[0].len(), grid[0][0].len());
    let directions = get_3d_neighbour_directions();

    for i in 1..dim_x - 1 {
        for j in 1..dim_y - 1 {
            for k in 1..dim_z - 1 {
                if !grid[i][j][k] {
                    continue;
                }
                for dir in directions {
                    let x = (i as i64 + dir.0) as usize;
                    let y = (j as i64 + dir.1) as usize;
                    let z = (k as i64 + dir.2) as usize;
                    if !grid[x][y][z] && (exterior_grid.len() == 0 || exterior_grid[x][y][z]) {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

fn create_lava_grid(lines: &Vec<String>) -> Vec<Vec<Vec<bool>>> {
    let droplets = lines
        .iter()
        .map(|el| {
            el.split(",")
                .map(|e| e.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let max_x = droplets.iter().map(|el| el[0]).max().unwrap();
    let max_y = droplets.iter().map(|el| el[1]).max().unwrap();
    let max_z = droplets.iter().map(|el| el[2]).max().unwrap();

    // +3 for padding
    let mut grid = vec![vec![vec![false; max_z + 3]; max_y + 3]; max_x + 3];

    for drop in droplets {
        // include padding for index 0 entries
        grid[drop[0] + 1][drop[1] + 1][drop[2] + 1] = true;
    }

    grid
}

fn create_exterior_indication_grid(grid: &Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<bool>>> {
    let (dim_x, dim_y, dim_z) = (grid.len(), grid[0].len(), grid[0][0].len());
    let dims = (dim_x, dim_y, dim_z);
    let mut exterior_grid = vec![vec![vec![false; dim_z]; dim_y]; dim_x];
    let mut visited = vec![vec![vec![false; dim_z]; dim_y]; dim_x];
    visited[0][0][0] = true;

    let directions = get_3d_neighbour_directions();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    while queue.len() > 0 {
        let next_index = queue.pop_front().unwrap();
        exterior_grid[next_index.0][next_index.1][next_index.2] = true;

        for dir in directions {
            let temp_coord = (
                next_index.0 as i64 + dir.0,
                next_index.1 as i64 + dir.1,
                next_index.2 as i64 + dir.2,
            );
            if !coord_is_in_dim_bounds(&temp_coord, &dims) {
                continue;
            }
            let coord = (
                temp_coord.0 as usize,
                temp_coord.1 as usize,
                temp_coord.2 as usize,
            );
            if visited[coord.0][coord.1][coord.2] {
                continue;
            }
            visited[coord.0][coord.1][coord.2] = true;
            if !grid[coord.0][coord.1][coord.2] {
                queue.push_back(coord);
            }
        }
    }

    exterior_grid
}

fn get_3d_neighbour_directions() -> [(i64, i64, i64); 6] {
    [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]
}

fn coord_is_in_dim_bounds(coord: &(i64, i64, i64), dims: &(usize, usize, usize)) -> bool {
    coord.0 >= 0
        && coord.0 < dims.0 as i64
        && coord.1 > 0
        && coord.1 < dims.1 as i64
        && coord.2 >= 0
        && coord.2 < dims.2 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 64);
    }

    #[test]
    fn test_solve1_large() {
        let lines = read_file_as_vector("./files/day18.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 3454);
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 58);
    }
}
