use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day25.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
}

fn solve1(lines: &Vec<String>) -> String {
    let fuel_per_balloon = parse_snafu_numbers(lines);
    let needed_fuel = fuel_per_balloon.iter().sum();
    decimal_to_snafu(needed_fuel)
}

fn parse_snafu_numbers(lines: &Vec<String>) -> Vec<i64> {
    lines.iter().map(|line| parse_snafu_number(line)).collect()
}

fn parse_snafu_number(line: &String) -> i64 {
    let mut result = 0;
    let mut base_value = 1;

    for digit in line.chars().rev() {
        result += calculate_snafu_digit_value(&digit, base_value);
        base_value *= 5;
    }

    result
}

fn calculate_snafu_digit_value(digit: &char, base_value: i64) -> i64 {
    match digit {
        '0' => 0,
        '1' => base_value,
        '2' => 2 * base_value,
        '-' => -base_value,
        '=' => -2 * base_value,
        _ => panic!("Unknown SNAFU digit {}.", digit),
    }
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut number = decimal as f64;
    let mut result = "".to_string();
    let num_digits = ((2.0 * number).log10() / (5 as f64).log10()).ceil() as u64;

    for i in (0..num_digits).rev() {
        let digit = (number / (5.0 as f64).powi(i as i32)).round();
        number -= digit * (5.0 as f64).powi(i as i32);
        result += &digit_to_snafu_symbol(digit as i32);
    }

    result.to_string()
}

fn digit_to_snafu_symbol(digit: i32) -> String {
    match digit {
        -2 => "=".to_string(),
        -1 => "-".to_string(),
        0 => "0".to_string(),
        1 => "1".to_string(),
        2 => "2".to_string(),
        _ => panic!("Invalid digit to convert to SNAFU symbol: '{}'.", digit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), "2=-1=0".to_string());
    }

    #[test]
    fn test_solve1_large() {
        let lines = read_file_as_vector("./files/day25.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), "2----0=--1122=0=0021".to_string());
    }
}
