use util::read_files::read_file_as_vector;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, PartialEq)]
enum GamePlanTile {
    Wall,
    Empty,
    Storm(Vec<Direction>),
}

type GamePlan = Vec<Vec<GamePlanTile>>;

fn main() {
    let lines = read_file_as_vector("./files/day24.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    let mut game_plan = create_game_plan(lines);
    let start_field = get_start_field(lines);
    let goal_field = get_goal_field(lines);

    move_through_game_plan(&mut game_plan, start_field, goal_field)
}

fn solve2(lines: &Vec<String>) -> u64 {
    let mut game_plan = create_game_plan(lines);
    let start_field = get_start_field(lines);
    let goal_field = get_goal_field(lines);

    let mut rounds = move_through_game_plan(&mut game_plan, start_field, goal_field);
    rounds += move_through_game_plan(&mut game_plan, goal_field, start_field);
    rounds += move_through_game_plan(&mut game_plan, start_field, goal_field);

    rounds
}

fn move_through_game_plan(
    game_plan: &mut GamePlan,
    start: (usize, usize),
    goal: (usize, usize),
) -> u64 {
    let mut possible_elve_positions = create_possible_elve_positions(start, &game_plan);

    let mut rounds = 0;
    loop {
        rounds += 1;
        process_round(game_plan, &mut possible_elve_positions);

        if goal_is_reached(&mut possible_elve_positions, goal) {
            break;
        }
    }

    rounds
}

fn create_game_plan(lines: &Vec<String>) -> GamePlan {
    let mut game_plan = vec![vec![GamePlanTile::Empty; lines[0].len()]; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        for (j, tile) in line.chars().enumerate() {
            match tile {
                '#' => game_plan[i][j] = GamePlanTile::Wall,
                '^' => game_plan[i][j] = GamePlanTile::Storm(vec![Direction::North]),
                '>' => game_plan[i][j] = GamePlanTile::Storm(vec![Direction::East]),
                'v' => game_plan[i][j] = GamePlanTile::Storm(vec![Direction::South]),
                '<' => game_plan[i][j] = GamePlanTile::Storm(vec![Direction::West]),
                _ => {}
            }
        }
    }

    game_plan
}

fn create_possible_elve_positions(
    start_pos: (usize, usize),
    game_plan: &GamePlan,
) -> Vec<Vec<bool>> {
    let mut possible_elve_positions = vec![vec![false; game_plan[0].len()]; game_plan.len()];
    possible_elve_positions[start_pos.0][start_pos.1] = true;
    possible_elve_positions
}

fn get_start_field(lines: &Vec<String>) -> (usize, usize) {
    let mut result = (0, 0);

    'outer: for (i, line) in lines.iter().enumerate() {
        for (j, tile) in line.chars().enumerate() {
            if tile == 'E' {
                result = (i, j);
                break 'outer;
            }
        }
    }

    result
}

fn get_goal_field(lines: &Vec<String>) -> (usize, usize) {
    let mut result = (0, 0);

    'outer: for (i, line) in lines.iter().enumerate() {
        for (j, tile) in line.chars().enumerate() {
            if tile == 'G' {
                result = (i, j);
                break 'outer;
            }
        }
    }

    result
}

fn process_round(game_plan: &mut GamePlan, possible_elve_positions: &mut Vec<Vec<bool>>) {
    spread_elve_positions(possible_elve_positions);
    move_storms(game_plan);
    thin_out_elve_positions(game_plan, possible_elve_positions);
}

fn spread_elve_positions(possible_elve_positions: &mut Vec<Vec<bool>>) {
    let mut new_possible_elve_positions =
        vec![vec![false; possible_elve_positions[0].len()]; possible_elve_positions.len()];

    for (i, row) in possible_elve_positions.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == true {
                new_possible_elve_positions[i][j] = true;
                if i > 0 {
                    new_possible_elve_positions[i - 1][j] = true;
                }
                if i < possible_elve_positions.len() - 1 {
                    new_possible_elve_positions[i + 1][j] = true;
                }
                if j > 0 {
                    new_possible_elve_positions[i][j - 1] = true;
                }
                if j < possible_elve_positions[0].len() - 1 {
                    new_possible_elve_positions[i][j + 1] = true;
                }
            }
        }
    }

    *possible_elve_positions = new_possible_elve_positions;
}

