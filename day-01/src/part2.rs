use std::collections::HashMap;
use anyhow::Result;
use anyhow::format_err;

pub fn process(input: &str) -> Result<String> {
    let mut result = 0;

    for line in input.lines() {
        result += parse_line_with_words(line)?;
    }

    Ok(result.to_string())
}

fn parse_line_with_words(line: &str) -> Result<u32> {
    let words: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut digits = vec![];

    for (i, c) in line.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            digits.push(digit);
        } else {
            for j in i+1..=line.len() {
                let subword = &line[i..j];
                if let Some(digit) = words.get(subword) {
                    digits.push(*digit);
                }
            }
        }
    }

    let mut digits = digits.into_iter();
    let first_digit = digits.nth(0).ok_or(format_err!("no digits"))?;
    let last_digit = digits.nth_back(0).unwrap_or(first_digit);
    let val = format!("{}{}", first_digit, last_digit);
    Ok(u32::from_str_radix(&val, 10)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
