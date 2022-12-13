use util::read_files::read_file_as_vector;

#[derive(Debug)]
enum List {
    Value(u32),
    ListElements(Vec<Box<List>>),
}

#[derive(Debug, PartialEq, Eq)]
enum Bool {
    True,
    False,
    Neutral,    
}

fn in_right_order(list1: &List, list2: &List) -> Bool {
    match (list1, list2) {
        (List::Value(x), List::Value(y)) => {
            if x > y {
                return Bool::False;
            } else if x == y {
                return Bool::Neutral;
            } else {
                return Bool::True;
            }
        },
        (List::ListElements(x_vec), List::ListElements(y_vec)) => {
            let max_index = x_vec.len().max(y_vec.len());

            for i in 0..max_index {
                if i >= x_vec.len() {
                    return Bool::True;
                } else if i >= y_vec.len() {
                    return Bool::False;
                }
                let l1 = &x_vec[i];
                let l2 = &y_vec[i];

                let is_right = in_right_order(&l1, &l2);
                match is_right {
                    Bool::Neutral => {},
                    _ => return is_right,
                }
            }
            return Bool::Neutral;
        },
        (List::Value(x), List::ListElements(_)) => {
            let l1 = List::ListElements(vec![Box::new(List::Value(*x))]);
            return in_right_order(&l1, list2);
        },
        (List::ListElements(_), List::Value(y)) => {
            let l2 = List::ListElements(vec![Box::new(List::Value(*y))]);
            return in_right_order(list1, &l2);
        },
    }
}

fn main() {
    let lines = read_file_as_vector("./files/day13.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
}

fn solve1(lines: &Vec<String>)-> u64 {
    let input = parse_input(lines);
    let mut result = 0;

    for (i, (l1, l2)) in input.iter().enumerate() {
        let is_right = in_right_order(&l1, &l2);
        match is_right {
            Bool::False => {},
            _ => {result += i + 1}
        }
    }

    result as u64
}

fn parse_input(lines: &Vec<String>) -> Vec<(List, List)> {
    let mut lists = vec![];

    for i in (0..lines.len()).step_by(3) {
        let list1 = parse_line(&lines[i][1..lines[i].len()-1]);
        let list2 = parse_line(&lines[i+1][1..lines[i+1].len()-1]);
        lists.push((list1, list2));
    }

    lists
}

fn parse_line(line: &str) -> List {
    let mut result = vec![];
    let mut bracket_count = 0;
    let mut comma_skip = false;

    for i in 0..line.len() {
        let curr_char = line.chars().nth(i).unwrap();
        if bracket_count > 0 && curr_char == '[' {
            bracket_count += 1;
            continue;
        }
        if curr_char == ']' {
            bracket_count -= 1;
            continue;
        }
        if curr_char == ',' {
            comma_skip = false;
            continue;
        }
        if bracket_count > 0 || comma_skip {
            continue;
        }

        if curr_char == '[' {
            let end_brackets = matching_end_bracket(line, i);
            let sublist = parse_line(&line[i+1..end_brackets]);
            result.push(Box::new(sublist));
            bracket_count += 1;
        } else {
            let end_comma = line[i..].find(|c| c == ',' || c == ']');
            let number = match end_comma {
                Some(x) => line[i..i+x].parse().unwrap(),
                None => line[i..].parse().unwrap(),
            };
            result.push(Box::new(List::Value(number)));
            comma_skip = true;
        }

    }

    List::ListElements(result)
}

fn matching_end_bracket(line: &str, bracket_index: usize) -> usize {
    let result = 0;
    let mut bracket_count = 0;
    for i in bracket_index+1..line.len() {
        let curr_char = line.chars().nth(i).unwrap();
        if curr_char == '[' {
            bracket_count += 1;
        } else if curr_char == ']' {
            if bracket_count == 0 {
                return i;
            } else {
                bracket_count -= 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 13);
    }
}
