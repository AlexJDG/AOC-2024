use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering_rules, page_lists) = input.split_once("\n\n").unwrap();

    let ordering_map = ordering_rules.lines().fold(
        HashMap::new(),
        |mut map: HashMap<&str, HashSet<&str>>, rule| {
            let (before, after) = rule.split_once("|").unwrap();

            map.entry(before).or_default().insert(after);
            map
        },
    );

    Some(
        page_lists
            .lines()
            .filter(|line| {
                let report_chars = line.split(",").collect::<Vec<_>>();
                report_chars.iter().enumerate().all(|(idx, char)| {
                    if let Some(prerequisites) = ordering_map.get(char) {
                        return !report_chars[..idx]
                            .iter()
                            .any(|prev_item| prerequisites.contains(prev_item));
                    }
                    true
                })
            })
            .map(|line| {
                let report_chars = line.split(",").collect::<Vec<_>>();
                report_chars
                    .get(report_chars.len() / 2)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            })
            .sum(),
    )
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
