use util::read_files::read_file_as_vector;

fn main() {
    let input = get_input("./files/day14.txt");
    println!("Solution part 1: {}", solve1(&input));
    println!("Solution part 2: {}", solve2(&input));
}

fn get_input(file: &str) -> Input {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    lines
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| match c {
                    'O' => Space::Round,
                    '#' => Space::Block,
                    '.' => Space::Empty,
                    _ => panic!("Unknown input character '{c}'."),
                })
                .collect()
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    let mut transposed = transpose(input);
    roll_stones_north(&mut transposed);
    get_load(&transposed)
}

fn roll_stones_north(input: &mut Input) {
    for col in 0..input.len() {
        let mut top_most = 0;
        for row in 0..input[col].len() {
            match input[col][row] {
                Space::Round => {
                    input[col][row] = Space::Empty;
                    input[col][top_most] = Space::Round;
                    top_most += 1;
                }
                Space::Block => top_most = row + 1,
                Space::Empty => (),
            }
        }
    }
}

fn get_load(input: &Input) -> usize {
    let height = input.len();
    input
        .iter()
        .map(|col| {
            col.iter()
                .enumerate()
                .map(|(i, s)| match s {
                    Space::Round => height - i,
                    Space::Block => 0,
                    Space::Empty => 0,
                })
                .into_iter()
                .sum::<usize>()
        })
        .into_iter()
        .sum()
}

fn solve2(input: &Input) -> usize {
    1
}

fn transpose<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Space {
    Round,
    Block,
    Empty,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::Round => "O",
                Space::Block => "#",
                Space::Empty => ".",
            }
        )
    }
}

type Input = Vec<Vec<Space>>;
