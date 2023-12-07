use std::{cmp::Ordering, hash::Hash, str::FromStr};

use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Hand>>();
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| {
            let rank = (i as u32) + 1;
            h.bid * rank
        })
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|l| HandPt2(l.parse::<Hand>().unwrap()))
        .collect::<Vec<_>>();
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| {
            let rank = (i as u32) + 1;
            h.0.bid * rank
        })
        .sum()
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, Hash)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

#[derive(Clone, Debug, PartialEq, Ord, Eq)]
struct HandPt2(Hand);

impl Hand {
    pub fn ty(&self) -> Type {
        let mut counts: Vec<usize> = self.cards.iter().counts().into_values().collect();
        counts.sort();
        counts.reverse();

        match &counts[..] {
            [5] => Type::FiveOfAKind,
            [4, 1] => Type::FourOfAKind,
            [3, 2] => Type::FullHouse,
            [3, 1, 1] => Type::ThreeOfAKind,
            [2, 2, 1] => Type::TwoPair,
            [2, 1, 1, 1] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ => panic!("wrong type?"),
        }
    }

    pub fn with_cards(&self, cards: &[Card]) -> Hand {
        Hand {
            cards: Vec::from(cards),
            bid: self.bid,
        }
    }
}

impl HandPt2 {
    pub fn ty(&self) -> Type {
        if self.0.cards.iter().all(|c| c == &Card::Joker) {
            self.0.with_cards(&vec![Card::Ace; 5]).ty()
        } else if self.0.cards.contains(&Card::Joker) {
            let counts = self.0.cards.iter().filter(|c| *c != &Card::Joker).counts();
            let most_others = counts.iter().max_by_key(|(_, v)| **v).unwrap().0;
            let best_possible_hand = self
                .0
                .cards
                .iter()
                .map(|c| match c {
                    Card::Joker => **most_others,
                    c => *c,
                })
                .collect::<Vec<_>>();
            self.0.with_cards(&best_possible_hand).ty()
        } else {
            self.0.ty()
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let bid = bid.parse().map_err(|_| "cannot parse bid")?;
        let cards = cards
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<_, String>>()?;
        Ok(Hand { cards, bid })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.ty()
                .cmp(&other.ty())
                .then(self.cards.cmp(&other.cards)),
        )
    }
}

impl PartialOrd for HandPt2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.ty().cmp(&other.ty()).then({
            self.0
                .cards
                .iter()
                .map(|&c| JokerLowest(c))
                .cmp(other.0.cards.iter().map(|&c| JokerLowest(c)))
        }))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Num(u32),
    Joker,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            c @ '0'..='9' => Ok(Card::Num(c.to_digit(10).unwrap())),
            'T' => Ok(Card::Num(10)),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'A' => Ok(Card::Ace),
            'J' => Ok(Card::Joker),
            c => Err(format!("{} is not a valid card", c)),
        }
    }
}

#[derive(PartialEq, Eq)]
struct JokerLowest(Card);

impl Ord for JokerLowest {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.0, other.0) {
            (Card::Joker, Card::Joker) => Ordering::Equal,
            (Card::Joker, _) => Ordering::Less,
            (_, Card::Joker) => Ordering::Greater,
            _ => self.0.cmp(&other.0),
        }
    }
}

impl PartialOrd for JokerLowest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/07.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(251029473, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(251003917, super::two(&input));
    }
}
