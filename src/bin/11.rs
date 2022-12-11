use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Value {
    Old,
    Const(u64),
}

impl Value {
    fn const_or(&self, old: u64) -> u64 {
        match self {
            Self::Old => old,
            Self::Const(i) => *i,
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Value {
        if value == "old" {
            Value::Old
        } else {
            Value::Const(value.parse().unwrap())
        }
    }
}

#[derive(Debug)]
struct Operation {
    a: Value,
    b: Value,
    op: fn(u64, u64) -> u64,
}

impl Operation {
    fn call(&self, old: u64) -> u64 {
        (self.op)(self.a.const_or(old), self.b.const_or(old))
    }
}

impl From<&str> for Operation {
    fn from(input: &str) -> Operation {
        if let [a, op, b] = &input.split_whitespace().collect::<Vec<&str>>()[..] {
            let op = match *op {
                "+" => std::ops::Add::add,
                "-" => std::ops::Sub::sub,
                "*" => std::ops::Mul::mul,
                "/" => std::ops::Div::div,
                _ => panic!(),
            };
            Operation {
                a: (*a).into(),
                b: (*b).into(),
                op,
            }
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    items: VecDeque<u64>,
    modulo: u64,
    true_index: usize,
    false_index: usize,
}

impl Monkey {
    fn inspect(&self, item: u64) -> u64 {
        self.operation.call(item)
    }
}

impl From<&str> for Monkey {
    fn from(block: &str) -> Monkey {
        let mut lines = block.lines().skip(1);
        let mut next_after_prefix = |prefix| lines.next().unwrap().strip_prefix(prefix).unwrap();

        let items = next_after_prefix("  Starting items: ")
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();

        let operation = next_after_prefix("  Operation: new = ").into();

        let modulo = next_after_prefix("  Test: divisible by ").parse().unwrap();

        let true_index = next_after_prefix("    If true: throw to monkey ")
            .parse()
            .unwrap();

        let false_index = next_after_prefix("    If false: throw to monkey ")
            .parse()
            .unwrap();

        Monkey {
            items,
            operation,
            modulo,
            true_index,
            false_index,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let blocks = input.split("\n\n");
    let mut monkeys = blocks.map(Monkey::from).collect::<Vec<_>>();
    let mut inboxes: HashMap<usize, VecDeque<u64>> = HashMap::new();
    let mut inspections: HashMap<usize, u64> = HashMap::new();

    for _round in 0..20 {
        monkeys = monkeys
            .into_iter()
            .enumerate()
            .map(|(i, mut monkey)| {
                if let Some(inbox) = inboxes.get_mut(&i) {
                    monkey.items.append(inbox);
                }

                while let Some(item) = monkey.items.pop_front() {
                    inspections.entry(i).and_modify(|v| *v += 1).or_insert(1);
                    let worry_level = monkey.inspect(item);
                    let worry_level = worry_level / 3;
                    let throw_to = if worry_level % monkey.modulo == 0 {
                        monkey.true_index
                    } else {
                        monkey.false_index
                    };
                    inboxes
                        .entry(throw_to)
                        .and_modify(|v| v.push_back(worry_level))
                        .or_insert_with(|| VecDeque::from([worry_level]));
                }

                monkey
            })
            .collect::<Vec<_>>();
    }

    let mut inspection_counts = inspections.into_values().collect::<Vec<_>>();
    inspection_counts.sort_unstable_by_key(|w| std::cmp::Reverse(*w));
    let result = inspection_counts.first().unwrap() * inspection_counts.get(1).unwrap();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let blocks = input.split("\n\n");
    let mut monkeys = blocks.map(Monkey::from).collect::<Vec<_>>();
    let mut inboxes: HashMap<usize, VecDeque<u64>> = HashMap::new();
    let mut inspections: HashMap<usize, u64> = HashMap::new();
    let product: u64 = monkeys.iter().map(|m| m.modulo).product();

    for _round in 0..10000 {
        monkeys = monkeys
            .into_iter()
            .enumerate()
            .map(|(i, mut monkey)| {
                if let Some(inbox) = inboxes.get_mut(&i) {
                    monkey.items.append(inbox);
                }

                while let Some(item) = monkey.items.pop_front() {
                    inspections.entry(i).and_modify(|v| *v += 1).or_insert(1);
                    let worry_level = monkey.inspect(item);
                    let worry_level = worry_level % product;
                    let throw_to = if worry_level % monkey.modulo == 0 {
                        monkey.true_index
                    } else {
                        monkey.false_index
                    };
                    inboxes
                        .entry(throw_to)
                        .and_modify(|v| v.push_back(worry_level))
                        .or_insert_with(|| VecDeque::from([worry_level]));
                }

                monkey
            })
            .collect::<Vec<_>>();
    }

    let mut inspection_counts = inspections.into_values().collect::<Vec<_>>();
    inspection_counts.sort_unstable_by_key(|w| std::cmp::Reverse(*w));
    let result = inspection_counts.first().unwrap() * inspection_counts.get(1).unwrap();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        assert_eq!(Operation::from("old + 1").call(3), 4);
        assert_eq!(Operation::from("old + old").call(3), 6);
    }

    #[test]
    fn test_monkey() {
        let input = advent_of_code::read_file("examples", 11);
        let input_block = input.split("\n\n").next().unwrap();
        let monkey = Monkey::from(input_block);
        println!("{:#?}", monkey);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
