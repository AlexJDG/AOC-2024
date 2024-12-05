use std::fmt::Debug;
use std::ops::Add;
use std::ops::Mul;

advent_of_code::solution!(4);

#[derive(Copy, Clone)]
struct V2D<T>(T, T)
where
    T: Copy + Clone;

impl<T, U> Add<V2D<U>> for V2D<T>
where
    T: Add<T, Output = T> + TryFrom<U> + Copy,
    <T as TryFrom<U>>::Error: Debug,
    U: Copy + Clone,
{
    type Output = V2D<T>;

    fn add(self, rhs: V2D<U>) -> V2D<T> {
        V2D(
            self.0 + T::try_from(rhs.0).unwrap(),
            self.1 + T::try_from(rhs.1).unwrap(),
        )
    }
}

impl<T, U> Mul<U> for V2D<T>
where
    T: Mul<T, Output = T> + TryFrom<U> + Copy + Clone,
    <T as TryFrom<U>>::Error: Debug,
    U: Copy + Clone,
{
    type Output = V2D<T>;

    fn mul(self, rhs: U) -> V2D<T> {
        V2D(
            self.0 * T::try_from(rhs).unwrap(),
            self.1 * T::try_from(rhs).unwrap(),
        )
    }
}

fn wordsearch_to_vec(wordsearch: &str) -> Vec<Vec<&str>> {
    wordsearch
        .lines()
        .map(|str| str.split("").filter(|c| c != &"").collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn get_char_positions(lines: &Vec<Vec<&str>>, search_char: &str) -> Vec<V2D<usize>> {
    let mut word_starts = Vec::new();

    lines.iter().enumerate().for_each(|(line_index, line)| {
        line.iter().enumerate().for_each(|(char_index, char)| {
            if char == &search_char {
                word_starts.push(V2D(line_index, char_index));
            }
        })
    });

    word_starts
}

fn is_point_in_wordsearch(ws: &Vec<Vec<&str>>, &V2D(y, x): &V2D<i32>) -> bool {
    x >= 0 && y >= 0 && y < (ws.len() as i32) && x < (ws[y as usize].len() as i32)
}

pub fn part_one(input: &str) -> Option<u32> {
    const DIR_OFFSETS: [V2D<i32>; 8] = [
        V2D(-1, 0),  // N
        V2D(-1, 1),  // NE
        V2D(0, 1),   // E
        V2D(1, 1),   // SE
        V2D(1, 0),   // S
        V2D(1, -1),  // SW
        V2D(0, -1),  // W
        V2D(-1, -1), // NW
    ];

    let word_chunks = "XMAS".split("").filter(|c| *c != "").collect::<Vec<_>>();
    let (first_char, rest_of_word) = word_chunks.split_first().unwrap();

    let lines = wordsearch_to_vec(input);
    let word_starts: Vec<V2D<usize>> = get_char_positions(&lines, first_char);

    Some(
        word_starts
            .iter()
            .map(|point| {
                DIR_OFFSETS
                    .iter()
                    .filter(|&offset| {
                        rest_of_word.iter().enumerate().all(|(char_index, char)| {
                            let offset_point = *offset * (char_index + 1) + *point;
                            is_point_in_wordsearch(&lines, &offset_point)
                                && lines[offset_point.0 as usize][offset_point.1 as usize] == *char
                        })
                    })
                    .count() as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    const CHAR_POSITIONS: [V2D<i32>; 4] = [
        V2D(1, 1),   // BR
        V2D(-1, -1), // TL
        V2D(-1, 1),  // TR
        V2D(1, -1),  // BL
    ];

    let lines = wordsearch_to_vec(input);
    let core_positions = get_char_positions(&lines, "A");

    Some(
        core_positions
            .iter()
            .filter(|&point| {
                let letters = CHAR_POSITIONS
                    .iter()
                    .map(|&offset| {
                        let offset_point = offset + *point;
                        if is_point_in_wordsearch(&lines, &offset_point) {
                            return lines[offset_point.0 as usize][offset_point.1 as usize];
                        }
                        "X"
                    })
                    .collect::<Vec<_>>();

                letters.iter().filter(|&&letter| letter == "S").count() == 2
                    && letters.iter().filter(|&&letter| letter == "M").count() == 2
                    && *letters[0] != *letters[1]
                    && *letters[2] != *letters[3]
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
