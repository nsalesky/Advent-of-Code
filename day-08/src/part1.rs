use std::collections::HashMap;
use itertools::Itertools;
use thiserror::Error;
use regex::Regex;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("no left/right instructions included in the input")]
    MissingInstructions,

    #[error("no locations were included in the input")]
    MissingLocations,

    #[error("invalid location line: `{0}`")]
    InvalidLocation(String),

    #[error("location `{0}` is not present in the map")]
    UnknownLocation(String),

    #[error("invalid regex")]
    RegexError(#[from] regex::Error),
}

fn parse_location_line(location_line: &str) -> Result<(String, (String, String)), ProcessError> {
    let parse_re = Regex::new(r"(.+) = \((.+), (.+)\)")?;
    let caps = parse_re.captures(location_line)
        .ok_or(ProcessError::InvalidLocation(location_line.to_string()))?;

    if caps.len() != 4 {
        return Err(ProcessError::InvalidLocation(location_line.to_string()));
    }

    Ok((caps[1].to_string(), (caps[2].to_string(), caps[3].to_string())))
}

pub fn process(input: &str) -> Result<String, ProcessError> {
    let mut input_parts = input.split("\n\n");

    let instructions_text = input_parts.next()
        .ok_or(ProcessError::MissingInstructions)?;

    let locations_text = input_parts.next()
        .ok_or(ProcessError::MissingLocations)?;

    let locations: HashMap<String, (String, String)> = locations_text
        .lines()
        .map(|location_line| parse_location_line(location_line))
        .fold_ok(HashMap::new(), |mut map, (location, left_right_options)| {
            map.insert(location, left_right_options);
            map
        })?;

    let mut num_moves = 0;

    let mut current_location = "AAA";
    for instruction in instructions_text.chars().cycle() {
        num_moves += 1;

        let left_right_options = locations.get(current_location)
            .ok_or(ProcessError::UnknownLocation(current_location.to_string()))?;

        if instruction == 'L' {
            current_location = &left_right_options.0;
        } else {
            current_location = &left_right_options.1;
        }

        if current_location == "ZZZ" {
            return Ok(num_moves.to_string());
        }
    }

    Ok("0".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_process_no_repeat() -> Result<()> {
        let input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_with_repeat() -> Result<()> {
        let input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}