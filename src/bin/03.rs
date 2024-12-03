use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let reginald = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        reginald
            .captures_iter(input)
            .map(|captures| captures.extract())
            .map(|(_, [lhs, rhs])| lhs.parse::<u32>().unwrap() * rhs.parse::<u32>().unwrap())
            .sum::<u32>(),
    )
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
