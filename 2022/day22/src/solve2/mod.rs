use crate::solve1::{
    create_grid, get_commands, get_start_position, Direction, NeighbourMap, Neighbours, Tile,
};

#[derive(Debug)]
struct Face {
    face_index: usize,
    row_index: usize,
    col_index: usize,
    square_size: usize,
}

impl Face {
    fn new(face_index: usize, row_index: usize, col_index: usize, square_size: usize) -> Face {
        Face {
            face_index,
            row_index,
            col_index,
            square_size,
        }
    }

    fn first_row(&self) -> usize {
        self.row_index * self.square_size
    }

    fn last_row(&self) -> usize {
        self.row_index * self.square_size + self.square_size - 1
    }

    fn first_col(&self) -> usize {
        self.col_index * self.square_size
    }

    fn last_col(&self) -> usize {
        self.col_index * self.square_size + self.square_size - 1
    }
}

#[derive(Clone, Copy)]
struct Edge {
    source_face: usize,
    source_direction: Direction,
    target_face: usize,
    target_direction: Direction,
}

impl Edge {
    fn new(
        source_face: usize,
        source_direction: Direction,
        target_face: usize,
        target_direction: Direction,
    ) -> Edge {
        Edge {
            source_face,
            source_direction,
            target_face,
            target_direction,
        }
    }
}

pub fn solve(lines: &Vec<String>, meta_info: &Vec<String>) -> u64 {
    let grid = create_grid(lines);
    let (faces, edges, square_size) = process_meta_information(meta_info);
    let neighbour_map = create_neighbour_map(&grid, square_size, &faces, &edges);
    let commands = get_commands(lines);
    let mut position = get_start_position(&grid, neighbour_map);

    for command in &commands {
        position.process_command(command);
    }

    position.calc_position_value()
}

fn process_meta_information(meta_info: &Vec<String>) -> (Vec<Face>, Vec<Edge>, usize) {
    let mut faces = vec![];
    let mut edges = vec![];
    let square_size = meta_info[35].parse().unwrap();

    for i in 1..7 {
        let face_coords: Vec<usize> = meta_info[i]
            .split(",")
            .map(|el| el.parse().unwrap())
            .collect();
        faces.push(Face::new(
            i - 1,
            face_coords[1],
            face_coords[0],
            square_size,
        ));
    }

    for i in 9..33 {
        let edge_coords: Vec<usize> = meta_info[i]
            .split(",")
            .map(|el| el.parse().unwrap())
            .collect();
        edges.push(Edge::new(
            edge_coords[0],
            Direction::from_usize(edge_coords[1]),
            edge_coords[2],
            Direction::from_usize(edge_coords[3]),
        ))
    }

    (faces, edges, square_size)
}

fn create_neighbour_map(
    grid: &Vec<Vec<char>>,
    square_size: usize,
    faces: &Vec<Face>,
    edges: &Vec<Edge>,
) -> NeighbourMap {
    let mut neighbour_map = NeighbourMap::new();

    for face in faces {
        let (row_low, row_up, col_low, col_up) = (
            face.row_index * square_size,
            face.row_index * square_size + square_size,
            face.col_index * square_size,
            face.col_index * square_size + square_size,
        );
        for row in row_low..row_up {
            for col in col_low..col_up {
                if grid[row][col] == 'x' {
                    panic!(
                        "Trying to access invalid grid field, row {}, col {}.",
                        row, col
                    );
                }
                neighbour_map.insert(
                    (row, col),
                    get_neighbours_for_field(
                        grid, row, col, face, faces, edges, row_low, row_up, col_low, col_up,
                    ),
                );
            }
        }
    }

    neighbour_map
}

