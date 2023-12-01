use util::read_files::read_file_as_vector;

fn main() {
    let calibrations = read_file_as_vector("./files/day1.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&calibrations));
    println!("Solution part 2: {}", solve2(&calibrations));
}

fn solve1(calibrations: &Vec<String>) -> i64 {
    let mut numbers = Vec::new();
    for s in calibrations {
        replace_number_words_by_literals(s);
        numbers.push(get_calibration_number(s));
    }
    return numbers.iter().sum();
}

fn solve2(calibrations: &Vec<String>) -> i64 {
    let mut numbers = Vec::new();
    for s in calibrations {
        let input = replace_number_words_by_literals(s);
        numbers.push(get_calibration_number(&input));
    }
    return numbers.iter().sum();
}

fn get_calibration_number(calibration: &String) -> i64 {
    let first_number = get_calibration_number_part(calibration.chars()).expect(&format!(
        "Error computing calibration number part '{}'.",
        calibration
    ));
    let last_number = get_calibration_number_part(calibration.chars().rev()).expect(&format!(
        "Error computing calibration number part '{}'.",
        calibration
    ));
    return first_number * 10 + last_number;
}

fn get_calibration_number_part<I>(chars: I) -> Result<i64, ()>
where
    I: IntoIterator<Item = char>,
{
    for c in chars {
        match c {
            '0' => return Ok(0),
            '1' => return Ok(1),
            '2' => return Ok(2),
            '3' => return Ok(3),
            '4' => return Ok(4),
            '5' => return Ok(5),
            '6' => return Ok(6),
            '7' => return Ok(7),
            '8' => return Ok(8),
            '9' => return Ok(9),
            _ => continue
        }
    }
    return Err(());
}

fn replace_number_words_by_literals(s: &str) -> String {
    let mut result: String = "".to_string();
    for i in 0..s.len() {
        result.push(s.chars().nth(i).unwrap());
        let last_three = s.chars().skip(i).take(3).collect::<String>();
        let last_four = s.chars().skip(i).take(4).collect::<String>();
        let last_five = s.chars().skip(i).take(5).collect::<String>();

        match last_three.as_str() {
            "one" => result.push('1'),
            "two" => result.push('2'),
            "six" => result.push('6'),
            _ => {}
        }

        match last_four.as_str() {
            "four" => result.push('4'),
            "five" => result.push('5'),
            "nine" => result.push('9'),
            _ => {}
        }

        match last_five.as_str() {
            "three" => result.push('3'),
            "seven" => result.push('7'),
            "eight" => result.push('8'),
            _ => {}
        }
    }
    return result;
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = read_file_as_vector("./files/day1.txt").expect("Error reading file.");
        let result = solve1(&input);
        assert_eq!(result, 55090);
    }

    #[test]
    fn test_solve2() {
        let input = read_file_as_vector("./files/day1.txt").expect("Error reading file.");
        let result = solve2(&input);
        assert_eq!(result, 54845);
    }
}

