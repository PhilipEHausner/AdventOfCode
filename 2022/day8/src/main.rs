use util::read_files::read_file_as_vector;

type Grid = Vec<Vec<u8>>;
type BoolGrid = Vec<Vec<bool>>;

fn main() {
    let lines = read_file_as_vector("./files/day8.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 21);
    }
}
