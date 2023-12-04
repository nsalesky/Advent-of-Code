use std::collections::HashSet;
use anyhow::Result;
use regex::{Regex, Captures};

const CARD_RE: &'static str = r"Card +\d+: ([\d ]+) +\| +([\d ]+)";

fn card_value(cap: Captures) -> u32 {
    assert_eq!(cap.len(), 3);

    let winning_numbers: HashSet<u32> = cap
        .get(1).expect("card text contains winning numbers")
        .as_str()
        .trim()
        .split(' ')
        .filter(|num_text| num_text.trim().len() > 0)
        .map(|num_text| num_text.parse::<u32>().expect("winning number is an integer"))
        .collect();

    let my_numbers = cap
        .get(2).expect("card text contains my numbers")
        .as_str()
        .trim()
        .split(' ')
        .filter(|num_text| num_text.trim().len() > 0)
        .map(|num_text| num_text.parse::<u32>().expect("my number is an integer"));

    let num_winners = my_numbers
        .filter(|num| winning_numbers.contains(num))
        .count() as u32;

    if num_winners == 0 {
        return 0;
    }

    2u32.pow(num_winners - 1)
}

pub fn process(input: &str) -> Result<String> {
    let card_re = Regex::new(CARD_RE)?;

    let result = card_re
        .captures_iter(input)
        .map(card_value)
        .fold(0, |acc, elem| acc + elem);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> Result<()> {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    #[case("Card   1: 61 73 92 28 96 76 32 62 44 53 | 61 17 26 13 92  5 73 29 53 42 62 46 96 32 21 97 99 28 12  4  7 44 19 71 76", 512)]
    fn test_card_value(#[case] card_text: &str, #[case] expected: u32) {
        let card_re = Regex::new(CARD_RE).expect("regex is valid");
        let cap = card_re.captures(card_text).expect("there is a capture");
        assert_eq!(card_value(cap), expected);
    }
}