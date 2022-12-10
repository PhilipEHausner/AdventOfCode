use util::read_files::read_file_as_vector;

#[derive(Debug)]
struct Command {
    add: i64,
}

impl Command {
    pub fn new(add: i64) -> Command {
        Command { add }
    }
}

fn main() {
    let lines = read_file_as_vector("./files/day10.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
}

fn solve1(instructions: &Vec<String>) -> i64 {
    let mut cycles: i64 = 1;
    let mut register: i64 = 1;
    let mut result: i64 = 0;
    let mut commands = vec![];

    for instruction in instructions {
        commands.append(&mut process_instruction(instruction));
    }

    for command in commands {
        cycles += 1;
        register += command.add;
        if (cycles - 20) % 40 == 0 {
            result += cycles * register;
        }
    }

    result
}

fn process_instruction(instruction: &String) -> Vec<Command> {
    let command = &instruction[..4];

    match command {
        "noop" => {
            vec![Command::new(0)]
        },
        "addx" => {
            let add = instruction[5..].parse::<i64>().expect("Error parsing number.");
            vec![Command::new(0), Command::new(add)]
        },
        _ => panic!("Invalid command."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 13140);
    }
}
