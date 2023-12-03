use crate::schema::Field;
use crate::schema::Schema;

mod schema;

fn main() {
    let schema = Schema::new("./files/day3.txt");
    println!("Solution part 1: {}", solve1(&schema));
    println!("Solution part 2: {}", solve2(&schema));
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

fn solve2(schema: &Schema) -> i64 {
    let mut result = 0;
    for x in 0..schema.dims().x {
        for y in 0..schema.dims().y {
            if schema.get_field(x, y).is_gear() {
                let numbers = schema.numbers_around_field(x, y);
                match numbers.len() {
                    ..=1 => continue,
                    2 => result += numbers.get(0).unwrap() * numbers.get(1).unwrap(),
                    _ => panic!("More than 2 numbers around gear. I did not expect that."),
                }
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

    #[test]
    fn test_solve2() {
        let input = Schema::new("./files/day3.txt");
        let result = solve2(&input);
        assert_eq!(result, 83279367);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = Schema::new("./files/test.txt");
        let result = solve2(&input);
        assert_eq!(result, 467835);
    }
}