fn move_storms(game_plan: &mut GamePlan) {
    let temp_plan = game_plan.clone();
    let new_storm_positions = get_new_storm_positions(&temp_plan);
    update_storms(game_plan, &new_storm_positions);
}

fn get_new_storm_positions(game_plan: &GamePlan) -> Vec<(&Direction, (usize, usize))> {
    let mut new_storm_positions = vec![];

    for (i, row) in game_plan.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            match tile {
                GamePlanTile::Storm(directions) => {
                    for direction in directions {
                        let next_pos = new_storm_pos(i, j, game_plan, direction);
                        new_storm_positions.push((direction, next_pos));
                    }
                }
                _ => continue,
            }
        }
    }

    new_storm_positions
}

fn new_storm_pos(
    row: usize,
    col: usize,
    game_plan: &GamePlan,
    direction: &Direction,
) -> (usize, usize) {
    match direction {
        Direction::North => {
            let mut n_index = row;
            loop {
                if n_index == 0 {
                    n_index = game_plan.len() - 1;
                } else {
                    n_index -= 1;
                }
                if game_plan[n_index][col] != GamePlanTile::Wall {
                    break;
                }
            }
            (n_index, col)
        }
        Direction::East => {
            let mut e_index = col;
            loop {
                if e_index == game_plan[0].len() - 1 {
                    e_index = 0;
                } else {
                    e_index += 1;
                }
                if game_plan[row][e_index] != GamePlanTile::Wall {
                    break;
                }
            }
            (row, e_index)
        }
        Direction::South => {
            let mut s_index = row;
            loop {
                if s_index == game_plan.len() - 1 {
                    s_index = 0;
                } else {
                    s_index += 1;
                }
                if game_plan[s_index][col] != GamePlanTile::Wall {
                    break;
                }
            }
            (s_index, col)
        }
        Direction::West => {
            let mut w_index = col;
            loop {
                if w_index == 0 {
                    w_index = game_plan[0].len() - 1;
                } else {
                    w_index -= 1;
                }
                if game_plan[row][w_index] != GamePlanTile::Wall {
                    break;
                }
            }
            (row, w_index)
        }
    }
}

fn update_storms(
    game_plan: &mut GamePlan,
    new_storm_positions: &Vec<(&Direction, (usize, usize))>,
) {
    for i in 0..game_plan.len() {
        for j in 0..game_plan[0].len() {
            match game_plan[i][j] {
                GamePlanTile::Storm(_) => game_plan[i][j] = GamePlanTile::Empty,
                _ => continue,
            }
        }
    }

    for (direction, pos) in new_storm_positions {
        match &mut game_plan[pos.0][pos.1] {
            GamePlanTile::Storm(directions) => directions.push(**direction),
            _ => game_plan[pos.0][pos.1] = GamePlanTile::Storm(vec![**direction]),
        }
    }
}

fn thin_out_elve_positions(game_plan: &GamePlan, possible_elve_positions: &mut Vec<Vec<bool>>) {
    for row in 0..possible_elve_positions.len() {
        for col in 0..possible_elve_positions[0].len() {
            if possible_elve_positions[row][col] && game_plan[row][col] != GamePlanTile::Empty {
                possible_elve_positions[row][col] = false;
            }
        }
    }
}

fn goal_is_reached(
    possible_elve_positions: &mut Vec<Vec<bool>>,
    goal_field: (usize, usize),
) -> bool {
    possible_elve_positions[goal_field.0][goal_field.1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 18);
    }

    #[test]
    fn test_solve1_large() {
        let lines = read_file_as_vector("./files/day24.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 245);
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 54);
    }

    #[test]
    fn test_solve2_large() {
        let lines = read_file_as_vector("./files/day24.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 798);
    }
}
