use std::collections::HashMap;
use std::str::FromStr;
use anyhow::Result;
use regex::Regex;

fn is_game_valid(game: &str) -> bool {
    let color_amounts: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    for round in game.split(";") {
        for color_info in round.split(",") {
            let parts: Vec<&str> = color_info.trim().split(" ").collect();
            let amount: u32 = parts[0].trim().parse().expect("amount is an integer");
            let color = parts[1].trim();

            if &amount > color_amounts.get(color).expect("color is one of red, green, or blue") {
                return false;
            }
        }
    }

    true
}

pub fn process(input: &str) -> Result<String> {
    let re = Regex::new(r"Game (\d+): (.+)")?;

    let mut result = 0;
    for line in input.lines() {
        let caps = re.captures(line).expect("line matches the expected format");
        let id: u32 = caps[1].parse().expect("id is an integer");
        let game = &caps[2];

        if is_game_valid(game) {
            result += id;
        }
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

        assert_eq!("8", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", true)]
    #[case("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", false)]
    #[case("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", false)]
    #[case("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_is_game_valid(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_game_valid(input), expected);
    }
}