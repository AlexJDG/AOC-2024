use std::collections::HashSet;

advent_of_code::solution!(10);

type TopoMap = Vec<Vec<u8>>;
type Point = (usize, usize);

fn parse(input: &str) -> TopoMap {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|char| !char.is_empty())
                .map(|char| char.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn find_trailheads(map: &TopoMap) -> Vec<Point> {
    map.iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                if *cell == 0 {
                    acc.push((y, x))
                }
            });
            acc
        })
}

fn find_neighbours(map: &TopoMap, (py, px): Point, term: u8) -> Vec<Point> {
    let mut valid_neighbours = Vec::new();

    if py > 0 && map[py - 1][px] == term {
        // Up
        valid_neighbours.push((py - 1, px));
    }

    if py < map.len() - 1 && map[py + 1][px] == term {
        // Down
        valid_neighbours.push((py + 1, px));
    }

    if px > 0 && map[py][px - 1] == term {
        // Left
        valid_neighbours.push((py, px - 1));
    }

    if px < map[py].len() - 1 && map[py][px + 1] == term {
        // Right
        valid_neighbours.push((py, px + 1));
    }

    valid_neighbours
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);

    let trailheads = find_trailheads(&map);

    Some(trailheads.iter().fold(0u32, |acc, trailhead| {
        let mut points: Vec<Point> = vec![*trailhead];

        for i in 1..=9 {
            points = points
                .iter()
                .flat_map(|point| find_neighbours(&map, *point, i))
                .collect()
        }

        acc + HashSet::<Point>::from_iter(points).len() as u32
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
