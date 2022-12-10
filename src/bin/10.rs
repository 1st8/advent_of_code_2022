pub fn part_one(input: &str) -> Option<i32> {
    let values = register_iter(input).enumerate().filter_map(|(i, x)| {
        if i % 40 == 19 {
            Some((i + 1) as i32 * x)
        } else {
            None
        }
    });

    Some(values.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    register_iter(input).enumerate().for_each(|(i, x)| {
        if (x..=(x + 2)).contains(&(((i as i32) % 40) + 1)) {
            print!("█");
        } else {
            print!(" ");
        };
        if i % 40 == 39 {
            println!();
        }
    });

    Some(1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn register_iter(input: &str) -> impl Iterator<Item = i32> + '_ {
    let mut x = 1;
    input.lines().flat_map(move |line| {
        let result;
        match &line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => result = vec![x],
            ["addx", n] => {
                result = vec![x, x];
                x += n.parse::<i32>().unwrap()
            }
            _ => panic!(),
        };

        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_iter() {
        assert_eq!(
            register_iter("addx 15\nnoop").collect::<Vec<_>>(),
            vec![1, 1, 16]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }
}
