mod monkey;

use monkey::Monkey;
use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day11.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines, 20));
}

fn solve1(lines: &Vec<String>, rounds: usize) -> u64 {
    let mut monkeys = parse_monkeys(lines, 3);

    for _ in 0..rounds {
        monkey_round(&mut monkeys);
    }
    
    let mut inspections = monkeys.iter().map(|monkey| monkey.num_inspected_items()).collect::<Vec<u64>>();
    inspections.sort_by_key(|&w| std::cmp::Reverse(w));
    inspections[0] * inspections[1]
}

fn parse_monkeys(lines: &Vec<String>, worry_decay: u64) -> Vec<Monkey> {
    let mut monkeys = vec![];
    for i in (0..lines.len()).step_by(7) {
        let number = &lines[i + 0];
        let items = &lines[i + 1];
        let operation = &lines[i + 2];
        let decision_test = &lines[i + 3];
        let decision_true = &lines[i + 4];
        let decision_false = &lines[i + 5];
        let monkey = Monkey::new_from_strings(
            number,
            items,
            operation,
            decision_test,
            decision_true,
            decision_false,
            worry_decay,
        );
        monkeys.push(monkey);
    }
    monkeys
}

fn monkey_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        while monkeys[i].has_next_item() {
            let monkey_throw = monkeys[i].process_next_item();
            monkeys[monkey_throw.target_monkey].push_item(monkey_throw.item_worry_level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines, 20), 10605);
    }
}
