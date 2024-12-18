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

fn walk_plot(
    (start_y, start_x): (i32, i32),
    garden: &Garden,
    visited: &mut HashSet<(i32, i32)>,
    corners: &Option<&mut HashSet<(i32, i32)>>,
) -> (u32, u32) {
    let mut area = 1u32;
    let mut perimeter = 4u32;
    let crop = garden[start_y as usize][start_x as usize];
    visited.insert((start_y, start_x));

    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .filter_map(|(y_offset, x_offset)| {
            let point = (start_y + y_offset, start_x + x_offset);
            get_crop(point, garden).map(|neighbour_crop| (point, neighbour_crop))
        })
        .filter(|(_, neighbour_crop)| *neighbour_crop == crop)
        .for_each(|(point, _)| {
            if !visited.contains(&point) {
                visited.insert(point);

                let (n_area, n_perimeter) = walk_plot(point, garden, visited, corners);
                area += n_area;
                perimeter += n_perimeter;
            }
            perimeter -= 1;
        });

    (area, perimeter)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut tot = 0u32;

    let garden = parse(input);

    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            if !visited.contains(&(y as i32, x as i32)) {
                let (plot_area, plot_perimeter) =
                    walk_plot((y as i32, x as i32), &garden, &mut visited, &None);

                tot += plot_area * plot_perimeter;
            }
        }
    }

    Some(tot)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
