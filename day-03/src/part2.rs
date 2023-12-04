use std::cmp::{max, min};
use std::collections::HashMap;
use anyhow::Result;
use regex::Regex;

/// Converts the given byte `offset` of the `text` to the index of the corresponding char,
/// assuming that the offset corresponds to the beginning of a char in the string.
///
/// Arguments
/// * `text` A string slice containing the char
/// * `offset` The byte offset of the beginning of the char
fn byte_offset_to_index(text: &str, offset: usize) -> Option<usize> {
    text
        .char_indices()
        .enumerate()
        .filter_map(|(index, (byte_pos, _char))| {
            if byte_pos == offset {
                return Some(index);
            }
            None
        })
        .next()
}

pub fn process(input: &str) -> Result<String> {
    let line_chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let numbers_regex = Regex::new(r"(\d+)")?;

    let mut pound_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut insert_number = |pos: (usize, usize), num: u32| {
        if let Some(numbers) = pound_numbers.get_mut(&pos) {
            numbers.push(num);
        } else {
            pound_numbers.insert(pos, Vec::from([num]));
        }
    };

    for (i, line) in input.lines().enumerate() {
        numbers_regex.find_iter(line).for_each(|re_match| {
            let num: u32 = re_match.as_str().parse().expect("regex match is a number");

            let line_length = line_chars[i].len();

            let start_index = byte_offset_to_index(line, re_match.start())
                .expect("regex start offset corresponds to a valid char");

            let end_index = {
                if re_match.end() >= line.len() {
                    line_length
                } else {
                    byte_offset_to_index(line, min(re_match.end(), line_length))
                        .expect("regex end offset corresponds to a valid char")
                }
            };

            let num_skip = max(start_index as i32 - 1, 0) as usize;
            let num_take = end_index + 1 - num_skip;
            if start_index > 0 && line_chars[i][start_index - 1] == '*' {
                insert_number((i, start_index - 1), num);
            }
            if end_index < line_length && line_chars[i][end_index] == '*' {
                insert_number((i, end_index), num);
            }
            if i > 0 {
                line_chars[i - 1]
                    .iter()
                    .enumerate()
                    .skip(num_skip)
                    .take(num_take)
                    .filter(|(_col, c)| **c == '*')
                    .for_each(|(col, _)| insert_number((i - 1, col), num));
            }
            if i < line_chars.len() - 1 {
                line_chars[i + 1]
                    .iter()
                    .enumerate()
                    .skip(num_skip)
                    .take(num_take)
                    .filter(|(_col, c)| **c == '*')
                    .for_each(|(col, _)| insert_number((i + 1, col), num));
            }
        });
    }

    let result = pound_numbers
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| {
            numbers.iter().fold(1, |a, b| a * b)
        })
        .fold(0, |a, b| a + b);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
