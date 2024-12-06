use std::collections::{HashMap, HashSet};
use std::iter::Map;
use std::num::ParseIntError;
use std::str::Lines;

advent_of_code::solution!(5);

type RulesMap<'a> = HashMap<&'a str, HashSet<&'a str>>;
type ReportLinesIterator<'a> = Map<Lines<'a>, fn(&str) -> Vec<&str>>;

fn parse(input: &str) -> Option<(RulesMap, ReportLinesIterator)> {
    if let Some((ordering_rules, page_lists)) = input.split_once("\n\n") {
        Some((
            ordering_rules
                .lines()
                .fold(HashMap::new(), |mut map: RulesMap, rule| {
                    let (before, after) = rule.split_once("|").unwrap();

                    map.entry(before).or_default().insert(after);
                    map
                }),
            page_lists
                .lines()
                .map(|line| line.split(",").collect::<Vec<_>>()),
        ))
    } else {
        None
    }
}

fn is_report_valid(report: &Vec<&str>, rules_map: &RulesMap) -> bool {
    report.iter().enumerate().all(|(idx, char)| {
        if let Some(prerequisites) = rules_map.get(char) {
            return !report[..idx]
                .iter()
                .any(|prev_item| prerequisites.contains(prev_item));
        }
        true
    })
}

fn get_middle_page(report: &[&str]) -> Result<u32, ParseIntError> {
    report.get(report.len() / 2).unwrap_or(&"0").parse::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    if let Some((rules_map, reports)) = parse(input) {
        Some(
            reports
                .filter(|report| is_report_valid(report, &rules_map))
                .map(|report| get_middle_page(&report).unwrap_or(0))
                .sum(),
        )
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    if let Some((rules_map, reports)) = parse(input) {
        Some(
            reports
                .filter(|report| !is_report_valid(report, &rules_map))
                .map(|report| {
                    let mut filtered_rules = rules_map
                        .iter()
                        .filter(|(before_page, _)| report.contains(before_page))
                        .collect::<HashMap<_, _>>();

                    let res = (0..(filtered_rules.len() - 1))
                        .map(|_| {
                            let (&next_page, _) = &filtered_rules
                                .iter()
                                .find(|(_, prerequisite_pages)| {
                                    !prerequisite_pages
                                        .iter()
                                        .any(|prereq| filtered_rules.contains_key(prereq))
                                })
                                .unwrap();

                            filtered_rules.remove(next_page);
                            *next_page
                        })
                        .rev()
                        .collect::<Vec<_>>();
                    res
                })
                .map(|report| get_middle_page(&report).unwrap_or(0))
                .sum(),
        )
    } else {
        None
    }
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
        assert_eq!(result, Some(123));
    }
}
