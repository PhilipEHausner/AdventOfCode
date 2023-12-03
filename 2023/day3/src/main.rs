use crate::schema::Field;
use crate::schema::Schema;

mod schema;

fn main() {
    let schema = Schema::new("./files/day3.txt");
    println!("Solution part 1: {}", solve1(&schema));
}

fn solve1(schema: &Schema) -> i64 {
    let mut result = 0;
    let dims = schema.dims();

    for x in 0..dims.x {
        let mut is_relevant = false;
        let mut number = 0;
        for y in 0..dims.y {
            let field = schema.get_field(x, y);
            match field {
                Field::Number(num) => {
                    number = 10 * number + num;
                    if !is_relevant {
                        is_relevant = schema.neighbour_is_symbol(x, y);
                    }
                }
                Field::Symbol(_) | Field::Empty => {
                    if is_relevant {
                        result += number;
                        is_relevant = false;
                    }
                    number = 0;
                },
            }
            if y == dims.y - 1 {
                if is_relevant {
                    result += number;
                    is_relevant = false;
                }
                number = 0;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = Schema::new("./files/day3.txt");
        let result = solve1(&input);
        assert_eq!(result, 531561);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = Schema::new("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 4361);
    }
}
