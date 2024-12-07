advent_of_code::solution!(7);

enum Operator {
    Add,
    Mul,
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (sum_str, vals_str) = line.split_once(":").unwrap();
            (
                sum_str.parse::<u64>().unwrap(),
                vals_str
                    .split(" ")
                    .filter(|el| !el.is_empty())
                    .map(|num_str| num_str.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let operators = [Operator::Add, Operator::Mul];
    Some(
        parse(input)
            .iter()
            .filter(|(target, nums)| {
                let (&first, rest) = nums.split_first().unwrap();
                rest.iter()
                    .fold(Vec::from([first]), |combos, num| {
                        combos
                            .iter()
                            .flat_map(|combo| {
                                operators
                                    .iter()
                                    .map(|op| match op {
                                        Operator::Add => combo + num,
                                        Operator::Mul => combo * num,
                                    })
                                    .collect::<Vec<u64>>()
                            })
                            .filter(|combo| combo <= target)
                            .collect()
                    })
                    .iter()
                    .any(|val| val == target)
            })
            .map(|(target, _)| target)
            .sum::<u64>(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
