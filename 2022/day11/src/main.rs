mod monkey;

use monkey::Monkey;
use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day11.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
    println!("Solution part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    solve_generic(lines, 20, 3)
}

fn solve2(lines: &Vec<String>) -> u64 {
    solve_generic(lines, 10000, 1)
}

fn solve_generic(lines: &Vec<String>, rounds: usize, worry_decay: u64) -> u64 {
    let mut monkeys = parse_monkeys(lines, worry_decay);

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

    set_modulo_ring(&mut monkeys);
    monkeys
}

fn set_modulo_ring(monkeys: &mut Vec<Monkey>) {
    let modulo_ring = monkeys.iter().map(|monkey| monkey.get_decision_test()).product();
    for monkey in monkeys {
        monkey.set_modulo_ring(modulo_ring);
    }
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
        assert_eq!(solve1(&lines), 10605);
    }

    #[test]
    fn test_solve1_real() {
        let lines = read_file_as_vector("./files/day11.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 113232);
    }

    #[test]
    fn test_solve2() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 2713310158);
    }

    #[test]
    fn test_solve2_real() {
        let lines = read_file_as_vector("./files/day11.txt").expect("Error reading file.");
        assert_eq!(solve2(&lines), 29703395016);
    }
}
