use std::ops::Range;

fn ranges(input: &str) -> impl Iterator<Item = (Range<u32>, Range<u32>)> + '_ {
    input.lines().map(|line| {
        let mut iter = line.split(',').map(|r| {
            let mut iter = r.split('-').map(|i| i.parse::<u32>().unwrap());
            let start = iter.next().unwrap();
            let end = iter.next().unwrap() + 1;
            Range { start, end }
        });
        let r1 = iter.next().unwrap();
        let r2 = iter.next().unwrap();
        (r1, r2)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    ranges(input).for_each(|(r1, r2)| {
        if (r1.start >= r2.start && r1.end <= r2.end) || (r2.start >= r1.start && r2.end <= r1.end)
        {
            result += 1;
        }
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    ranges(input).for_each(|(r1, r2)| {
        if !(r1.start >= r2.end || r2.start >= r1.end) {
            result += 1;
        }
    });

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
