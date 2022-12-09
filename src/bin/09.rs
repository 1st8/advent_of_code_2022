use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn apply(&self, pos: &mut (i32, i32)) {
        match &self {
            Move::Up => pos.1 -= 1,
            Move::Right => pos.0 += 1,
            Move::Down => pos.1 += 1,
            Move::Left => pos.0 -= 1,
        };
    }
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
        mov.apply(&mut head);

        let vec = (tail.0 - head.0, tail.1 - head.1);
        if vec.0.abs() > 1 || vec.1.abs() > 1 {
            tail = old_head;
            visited.insert(tail);
        }
    });
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let length = 10;
    let mut knots = vec![(0, 0); length];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    moves(input).iter().for_each(|mov| {
        let mut iter = knots.iter_mut();
        let head = iter.next().unwrap();
        mov.apply(head);

        let mut prev = head;
        iter.for_each(|knot| {
            let vec: (i32, i32) = (knot.0 - prev.0, knot.1 - prev.1);

            if vec.0.abs() > 1 || vec.1.abs() > 1 {
                match vec.0 {
                    n if n < 0 => Move::Right.apply(knot),
                    n if n > 0 => Move::Left.apply(knot),
                    _ => (),
                }

                match vec.1 {
                    n if n < 0 => Move::Down.apply(knot),
                    n if n > 0 => Move::Up.apply(knot),
                    _ => (),
                }
            }

            prev = knot;
        });

        visited.insert(knots[9]);
    });
    Some(visited.len())
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
    fn test_moves() {
        let input = "R 2\nU 1\nL 2\nD 2";
        assert_eq!(
            moves(input),
            [
                Move::Right,
                Move::Right,
                Move::Up,
                Move::Left,
                Move::Left,
                Move::Down,
                Move::Down
            ]
        );
    }

    #[test]
    fn test_u_turn() {
        let input = "R 2\nU 1\nL 2";
        assert_eq!(part_one(input), Some(2));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        assert_eq!(part_two(input), Some(36));
    }

    #[test]
    fn test_part_two_3() {
        let input = "R 10";
        assert_eq!(part_two(input), Some(2));
    }

    #[test]
    fn test_part_two_l() {
        let input = "L 10";
        assert_eq!(part_two(input), Some(2));
    }

    #[test]
    fn test_part_two_d() {
        let input = "D 10";
        assert_eq!(part_two(input), Some(2));
    }

    #[test]
    fn test_part_two_u() {
        let input = "U 10";
        assert_eq!(part_two(input), Some(2));
    }
}
