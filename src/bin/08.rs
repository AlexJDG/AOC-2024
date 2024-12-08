use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split("")
                .filter(|char| !char.is_empty())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn into_hashmap<'a>(map: &[Vec<&'a str>]) -> HashMap<&'a str, Vec<(i32, i32)>> {
    map.iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (y, row)| {
            row.iter().enumerate().for_each(|(x, &char)| {
                if char != "." {
                    acc.entry(char).or_default().push((y as i32, x as i32))
                }
            });
            acc
        })
}

fn is_point_in_grid<T>(grid: &[Vec<T>], (y, x): (i32, i32)) -> bool {
    x >= 0 && y >= 0 && y < (grid.len() as i32) && x < (grid[y as usize].len() as i32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let map = into_hashmap(&grid);

    let antinodes = map
        .iter()
        .fold(HashSet::new(), |mut antinodes, (_, antennae)| {
            antennae[..antennae.len() - 1]
                .iter()
                .enumerate()
                .for_each(|(i, (ay, ax))| {
                    antennae[i + 1..antennae.len()].iter().for_each(|(by, bx)| {
                        let (dy, dx) = (ay - by, ax - bx);

                        let antinode_a = (ay + dy, ax + dx);
                        let antinode_b = (by - dy, bx - dx);

                        if is_point_in_grid(&grid, antinode_a) {
                            antinodes.insert(antinode_a);
                        }

                        if is_point_in_grid(&grid, antinode_b) {
                            antinodes.insert(antinode_b);
                        }
                    });
                });

            antinodes
        });

    dbg!(&grid.iter().map(|row| row.join("")).collect::<Vec<_>>());

    dbg!(&grid
        .iter()
        .enumerate()
        .map(|(y, row)| row
            .iter()
            .enumerate()
            .map(|(x, cell)| if antinodes.contains(&(y as i32, x as i32)) {
                "#"
            } else {
                cell
            })
            .collect::<Vec<_>>()
            .join(""))
        .collect::<Vec<_>>());

    Some(antinodes.len() as u32)
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
