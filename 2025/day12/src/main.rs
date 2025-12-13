use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day12.txt");
    println!("Solution part 1: {}", solve1(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");

    let first_idx = lines
        .iter()
        .enumerate()
        .filter(|(_, el)| el.contains("x"))
        .next()
        .unwrap()
        .0;

    lines
        .iter()
        .skip(first_idx)
        .map(|line| {
            let size_string = line.split_once(':').unwrap().0.split_once('x').unwrap();
            let size = (
                size_string.0.parse().unwrap(),
                size_string.1.parse().unwrap(),
            );
            let shapes = line
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split(' ')
                .map(|el| el.parse::<usize>().unwrap())
                .collect();
            Problem { size, shapes }
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    input
        .iter()
        .filter(|problem| region_can_fit_shapes(&problem))
        .collect::<Vec<&Problem>>()
        .len()
}

fn region_can_fit_shapes(problem: &Problem) -> bool {
    let region_size = problem.size.0 * problem.size.1;
    let shape_total_size = problem.shapes.iter().sum::<usize>() * 9;
    let shape_size = problem.shapes[0] * 6
        + problem.shapes[1] * 7
        + problem.shapes[2] * 5
        + problem.shapes[3] * 7
        + problem.shapes[4] * 7
        + problem.shapes[5] * 7;
    if shape_total_size <= region_size {
        true
    } else if shape_size > region_size {
        false
    } else {
        panic!("Non-trivial.")
    }
}

#[derive(Debug)]
struct Problem {
    size: (usize, usize),
    shapes: Vec<usize>,
}

type Input = Vec<Problem>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day12.txt");
        let result = solve1(&input);
        // assert_eq!(result, );
    }
}
