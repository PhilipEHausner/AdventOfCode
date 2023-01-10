use util::read_files::read_file_as_vector;

mod solve1;
mod solve2;

fn main() {
    let lines = read_file_as_vector("./files/day22.txt").expect("Error reading file.");
    let meta_information =
        read_file_as_vector("./files/day22_connections.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1::solve(&lines));
    println!("Solution part 2: {}", solve2::solve(&lines, &meta_information));
}
