use regex::Regex;
use util::read_files::read_file_as_vector;

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let games = read_file_as_vector("./files/day2.txt").expect("Could not read file.");
    println!("Solution part 1: {}", solve1(&games));
    println!("Solution part 2: {}", solve2(&games));
}

fn solve1(games: &Vec<String>) -> i32 {
    let mut result = 0;
    for game in games {
        if get_max_number_cubes(game, Color::Blue) > 14
            || get_max_number_cubes(game, Color::Green) > 13
            || get_max_number_cubes(game, Color::Red) > 12
        {
            continue;
        }
        result += get_game_number(game);
    }
    return result;
}

fn solve2(games: &Vec<String>) -> i64 {
    let mut result = 0;
    for game in games {
        result += get_max_number_cubes(game, Color::Blue)
            * get_max_number_cubes(game, Color::Green)
            * get_max_number_cubes(game, Color::Red);
    }
    return result;
}

fn get_game_number(game: &str) -> i32 {
    let game_re = Regex::new(r"Game (\d+):").unwrap();
    let captures = game_re.captures(game).unwrap();
    return captures.get(1).unwrap().as_str().parse().unwrap();
}

fn get_max_number_cubes(game: &str, color: Color) -> i64 {
    return match color {
        Color::Red => _get_max_number_cubes(game, Regex::new(r"(\d+) red").unwrap()),
        Color::Green => _get_max_number_cubes(game, Regex::new(r"(\d+) green").unwrap()),
        Color::Blue => _get_max_number_cubes(game, Regex::new(r"(\d+) blue").unwrap()),
    };
}

fn _get_max_number_cubes(game: &str, color_re: regex::Regex) -> i64 {
    return color_re
        .captures_iter(game)
        .map(|it| it.get(1).unwrap().as_str().parse::<i64>().unwrap())
        .max()
        .unwrap_or(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = read_file_as_vector("./files/day2.txt").expect("Error reading file.");
        let result = solve1(&input);
        assert_eq!(result, 2685);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        let result = solve1(&input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_solve2() {
        let input = read_file_as_vector("./files/day2.txt").expect("Error reading file.");
        let result = solve2(&input);
        assert_eq!(result, 83707);
    }


    #[test]
    fn test_solve2_testdata() {
        let input = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        let result = solve2(&input);
        assert_eq!(result, 2286);
    }

    #[test]
    fn test_get_game_number() {
        assert_eq!(get_game_number("Game 2:"), 2, "Only prefix");
        assert_eq!(get_game_number("whatever Game 3:"), 3, "With text before");
        assert_eq!(get_game_number("Game 6: 4 green"), 6, "With text after");
        assert_eq!(get_game_number("Game 22:"), 22, "Double Digit");
    }

    #[test]
    fn test_get_max_number_cubes() {
        assert_eq!(
            get_max_number_cubes(
                "Game 6: 3 green, 7 blue, 5 red; 3 green, 6 red; 11 blue, 6 red, 1 green",
                Color::Green
            ),
            3
        );
        assert_eq!(
            get_max_number_cubes(
                "Game 6: 3 green, 7 blue, 5 red; 3 green, 6 red; 11 blue, 6 red, 1 green",
                Color::Blue
            ),
            11
        );
        assert_eq!(
            get_max_number_cubes(
                "Game 6: 3 green, 7 blue, 5 red; 3 green, 6 red; 11 blue, 6 red, 1 green",
                Color::Red
            ),
            6
        );
    }
}
