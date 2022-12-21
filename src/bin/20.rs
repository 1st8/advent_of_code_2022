pub fn part_one(input: &str) -> Option<i64> {
    let result = apply_moves(input.lines().map(|l| l.parse::<i64>().unwrap()).collect(), 1, 1);
    let len = result.len();
    let zero = result.iter().position(|&i| i == 0).unwrap();
    let sum = [1000, 2000, 3000]
        .into_iter()
        .map(|i| result[(i + zero) % len])
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let result = apply_moves(input.lines().map(|l| l.parse::<i64>().unwrap()).collect(), 811589153, 10);
    let len = result.len();
    let zero = result.iter().position(|&i| i == 0).unwrap();
    let sum = [1000, 2000, 3000]
        .into_iter()
        .map(|i| result[(i + zero) % len])
        .sum();
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn apply_moves(initial: Vec<i64>, multiplicator: i64, times: usize) -> Vec<i64> {
    let len = initial.len() as i64;
    let mut tuples: Vec<_> = initial.into_iter().map(|n| n * multiplicator).enumerate().collect();

    for j in 0..times {
        for i in 0..(len as usize) {
            let src_idx = tuples.iter().position(|tuple| tuple.0 == i).unwrap();
    
            let tuple = tuples.remove(src_idx);
            let dst_idx = (src_idx as i64) + tuple.1;
            let dst_idx = dst_idx.rem_euclid(len - 1) as usize;
            tuples.insert(dst_idx, tuple);
        }
    }

    tuples.into_iter().map(|(_, val)| val).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_moves() {
        let initial = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(apply_moves(initial, 1, 1), vec![-2, 1, 2, -3, 4, 0, 3]);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
