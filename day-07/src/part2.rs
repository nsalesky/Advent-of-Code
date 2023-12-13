use std::cmp::max;
use std::collections::HashMap;
use anyhow::Result;

#[derive(PartialOrd, PartialEq, Eq, Ord, Hash, Debug)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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
        let hand_without_jokers = HandType::from_str_no_jokers(cards_text)
            .expect("line converts to a valid hand");
        let hand_with_jokers = HandType::from_str_with_jokers(cards_text);

        match hand_with_jokers {
            Some(hand_with_jokers) => Some(max(hand_with_jokers, hand_without_jokers)),
            None => Some(hand_without_jokers),
        }
    }

    fn from_str_with_jokers(cards_text: &str) -> Option<Self> {
        if cards_text.len() != 5 {
            return None;
        }

        let cards = cards_text
            .chars()
            .take(5)
            .map(|c| Card::from_char(c).expect("char converts to a valid card"))
            .collect::<Vec<Card>>()
            .try_into()
            .expect("cards converts to a 5-card array");

        let mut card_amounts: HashMap<char, u32> = HashMap::new();
        let mut num_jokers = 0;
        for c in cards_text.chars() {
            if c == 'J' {
                num_jokers += 1;
            } else if let Some(prev_amount) = card_amounts.get(&c) {
                card_amounts.insert(c, prev_amount + 1);
            } else {
                card_amounts.insert(c, 1);
            }
        }

        if num_jokers == 5 {
            return None;
        }

        let mut sorted_amounts: Vec<(char, u32)> = card_amounts
            .iter()
            .map(|(c, num)| (*c, *num))
            .collect();
        sorted_amounts.sort_by(|(_, num_a), (_, num_b)| num_b.cmp(num_a));

        return if sorted_amounts.len() == 1 || sorted_amounts[0].1 + num_jokers >= 5 {
            Some(HandType::FiveOfAKind(cards))
        } else if sorted_amounts[0].1 == 4 || (sorted_amounts[0].1 + num_jokers >= 4) {
            Some(HandType::FourOfAKind(cards))
        } else if sorted_amounts.len() == 2 || (sorted_amounts.len() == 2 && num_jokers > 0) {
            Some(HandType::FullHouse(cards))
        } else if sorted_amounts[0].1 == 3 || (sorted_amounts[0].0 != 'J' && sorted_amounts[0].1 + num_jokers >= 3) {
            Some(HandType::ThreeOfAKind(cards))
        } else if sorted_amounts[0].1 == 2 && sorted_amounts[1].1 == 2 || num_jokers >= 3 || (sorted_amounts[0].0 != 'J' && sorted_amounts[0].1 == 2 && num_jokers >= 1) {
            Some(HandType::TwoPair(cards))
        } else if sorted_amounts[0].1 == 2 || num_jokers > 0 {
            Some(HandType::OnePair(cards))
        } else {
            Some(HandType::HighCard(cards))
        }
    }

    fn from_str_no_jokers(cards_text: &str) -> Option<Self> {
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
    use rstest::rstest;
    use HandType::*;
    use Card::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }


    #[rstest]
    #[case("32T3K 765", OnePair([Three, Two, Ten, Three, King]), 765)]
    #[case("T55J5 684", FourOfAKind([Ten, Five, Five, Jack, Five]), 684)]
    #[case("KK677 28", TwoPair([King, King, Six, Seven, Seven]), 28)]
    #[case("KTJJT 220", FourOfAKind([King, Ten, Jack, Jack, Ten]), 220)]
    #[case("QQQJA 483", FourOfAKind([Queen, Queen, Queen, Jack, Ace]), 483)]
    #[case("224J3 101", ThreeOfAKind([Two, Two, Four, Jack, Three]), 101)]
    #[case("9876J 50", OnePair([Nine, Eight, Seven, Six, Jack]), 50)]
    fn test_parse_hand(#[case] input: &str,
                       #[case] hand_type: HandType,
                       #[case] bid_amount: u32
    ) -> Result<()> {
        assert_eq!(Hand::from_str(input),
        Hand {
            hand_type,
            bid_amount,
        });
        Ok(())
    }
}
