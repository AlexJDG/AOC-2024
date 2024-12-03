advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| {
        let num_strings: Vec<_> = line.split(" ").collect();
        let (first, rest) = num_strings.split_first().unwrap();

        let mut last = first.parse::<i32>().unwrap();
        let mut sign = None;

        for el in rest.iter() {
            let current = el.parse::<i32>().unwrap();

            let diff: i32 = last - current;

            if sign.is_none() {
                sign = Some(diff > 0);
            }

            if (sign.unwrap() != (diff > 0)) || !(1..=3).contains(&diff.unsigned_abs()) {
                return acc;
            }

            last = current
        }

        return acc + 1;
    }))
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
