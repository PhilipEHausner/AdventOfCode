use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Tile {
    tile_type: char,
    row: usize,
    col: usize,
}

impl Tile {
    fn new(tile_type: char, row: usize, col: usize) -> Tile {
        if tile_type != '#' && tile_type != '.' {
            panic!("Unexpected tile type '{}'.", tile_type)
        }
        Tile {
            tile_type,
            row,
            col,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Neighbours {
    north: Tile,
    east: Tile,
    south: Tile,
    west: Tile,
}

impl Neighbours {
    fn new(north: Tile, east: Tile, south: Tile, west: Tile) -> Neighbours {
        Neighbours {
            north,
            east,
            south,
            west,
        }
    }

    fn get_direction_tile(&self, direction: &Direction) -> &Tile {
        match direction {
            Direction::North => &self.north,
            Direction::East => &self.east,
            Direction::South => &self.south,
            Direction::West => &self.west,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
    facing: Direction,
    neighbour_map: NeighbourMap,
}

impl Position {
    fn new(row: usize, col: usize, facing: Direction, neighbour_map: NeighbourMap) -> Position {
        Position {
            row,
            col,
            facing,
            neighbour_map,
        }
    }

    fn process_command(&mut self, command: &Command) {
        match command {
            Command::Turn(turn_direction) => self.turn(turn_direction),
            Command::Move(num_steps) => self.step(*num_steps),
        }
    }

    fn turn(&mut self, turn_direction: &TurnDirection) {
        self.facing = self.facing.change_direction(turn_direction);
    }

    fn step(&mut self, num_steps: u64) {
        for _ in 0..num_steps {
            let neighbour = self.neighbour_map.get(&(self.row, self.col)).unwrap();
            let tile = neighbour.get_direction_tile(&self.facing);
            if tile.tile_type == '#' {
                break;
            } else if tile.tile_type == '.' {
                self.row = tile.row;
                self.col = tile.col;
            }
        }
    }

    fn calc_position_value(&self) -> u64 {
        1000 * (self.row + 1) as u64 + 4 * (self.col + 1) as u64 + self.facing.value()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "Position {{ row: {}, col: {}, facing: {:?} }}",
            self.row, self.col, self.facing
        );
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn change_direction(&self, turn_direction: &TurnDirection) -> Direction {
        match (self, &turn_direction) {
            (Direction::North, TurnDirection::Left) => Direction::West,
            (Direction::North, TurnDirection::Right) => Direction::East,
            (Direction::East, TurnDirection::Left) => Direction::North,
            (Direction::East, TurnDirection::Right) => Direction::South,
            (Direction::South, TurnDirection::Left) => Direction::East,
            (Direction::South, TurnDirection::Right) => Direction::West,
            (Direction::West, TurnDirection::Left) => Direction::South,
            (Direction::West, TurnDirection::Right) => Direction::North,
        }
    }

    fn value(&self) -> u64 {
        match self {
            Direction::North => 3,
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
        }
    }
}

#[derive(Debug, PartialEq)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Command {
    Turn(TurnDirection),
    Move(u64),
}

type NeighbourMap = HashMap<(usize, usize), Neighbours>;

pub fn solve(lines: &Vec<String>) -> u64 {
    let grid = create_grid(lines);
    let neighbour_map = create_neighbour_map(&grid);
    let commands = get_commands(lines);
    let mut position = get_start_position(&grid, neighbour_map);

    for command in &commands {
        position.process_command(command);
    }

    position.calc_position_value()
}

fn create_grid(lines: &Vec<String>) -> Vec<Vec<char>> {
    let (rows, cols) = get_number_cols_and_rows(lines);
    let mut grid = vec![vec!['x'; cols]; rows];

    for (row, line) in lines.iter().enumerate() {
        if row >= rows {
            break;
        }
        for (col, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            grid[row][col] = c;
        }
    }

    grid
}

fn get_number_cols_and_rows(lines: &Vec<String>) -> (usize, usize) {
    let mut rows = 0;
    let mut cols = 0;

    for line in lines {
        if line.len() == 0 {
            break;
        }
        cols = std::cmp::max(cols, line.len());
        rows += 1;
    }

    (rows, cols)
}

fn create_neighbour_map(grid: &Vec<Vec<char>>) -> NeighbourMap {
    let mut neighbour_map = HashMap::new();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'x' {
                continue;
            }
            neighbour_map.insert((row, col), get_neighbours_for_field(grid, row, col));
        }
    }

    neighbour_map
}

fn get_neighbours_for_field(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Neighbours {
    let north;
    let mut n_index = if row == 0 { grid.len() - 1 } else { row - 1 };
    loop {
        if grid[n_index][col] != 'x' {
            north = Tile::new(grid[n_index][col], n_index, col);
            break;
        }
        n_index = if n_index == 0 {
            grid.len() - 1
        } else {
            n_index - 1
        };
    }

    let south;
    let mut s_index = if row == grid.len() - 1 { 0 } else { row + 1 };
    loop {
        if grid[s_index][col] != 'x' {
            south = Tile::new(grid[s_index][col], s_index, col);
            break;
        }
        s_index = if s_index == grid.len() - 1 {
            0
        } else {
            s_index + 1
        };
    }

    let east;
    let mut e_index = if col == grid[row].len() - 1 {
        0
    } else {
        col + 1
    };
    loop {
        if grid[row][e_index] != 'x' {
            east = Tile::new(grid[row][e_index], row, e_index);
            break;
        }
        e_index = if e_index == grid[row].len() - 1 {
            0
        } else {
            e_index + 1
        };
    }

    let west;
    let mut w_index = if col == 0 {
        grid[row].len() - 1
    } else {
        col - 1
    };
    loop {
        if grid[row][w_index] != 'x' {
            west = Tile::new(grid[row][w_index], row, w_index);
            break;
        }
        w_index = if w_index == 0 {
            grid[row].len() - 1
        } else {
            w_index - 1
        };
    }

    Neighbours::new(north, east, south, west)
}

fn get_commands(lines: &Vec<String>) -> Vec<Command> {
    let mut commands = vec![];

    let command_line = &lines[lines.len() - 1];
    let re = regex::Regex::new(r"(\d+|R|L)").unwrap();

    for field in re.find_iter(&command_line) {
        let command = match field.as_str() {
            "L" => Command::Turn(TurnDirection::Left),
            "R" => Command::Turn(TurnDirection::Right),
            def => Command::Move(def.parse().unwrap()),
        };
        commands.push(command);
    }

    commands
}

fn get_start_position(grid: &Vec<Vec<char>>, neighbour_map: NeighbourMap) -> Position {
    let col = grid[0]
        .iter()
        .enumerate()
        .map(|(i, &c)| if c != 'x' { i } else { 10000000 })
        .min()
        .unwrap();
    Position::new(0, col, Direction::East, neighbour_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::read_files::read_file_as_vector;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve(&lines), 6032);
    }

    #[test]
    fn test_solve1_large() {
        let lines = read_file_as_vector("./files/day22.txt").expect("Error reading file.");
        assert_eq!(solve(&lines), 189140);
    }

    #[test]
    fn test_create_grid() {
        let lines = read_file_as_vector("./files/minimal.txt").expect("Error reading file.");
        let grid = create_grid(&lines);
        assert_eq!(
            grid,
            vec![
                vec!['.', '#', 'x', 'x'],
                vec!['.', '.', '#', 'x'],
                vec!['x', 'x', '.', '.']
            ]
        );
    }

    #[test]
    fn test_get_number_cols_and_rows() {
        let lines = read_file_as_vector("./files/minimal.txt").expect("Error reading file.");
        let (rows, cols) = get_number_cols_and_rows(&lines);
        assert_eq!(rows, 3);
        assert_eq!(cols, 4);
    }

    #[test]
    fn test_create_neighbour_map() {
        let lines = read_file_as_vector("./files/minimal.txt").expect("Error reading file.");
        let grid = create_grid(&lines);

        let neighbour_map = create_neighbour_map(&grid);
        assert_eq!(neighbour_map.len(), 7);
    }

    #[test]
    fn test_get_neighbours_for_field() {
        let lines = read_file_as_vector("./files/minimal.txt").expect("Error reading file.");
        let grid = create_grid(&lines);

        let neighbours1 = get_neighbours_for_field(&grid, 1, 1);
        assert_eq!(neighbours1.north, Tile::new('#', 0, 1));
        assert_eq!(neighbours1.east, Tile::new('#', 1, 2));
        assert_eq!(neighbours1.south, Tile::new('#', 0, 1));
        assert_eq!(neighbours1.west, Tile::new('.', 1, 0));

        let neighbours2 = get_neighbours_for_field(&grid, 1, 2);
        assert_eq!(neighbours2.north, Tile::new('.', 2, 2));
        assert_eq!(neighbours2.east, Tile::new('.', 1, 0));
        assert_eq!(neighbours2.south, Tile::new('.', 2, 2));
        assert_eq!(neighbours2.west, Tile::new('.', 1, 1));

        let neighbours3 = get_neighbours_for_field(&grid, 2, 2);
        assert_eq!(neighbours3.north, Tile::new('#', 1, 2));
        assert_eq!(neighbours3.east, Tile::new('.', 2, 3));
        assert_eq!(neighbours3.south, Tile::new('#', 1, 2));
        assert_eq!(neighbours3.west, Tile::new('.', 2, 3));
    }

    #[test]
    fn test_get_commands() {
        let lines = read_file_as_vector("./files/minimal.txt").expect("Error reading file.");
        let commands = get_commands(&lines);

        assert_eq!(commands.len(), 9);
        assert_eq!(commands[0], Command::Move(1));
        assert_eq!(commands[1], Command::Turn(TurnDirection::Right));
        assert_eq!(commands[2], Command::Move(1));
        assert_eq!(commands[3], Command::Turn(TurnDirection::Left));
        assert_eq!(commands[4], Command::Move(2));
        assert_eq!(commands[5], Command::Turn(TurnDirection::Right));
        assert_eq!(commands[6], Command::Move(10));
        assert_eq!(commands[7], Command::Turn(TurnDirection::Left));
        assert_eq!(commands[8], Command::Move(2));
    }

    #[test]
    fn test_get_start_position() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        let grid = create_grid(&lines);

        let start_pos = get_start_position(&grid, HashMap::new());
        assert_eq!(start_pos.row, 0);
        assert_eq!(start_pos.col, 8);
        assert_eq!(start_pos.facing, Direction::East);
    }
}
