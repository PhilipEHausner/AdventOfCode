use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day25.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    // println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), "2=-1=0".to_string());
    }

    // #[test]
    // fn test_solve1_large() {
    //     let lines = read_file_as_vector("./files/day25.txt").expect("Error reading file.");
    //     assert_eq!(solve1(&lines), 245);
    // }
}

