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
    println!("Solution part 2:");
    solve2(&lines);
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

fn solve2(instructions: &Vec<String>) {
    let mut cycle: usize = 0;
    let mut sprite_position: i64 = 1;
    let mut commands = vec![];
    let mut screen = Vec::<&str>::new();

    for instruction in instructions {
        commands.append(&mut process_instruction(instruction));
    }

    for command in commands {
        let does_overlap = pixel_overlaps_with_sprite(cycle, sprite_position);
        screen.push(if does_overlap { "#" } else { "." });
        sprite_position += command.add;
        cycle += 1;
    }

    print_screen(&screen);
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

fn pixel_overlaps_with_sprite(pixel_pos: usize, sprite_pos: i64) -> bool {
    sprite_pos < 0 || ( pixel_pos % 40 >= sprite_pos as usize - 1 && pixel_pos % 40 <= sprite_pos as usize + 1 )
}

fn print_screen(screen: &Vec<&str>) {
    for i in 0..6 {
        let start_pos = 40 * i;
        let end_pos = 40 * i + 40;
        println!("{}", screen[start_pos..end_pos].join(""));
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
