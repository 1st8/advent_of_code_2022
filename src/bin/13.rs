use serde_json::{json, Result, Value};
use std::cmp::Ordering;
use std::iter::zip;

fn pairs(input: &str) -> impl Iterator<Item = (Value, Value)> + '_ {
    input.split("\n\n").map(|lines| {
        let mut lines = lines.lines();
        let a: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        let b: Value = serde_json::from_str(lines.next().unwrap()).unwrap();

        (a, b)
    })
}

fn compare(a: Value, b: Value) -> Option<bool> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => {
            if a == b {
                None
            } else {
                Some(a.as_i64() < b.as_i64())
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            let len_cmp = a.len().cmp(&b.len());
            let a_iter = a.into_iter();
            let b_iter = b.into_iter();
            if let Some(result) = zip(a_iter, b_iter).find_map(|(a, b)| compare(a, b)) {
                Some(result)
            } else if len_cmp == Ordering::Equal {
                None
            } else {
                Some(len_cmp == Ordering::Less)
            }
        }
        (Value::Number(a), Value::Array(b)) => compare(json!([a]), Value::Array(b)),
        (Value::Array(a), Value::Number(b)) => compare(Value::Array(a), json!([b])),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let sum: usize = pairs(input)
        .enumerate()
        .map(|(i, (a, b))| {
            if let Some(true) = compare(a, b) {
                i + 1
            } else {
                0
            }
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compare() {
        assert_eq!(compare(json!(0), json!(1)), Some(true));
        assert_eq!(compare(json!([0]), json!([1])), Some(true));
        assert_eq!(compare(json!([]), json!([1])), Some(true));
        assert_eq!(compare(json!([0]), json!([])), Some(false));
        assert_eq!(compare(json!([2, 3, 4]), json!(4)), Some(true));
        assert_eq!(
            compare(json!([[1], [2, 3, 4]]), json!([[1], 4])),
            Some(true)
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
