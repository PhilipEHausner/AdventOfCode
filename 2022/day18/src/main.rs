use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day18.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    let grid = create_lava_grid(lines);

    let mut result = 0;

    let (dim_x, dim_y, dim_z) = (grid.len(), grid[0].len(), grid[0][0].len());
    let directions: [(i64, i64, i64); 6] = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

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
                    if !grid[x][y][z] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 64);
    }
}
