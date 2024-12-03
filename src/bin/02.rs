advent_of_code::solution!(2);

fn split_report(report: &str) -> Vec<&str> {
    report.split(" ").collect()
}

fn check_levels(levels: Vec<&str>) -> bool {
    let (first, rest) = levels.split_first().unwrap();

    let mut last = first.parse::<i32>().unwrap();
    let mut is_ascending = None;

    rest.iter().all(|el| {
        let diff = last - el.parse::<i32>().unwrap();
        last -= diff;
        is_ascending = is_ascending.or(Some(diff > 0));

        (is_ascending.unwrap() == (diff > 0)) && (1..=3).contains(&diff.abs())
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| check_levels(split_report(line)))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let levels = split_report(line);

                (0..levels.len())
                    .map(|i| [&levels[..i], &levels[i + 1..]].concat())
                    .any(check_levels)
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
