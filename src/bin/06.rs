use std::collections::HashSet;

advent_of_code::solution!(6);

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn parse(input: &str) -> (Vec<Vec<bool>>, (i32, i32)) {
    let mut pos = (0, 0);

    (
        input
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .map(|(y, line)| {
                line.split("")
                    .filter(|char| !char.is_empty())
                    .enumerate()
                    .map(|(x, char)| {
                        if char == "#" {
                            return true;
                        }

                        if char == "^" {
                            pos = (y as i32, x as i32);
                        }

                        false
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        pos,
    )
}

fn is_point_in_map(map: &[Vec<bool>], (y, x): (i32, i32)) -> bool {
    x >= 0 && y >= 0 && y < (map.len() as i32) && x < (map[y as usize].len() as i32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut pos) = parse(input);
    let mut dir = Dir::Up;
    let mut positions = HashSet::new();

    while is_point_in_map(&map, pos) {
        positions.insert(pos);

        let (yn, xn) = match dir {
            Dir::Up => (pos.0 - 1, pos.1),
            Dir::Right => (pos.0, pos.1 + 1),
            Dir::Down => (pos.0 + 1, pos.1),
            Dir::Left => (pos.0, pos.1 - 1),
        };

        if is_point_in_map(&map, (yn, xn))
            && *map.get(yn as usize).unwrap().get(xn as usize).unwrap()
        {
            dir = match dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            }
        } else {
            pos = (yn, xn);
        }
    }

    u32::try_from(positions.len()).ok()
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
