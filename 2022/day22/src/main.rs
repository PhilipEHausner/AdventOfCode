use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day22.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
}

fn solve1(lines: &Vec<String>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 6032);
    }
}
