use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

fn moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .flat_map(|line| {
            let mut parts = line.split_whitespace();
            let mov = match parts.next().unwrap() {
                "U" => Move::Up,
                "R" => Move::Right,
                "D" => Move::Down,
                "L" => Move::Left,
                _ => panic!(),
            };
            let times = parts.next().unwrap().parse::<usize>().unwrap();
            vec![mov; times]
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail);
    moves(input).iter().for_each(|mov| {
        let old_head = head;
        match mov {
            Move::Up => head = (head.0, head.1 - 1),
            Move::Right => head = (head.0 + 1, head.1),
            Move::Down => head = (head.0, head.1 + 1),
            Move::Left => head = (head.0 - 1, head.1),
        };

        let vec = (tail.0 - head.0, tail.1 - head.1);
        if vec.0.abs() > 1 || vec.1.abs() > 1 {
            tail = old_head;
            visited.insert(tail);

        }
    });
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u_turn() {
        let input = "R 2\nU 1\nL 2";
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
