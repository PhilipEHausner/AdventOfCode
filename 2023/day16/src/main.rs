fn main() {
    let input = get_input("./files/day13.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Input {
    vec![]
}

fn solve1(input: &Input,) -> usize {
    1
}

fn solve2(input: &Input) -> usize {
    1
}

enum Space {
    Empty,
    Mirror,
    Block,
}

type Input = Vec<Vec<Space>>;
