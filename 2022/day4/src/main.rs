use std::cmp;

use util::read_files::read_file_as_vector;

struct AssignmentRange {
    pub start: u64,
    pub end: u64,
}

struct ElvePair {
    assignment_elve1: AssignmentRange,
    assignment_elve2: AssignmentRange,
}

impl ElvePair {
    pub fn do_fully_overlap(&self) -> bool {
        let (start1, end1) = (self.assignment_elve1.start, self.assignment_elve1.end);
        let (start2, end2) = (self.assignment_elve2.start, self.assignment_elve2.end);

        if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
            true
        } else {
            false
        }
    }

    pub fn overlapping_fields(&self) -> u64 {
        let (start1, end1) = (self.assignment_elve1.start as i64, self.assignment_elve1.end as i64);
        let (start2, end2) = (self.assignment_elve2.start as i64, self.assignment_elve2.end as i64);

        cmp::max(cmp::min(end1, end2) - cmp::max(start1, start2) + 1, 0) as u64
    }
}

fn main() {
    let solution1 = solve1("./files/day4.txt");
    println!("Solution problem1: {}", solution1);
    let solution2 = solve2("./files/day4.txt");
    println!("Solution problem2: {}", solution2);
}

fn solve1(filename: &str) -> u64 {
    let lines = read_file_as_vector(filename).expect("Error reading file.");
    let elve_pairs = get_elve_pairs(&lines);
    elve_pairs
        .iter()
        .map(|pair| if pair.do_fully_overlap() { 1 } else { 0 })
        .sum()
}

fn solve2(filename: &str) -> u64 {
    let lines = read_file_as_vector(filename).expect("Error reading file.");
    let elve_pairs = get_elve_pairs(&lines);
    elve_pairs
        .iter()
        .map(|pair| if pair.overlapping_fields() > 0 { 1 } else { 0 })
        .sum()
}

fn get_elve_pairs(lines: &Vec<String>) -> Vec<ElvePair> {
    let ranges: Vec<Vec<&str>> = lines.iter().map(|line| line.split(",").collect()).collect();
    ranges
        .iter()
        .map(|split| ElvePair {
            assignment_elve1: parse_assignment_range(split[0]),
            assignment_elve2: parse_assignment_range(split[1]),
        })
        .collect()
}

fn parse_assignment_range(range: &str) -> AssignmentRange {
    let bounds: Vec<u64> = range
        .split("-")
        .map(|r| r.parse().expect("Bound of range invalid."))
        .collect();
    debug_assert_eq!(bounds.len(), 2);
    AssignmentRange {
        start: bounds[0],
        end: bounds[1],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("./files/test.txt"), 2);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2("./files/test.txt"), 4);
    }

    #[test]
    fn test_elve_pairs_do_fully_overlap() {
        test_elve_pairs_do_fully_overlap_helper(1, 4, 2, 3, true);
        test_elve_pairs_do_fully_overlap_helper(2, 3, 1, 4, true);
        test_elve_pairs_do_fully_overlap_helper(1, 4, 2, 5, false);
        test_elve_pairs_do_fully_overlap_helper(2, 5, 1, 4, false);
        test_elve_pairs_do_fully_overlap_helper(4, 4, 2, 5, true);
        test_elve_pairs_do_fully_overlap_helper(2, 4, 1, 3, false);
        test_elve_pairs_do_fully_overlap_helper(1, 3, 4, 5, false);
        test_elve_pairs_do_fully_overlap_helper(4, 5, 1, 3, false);
        test_elve_pairs_do_fully_overlap_helper(4, 4, 2, 5, true);
    }

    fn test_elve_pairs_do_fully_overlap_helper(
        start1: u64,
        end1: u64,
        start2: u64,
        end2: u64,
        outcome: bool,
    ) {
        let pair = ElvePair {
            assignment_elve1: AssignmentRange {
                start: start1,
                end: end1,
            },
            assignment_elve2: AssignmentRange {
                start: start2,
                end: end2,
            },
        };
        assert_eq!(
            pair.do_fully_overlap(),
            outcome,
            "Ranges are: {} - {} and {} - {}, expected outcome: {}",
            start1,
            end1,
            start2,
            end2,
            outcome
        );
    }

    #[test]
    fn test_elve_pairs_overlapping_fields() {
        test_elve_pairs_overlapping_fields_helper(2, 5, 3, 4, 2);
        test_elve_pairs_overlapping_fields_helper(2, 4, 3, 5, 2);
        test_elve_pairs_overlapping_fields_helper(2, 4, 5, 7, 0);
        test_elve_pairs_overlapping_fields_helper(2, 4, 2, 4, 3);
    }

    fn test_elve_pairs_overlapping_fields_helper(
        start1: u64,
        end1: u64,
        start2: u64,
        end2: u64,
        outcome: u64,
    ) {
        let pair = ElvePair {
            assignment_elve1: AssignmentRange {
                start: start1,
                end: end1,
            },
            assignment_elve2: AssignmentRange {
                start: start2,
                end: end2,
            },
        };
        assert_eq!(
            pair.overlapping_fields(),
            outcome,
            "Ranges are: {} - {} and {} - {}, expected outcome: {}",
            start1,
            end1,
            start2,
            end2,
            outcome
        );
    }
}
