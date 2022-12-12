use pathfinding::prelude::astar;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    // fn apply_to_3<F>(f: F) -> i32 where
    // // The closure takes an `i32` and returns an `i32`.
    // F: Fn(i32) -> i32 {

    // f(3)
    // }

    fn successors<F>(&self, heightmap: &HashMap<Pos, char>, check: F) -> Vec<(Pos, u32)>
    where
        F: Fn(i32) -> bool,
    {
        let current_height = heightmap.get(self).unwrap();
        let mut successors = vec![];
        maybe_add(
            vec![
                Pos(self.0, self.1 - 1),
                Pos(self.0 - 1, self.1),
                Pos(self.0 + 1, self.1),
                Pos(self.0, self.1 + 1),
            ],
            &mut successors,
            heightmap,
            current_height,
            check,
        );
        successors
    }
}

fn maybe_add<F>(
    positions: Vec<Pos>,
    acc: &mut Vec<(Pos, u32)>,
    heightmap: &HashMap<Pos, char>,
    current_height: &char,
    check: F,
) where
    F: Fn(i32) -> bool,
{
    for pos in positions {
        if let Some(height) = heightmap.get(&pos) {
            if check((*height as i32) - (*current_height as i32)) {
                acc.push((pos, 1));
            }
        }
    }
}

fn search(search: char) -> impl for<'r, 's> Fn((&'r Pos, &'s char)) -> std::option::Option<Pos> {
    move |(pos, height): (&Pos, &char)| {
        if *height == search {
            Some(pos.clone())
        } else {
            None
        }
    }
}

fn parse(input: &str) -> (Pos, Pos, HashMap<Pos, char>) {
    let mut heightmap: HashMap<Pos, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, height)| (Pos(x as i32, y as i32), height))
        })
        .collect();

    let start = heightmap.iter().find_map(search('S')).unwrap();
    let goal = heightmap.iter().find_map(search('E')).unwrap();
    heightmap.insert(start.clone(), 'a');
    heightmap.insert(goal.clone(), 'z');

    (start, goal, heightmap)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, goal, heightmap) = parse(input);

    let (_, count) = astar(
        &start,
        |p| p.successors(&heightmap, |height_diff| height_diff <= 1),
        |p| p.distance(&goal) / 3,
        |p| *p == goal,
    )
    .unwrap();

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, goal, heightmap) = parse(input);

    let (_, count) = astar(
        &goal,
        |p| p.successors(&heightmap, |height_diff| height_diff >= -1),
        |p| p.distance(&goal) / 3,
        |p| *heightmap.get(p).unwrap() == 'a',
    )
    .unwrap();

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
