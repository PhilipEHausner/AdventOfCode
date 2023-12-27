use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/test.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(filename: &str) -> Input {
    let lines = read_file_as_vector(filename).expect("Cannot read file.");
    1
}

fn solve1(input: &Input) -> usize {
    1
}

fn solve2(input: &Input) -> usize {
    1
}

type Input = i64;