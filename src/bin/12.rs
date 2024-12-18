use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(12);

type Garden<'a> = Vec<Vec<&'a str>>;

fn parse(input: &str) -> Garden {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|char| !char.is_empty())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn get_crop<'a>((y, x): (i32, i32), garden: &'a Garden) -> Option<&'a str> {
    if y >= 0 && (y as usize) < garden.len() && x >= 0 && (x as usize) < garden[y as usize].len() {
        Some(garden[y as usize][x as usize])
    } else {
        None
    }
}

fn count_mismatched_offsets(
    crop: &str,
    (start_y, start_x): (i32, i32),
    offsets: &[(i32, i32)],
    garden: &Garden,
) -> u32 {
    offsets
        .iter()
        .filter(|(y_offset, x_offset)| {
            let point = (start_y + y_offset, start_x + x_offset);
            get_crop(point, garden).is_some_and(|neighbour_crop| neighbour_crop != crop)
        })
        .count() as u32
}

fn sum_points(points: impl IntoIterator<Item = (i32, i32)>) -> (i32, i32) {
    points
        .into_iter()
        .fold((0, 0), |(y, x), (y_o, x_o)| (y + y_o, x + x_o))
}

fn walk_plot(
    (start_y, start_x): (i32, i32),
    garden: &Garden,
    visited: &mut HashSet<(i32, i32)>,
) -> (u32, u32, u32) {
    let mut area = 1u32;
    let mut perimeter = 4u32;
    let mut corners = 0;
    let crop = garden[start_y as usize][start_x as usize];
    visited.insert((start_y, start_x));

    let neighbour_points = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .filter_map(|(y_offset, x_offset)| {
            let point = (start_y + y_offset, start_x + x_offset);
            get_crop(point, garden)
                .map(|neighbour_crop| (point, neighbour_crop, (y_offset, x_offset)))
        })
        .filter(|(_, neighbour_crop, _)| *neighbour_crop == crop);

    for (point, _, _) in neighbour_points.clone() {
        if !visited.contains(&point) {
            visited.insert(point);

            let (n_area, n_perimeter, n_corners) = walk_plot(point, garden, visited);
            area += n_area;
            perimeter += n_perimeter;
            corners += n_corners
        }
        perimeter -= 1;
    }

    let neighbour_offsets = neighbour_points
        .map(|(_, _, (&y, &x))| (y, x))
        .collect::<HashSet<_>>();

    if neighbour_offsets.is_empty() {
        corners += 4;
    } else if neighbour_offsets.len() == 1 {
        corners += 2;
    } else if neighbour_offsets.len() == 2 {
        let sum = sum_points(neighbour_offsets);

        if sum != (0, 0) {
            corners += 1 + count_mismatched_offsets(crop, (start_y, start_x), &[sum], garden);
        }
    } else {
        corners += count_mismatched_offsets(
            crop,
            (start_y, start_x),
            &neighbour_offsets
                .into_iter()
                .combinations(2)
                .map(sum_points)
                .collect::<Vec<_>>(),
            garden,
        );
    }

    (area, perimeter, corners)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut tot = 0u32;

    let garden = parse(input);

    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            if !visited.contains(&(y as i32, x as i32)) {
                let (plot_area, plot_perimeter, _) =
                    walk_plot((y as i32, x as i32), &garden, &mut visited);
                tot += plot_area * plot_perimeter;
            }
        }
    }

    Some(tot)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut tot = 0u32;

    let garden = parse(input);

    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            if !visited.contains(&(y as i32, x as i32)) {
                let (plot_area, _, plot_corners) =
                    walk_plot((y as i32, x as i32), &garden, &mut visited);
                tot += plot_area * plot_corners;
            }
        }
    }

    Some(tot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(808796));
    }
}
