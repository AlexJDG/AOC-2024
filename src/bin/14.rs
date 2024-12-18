use regex::Regex;
use std::collections::HashSet;
use std::io;
use std::io::{stdout, Write};

advent_of_code::solution!(14);

type Robot = ((i32, i32), (i32, i32));

fn parse(input: &str) -> (i32, i32, Vec<Robot>) {
    let (first, rest) = input.split_once("\n").unwrap();
    let (w_str, h_str) = first.split_once(",").unwrap();

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    (
        w_str.parse::<i32>().unwrap(),
        h_str.parse::<i32>().unwrap(),
        re.captures_iter(rest)
            .map(|c| c.extract().1.map(|el| el.parse::<i32>().unwrap()))
            .map(|[x, y, v_x, v_y]| ((x, y), (v_x, v_y)))
            .collect(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    const T: i32 = 100;
    let (w, h, robots) = parse(input);

    let mut quadrants: Vec<u32> = vec![0, 0, 0, 0];
    for (x, y) in robots
        .iter()
        .map(|((x, y), (v_x, v_y))| {
            let x_raw = ((x + v_x * T) + 1) % w;
            let y_raw = ((y + v_y * T) + 1) % h;

            (
                if x_raw > 0 { x_raw } else { w + x_raw } - 1,
                if y_raw > 0 { y_raw } else { h + y_raw } - 1,
            )
        })
        .filter(|&(x, y)| x != (w - 1) / 2 && y != (h - 1) / 2)
    {
        if y < (h + 1) / 2 {
            if x < (w + 1) / 2 {
                quadrants[0] += 1;
            } else {
                quadrants[1] += 1;
            }
        } else if x < (w + 1) / 2 {
            quadrants[2] += 1;
        } else {
            quadrants[3] += 1;
        }
    }

    Some(quadrants.iter().fold(1, |acc, el| acc * *el))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (w, h, robots) = parse(input);

    let stdout = stdout().lock();
    let mut buf = io::BufWriter::with_capacity((w * h * 100) as usize, stdout);

    print!("{}", termion::clear::All);
    for i in 7286..7287 {
        let map = robots
            .iter()
            .map(|&((x, y), (v_x, v_y))| {
                let x_raw = ((x + v_x * i) + 1) % w;
                let y_raw = ((y + v_y * i) + 1) % h;

                (
                    if x_raw > 0 { x_raw } else { w + x_raw } - 1,
                    if y_raw > 0 { y_raw } else { h + y_raw } - 1,
                )
            })
            .collect::<HashSet<_>>();

        let pixel_matrix = String::from("")
            + "Seconds elapsed: "
            + &*i.to_string()
            + "\n\r"
            + &*(0..w)
                .map(|x| {
                    (0..h)
                        .map(|y| if map.contains(&(x, y)) { "O" } else { "." })
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("\n\r");

        write!(buf, "{}{}", termion::cursor::Goto(1, 1), pixel_matrix).unwrap();

        buf.flush().expect("Can't write to stdout!");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
