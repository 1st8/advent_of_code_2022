use std::collections::HashSet;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => (c as u32) - 38,
        _ => panic!(),
    }
}

fn compartments(line: &str) -> (&str, &str) {
    let len = line.len();
    (&line[0..(len / 2)], &line[(len / 2)..len])
}

fn overlap<'a>((l, r): (&'a str, &'a str)) -> Vec<char> {
    let l: HashSet<char> = l.chars().collect();
    let mut duplicates: HashSet<char> = HashSet::new();
    r.chars().for_each(|c| {
        if l.contains(&c) {
            duplicates.insert(c);
        }
    });
    duplicates.into_iter().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(compartments)
            .flat_map(overlap)
            .map(priority)
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;

    lines
        .chunks(3)
        .map(|lines| {
            let mut iter = lines.iter();
            let mut init: HashSet<char> = iter.next().unwrap().chars().collect();
            iter.for_each(|l| {
                let s: HashSet<char> = l.chars().collect();
                init.retain(|c| s.contains(c))
            });
            init
        })
        .for_each(|s| s.iter().for_each(|c| sum += priority(*c)));

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_compartments() {
        assert_eq!(
            compartments("vJrwpWtwJgWrhcsFMMfFFhFp"),
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        );
    }

    #[test]
    fn test_overlap() {
        assert_eq!(overlap(("vJrwpWtwJgWr", "hcsFMMfFFhFp")), vec!['p']);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
