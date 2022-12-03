#[derive(PartialEq, Clone, Debug)]
enum Signs {
    Rock,
    Paper,
    Scissor,
}

impl Signs {
    fn new(sign: char) -> Self {
        match sign {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissor,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissor,
            _ => todo!(),
        }
    }
}

fn loses_to(sign: &Signs) -> &Signs {
    match sign {
        Signs::Paper => &Signs::Scissor,
        Signs::Rock => &Signs::Paper,
        Signs::Scissor => &Signs::Rock,
    }
}

fn wins_over(sign: &Signs) -> &Signs {
    match sign {
        Signs::Scissor => &Signs::Paper,
        Signs::Paper => &Signs::Rock,
        Signs::Rock => &Signs::Scissor,
    }
}

fn score(pair: (Signs, Signs)) -> u32 {
    let game = match &pair {
        (a, b) if loses_to(a) == b => 6,
        (a, b) if a == b => 3,
        _ => 0,
    };
    let own = match &pair {
        (_, Signs::Rock) => 1,
        (_, Signs::Paper) => 2,
        (_, Signs::Scissor) => 3,
    };
    game + own
}

fn pairs(input: &str) -> impl Iterator<Item = (Signs, Signs)> + '_ {
    input.trim_end_matches("\n").split("\n").map(|line| {
        (
            Signs::new(line.chars().nth(0).unwrap()),
            Signs::new(line.chars().nth(2).unwrap()),
        )
    })
}

fn pairs2(input: &str) -> impl Iterator<Item = (Signs, Signs)> + '_ {
    input.trim_end_matches("\n").split("\n").map(line2)
}

fn line2(line: &str) -> (Signs, Signs) {
    let opp = Signs::new(line.chars().nth(0).unwrap());
    let own = match line.chars().nth(2).unwrap() {
        'X' => wins_over(&opp),
        'Y' => &opp,
        'Z' => loses_to(&opp),
        _ => panic!(),
    };
    (opp.clone(), own.clone())
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(pairs(input).map(score).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(pairs2(input).map(score).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_score() {
        assert_eq!(score((Signs::Rock, Signs::Paper)), 8);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_line2() {
        assert_eq!(line2("A Y"), (Signs::Rock, Signs::Rock));
        assert_eq!(line2("B X"), (Signs::Paper, Signs::Rock));
        assert_eq!(line2("C Z"), (Signs::Scissor, Signs::Rock));
    }
}
