use std::collections::{HashMap, HashSet};
use std::iter::Map;
use std::str::Lines;

advent_of_code::solution!(5);

type RulesMap = HashMap<u32, HashSet<u32>>;
type ReportLinesIterator<'a> = Map<Lines<'a>, fn(&str) -> Vec<u32>>;

fn parse(input: &str) -> Option<(RulesMap, ReportLinesIterator)> {
    if let Some((ordering_rules, page_lists)) = input.split_once("\n\n") {
        Some((
            ordering_rules
                .lines()
                .map(|line| {
                    let (before, after) = line.split_once("|").unwrap();

                    let before_num = before.parse::<u32>().unwrap_or_default();
                    let after_num = after.parse::<u32>().unwrap_or_default();

                    (before_num, after_num)
                })
                .fold(
                    HashMap::new(),
                    |mut map: RulesMap, (before_num, after_num)| {
                        map.entry(before_num).or_default().insert(after_num);
                        map
                    },
                ),
            page_lists.lines().map(|line| {
                line.split(",")
                    .map(|num_string| num_string.parse::<u32>().unwrap_or_default())
                    .collect::<Vec<_>>()
            }),
        ))
    } else {
        None
    }
}

fn is_report_valid(report: &[u32], rules_map: &RulesMap) -> bool {
    report.iter().enumerate().all(|(idx, char)| {
        if let Some(prerequisites) = rules_map.get(char) {
            return !report[..idx]
                .iter()
                .any(|prev_item| prerequisites.contains(prev_item));
        }
        true
    })
}

fn get_middle_page(report: Vec<u32>) -> u32 {
    *report.get(report.len() / 2).unwrap_or(&0u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    if let Some((rules_map, reports)) = parse(input) {
        Some(
            reports
                .filter(|report| is_report_valid(report, &rules_map))
                .map(get_middle_page)
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
                .map(HashSet::from_iter)
                .map(|report_set: HashSet<u32>| {
                    let mut filtered_rules = rules_map
                        .iter()
                        .filter(|(key, _)| report_set.contains(key))
                        .collect::<HashMap<_, _>>();

                    (1..filtered_rules.len())
                        .map(|_| {
                            let requisite_set: HashSet<u32> =
                                filtered_rules.keys().map(|&&s| s).collect();
                            let (&next_page, _) = &filtered_rules
                                .iter()
                                .find(|(_, prerequisite_pages)| {
                                    prerequisite_pages.is_disjoint(&requisite_set)
                                })
                                .unwrap();

                            filtered_rules.remove(next_page);
                            *next_page
                        })
                        .rev()
                        .collect::<Vec<_>>()
                })
                .map(get_middle_page)
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
