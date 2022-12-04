use std::{collections::HashSet, ops::Range};

fn sets(input: &str) -> impl Iterator<Item = (HashSet<u32>, HashSet<u32>)> + '_ {
    input.trim_end_matches("\n").split("\n").map(|line| {
        let mut iter = line.split(',').map(|r| {
            let mut iter = r.split('-').map(|i| i.parse::<u32>().unwrap());
            let start = iter.next().unwrap();
            let end = iter.next().unwrap() + 1;
            Range { start, end }
        });
        let r1: HashSet<u32> = iter.next().unwrap().collect();
        let r2: HashSet<u32> = iter.next().unwrap().collect();
        (r1, r2)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = sets(input)
        .map(|(r1, r2)| {
            if r1.is_subset(&r2) || r2.is_subset(&r1) {
                1
            } else {
                0
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = sets(input)
        .map(|(r1, r2)| if !r1.is_disjoint(&r2) { 1 } else { 0 })
        .sum();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
