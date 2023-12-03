use std::collections::HashSet;
use std::fmt;

use util::read_files::read_file_as_vector;

#[derive(Debug, Clone)]
pub enum Field {
    Number(i64),
    Symbol(char),
    Empty,
}

impl Field {
    fn is_symbol(&self) -> bool {
        match self {
            Field::Number(_) => false,
            Field::Symbol(_) => true,
            Field::Empty => false,
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Field::Number(_) => true,
            Field::Symbol(_) => false,
            Field::Empty => false,
        }
    }

    pub fn is_gear(&self) -> bool {
        match self {
            Field::Number(_) => false,
            Field::Symbol(s) => return s == &'*',
            Field::Empty => false,
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Field::Number(n) => n.to_string(),
            Field::Symbol(s) => s.to_string(),
            Field::Empty => ".".to_string(),
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
pub struct Dims {
    pub x: usize,
    pub y: usize,
}

impl Dims {
    pub fn new(x: usize, y: usize) -> Dims {
        Dims { x, y }
    }
}

#[derive(Debug)]
pub struct Schema {
    fields: Vec<Vec<Field>>,
    dims: Dims,
}

impl Schema {
    pub fn new(file: &str) -> Schema {
        let lines: Vec<String> = read_file_as_vector(file).expect("File not found.");
        let fields = lines
            .into_iter()
            .map(|it| Schema::parse_line(&it))
            .collect();
        Schema::assert_all_rows_same_length(&fields);
        let dims = Schema::compute_dims(&fields);
        Schema { fields, dims }
    }

    fn assert_all_rows_same_length<T>(fields: &Vec<Vec<T>>) {
        assert_eq!(
            1,
            fields
                .into_iter()
                .map(|line| line.len())
                .collect::<HashSet<usize>>()
                .len()
        );
    }

    fn compute_dims<T>(fields: &Vec<Vec<T>>) -> Dims {
        Dims::new(
            fields.len(),
            if fields.len() > 0 {
                fields.get(0).unwrap().len()
            } else {
                0
            },
        )
    }

    fn parse_line(line: &str) -> Vec<Field> {
        line.chars()
            .map(|c| match c {
                '.' => Field::Empty,
                '0'..='9' => Field::Number(c.to_digit(10).unwrap() as i64),
                _ => Field::Symbol(c),
            })
            .collect()
    }

    pub fn dims(&self) -> &Dims {
        &self.dims
    }

    pub fn get_field(&self, x: usize, y: usize) -> Field {
        self.fields.get(x).unwrap().get(y).unwrap().clone()
    }

    pub fn neighbour_is_symbol(&self, x: usize, y: usize) -> bool {
        assert!(x < self.dims.x);
        assert!(y < self.dims.y);
        if x > 0 {
            if y > 0 && self.get_field(x - 1, y - 1).is_symbol() {
                return true;
            }
            if self.get_field(x - 1, y).is_symbol() {
                return true;
            }
            if y < self.dims.y - 1 && self.get_field(x - 1, y + 1).is_symbol() {
                return true;
            }
        }
        if y > 0 && self.get_field(x, y - 1).is_symbol() {
            return true;
        }
        if y < self.dims.y - 1 && self.get_field(x, y + 1).is_symbol() {
            return true;
        }
        if x < self.dims.x - 1 {
            if y > 0 && self.get_field(x + 1, y - 1).is_symbol() {
                return true;
            }
            if self.get_field(x + 1, y).is_symbol() {
                return true;
            }
            if y < self.dims.y - 1 && self.get_field(x + 1, y + 1).is_symbol() {
                return true;
            }
        }
        false
    }

    pub fn numbers_around_field(&self, x: usize, y: usize) -> Vec<i64> {
        let mut result = vec![];

        // top row
        if x > 0 {
            if self.get_field(x - 1, y).is_number() {
                result.push(self.parse_number_from_index(x - 1, y));
            } else {
                if y > 0 && self.get_field(x - 1, y - 1).is_number() {
                    result.push(self.parse_number_from_index(x - 1, y - 1));
                }
                if y < self.dims.y - 1 && self.get_field(x - 1, y + 1).is_number() {
                    result.push(self.parse_number_from_index(x - 1, y + 1));
                }
            }
        }

        // mid row
        if self.get_field(x, y).is_number() {
            result.push(self.parse_number_from_index(x, y));
        } else {
            if y > 0 && self.get_field(x, y - 1).is_number() {
                result.push(self.parse_number_from_index(x, y - 1));
            }
            if y < self.dims.y - 1 && self.get_field(x, y + 1).is_number() {
                result.push(self.parse_number_from_index(x, y + 1));
            }
        }

        // bottom row
        if x < self.dims.x - 1 {
            if self.get_field(x + 1, y).is_number() {
                result.push(self.parse_number_from_index(x + 1, y));
            } else {
                if y > 0 && self.get_field(x + 1, y - 1).is_number() {
                    result.push(self.parse_number_from_index(x + 1, y - 1));
                }
                if y < self.dims.y - 1 && self.get_field(x + 1, y + 1).is_number() {
                    result.push(self.parse_number_from_index(x + 1, y + 1));
                }
            }
        }

        result
    }

    fn parse_number_from_index(&self, x: usize, mut y: usize) -> i64 {
        while y > 0 && self.get_field(x, y - 1).is_number() {
            y -= 1;
        }
        let mut result = 0;
        while y < self.dims.y {
            match self.get_field(x, y) {
                Field::Number(num) => result = result * 10 + num,
                Field::Symbol(_) => break,
                Field::Empty => break,
            }
            y += 1;
        }
        result
    }
}
