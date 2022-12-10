use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day10.txt").expect("Error reading file.");
}

fn solve1(lines: &Vec<String>) -> u64 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1_small() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 13140);
    }
}
