use std::collections::HashMap;

fn get_folder_sizes(input: &str) -> HashMap<String, u32> {
    let mut cwd: Vec<String> = vec![];
    let mut folder_sizes: HashMap<String, u32> = HashMap::from([(String::from("/"), 0)]);
    input.lines().for_each(|line| {
        match &line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {
                cwd.clear();
            }
            ["$", "cd", ".."] => {
                cwd.pop();
            }
            ["$", "cd", path] => {
                cwd.push(format!("{}/{}", cwd.join("/"), path));
            }
            ["$", _, _] => {}
            ["$", _] => {}
            ["dir", _] => {}
            [size, _] => {
                let size = size.parse::<u32>().unwrap();
                folder_sizes
                    .entry(String::from("/"))
                    .and_modify(|sum| *sum += size);
                for path in cwd.iter() {
                    folder_sizes
                        .entry(path.clone())
                        .and_modify(|sum| *sum += size)
                        .or_insert(size);
                }
            }
            _ => panic!(),
        };
    });
    folder_sizes
}

pub fn part_one(input: &str) -> Option<u32> {
    let folder_sizes = get_folder_sizes(input);

    let result: u32 = folder_sizes.into_values().filter(|v| *v < 100000).sum();
    Some(result)
}

const CAPACITY: u32 = 70000000;
const SPACE_WANTED: u32 = 30000000;

pub fn part_two(input: &str) -> Option<u32> {
    let folder_sizes = get_folder_sizes(input);
    let free = CAPACITY - *folder_sizes.get("/").unwrap();

    let mut sizes = folder_sizes.into_values().collect::<Vec<_>>();
    sizes.sort_unstable();
    sizes.into_iter().find(|v| free + v > SPACE_WANTED)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
