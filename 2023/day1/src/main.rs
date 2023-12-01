use util::read_files::read_file_as_vector;

fn main() {
    let calibrations = read_file_as_vector("./files/day1.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&calibrations));
}

fn solve1(calibrations: &Vec<String>) -> i64 {
    let mut numbers = Vec::new();
    for s in calibrations {
        numbers.push(get_calibration_number(s));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = read_file_as_vector("./files/day1.txt").expect("Error reading file.");
        let result = solve1(&input);
        assert_eq!(result, 55090);
    }
}

