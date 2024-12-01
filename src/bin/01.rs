use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

advent_of_code::solution!(1);

fn get_lists<T>(input: &str) -> (Vec<T>, Vec<T>)
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let lines: Vec<&str> = input.lines().collect();
    let mut first_list: Vec<T> = Vec::new();
    let mut second_list: Vec<T> = Vec::new();

    for line in lines {
        let (first_str, second_str) = line.split_once("   ").unwrap();
        first_list.push(first_str.parse::<T>().unwrap());
        second_list.push(second_str.parse::<T>().unwrap());
    }

    (first_list, second_list)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first_list, mut second_list) = get_lists::<i32>(input);

    first_list.sort();
    second_list.sort();

    Some(first_list.iter().enumerate().fold(0, |acc, (i, _)| {
        acc + (first_list[i] - second_list[i]).unsigned_abs()
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first_list, second_list) = get_lists::<u32>(input);

    let mut occurrence_map: HashMap<u32, u32> = HashMap::new();

    for val in second_list {
        occurrence_map.insert(val, *occurrence_map.get(&val).unwrap_or(&0u32) + 1u32);
    }

    Some(first_list.iter().fold(0, |acc, item| {
        acc + item * *occurrence_map.get(item).unwrap_or(&0u32)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 11u32);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 31u32);
    }
}
