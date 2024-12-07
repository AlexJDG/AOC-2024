use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
    fn get_next_point(&self, (y, x): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::Up => (y - 1, x),
            Dir::Right => (y, x + 1),
            Dir::Down => (y + 1, x),
            Dir::Left => (y, x - 1),
        }
    }
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

fn walk(map: &[Vec<bool>], pos: &mut (i32, i32), dir: &mut Dir, obstacle: Option<(i32, i32)>) {
    let (yn, xn) = dir.get_next_point(*pos);

    let collides_with_obstacle = if let Some(obs) = obstacle {
        obs == (yn, xn)
    } else {
        false
    };

    if is_point_in_map(map, (yn, xn))
        && (*map.get(yn as usize).unwrap().get(xn as usize).unwrap() || collides_with_obstacle)
    {
        dir.turn_right()
    } else {
        *pos = (yn, xn);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut pos) = parse(input);
    let mut dir = Dir::Up;
    let mut positions = HashSet::new();

    while is_point_in_map(&map, pos) {
        positions.insert(pos);
        walk(&map, &mut pos, &mut dir, None);
    }

    u32::try_from(positions.len()).ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, original_pos) = parse(input);
    let mut pos = original_pos;

    let mut dir = Dir::Up;
    let mut path = HashSet::new();

    while is_point_in_map(&map, pos) {
        path.insert((pos.0, pos.1));
        walk(&map, &mut pos, &mut dir, None);
    }

    u32::try_from(
        path.iter()
            .filter(|(y, x)| {
                let mut subpath_dir = Dir::Up;
                let mut subpath_pos = original_pos;
                let mut step_set: HashSet<(i32, i32, Dir)> = HashSet::new();

                while is_point_in_map(&map, subpath_pos) {
                    if !step_set.insert((subpath_pos.0, subpath_pos.1, subpath_dir)) {
                        return true;
                    }
                    walk(&map, &mut subpath_pos, &mut subpath_dir, Some((*y, *x)));
                }

                false
            })
            .count(),
    )
    .ok()
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
        assert_eq!(result, Some(6));
    }
}
