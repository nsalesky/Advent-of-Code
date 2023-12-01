use anyhow::Result;
use anyhow::format_err;

pub fn process(input: &str) -> Result<String> {
    let mut result = 0;

    for line in input.lines() {
        result += parse_line(line)?;
    }

    Ok(result.to_string())
}
fn parse_line(line: &str) -> Result<u32> {
    let chars = line.chars();
    let mut digits = chars
        .filter(|c| c.is_digit(10));

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
