use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day24.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    // println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 18);
    }

    // #[test]
    // fn test_solve2() {
    //     let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
    //     assert_eq!(solve2(&lines), 20);
    // }
}
