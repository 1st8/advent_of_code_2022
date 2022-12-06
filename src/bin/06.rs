use std::collections::{hash_map::Entry, HashMap, VecDeque};

fn find_uniq_sequence(input: &str, length: usize) -> Option<u32> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut buffer = VecDeque::new();
    let result = input.chars().enumerate().find_map(|(i, char)| {
        buffer.push_back(char);
        counts
            .entry(char)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        if i > length - 1 {
            let rem = buffer.pop_front().unwrap();
            if let Entry::Occupied(o) = counts.entry(rem).and_modify(|count| *count -= 1) {
                if *o.get() == 0 {
                    counts.remove(&rem);
                }
            };
        } else {
            return None;
        }

        if counts.len() == length {
            Some((i + 1) as u32)
        } else {
            None
        }
    });

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    find_uniq_sequence(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_uniq_sequence(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
    }
}
