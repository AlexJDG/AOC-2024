use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<u64> {
    input
        .split(" ")
        .map(|el| el.parse::<u64>().unwrap())
        .collect()
}

fn blink(stone: u64, count: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if count == 0 {
        return 1;
    }

    if cache.contains_key(&(stone, count)) {
        return *cache.get(&(stone, count)).unwrap();
    }

    let output;

    if stone == 0 {
        output = blink(1, count - 1, cache);
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let (left, right) = stone_str.split_at(stone_str.len() / 2);

            output = blink(left.parse::<u64>().unwrap(), count - 1, cache)
                + blink(right.parse::<u64>().unwrap(), count - 1, cache);
        } else {
            output = blink(stone * 2024, count - 1, cache);
        }
    }

    cache.insert((stone, count), output);

    output
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut cache = HashMap::new();

    Some(
        parse(input)
            .iter()
            .map(|stone| blink(*stone, 25, &mut cache))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cache = HashMap::new();

    Some(
        parse(input)
            .iter()
            .map(|stone| blink(*stone, 75, &mut cache))
            .sum(),
    )
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
        assert_eq!(result, Some(65601038650482));
    }
}
