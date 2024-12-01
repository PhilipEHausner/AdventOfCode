use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file_as_vector<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    lines.collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let lines: Vec<String> = read_lines("./testfiles/readFileTest.txt")
            .expect("File could not be read.")
            .map(|line| line.expect("Could not parse line."))
            .collect();
        let results = vec!["abc".to_string(), "123".to_string(), "".to_string(), "abg12".to_string(), "$3".to_string()];

        assert_eq!(lines.len(), results.len());
        assert_eq!(vecs_equal(&lines, &results), true);
    }

    #[test]
    fn test_read_file_as_vector() {
        let lines: Vec<String> = read_file_as_vector("./testfiles/readFileTest.txt").expect("File could not be read.");
        let results = vec!["abc".to_string(), "123".to_string(), "".to_string(), "abg12".to_string(), "$3".to_string()];

        assert_eq!(lines.len(), results.len());
        assert_eq!(vecs_equal(&lines, &results), true);
    }

    fn vecs_equal<T: std::fmt::Display + std::cmp::PartialEq>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {
        let it = vec1.iter().zip(vec2.iter());

        for (_, (x, y)) in it.enumerate() {
            if *x != *y { return false; }
        }

        true
    }
}