fn get_neighbours_for_field(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    face: &Face,
    faces: &Vec<Face>,
    edges: &Vec<Edge>,
    row_low: usize,
    row_up: usize,
    col_low: usize,
    col_up: usize,
) -> Neighbours {
    assert!(row >= row_low);
    assert!(row < row_up);
    assert!(col >= col_low);
    assert!(col < col_up);

    let north;
    if row > row_low {
        let n_index = row - 1;
        north = Tile::new(grid[n_index][col], n_index, col, Direction::North);
    } else {
        let edge = get_edge_for_face_and_direction(face, edges, Direction::North);
        let coords = get_over_boundary_coordinates(row, col, faces, &edge);
        north = Tile::new(
            grid[coords.0][coords.1],
            coords.0,
            coords.1,
            edge.target_direction,
        );
    }

    let south;
    if row < row_up - 1 {
        let s_index = row + 1;
        south = Tile::new(grid[s_index][col], s_index, col, Direction::South);
    } else {
        let edge = get_edge_for_face_and_direction(face, edges, Direction::South);
        let coords = get_over_boundary_coordinates(row, col, faces, &edge);
        south = Tile::new(
            grid[coords.0][coords.1],
            coords.0,
            coords.1,
            edge.target_direction,
        );
    }

    let east;
    if col < col_up - 1 {
        let e_index = col + 1;
        east = Tile::new(grid[row][e_index], row, e_index, Direction::East);
    } else {
        let edge = get_edge_for_face_and_direction(face, edges, Direction::East);
        let coords = get_over_boundary_coordinates(row, col, faces, &edge);
        east = Tile::new(
            grid[coords.0][coords.1],
            coords.0,
            coords.1,
            edge.target_direction,
        );
    }

    let west;
    if col > col_low {
        let w_index = col - 1;
        west = Tile::new(grid[row][w_index], row, w_index, Direction::West);
    } else {
        let edge = get_edge_for_face_and_direction(face, edges, Direction::West);
        let coords = get_over_boundary_coordinates(row, col, faces, &edge);
        west = Tile::new(
            grid[coords.0][coords.1],
            coords.0,
            coords.1,
            edge.target_direction,
        );
    }

    Neighbours::new(north, east, south, west)
}

fn get_edge_for_face_and_direction(face: &Face, edges: &Vec<Edge>, direction: Direction) -> Edge {
    for edge in edges {
        if edge.source_face == face.face_index && edge.source_direction == direction {
            return *edge;
        }
    }
    panic!(
        "No edge found for face {} and dir {:?}",
        face.face_index, direction
    );
}

fn get_over_boundary_coordinates(
    row: usize,
    col: usize,
    faces: &Vec<Face>,
    edge: &Edge,
) -> (usize, usize) {
    // only valid for AoC, there are some edge cases left for the arbitrary case.
    // north/north, west/west, east/east do not work in all cases.

    let other_face = &faces[edge.target_face];
    let square_size = other_face.square_size;
    match (edge.source_direction, edge.target_direction) {
        (Direction::North, Direction::North) => (other_face.last_row(), col),
        (Direction::North, Direction::East) => (
            other_face.first_row() + col % square_size,
            other_face.first_col(),
        ),
        (Direction::North, Direction::South) => (
            other_face.first_row(),
            other_face.last_col() - col % square_size,
        ),
        (Direction::North, Direction::West) => (
            other_face.last_row() - col % square_size,
            other_face.last_col(),
        ),
        (Direction::East, Direction::North) => (
            other_face.last_row(),
            other_face.first_col() + row % square_size,
        ),
        (Direction::East, Direction::East) => (row, other_face.first_col()),
        (Direction::East, Direction::South) => (
            other_face.first_row(),
            other_face.last_col() - row % square_size,
        ),
        (Direction::East, Direction::West) => (
            other_face.last_row() - row % square_size,
            other_face.last_col(),
        ),
        (Direction::South, Direction::North) => (
            other_face.last_row(),
            other_face.last_col() - col % square_size,
        ),
        (Direction::South, Direction::East) => (
            other_face.first_row(),
            other_face.last_col() - row % square_size,
        ),
        (Direction::South, Direction::South) => (
            other_face.first_row(),
            other_face.first_col() + col % square_size,
        ),
        (Direction::South, Direction::West) => (
            other_face.first_row() + col % square_size,
            other_face.last_col(),
        ),
        (Direction::West, Direction::North) => (
            other_face.last_row(),
            other_face.last_col() - row % square_size,
        ),
        (Direction::West, Direction::East) => (
            other_face.last_row() - row % square_size,
            other_face.first_col(),
        ),
        (Direction::West, Direction::South) => (
            other_face.first_row(),
            other_face.first_col() + row % square_size,
        ),
        (Direction::West, Direction::West) => (row, other_face.last_col()),
    }
}
