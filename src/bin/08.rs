use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> (usize, usize, HashMap<(usize, usize), u32>) {
    let mut result = HashMap::new();
    let mut last = (0, 0);
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            result.insert((x, y), c.to_digit(10).unwrap());
            last = (x, y);
        })
    });
    (last.0, last.1, result)
}

fn check(
    trees: &HashMap<(usize, usize), u32>,
    coords: (usize, usize),
    max: &mut i32,
    visible: &mut HashSet<(usize, usize)>,
) {
    let height = (*trees.get(&coords).unwrap()) as i32;
    if height > *max {
        visible.insert(coords);
        *max = height;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mx, my, trees) = parse(input);
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..=my {
        let mut max = -1;
        for x in 0..=mx {
            check(&trees, (x, y), &mut max, &mut visible);
        }
        let mut max = -1;
        for x in (0..=mx).rev() {
            check(&trees, (x, y), &mut max, &mut visible);
        }
    }

    for x in 0..=mx {
        let mut max = -1;
        for y in 0..=my {
            check(&trees, (x, y), &mut max, &mut visible);
        }
        let mut max = -1;
        for y in (0..=my).rev() {
            check(&trees, (x, y), &mut max, &mut visible);
        }
    }

    Some(visible.len())
}

fn scenic_score(
    coords: (usize, usize),
    trees: &HashMap<(usize, usize), u32>,
    limits: (usize, usize),
) -> usize {
    let (x, y) = coords;
    let (mx, my) = limits;
    let height = trees.get(&(x, y)).unwrap();
    let up = ((y + 1)..=my)
        .position(|i| trees.get(&(x, i)).unwrap() >= height)
        .unwrap_or(my - y - 1)
        + 1;
    let right = ((x + 1)..=mx)
        .position(|i| trees.get(&(i, y)).unwrap() >= height)
        .unwrap_or(mx - x - 1)
        + 1;
    let down = (0..y)
        .rev()
        .position(|i| trees.get(&(x, i)).unwrap() >= height)
        .unwrap_or(y - 1)
        + 1;
    let left = (0..x)
        .rev()
        .position(|i| trees.get(&(i, y)).unwrap() >= height)
        .unwrap_or(x - 1)
        + 1;

    up * right * down * left
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mx, my, trees) = parse(input);

    let mut max = 0;
    for tx in 1..mx {
        for ty in 1..my {
            let score = scenic_score((tx, ty), &trees, (mx, my));
            if score > max {
                max = score;
            }
        }
    }
    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
