#[derive(Debug)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

#[derive(Debug)]
pub struct MonkeyThrow {
    pub target_monkey: usize,
    pub item_worry_level: u64,
}

#[derive(Debug)]
pub struct Monkey {
    _number: u64,
    pub items: Vec<u64>,
    operation: Operation,
    decision_test: u64,
    decision_true: usize,
    decision_false: usize,
    worry_decay: u64,
    inspected_items: u64,
    modulo_ring: u64,
}

impl Monkey {
    pub fn has_next_item(&self) -> bool {
        self.items.len() > 0
    }

    pub fn process_next_item(&mut self) -> MonkeyThrow {
        self.inspected_items += 1;

        let next_item = self.items.pop().unwrap();
        let item_worry_level = self.updated_item_worry_level(next_item);
        let target_monkey = if item_worry_level % self.decision_test == 0 {
            self.decision_true
        } else {
            self.decision_false
        };

        MonkeyThrow {
            target_monkey,
            item_worry_level,
        }
    }

    fn updated_item_worry_level(&self, item_worry_level: u64) -> u64 {
        let mut worry_level = item_worry_level;
        match self.operation {
            Operation::Add(x) => {
                worry_level += x;
            }
            Operation::Mult(x) => {
                worry_level *= x;
            }
            Operation::Square => {
                worry_level *= worry_level;
            }
        }
        worry_level /= self.worry_decay;
        worry_level % self.modulo_ring
    }

    pub fn push_item(&mut self, item_worry_level: u64) {
        self.items.push(item_worry_level);
    }

    pub fn num_inspected_items(&self) -> u64 {
        self.inspected_items
    }

    pub fn get_decision_test(&self) -> u64{
        self.decision_test
    }

    pub fn set_modulo_ring(&mut self, modulo_ring: u64) {
        self.modulo_ring = modulo_ring;
    }
}

// PARSER
impl Monkey {
    pub fn new_from_strings(
        number: &String,
        items: &String,
        operation: &String,
        decision_test: &String,
        decision_true: &String,
        decision_false: &String,
        worry_decay: u64,
    ) -> Monkey {
        let m_number = Monkey::parse_number(number);
        let m_items = Monkey::parse_items(items);
        let m_operation = Monkey::parse_operation(operation);
        let m_decision_test = Monkey::parse_decision_test(decision_test);
        let m_decision_true = Monkey::parse_decision_true(decision_true);
        let m_decision_false = Monkey::parse_decision_false(decision_false);

        Monkey {
            _number: m_number,
            items: m_items,
            operation: m_operation,
            decision_test: m_decision_test,
            decision_true: m_decision_true,
            decision_false: m_decision_false,
            inspected_items: 0,
            worry_decay,
            modulo_ring: 0,
        }
    }

    fn parse_number(number: &String) -> u64 {
        number.split(" ").collect::<Vec<&str>>()[1]
            .replace(":", "")
            .parse::<u64>()
            .expect("Error parsing number.")
    }

    fn parse_items(items: &String) -> Vec<u64> {
        items
            .trim()
            .replace(",", "")
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .skip(2)
            .map(|el| el.parse::<u64>().expect("Error parsing item number."))
            .collect::<Vec<u64>>()
    }

    fn parse_operation(operation: &String) -> Operation {
        let op = operation.trim().split(" ").skip(4).collect::<Vec<&str>>();

        match op[..2] {
            ["*", "old"] => Operation::Square,
            ["*", _] => Operation::Mult(op[1].parse().unwrap()),
            ["+", _] => Operation::Add(op[1].parse().unwrap()),
            _ => panic!("Invalid operation: {:?}", op),
        }
    }

    fn parse_decision_test(decision_test: &String) -> u64 {
        Monkey::parse_decision_generic(decision_test) as u64
    }

    fn parse_decision_true(decision_true: &String) -> usize {
        Monkey::parse_decision_generic(decision_true)
    }

    fn parse_decision_false(decision_false: &String) -> usize {
        Monkey::parse_decision_generic(decision_false)
    }

    fn parse_decision_generic(line: &String) -> usize {
        line.trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse()
            .unwrap()
    }
}
