use std::collections::HashMap;
use anyhow::Result;

#[derive(PartialOrd, PartialEq, Eq, Ord, Hash, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug)]
enum HandType {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfAKind([Card; 5]),
    FiveOfAKind([Card; 5]),
}

impl HandType {
    fn from_str(cards_text: &str) -> Option<Self> {
        if cards_text.len() != 5 {
            return None;
        }

        let cards= cards_text
            .chars()
            .take(5)
            .map(|c| Card::from_char(c).expect("char converts to a valid card"))
            .collect::<Vec<Card>>()
            .try_into()
            .expect("cards converts to a 5-card array");

        let mut card_amounts: HashMap<char, u32> = HashMap::new();
        for c in cards_text.chars() {
            if let Some(prev_amount) = card_amounts.get(&c) {
                card_amounts.insert(c, prev_amount + 1);
            } else {
                card_amounts.insert(c, 1);
            }
        }

        let mut sorted_amounts: Vec<u32> = card_amounts
            .iter()
            .map(|(_, amount)| *amount)
            .collect();
        sorted_amounts.sort_by(|a, b| b.cmp(a));

        return if sorted_amounts.len() == 1 {
            Some(HandType::FiveOfAKind(cards))
        } else if sorted_amounts.len() == 2 {
            if sorted_amounts[0] == 4 {
                Some(HandType::FourOfAKind(cards))
            } else {
                Some(HandType::FullHouse(cards))
            }
        } else if sorted_amounts.len() == 3 {
            if sorted_amounts[0] == 3 {
                Some(HandType::ThreeOfAKind(cards))
            } else {
                Some(HandType::TwoPair(cards))
            }
        } else if sorted_amounts.len() == 4 {
            Some(HandType::OnePair(cards))
        } else {
            Some(HandType::HighCard(cards))
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug)]
struct Hand {
    hand_type: HandType,
    bid_amount: u32,
}

impl Hand {
    fn from_str(text: &str) -> Self {
        let mut parts = text.split(' ');

        let cards_text = parts.next().expect("a hand should contain cards");
        let bid_text = parts.next().expect("a hand should contain a bid amount");

        Self {
            hand_type: HandType::from_str(cards_text).expect("cards text should be a valid hand"),
            bid_amount: bid_text.parse().expect("bid amount should be an integer"),
        }
    }
}

fn total_winnings(mut hands: Vec<Hand>) -> u32 {
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid_amount)
        .fold(0, |a, b| a + b)
}

pub fn process(input: &str) -> Result<String> {
    let total_winnings = total_winnings(
        input
        .lines()
        .map(|line| Hand::from_str(line))
            .collect());

    Ok(total_winnings.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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


    // #[rstest]
    // #[case("32T3K 765", HandType::OnePair, 765)]
    // #[case("T55J5 684", HandType::ThreeOfAKind, 684)]
    // #[case("KK677 28", HandType::TwoPair, 28)]
    // #[case("KTJJT 220", HandType::TwoPair, 220)]
    // #[case("QQQJA 483", HandType::ThreeOfAKind, 483)]
    // fn text_parse_hand(#[case] input: &str,
    //                    #[case] hand_type: HandType,
    //                    #[case] bid_amount: u32
    // ) -> Result<()> {
    //     assert_eq!(Hand::from_str(input)?,
    //     Hand {
    //         hand_type,
    //         card_text: card_text.to_owned(),
    //         bid_amount,
    //     });
    //     Ok(())
    // }

    #[test]
    fn test_parse_hand() {
        let expected = Hand {
            hand_type: HandType::ThreeOfAKind([
                Card::Queen,
                Card::Queen,
                Card::Queen,
                Card::Jack,
                Card::Ace,
            ]),
            bid_amount: 483,
        };

        assert_eq!(Hand::from_str("QQQJA 483"), expected);
    }
}