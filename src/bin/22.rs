use array2d::Array2D;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum Block {
    Void,
    Open,
    Wall,
}

impl From<char> for Block {
    fn from(c: char) -> Block {
        match c {
            ' ' => Block::Void,
            '.' => Block::Open,
            '#' => Block::Wall,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn value(&self) -> i32 {
        *self as i32
    }
    fn rotate(&self, rot: &Direction) -> Direction {
        match *rot {
            Direction::Right => ((self.value() + 1) % 4).into(),
            Direction::Left => ((self.value() - 1).rem_euclid(4)).into(),
            _ => panic!(),
        }
    }
}

impl From<&str> for Direction {
    fn from(c: &str) -> Direction {
        match c {
            "R" => Self::Right,
            "L" => Self::Left,
            _ => panic!(),
        }
    }
}
impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        match value {
            x if x == Self::Right.value() => Self::Right,
            x if x == Self::Down.value() => Self::Down,
            x if x == Self::Left.value() => Self::Left,
            x if x == Self::Up.value() => Self::Up,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Action {
    Walk(usize),
    Rotate(Direction),
}

fn find_start(map: &Array2D<Block>) -> (usize, usize) {
    (
        map.row_iter(0)
            .unwrap()
            .position(|b| *b == Block::Open)
            .unwrap(),
        0_usize,
    )
}

fn parse_actions(actions_input: &str) -> Vec<Action> {
    let mut actions = vec![];
    let re = Regex::new(r"(?P<dir>[RDLU])?(?P<dist>\d+)").unwrap();
    for caps in re.captures_iter(actions_input) {
        if let Some(dir) = caps.name("dir") {
            actions.push(Action::Rotate(Direction::from(dir.as_str())));
        }
        actions.push(Action::Walk(
            caps.name("dist").unwrap().as_str().parse().unwrap(),
        ));
    }
    actions
}

fn walk(
    position: &(usize, usize),
    direction: &Direction,
    num_rows: usize,
    num_columns: usize,
) -> (usize, usize) {
    match direction {
        Direction::Right => ((position.0 + 1).rem_euclid(num_columns), position.1),
        Direction::Down => (position.0, (position.1 + 1).rem_euclid(num_rows)),
        Direction::Left => (
            (position.0 as i32 - 1).rem_euclid(num_columns as i32) as usize,
            position.1,
        ),
        Direction::Up => (
            position.0,
            (position.1 as i32 - 1).rem_euclid(num_rows as i32) as usize,
        ),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_parts = input.split("\n\n");
    let map_input = input_parts.next().unwrap();
    let map_raw = map_input
        .lines()
        .map(|l| l.chars().map(Block::from).collect::<Vec<Block>>())
        .collect::<Vec<Vec<Block>>>();
    let num_columns = map_raw
        .iter()
        .map(|row| row.len())
        .max()
        .expect("max row length");
    let num_rows = map_raw.len();

    let map = Array2D::from_iter_row_major(
        map_raw.into_iter().flat_map(|row| {
            row.into_iter()
                .chain(std::iter::repeat(Block::Void))
                .take(num_columns)
        }),
        num_rows,
        num_columns,
    )
    .expect("to array2d");

    let mut direction = Direction::Right;
    let mut position = find_start(&map);
    let actions = parse_actions(input_parts.next().unwrap());

    for action in actions {
        match action {
            Action::Walk(dist) => {
                for _ in 0..dist {
                    let mut next_position = walk(&position, &direction, num_rows, num_columns);

                    if let Some(block) = map.get(next_position.1, next_position.0) {
                        match block {
                            Block::Open => position = next_position,
                            Block::Wall => break,
                            Block::Void => {
                                while let Some(Block::Void) =
                                    map.get(next_position.1, next_position.0)
                                {
                                    next_position =
                                        walk(&next_position, &direction, num_rows, num_columns);
                                }
                                if let Some(Block::Open) = map.get(next_position.1, next_position.0)
                                {
                                    position = next_position;
                                }
                            }
                        }
                    }
                }
            }
            Action::Rotate(rot) => {
                direction = direction.rotate(&rot);
            }
        }
    }

    Some(1000 * (position.1 + 1) as u32 + 4 * (position.0 + 1) as u32 + direction.value() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
