use itertools::Itertools;
use regex::Regex;
use std::cmp::min;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32),
}

fn push_exhaustively(a: (u32, u32), b: (u32, u32), prize: (u32, u32)) -> Option<(u32, u32)> {
    let a_max = *[prize.0 / a.0, prize.1 / a.1, 100].iter().min().unwrap();

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
        .map(|c| c.extract().1.map(|el| el.parse::<u32>().unwrap()))
        .map(|[a_x, a_y, b_x, b_y, p_x, p_y]| Machine {
            a: (a_x, a_y),
            b: (b_x, b_y),
            prize: (p_x, p_y),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let machines = parse(input);

    Some(
        machines
            .iter()
            .filter_map(|&Machine { a, b, prize }| {
                let a_first = push_exhaustively(a, b, prize)
                    .map(|(a_presses, b_presses)| a_presses * 3 + b_presses);
                let b_first = push_exhaustively(b, a, prize)
                    .map(|(b_presses, a_presses)| a_presses * 3 + b_presses);

                if a_first.is_some() && b_first.is_some() {
                    Some(min(a_first.unwrap(), b_first.unwrap()))
                } else {
                    a_first.or(b_first)
                }
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
        assert_eq!(result, None);
    }
}
