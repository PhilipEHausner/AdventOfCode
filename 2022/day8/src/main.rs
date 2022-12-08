use util::read_files::read_file_as_vector;

type Grid = Vec<Vec<u8>>;
type U64Grid = Vec<Vec<u64>>;
type BoolGrid = Vec<Vec<bool>>;

fn main() {
    let lines = read_file_as_vector("./files/day8.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    let mut grid = create_grid(lines);
    let mut visible: BoolGrid = vec![vec![false; grid[0].len()]; grid.len()];

    process_row_left_to_right(&mut grid, &mut visible);
    process_row_right_to_left(&mut grid, &mut visible);
    process_col_top_to_bot(&mut grid, &mut visible);
    process_col_bot_to_top(&mut grid, &mut visible);

    visible.iter().map(|row| row.iter().map(|el| if *el == true {1} else {0}).sum::<u64>()).sum()
}

fn solve2(lines: &Vec<String>) -> u64 {
    let mut grid = create_grid(lines);
    let mut scenic_scores: U64Grid = vec![vec![0; grid[0].len()]; grid.len()];

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            scenic_scores[row][col] = compute_scenic_score(&mut grid, row, col);
        }
    }

    scenic_scores.iter().map(|row| *row.iter().max().unwrap()).max().unwrap()
}

fn create_grid(lines: &Vec<String>) -> Grid {
    lines.iter().map(|line| line.chars().into_iter().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect()
}

fn process_row_left_to_right(grid: &mut Grid, visible: &mut BoolGrid) {
    for row in 0..grid.len() {
        let mut curr_highest = grid[row][0];
        visible[row][0] = true;
        for col in 0..grid[row].len() {
            update_row_col_entry(grid, visible, &mut curr_highest, row, col);
        }
    }
}

fn process_row_right_to_left(grid: &mut Grid, visible: &mut BoolGrid) {
    for row in 0..grid.len() {
        let last_col = grid[row].len() - 1;
        let mut curr_highest = grid[row][last_col];
        visible[row][last_col] = true;
        for col in (0..grid[row].len()).rev() {
            update_row_col_entry(grid, visible, &mut curr_highest, row, col);
        }
    }
}

fn process_col_top_to_bot(grid: &mut Grid, visible: &mut BoolGrid) {
    for col in 0..grid[0].len() {
        let mut curr_highest = grid[0][col];
        visible[0][col] = true;
        for row in 0..grid.len() {
            update_row_col_entry(grid, visible, &mut curr_highest, row, col);
        }
    }
}

fn process_col_bot_to_top(grid: &mut Grid, visible: &mut BoolGrid) {
    for col in 0..grid[0].len() {
        let last_row = grid.len() - 1;
        let mut curr_highest = grid[last_row][col];
        visible[last_row][col] = true;
        for row in (0..grid.len()).rev() {
            update_row_col_entry(grid, visible, &mut curr_highest, row, col);
        }
    }
}

fn update_row_col_entry(grid: &mut Grid, visible: &mut BoolGrid, curr_highest: &mut u8, row: usize, col: usize) {
    if grid[row][col] > *curr_highest {
        visible[row][col] = true;
        *curr_highest = grid[row][col];
    }
}

fn compute_scenic_score(grid: &mut Grid, row: usize, col: usize) -> u64 {
    let mut result = 1;

    result *= scenic_score_top(grid, row, col);
    result *= scenic_score_bot(grid, row, col);
    result *= scenic_score_left(grid, row, col);
    result *= scenic_score_right(grid, row, col);

    result
}

fn scenic_score_top(grid: &mut Grid, row: usize, col: usize) -> u64 {
    let mut result = 0;

    let tree_size = grid[row][col];
    for i in (0..row).rev() {
        result += 1;
        if grid[i][col] >= tree_size {
            break;
        }
    }

    result
}

fn scenic_score_bot(grid: &mut Grid, row: usize, col: usize) -> u64 {
    let mut result = 0;

    let tree_size = grid[row][col];
    for i in (row+1)..grid.len() {
        result += 1;
        if grid[i][col] >= tree_size {
            break;
        }
    }

    result
}

fn scenic_score_left(grid: &mut Grid, row: usize, col: usize) -> u64 {
    let mut result = 0;

    let tree_size = grid[row][col];
    for i in (0..col).rev() {
        result += 1;
        if grid[row][i] >= tree_size {
            break;
        }
    }

    result
}

fn scenic_score_right(grid: &mut Grid, row: usize, col: usize) -> u64 {
    let mut result = 0;

    let tree_size = grid[row][col];
    for i in (col+1)..grid[row].len() {
        result += 1;
        if grid[row][i] >= tree_size {
            break;
        }
        
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 21);
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 8);
    }
}
