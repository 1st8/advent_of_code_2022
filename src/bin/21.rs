use std::{cmp::Ordering, collections::HashMap, thread::panicking};

type Expressions<'a> = HashMap<&'a str, Expression<'a>>;

fn parse(input: &str) -> Expressions {
    input
        .lines()
        .map(|line| {
            if let [key, expr] = &line.split(": ").collect::<Vec<&str>>()[..] {
                (*key, Expression::from(*expr))
            } else {
                panic!()
            }
        })
        .collect()
}

#[derive(Debug)]
enum Expression<'a> {
    Dynamic(Operation<'a>),
    Const(i64),
}

impl Expression<'_> {
    fn solve(&self, expressions: &Expressions) -> i64 {
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
    op: &'a str,
    b: &'a str,
}

impl Operation<'_> {
    fn call(&self, expressions: &Expressions) -> i64 {
        let a = expressions.get(self.a).unwrap();
        let b = expressions.get(self.b).unwrap();
        let op = match self.op {
            "+" => std::ops::Add::add,
            "-" => std::ops::Sub::sub,
            "*" => std::ops::Mul::mul,
            "/" => std::ops::Div::div,
            _ => panic!(),
        };
        op(a.solve(expressions), b.solve(expressions))
    }
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(input: &'a str) -> Operation<'a> {
        if let [a, op, b] = &input.split_whitespace().collect::<Vec<&str>>()[..] {
            Operation { a, b, op }
        } else {
            panic!()
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut expressions = parse(input);
    let root = expressions.remove("root").unwrap();

    Some(root.solve(&expressions))
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut expressions = parse(input);
    let root = expressions.remove("root").unwrap();
    let left;
    let right;
    if let Expression::Dynamic(op) = root {
        left = expressions.remove(op.a).unwrap();
        right = expressions.remove(op.b).unwrap();
    } else {
        panic!()
    };

    let mut i = 0;
    let mut test = 0;
    let mut incr = 100000000;
    let right_res = right.solve(&expressions);
    let init_comp = right_res.cmp(&left.solve(&expressions));
    loop {
        expressions.insert("humn", Expression::Const(test));
        let left_res = left.solve(&expressions);
        let comp = right_res.cmp(&left_res);
        match comp {
            Ordering::Equal => return Some(test),
            comp => {
                if i % 1000 == 0 {
                    println!("{} != {} ({} {})", left_res, right_res, test, incr);
                }
                if comp != init_comp {
                    println!(
                        "DECR {} {:?} {} (test={} incr={} i={})",
                        left_res, comp, right_res, test, incr, i
                    );
                    test -= incr;
                    incr /= 10;
                    if incr == 0 {
                        panic!()
                    }
                } else {
                    test += incr;
                }
                i += 1;
            }
        }
    }
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
        assert_eq!(part_two(&input), Some(301));
    }
}
