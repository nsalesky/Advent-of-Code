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

    let mut curr_locations: Vec<&str> = locations
        .iter()
        .filter(|(location, _)| location.ends_with('A'))
        .map(|(location, _)| location.as_str())
        .collect();

    let mut num_moves = 0;
    for instruction in instructions_text.chars().cycle() {
        num_moves += 1;

        for i in 0..curr_locations.len() {
            let current_location = curr_locations[i];
            let left_right_options = locations.get(current_location)
                .ok_or(ProcessError::UnknownLocation(current_location.to_string()))?;

            if instruction == 'L' {
                curr_locations[i] = &left_right_options.0;
            } else {
                curr_locations[i] = &left_right_options.1;
            }

        }

        if curr_locations.iter()
            .all(|loc| loc.ends_with('Z')) {
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
    fn test_process() -> Result<()> {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
