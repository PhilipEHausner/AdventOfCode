use core::panic;

use util::read_files::read_file_as_vector;

fn main() {
    let lines = read_file_as_vector("./files/day7.txt").expect("Could not read file.");
    println!("Solution part 1: {}", solve1::solve1(&lines));
    println!("Solution part 2: {}", solve2::solve2(&lines));
}

mod solve1 {
    use std::cmp::Ordering;

    use itertools::Itertools;

    use crate::{Card, Hand, HandType};

    pub fn solve1(lines: &Vec<String>) -> i64 {
        let mut hands: Vec<Hand> = lines.iter().map(|it| Hand::new(it)).collect();
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(n, it)| (n + 1) as i64 * it.bid)
            .sum()
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            let ht1: crate::HandType = self.hand_type();
            let ht2 = other.hand_type();
            if ht1 > ht2 {
                return Ordering::Greater;
            } else if ht2 > ht1 {
                return Ordering::Less;
            }
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                if c1 > c2 {
                    return Ordering::Greater;
                } else if c2 > c1 {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Hand {
        pub fn hand_type(&self) -> HandType {
            let mut sorted_cards = self.cards.iter().collect::<Vec<&Card>>();
            sorted_cards.sort();
            let mut frequencies: Vec<usize> = sorted_cards
                .iter()
                .dedup_with_count()
                .map(|it| it.0)
                .collect();
            frequencies.sort();
            frequencies.reverse();
            match frequencies[0] {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => match frequencies[1] {
                    2 => HandType::FullHouse,
                    _ => HandType::ThreeOfAKind,
                },
                2 => match frequencies[1] {
                    2 => HandType::TwoPair,
                    _ => HandType::OnePair,
                },
                _ => HandType::HighCard,
            }
        }
    }
}

mod solve2 {
    use crate::{Card, Hand2, HandType};
    use itertools::Itertools;
    use std::cmp::Ordering;

    pub fn solve2(lines: &Vec<String>) -> i64 {
        let mut hands: Vec<Hand2> = lines.iter().map(|it| Hand2::new(it)).collect();
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(n, it)| (n + 1) as i64 * it.bid)
            .sum()
    }

    impl Ord for Hand2 {
        fn cmp(&self, other: &Self) -> Ordering {
            let ht1: crate::HandType = self.hand_type();
            let ht2 = other.hand_type();
            if ht1 > ht2 {
                return Ordering::Greater;
            } else if ht2 > ht1 {
                return Ordering::Less;
            }
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                let comp = cmp_cards(c1, c2);
                if comp == Ordering::Greater {
                    return Ordering::Greater;
                } else if comp == Ordering::Less {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }

    impl PartialOrd for Hand2 {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Hand2 {
        pub fn hand_type(&self) -> HandType {
            let jokers = self.cards.iter().filter(|it| it == &&Card::Jack).count();
            if jokers == 5 {
                return HandType::FiveOfAKind;
            }
            let mut sorted_cards = self
                .cards
                .iter()
                .filter(|it| it != &&Card::Jack)
                .collect::<Vec<&Card>>();
            sorted_cards.sort();
            let mut frequencies: Vec<usize> = sorted_cards
                .iter()
                .dedup_with_count()
                .map(|it| it.0)
                .collect();
            frequencies.sort();
            frequencies.reverse();
            frequencies[0] += jokers;
            match frequencies[0] {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => match frequencies[1] {
                    2 => HandType::FullHouse,
                    _ => HandType::ThreeOfAKind,
                },
                2 => match frequencies[1] {
                    2 => HandType::TwoPair,
                    _ => HandType::OnePair,
                },
                _ => HandType::HighCard,
            }
        }
    }

    fn cmp_cards(card1: &Card, card2: &Card) -> std::cmp::Ordering {
        if card1 == card2 {
            return Ordering::Equal;
        }
        match (card1, card2) {
            (Card::Ace, _) => Ordering::Greater,
            (_, Card::Ace) => Ordering::Less,
            (Card::King, _) => Ordering::Greater,
            (_, Card::King) => Ordering::Less,
            (Card::Queen, _) => Ordering::Greater,
            (_, Card::Queen) => Ordering::Less,
            (Card::Ten, _) => Ordering::Greater,
            (_, Card::Ten) => Ordering::Less,
            (Card::Nine, _) => Ordering::Greater,
            (_, Card::Nine) => Ordering::Less,
            (Card::Eight, _) => Ordering::Greater,
            (_, Card::Eight) => Ordering::Less,
            (Card::Seven, _) => Ordering::Greater,
            (_, Card::Seven) => Ordering::Less,
            (Card::Six, _) => Ordering::Greater,
            (_, Card::Six) => Ordering::Less,
            (Card::Five, _) => Ordering::Greater,
            (_, Card::Five) => Ordering::Less,
            (Card::Four, _) => Ordering::Greater,
            (_, Card::Four) => Ordering::Less,
            (Card::Three, _) => Ordering::Greater,
            (_, Card::Three) => Ordering::Less,
            (Card::Two, _) => Ordering::Greater,
            (_, Card::Two) => Ordering::Less,
            (Card::Jack, _) => Ordering::Greater,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    pub fn from_char(c: &char) -> Card {
        return match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Invalid card '{}'.", c),
        };
    }
}
#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
}

impl Hand {
    pub fn new(input: &str) -> Hand {
        let parsed: Vec<&str> = input.split_whitespace().collect();
        assert!(parsed.len() == 2);
        let hand = parsed[0];
        let bid = parsed[1].parse().unwrap();
        if hand.len() != 5 {
            panic!("'{}' is not a valid hand.", hand);
        }
        let cv: Vec<Card> = hand
            .chars()
            .into_iter()
            .map(|it| Card::from_char(&it))
            .collect();
        let cards = [cv[0], cv[1], cv[2], cv[3], cv[4]];
        Hand { cards, bid }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand2 {
    cards: [Card; 5],
    bid: i64,
}

impl Hand2 {
    pub fn new(input: &str) -> Hand2 {
        let parsed: Vec<&str> = input.split_whitespace().collect();
        assert!(parsed.len() == 2);
        let hand = parsed[0];
        let bid = parsed[1].parse().unwrap();
        if hand.len() != 5 {
            panic!("'{}' is not a valid hand.", hand);
        }
        let cv: Vec<Card> = hand
            .chars()
            .into_iter()
            .map(|it| Card::from_char(&it))
            .collect();
        let cards = [cv[0], cv[1], cv[2], cv[3], cv[4]];
        Hand2 { cards, bid }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = read_file_as_vector("./files/day7.txt").expect("Cannot read file.");
        let result = solve1::solve1(&input);
        assert_eq!(result, 248179786);
    }

    #[test]
    fn test_solve1_testdata() {
        let input = read_file_as_vector("./files/test.txt").expect("Cannot read file.");
        let result = solve1::solve1(&input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_solve2() {
        let input = read_file_as_vector("./files/day7.txt").expect("Cannot read file.");
        let result = solve2::solve2(&input);
        assert_eq!(result, 247885995);
    }

    #[test]
    fn test_solve2_testdata() {
        let input = read_file_as_vector("./files/test.txt").expect("Cannot read file.");
        let result = solve2::solve2(&input);
        assert_eq!(result, 5905);
    }
}
