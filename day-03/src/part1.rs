use std::cmp::{max, min};
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

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

pub fn process(input: &str) -> Result<String> {
    let mut result: u32 = 0;

    let line_chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let numbers_regex = Regex::new(r"(\d+)")?;

    for (i, line) in input.lines().enumerate() {
        numbers_regex.find_iter(line).filter_map(|re_match| {
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


            let mut possible_symbols = Vec::with_capacity(6 + 3 * line_length);

            let num_skip = max(start_index as i32 - 1, 0) as usize;
            let num_take = end_index + 1 - num_skip;
            if start_index > 0 {
                possible_symbols.push(line_chars[i][start_index - 1]);
            }
            if end_index < line_length {
                possible_symbols.push(line_chars[i][end_index]);
            }
            if i > 0 {
                possible_symbols.extend(
                    line_chars[i - 1]
                        .iter()
                        .skip(num_skip)
                        .take(num_take));
            }
            if i < line_chars.len() - 1 {
                possible_symbols.extend(
                    line_chars[i + 1]
                        .iter()
                        .skip(num_skip)
                        .take(num_take));
            }

            if
            possible_symbols
                .iter()
                .any(|c| is_symbol(*c)) {
                return Some(re_match.as_str().parse::<u32>().expect("regex match is an integer"));
            }
            None
        })
            .for_each(|i| result += i);
    }

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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}