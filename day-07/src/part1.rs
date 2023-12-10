use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

#[derive(PartialEq, PartialOrd, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Error, Debug)]
enum ParseHandTypeError {
    #[error("the input text was not 5 characters long")]
    InvalidLength(),
}

impl FromStr for HandType {
    type Err = ParseHandTypeError;

    fn from_str(card_text: &str) -> Result<Self, Self::Err> {
        if card_text.len() != 5 {
            return Err(ParseHandTypeError::InvalidLength());
        }

        let mut card_amounts: HashMap<char, u32> = HashMap::new();
        for card in card_text.chars() {
            if let Some(prev_amount) = card_amounts.get(&card) {
                card_amounts.insert(card, prev_amount + 1);
            } else {
                card_amounts.insert(card, 1);
            }
        }

        let mut sorted_amounts: Vec<u32> = card_amounts
            .iter()
            .map(|(_, amount)| *amount)
            .collect();
        sorted_amounts.sort_by(|a, b| b.cmp(a));

        return if sorted_amounts.len() == 1 {
            Ok(HandType::FiveOfAKind)
        } else if sorted_amounts.len() == 2 {
            if sorted_amounts[0] == 4 {
                Ok(HandType::FourOfAKind)
            } else {
                Ok(HandType::FullHouse)
            }
        } else if sorted_amounts.len() == 3 {
            if sorted_amounts[0] == 3 {
                Ok(HandType::ThreeOfAKind)
            } else {
                Ok(HandType::TwoPair)
            }
        } else if sorted_amounts.len() == 4 {
            Ok(HandType::OnePair)
        } else {
            Ok(HandType::HighCard)
        }
    }
}

#[derive(Error, Debug)]
enum ParseHandError {
    #[error("there was no card text")]
    NoCardText(),

    #[error("there was no bid amount")]
    NoBidAmount(),

    #[error("bid amount was not an integer")]
    BidAmountNotInteger(#[from] ParseIntError),

    #[error("card text was not a valid hand")]
    CardNotAValidHand(#[from] ParseHandTypeError),
}

#[derive(Debug)]
struct Hand {
    card_text: String,
    bid_amount: u32,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let card_text = parts.next().ok_or(ParseHandError::NoCardText())?;
        let bid_text = parts.next().ok_or(ParseHandError::NoBidAmount{})?;

        Ok(Hand{
            card_text: card_text.to_string(),
            bid_amount: bid_text.parse()?,
            hand_type: HandType::from_str(card_text)?,
        })
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.card_text == other.card_text
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(ord) => match ord {
                Ordering::Equal => Some(self.card_text.cmp(&other.card_text)),
                _ => Some(ord),
            }
            None => Some(self.card_text.cmp(&other.card_text))
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(ord) => match ord {
                Ordering::Equal => self.card_text.cmp(&other.card_text),
                _ => ord,
            }
            None => self.card_text.cmp(&other.card_text)
        }
    }
}

fn winnings_per_five_hands(hands: (Hand, Hand, Hand, Hand, Hand)) -> u32 {
    let mut hands = vec![hands.0, hands.1, hands.2, hands.3, hands.4];
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid_amount)
        .fold(0, |a, b| a + b)
}

pub fn process(input: &str) -> Result<String> {
    let total_winnings = input
        .lines()
        .map(|line| Hand::from_str(line).expect("every line is a valid hand"))
        .tuples::<(Hand, Hand, Hand, Hand, Hand)>()
        .map(winnings_per_five_hands)
        .fold(0, |a, b| a + b);

    Ok(total_winnings.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> Result<()> {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }


    #[rstest]
    #[case("32T3K 765", HandType::OnePair, "32T3K", 765)]
    #[case("T55J5 684", HandType::ThreeOfAKind, "T55J5", 684)]
    #[case("KK677 28", HandType::TwoPair, "KK677", 28)]
    #[case("KTJJT 220", HandType::TwoPair, "KTJJT", 220)]
    #[case("QQQJA 483", HandType::ThreeOfAKind, "QQQJA", 483)]
    fn text_parse_hand(#[case] input: &str,
                       #[case] hand_type: HandType,
                       #[case] card_text: &str,
                       #[case] bid_amount: u32
    ) -> Result<()> {
        assert_eq!(Hand::from_str(input)?,
        Hand {
            hand_type,
            card_text: card_text.to_owned(),
            bid_amount,
        });
        Ok(())
    }
}