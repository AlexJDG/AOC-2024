use std::cmp::min;

advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<i32> {
    input
        .split("")
        .filter(|char| !char.is_empty())
        .map(|el| el.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_nums = parse(input);

    let mut id_count = 0;

    let mut disk = input_nums
        .chunks(2)
        .fold(Vec::<i32>::new(), |mut disk, chunk| {
            let size = chunk[0];

            for _ in 0..size {
                disk.push(id_count);
            }

            id_count += 1;

            if let Some(padding) = chunk.get(1) {
                for _ in 0..*padding {
                    disk.push(-1);
                }
            }

            disk
        });

    let taken = disk
        .iter()
        .filter(|&&cell| cell != -1)
        .copied()
        .collect::<Vec<i32>>();

    let free = disk
        .iter()
        .enumerate()
        .filter(|(_, cell)| **cell == -1)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    for i in 0..min(free.len(), taken.len()) {
        let free_el: &mut i32 = &mut disk[free[i]];
        *free_el = taken[taken.len() - 1 - i];
    }

    Some(
        disk[0..taken.len()]
            .iter()
            .enumerate()
            .map(|(index, val)| {
                if val > &0 {
                    (*val as i64) * (index as i64)
                } else {
                    0
                }
            })
            .sum::<i64>() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
