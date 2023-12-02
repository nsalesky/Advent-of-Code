use std::collections::HashMap;
use anyhow::Result;
use regex::Regex;

fn game_power(game: &str) -> u32 {
    let mut max_counts = HashMap::new();

    for round in game.split(";") {
        for color_info in round.split(",") {
            let parts: Vec<&str> = color_info.trim().split(" ").collect();
            let amount: u32 = parts[0].trim().parse().expect("amount is an integer");
            let color = parts[1].trim();

            match max_counts.get(color) {
                Some(prevous_max) => {
                    if &amount > prevous_max {
                        max_counts.insert(color, amount);
                    }
                },
                None => {
                    max_counts.insert(color, amount);
                },
            };
        }
    }

    max_counts.values()
        .fold(1, |acc, amount| acc * amount)
}

pub fn process(input: &str) -> Result<String> {
    let re = Regex::new(r"Game (\d+): (.+)")?;

    let mut result = 0;
    for line in input.lines() {
        let caps = re.captures(line).expect("line matches the expected format");
        let game = &caps[2];
        result += game_power(game)
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> Result<()> {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("2286", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_game_power(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(game_power(input), expected);
    }
}