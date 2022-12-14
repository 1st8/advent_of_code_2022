use serde_json::{json, Value};
use std::cmp::Ordering::{self, Equal, Less};
use std::iter::zip;

fn pairs(input: &str) -> impl Iterator<Item = (Value, Value)> + '_ {
    input.split("\n\n").map(|lines| {
        let mut lines = lines.lines();
        let a: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        let b: Value = serde_json::from_str(lines.next().unwrap()).unwrap();

        (a, b)
    })
}

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a.as_i64().cmp(&b.as_i64()),
        (Value::Array(a), Value::Array(b)) => {
            let len_cmp = a.len().cmp(&b.len());
            let a_iter = a.iter();
            let b_iter = b.iter();
            if let Some(result) = zip(a_iter, b_iter).find_map(|(a, b)| {
                let result = compare(a, b);
                if result == Equal {
                    None
                } else {
                    Some(result)
                }
            }) {
                result
            } else {
                len_cmp
            }
        }
        (Value::Number(a), Value::Array(_)) => compare(&json!([a]), b),
        (Value::Array(_), Value::Number(b)) => compare(a, &json!([b])),
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let sum: usize = pairs(input)
        .enumerate()
        .map(|(i, (a, b))| if compare(&a, &b) == Less { i + 1 } else { 0 })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut packets = pairs(input).flat_map(|(a, b)| [a, b]).collect::<Vec<_>>();
    packets.push(json!([[2]]));
    packets.push(json!([[6]]));
    packets.sort_by(compare);
    let index1 = packets.iter().position(|p| *p == json!([[2]])).unwrap() + 1;
    let index2 = packets.iter().position(|p| *p == json!([[6]])).unwrap() + 1;
    Some(index1 * index2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::cmp::Ordering::{Equal, Greater, Less};

    #[test]
    fn test_compare() {
        assert_eq!(compare(&json!(0), &json!(1)), Less);
        assert_eq!(compare(&json!([0]), &json!([1])), Less);
        assert_eq!(compare(&json!([]), &json!([1])), Less);
        assert_eq!(compare(&json!([0]), &json!([])), Greater);
        assert_eq!(compare(&json!([2, 3, 4]), &json!(4)), Less);
        assert_eq!(compare(&json!([[1], [2, 3, 4]]), &json!([[1], 4])), Less);
        assert_eq!(compare(&json!(0), &json!(0)), Equal);
        assert_eq!(compare(&json!([]), &json!([])), Equal);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
