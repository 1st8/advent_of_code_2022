use std::collections::{HashSet, VecDeque};

fn find_uniq_sequence(input: &str, length: usize) -> Option<u32> {
    let iter = input.chars();
    let mut buffer = VecDeque::new();
    let result = iter.enumerate().find_map(|(i, char)| {
        buffer.push_back(char);

        if i > length - 1 {
            buffer.pop_front();
        } else {
            return None;
        }

        let mut uniqs: HashSet<&char> = HashSet::new();
        for char in buffer.iter() {
            if !uniqs.insert(char) {
                return None;
            }
        }

        Some((i + 1) as u32)
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
