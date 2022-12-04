fn main() {
    println!("Hello, world!");
}

fn solve1(filename: &str) -> u64 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("./files/test.txt"), 2);
    }

}