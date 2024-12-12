advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<u64> {
    input
        .split(" ")
        .map(|el| el.parse::<u64>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = parse(input);

    for _ in 0..25 {
        stones = stones
            .iter()
            .flat_map(|stone| {
                if *stone == 0 {
                    return vec![1u64];
                }

                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let (left, right) = stone_str.split_at(stone_str.len() / 2);
                    return vec![left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()];
                }

                vec![stone * 2024]
            })
            .collect()
    }

    Some(stones.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
