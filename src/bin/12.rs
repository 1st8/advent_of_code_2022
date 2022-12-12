use pathfinding::prelude::astar;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, heightmap: &HashMap<Pos, char>) -> Vec<(Pos, u32)> {
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
        );
        successors
    }
}

fn maybe_add(
    positions: Vec<Pos>,
    acc: &mut Vec<(Pos, u32)>,
    heightmap: &HashMap<Pos, char>,
    current_height: &char,
) {
    for pos in positions {
        if let Some(height) = heightmap.get(&pos) {
            if (*height as i32) - (*current_height as i32) <= 1 {
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

pub fn part_one(input: &str) -> Option<u32> {
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

    let (_, count) = astar(
        &start,
        |p| p.successors(&heightmap),
        |p| p.distance(&goal) / 3,
        |p| *p == goal,
    )
    .unwrap();

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
