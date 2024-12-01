advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for line in lines {
        let (first_str, second_str) = line.split_once("   ").unwrap();
        first_list.push(first_str.parse::<i32>().unwrap());
        second_list.push(second_str.parse::<i32>().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let mut sum: u32 = 0;

    for (i, _) in first_list.iter().enumerate() {
        sum += (first_list[i] - second_list[i]).abs() as u32;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
