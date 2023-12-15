use core::panic;
use std::{cell::RefCell, rc::Rc};

use util::read_files::read_file_as_vector;

use std::time::Instant;

fn main() {
    let input = get_input("./files/test.txt");
    let now1 = Instant::now();
    println!("Solution part 1: {}", solve1(&input));
    let elapsed1 = now1.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
    let now2 = Instant::now();
    println!("Solution part 2: {}", solve2(&input));
    let elapsed2 = now2.elapsed();
    println!("Elapsed: {:.2?}", elapsed2);
}

fn get_input(file: &str) -> Input {
    let lines = read_file_as_vector(file).expect("Could not read file.");
    lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                panic!("Malformed input.");
            }
            let springs: Vec<Spring> = parts[0]
                .chars()
                .into_iter()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => panic!("Malformed input. unknown spring symbol."),
                })
                .collect();
            let control: Vec<usize> = parts[1].split(",").map(|el| el.parse().unwrap()).collect();
            (springs, control)
        })
        .collect()
}

fn solve1(input: &Input) -> usize {
    solve(input, 1)
}

fn solve2(input: &Input) -> usize {
    solve(input, 5)
}

fn solve(input: &Input, multiplier: usize) -> usize {
    let mut total = 0;
    for (springs, record) in input.iter() {
        let mut state_machine = StateMachine::new(record);
        let temp = state_machine.run(springs);
        println!("{}", temp);
        total += temp;
    }
    total
}

fn multiply_vec<T: Clone>(input: &Vec<T>, multiplier: usize) -> Vec<T> {
    input
        .iter()
        .cycle()
        .take(input.len() * multiplier)
        .map(|el| el.clone())
        .collect()
}

fn multiply_springs(springs: &Vec<Spring>, multiplier: usize) -> Vec<Spring> {
    let mut result = springs.clone();
    for _ in 1..multiplier {
        result.push(Spring::Unknown);
        result.append(&mut springs.clone());
    }
    result
}

struct StateMachine {
    states: Vec<Rc<RefCell<State>>>,
    lost_state: Rc<RefCell<State>>,
}

impl StateMachine {
    pub fn new(record: &Vec<usize>) -> StateMachine {
        let mut states = vec![];
        let lost_state = StateMachine::wrap_state(State::Dummy);

        let mut last: Rc<RefCell<State>> = StateMachine::wrap_state(State::Dummy);
        for entry in record.iter().rev() {
            last =
                StateMachine::wrap_state(State::new(last.clone(), true, lost_state.clone(), false));
            states.push(last.clone());
            for i in 0..*entry {
                let is_self_loop = if i == entry - 1 { true } else { false };
                last = StateMachine::wrap_state(State::new(
                    lost_state.clone(),
                    is_self_loop,
                    last.clone(),
                    false,
                ));
                states.push(last.clone());
            }
        }
        states.reverse();
        {
            RefCell::borrow_mut(&states[0]).accept(1);
        }

        for s in states.iter() {
            println!("S {}", RefCell::borrow_mut(&states[0]).get_count())
        }

        StateMachine { states, lost_state }
    }

    fn wrap_state(state: State) -> Rc<RefCell<State>> {
        Rc::new(RefCell::new(state))
    }

    fn run(&mut self, springs: &Vec<Spring>) -> usize {
        for spring in springs {
            for state in self.states.iter_mut().rev() {
                RefCell::borrow_mut(state).step(spring);
            }
        }

        RefCell::borrow_mut(&self.states.last().unwrap()).get_count()
    }
}

struct _State {
    operational_transition: Rc<RefCell<State>>,
    operational_self: bool,
    damage_transition: Rc<RefCell<State>>,
    damage_self: bool,
    count: usize,
}

enum State {
    Node(_State),
    Dummy,
}

impl State {
    pub fn new(
        operational_transition: Rc<RefCell<State>>,
        operational_self: bool,
        damage_transition: Rc<RefCell<State>>,
        damage_self: bool,
    ) -> State {
        State::Node(_State {
            operational_transition,
            operational_self,
            damage_transition,
            damage_self,
            count: 0,
        })
    }

    fn accept(&mut self, count: usize) {
        match self {
            State::Node(s) => s.count += count,
            State::Dummy => (),
        }
    }

    fn get_count(&self) -> usize {
        match self {
            State::Node(s) => s.count,
            State::Dummy => panic!(),
        }
    }

    pub fn step(&mut self, spring: &Spring) {
        match self {
            State::Node(s) => State::_step(s, spring),
            State::Dummy => (),
        }
    }

    fn _step(state: &mut _State, spring: &Spring) {
        let count = state.count;
        state.count = 0;
        match spring {
            Spring::Operational => State::step_operational(state, count),
            Spring::Damaged => State::step_damage(state, count),
            Spring::Unknown => {
                State::step_operational(state, count);
                State::step_damage(state, count);
            }
        }
    }

    fn step_operational(state: &mut _State, count: usize) {
        if state.operational_self {
            state.count = count;
        } else {
            RefCell::borrow_mut(&state.operational_transition).accept(count);
        }
    }

    fn step_damage(state: &mut _State, count: usize) {
        if state.damage_self {
            state.count = count;
        } else {
            RefCell::borrow_mut(&state.damage_transition).accept(count);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

type Input = Vec<(Vec<Spring>, Vec<usize>)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = get_input("./files/day12.txt");
        let result = solve1(&input);
        assert_eq!(result, 7163);
    }

    #[test]
    fn test_solve1_testdata1() {
        let input = get_input("./files/test.txt");
        let result = solve1(&input);
        assert_eq!(result, 21);
    }
}
