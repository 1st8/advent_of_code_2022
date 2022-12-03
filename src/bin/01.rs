use std::collections::BinaryHeap;

fn block_sums(input: &str) -> impl Iterator<Item = u32> + '_ {
    let result = input
        .strip_suffix("\n")
        .unwrap()
        .split("\n\n")
        .map(|block| {
            block
                .split("\n")
                .map(|s| s.parse::<u32>().unwrap())
                .sum::<u32>()
        });
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut max = 0;
    block_sums(input).for_each(|i| {
        if i > max {
            max = i;
        }
    });

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut heap: BinaryHeap<u32> = block_sums(input).collect();

    let mut max3 = 0;
    for _ in 0..3 {
        max3 += heap.pop().unwrap()
    }

    Some(max3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
