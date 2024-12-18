use regex::Regex;
use std::cmp::min;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

fn push_exhaustively(
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
    max: Option<u64>,
) -> Option<(u64, u64)> {
    let a_max = *[prize.0 / a.0, prize.1 / a.1, max.unwrap_or(u64::MAX)]
        .iter()
        .min()
        .unwrap();

    for a_qty in (0..a_max).rev() {
        // dbg!(a_qty, a_max, machine);
        let remaining = (prize.0 - a_qty * a.0, prize.1 - a_qty * a.1);

        // dbg!(a_max);

        if remaining.0 / b.0 == remaining.1 / b.1
            && remaining.0 % b.0 == 0
            && remaining.1 % b.1 == 0
        {
            return Some((a_qty, remaining.0 / b.0));
        }
    }

    None
}

fn parse(input: &str) -> Vec<Machine> {
    let re = Regex::new(r"\D*(\d+)\D*(\d+)\s\D+(\d+)\D*(\d+)\s\D+(\d+)\D+(\d+)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract().1.map(|el| el.parse::<u64>().unwrap()))
        .map(|[a_x, a_y, b_x, b_y, p_x, p_y]| Machine {
            a: (a_x, a_y),
            b: (b_x, b_y),
            prize: (p_x, p_y),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse(input);

    Some(
        machines
            .iter()
            .filter_map(|&Machine { a, b, prize }| {
                let a_first = push_exhaustively(a, b, prize, Some(100))
                    .map(|(a_presses, b_presses)| a_presses * 3 + b_presses);
                let b_first = push_exhaustively(b, a, prize, Some(100))
                    .map(|(b_presses, a_presses)| a_presses * 3 + b_presses);

                if a_first.is_some() && b_first.is_some() {
                    Some(min(a_first?, b_first?))
                } else {
                    a_first.or(b_first)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    const OFFSET: u64 = 10000000000000;
    let machines = parse(input);

    Some(
        machines
            .iter()
            .filter_map(
                |Machine {
                     a,
                     b,
                     prize: (px, py),
                 }| {
                    let d = (a.0 * b.1) as i64 - (a.1 * b.0) as i64;
                    let a_presses =
                        (((px + OFFSET) * b.1) as i64 - ((py + OFFSET) * b.0) as i64) / d;
                    let b_presses =
                        ((a.0 * (py + OFFSET)) as i64 - (a.1 * (px + OFFSET)) as i64) / d;

                    if (
                        (a.0 as i64 * a_presses + b.0 as i64 * b_presses) as u64,
                        (a.1 as i64 * a_presses + b.1 as i64 * b_presses) as u64,
                    ) == (px + OFFSET, py + OFFSET)
                    {
                        Some((a_presses * 3 + b_presses) as u64)
                    } else {
                        None
                    }
                },
            )
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(37686));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
