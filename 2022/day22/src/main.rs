use util::read_files::read_file_as_vector;

mod solve1;

fn main() {
    let lines = read_file_as_vector("./files/day22.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1::solve(&lines));
}
