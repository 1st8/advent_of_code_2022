use std::collections::VecDeque;

fn split_input(input: &str) -> (&str, &str) {
    let mut iter = input.split("\n\n");
    let raw_state = iter.next().unwrap();
    let raw_moves = iter.next().unwrap();
    (raw_state, raw_moves)
}

fn parse_state(raw_state: &str) -> State {
    let lines = raw_state.lines().collect::<Vec<&str>>();
    let mut lines_rev_iter = lines.iter().rev();
    let numbers = lines_rev_iter.next().unwrap();

    let mut state = vec![vec![]; (numbers.len() + 1) / 4];

    for l in lines_rev_iter {
        let chars = l
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if i % 4 == 1 { Some(c) } else { None });

        state
            .iter_mut()
            .zip(chars)
            .for_each(|(stack, char)| match char {
                ' ' => (),
                char => stack.push(char),
            });
    }

    state
}

fn parse_moves(raw_moves: &str) -> Vec<(usize, usize, usize)> {
    raw_moves
        .lines()
        .map(|l| {
            let mut numbers = l.split_whitespace().enumerate().filter_map(|(i, s)| {
                if i % 2 == 1 {
                    Some(s.parse::<usize>().unwrap())
                } else {
                    None
                }
            });
            (
                numbers.next().unwrap(),
                numbers.next().unwrap() - 1,
                numbers.next().unwrap() - 1,
            )
        })
        .collect()
}

type State = Vec<Vec<char>>;
type Move = (usize, usize, usize);

fn parse_input(input: &str) -> (State, Vec<Move>) {
    let (raw_state, raw_moves) = split_input(input);
    (parse_state(raw_state), parse_moves(raw_moves))
}

fn apply_move(mut state: State, mv: Move) -> State {
    let (amount, from, to) = mv;

    for _ in 0..amount {
        let from = state.get_mut(from).unwrap();
        let tmp = from.pop().unwrap();
        let to = state.get_mut(to).unwrap();
        to.push(tmp)
    }
    state
}

fn apply_move2(mut state: State, mv: Move) -> State {
    let (amount, from, to) = mv;

    let from = state.get_mut(from).unwrap();
    let mut tmp = VecDeque::new();
    for _ in 0..amount {
        tmp.push_front(from.pop().unwrap());
    }
    let to = state.get_mut(to).unwrap();
    for item in tmp {
        to.push(item);
    }

    state
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut state, moves) = parse_input(input);

    for mv in moves {
        state = apply_move(state, mv);
    }

    let result: String = state.iter().filter_map(|stack| stack.last()).collect();
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut state, moves) = parse_input(input);

    for mv in moves {
        state = apply_move2(state, mv);
    }

    let result: String = state.iter().filter_map(|stack| stack.last()).collect();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_move() {
        let state = vec![vec!['A', 'B'], vec![]];
        assert_eq!(apply_move(state, (2, 0, 1)), vec![vec![], vec!['B', 'A']]);
    }

    #[test]
    fn test_apply_move2() {
        let state = vec![vec!['A', 'B'], vec![]];
        assert_eq!(apply_move2(state, (2, 0, 1)), vec![vec![], vec!['A', 'B']]);
    }

    #[test]
    fn test_parse_state() {
        let input = advent_of_code::read_file("examples", 5);
        let (raw_state, _raw_moves) = split_input(&input);
        assert_eq!(
            parse_state(raw_state),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }

    #[test]
    fn test_parse_moves() {
        let input = advent_of_code::read_file("examples", 5);
        let (_raw_state, raw_moves) = split_input(&input);
        assert_eq!(parse_moves("move 1 from 2 to 1"), vec![(1, 1, 0)]);
        assert_eq!(
            parse_moves(raw_moves),
            vec![(1, 1, 0), (3, 0, 2), (2, 1, 0), (1, 0, 1)]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
