fn main() {
    println!("Hello, world!");
}

fn solve1(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!("bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
        assert_eq!("nppdvjthqldpwncqszvftbrmjlhg", 6);
        assert_eq!("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
        assert_eq!("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);
    }
}