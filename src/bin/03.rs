use regex::Regex;

advent_of_code::solution!(3);

fn sum_mul(input: &str) -> u32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| captures.extract::<2>().1)
        .map(|[lhs, rhs]| lhs.parse::<u32>().unwrap() * rhs.parse::<u32>().unwrap())
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_mul(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        Regex::new(r"(?:^|do\(\))(.*?)(?:don't\(\)|$)")
            .unwrap()
            .captures_iter(&input.lines().collect::<Vec<_>>().join(""))
            .flat_map(|captures| captures.extract::<1>().1)
            .map(sum_mul)
            .sum(),
    )
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
