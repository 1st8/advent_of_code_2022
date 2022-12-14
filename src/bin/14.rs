use std::{collections::HashMap, ops::RangeInclusive};

enum Block {
    Rock,
    Sand,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pos(u32, u32);
impl From<&str> for Pos {
    fn from(coords: &str) -> Pos {
        if let [x, y] = &coords.split(',').collect::<Vec<&str>>()[..] {
            Pos(x.parse().unwrap(), y.parse().unwrap())
        } else {
            panic!()
        }
    }
}

impl Pos {
    fn down(&self) -> Pos {
        Pos(self.0, self.1 + 1)
    }
    fn down_left(&self) -> Pos {
        Pos(self.0 - 1, self.1 + 1)
    }
    fn down_right(&self) -> Pos {
        Pos(self.0 + 1, self.1 + 1)
    }
}

fn parse(
    input: &str,
) -> (
    HashMap<Pos, Block>,
    RangeInclusive<u32>,
    RangeInclusive<u32>,
) {
    let mut min_x = u32::MAX;
    let mut max_x = 0;
    let min_y = 0;
    let mut max_y = 0;
    let mut result = HashMap::new();

    let mut assign_rock = |pos| {
        result.insert(pos, Block::Rock);
    };
    let mut widen_ranges = |pos: &Pos| {
        if pos.0 < min_x {
            min_x = pos.0;
        }
        // if pos.1 < min_y {
        //     min_y = pos.1;
        // }
        if pos.0 > max_x {
            max_x = pos.0;
        }
        if pos.1 > max_y {
            max_y = pos.1;
        }
    };

    input.lines().for_each(|line| {
        let mut coords = line.split(" -> ").map(Pos::from);

        let mut start: Pos = coords.next().unwrap();
        widen_ranges(&start);
        for end in coords {
            widen_ranges(&end);
            if start.0 == end.0 {
                if end.1 > start.1 {
                    (start.1)..=(end.1)
                } else {
                    (end.1)..=(start.1)
                }
                .map(|y| Pos(start.0, y))
                .for_each(&mut assign_rock)
            } else if start.1 == end.1 {
                if end.0 > start.0 {
                    (start.0)..=(end.0)
                } else {
                    (end.0)..=(start.0)
                }
                .map(|x| Pos(x, start.1))
                .for_each(&mut assign_rock)
            } else {
                panic!()
            };
            start = end;
        }
    });

    (result, min_x..=max_x, min_y..=max_y)
}

fn advance(pos: &Pos, cave: &HashMap<Pos, Block>) -> Option<Pos> {
    if cave.contains_key(&pos.down()) {
        if cave.contains_key(&pos.down_left()) {
            if cave.contains_key(&pos.down_right()) {
                None
            } else {
                Some(pos.down_right())
            }
        } else {
            Some(pos.down_left())
        }
    } else {
        Some(pos.down())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut cave, rx, ry) = parse(input);
    let mut sand_count = 0;

    'outer: loop {
        sand_count += 1;
        let mut pos = Pos(500, 0);
        'inner: loop {
            if !rx.contains(&pos.0) || !ry.contains(&pos.1) {
                break 'outer;
            };

            if let Some(new_pos) = advance(&pos, &cave) {
                pos = new_pos
            } else {
                cave.insert(pos, Block::Sand);
                break 'inner;
            }
        }
    }
    Some(sand_count - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut cave, _rx, ry) = parse(input);
    let mut sand_count = 0;

    'outer: loop {
        sand_count += 1;
        let mut pos = Pos(500, 0);
        if cave.contains_key(&pos) {
            break 'outer;
        }
        'inner: loop {
            if pos.1 == ry.end() + 1 {
                cave.insert(pos, Block::Sand);
                break 'inner;
            }

            if let Some(new_pos) = advance(&pos, &cave) {
                pos = new_pos
            } else {
                cave.insert(pos, Block::Sand);
                break 'inner;
            }
        }
    }
    Some(sand_count - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (parsed, rx, ry) = parse("5,0 -> 7,0 -> 7,2 -> 9,2 -> 9,0 -> 8,0");
        assert_eq!(parsed.len(), 10);
        assert!(parsed.contains_key(&Pos(5, 0)));
        assert!(parsed.contains_key(&Pos(6, 0)));
        assert!(parsed.contains_key(&Pos(7, 0)));
        assert!(parsed.contains_key(&Pos(7, 1)));
        assert!(parsed.contains_key(&Pos(7, 2)));
        assert!(parsed.contains_key(&Pos(8, 2)));
        assert!(parsed.contains_key(&Pos(9, 2)));
        assert!(parsed.contains_key(&Pos(9, 1)));
        assert!(parsed.contains_key(&Pos(9, 0)));
        assert!(parsed.contains_key(&Pos(8, 0)));
        assert_eq!(rx, 5..=9);
        assert_eq!(ry, 0..=2);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
