use std::collections::HashMap;

#[derive(Debug)]
enum Expression<'a> {
    Dynamic(Operation<'a>),
    Const(i64),
}

impl Expression<'_> {
    fn solve(&self, expressions: &HashMap<&str, Expression>) -> i64 {
        match self {
            Self::Dynamic(op) => op.call(expressions),
            Self::Const(i) => *i,
        }
    }
}

impl<'a> From<&'a str> for Expression<'a> {
    fn from(value: &'a str) -> Expression<'a> {
        if let Ok(value) = value.parse::<i64>() {
            Expression::Const(value)
        } else {
            Expression::Dynamic(Operation::from(value))
        }
    }
}

#[derive(Debug)]
struct Operation<'a> {
    a: &'a str,
    b: &'a str,
    op: fn(i64, i64) -> i64,
}

impl Operation<'_> {
    fn call(&self, expressions: &HashMap<&str, Expression>) -> i64 {
        let a = expressions.get(self.a).unwrap();
        let b = expressions.get(self.b).unwrap();
        (self.op)(a.solve(expressions), b.solve(expressions))
    }
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(input: &'a str) -> Operation<'a> {
        if let [a, op, b] = &input.split_whitespace().collect::<Vec<&str>>()[..] {
            let op = match *op {
                "+" => std::ops::Add::add,
                "-" => std::ops::Sub::sub,
                "*" => std::ops::Mul::mul,
                "/" => std::ops::Div::div,
                _ => panic!(),
            };
            Operation { a, b, op }
        } else {
            panic!()
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut expressions: HashMap<&str, Expression> = input
        .lines()
        .map(|line| {
            if let [key, expr] = &line.split(": ").collect::<Vec<&str>>()[..] {
                (*key, Expression::from(*expr))
            } else {
                panic!()
            }
        })
        .collect();

    let root = expressions.remove("root").unwrap();

    Some(root.solve(&expressions))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
