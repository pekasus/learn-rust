use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    hands
        .iter()
        .map(|hand| -> Hand { (*hand).try_into().unwrap() })
        .fold(vec![], |mut winning_hands, hand| {
            if winning_hands.len() == 0 {
                winning_hands.push(hand)
            } else {
                match hand.partial_cmp(&winning_hands[0]) {
                    Some(Ordering::Greater) => {
                        winning_hands.clear();
                        winning_hands.push(hand);
                    }
                    Some(Ordering::Equal) | None => winning_hands.push(hand),
                    _ => (),
                }
            }
            winning_hands
        })
        .drain(..)
        .map(|hand| hand.original_str())
        .collect()
}

#[derive(PartialEq)]
struct Hand<'a> {
    original: &'a str,
}

impl<'a> Hand<'a> {
    fn original_str(self) -> &'a str {
        self.original
    }
    //TODO Fix this caching!!!
    fn category(&self) -> HandCategory {
        use CardNumber::*;
        let is_same_suit = self.cards().iter().map(|card| card.1).counts().len() == 1;
        let straight_discriminant = if let [Ace, Five, Four, Three, Two] = self.card_numbers() {
            Some(Three)
        } else if self
            .card_numbers()
            .windows(2)
            .all(|card_numbers| card_numbers[0] as usize == card_numbers[1] as usize + 1)
        {
            Some(self.card_numbers()[2])
        } else {
            None
        };

        if let (Some(straight_discriminant), true) = (straight_discriminant, is_same_suit) {
            // return HandCategory::StraightFlush {
            //     middle: straight_discriminant,
            // };
            return HandCategory::StraightFlush;
        }

        let four_of_kind = self
            .counts()
            .iter()
            .find(|(_, &count)| count == 4)
            .map(|(&card_number, _)| card_number);
        if let Some(four) = four_of_kind {
            // return HandCategory::FourOfAKind {
            //     four,
            //     kicker: counts
            //         .iter()
            //         .find(|(_, &count)| count == 1)
            //         .map(|(&card_number, _)| card_number)
            //         .unwrap(),
            // };
            return HandCategory::FourOfAKind;
        }

        let three_of_kind = self
            .counts()
            .iter()
            .find(|(_, &count)| count == 3)
            .map(|(&card_number, _)| card_number);

        if let (Some(three), &[pair]) = (three_of_kind, self.pairs().as_slice()) {
            // return HandCategory::FullHouse { three, pair };
            return HandCategory::FullHouse;
        }

        if is_same_suit {
            // return HandCategory::Flush {
            //     singles: card_numbers,
            // };
            return HandCategory::Flush;
        }

        if let Some(straight_discriminant) = straight_discriminant {
            // return HandCategory::Straight {
            //     middle: straight_discriminant,
            // };
            return HandCategory::Straight;
        }

        if let Some(three) = three_of_kind {
            // let kickers = card_numbers
            //     .into_iter()
            //     .filter(|&card_number| card_number != three)
            //     .collect::<Vec<_>>()
            //     .try_into()
            //     .unwrap();
            // return HandCategory::ThreeOfAKind { three, kickers };
            return HandCategory::ThreeOfAKind;
        }

        // TODO: match slice directly
        if self.pairs().len() == 2 {
            //pairs = [J, 3]
            let pairs: [CardNumber; 2] = self.pairs().try_into().unwrap();
            let kicker = self
                .card_numbers()
                .into_iter()
                .find(|card_number| !pairs.contains(card_number))
                .unwrap();
            // return HandCategory::TwoPairs { pairs, kicker };
            return HandCategory::TwoPairs;
        }

        if self.pairs().len() == 1 {
            return HandCategory::OnePair;
        }

        HandCategory::HighCard
    }

    fn kickers(&self) -> Vec<CardNumber> {
        match self.category() {
            HandCategory::HighCard => self.card_numbers(),
            HandCategory::OnePair | HandCategory::TwoPairs => self
                .card_numbers()
                .into_iter()
                .filter(|card_number| !self.pairs().contains(card_number))
                .collect(),
            HandCategory::ThreeOfAKind => self
                .card_numbers()
                .into_iter()
                .filter(|&card_number| card_number != self.three().unwrap())
                .collect::<Vec<_>>(),
            HandCategory::Straight => self.card_numbers(),
            HandCategory::Flush => todo!(),
            HandCategory::FullHouse => todo!(),
            HandCategory::FourOfAKind => todo!(),
            HandCategory::StraightFlush => self.card_numbers(),
        }
    }

    fn three(&self) -> Option<CardNumber> {
        self.counts()
            .iter()
            .find(|(_, &count)| count == 3)
            .map(|(&card_number, _)| card_number)
    }

    fn middle(&self) -> Option<CardNumber> {
        todo!()
    }

    fn four(&self) -> Option<CardNumber> {
        self.counts()
            .iter()
            .find(|(_, &count)| count == 4)
            .map(|(&card_number, _)| card_number)
    }

    fn discriminants(&self) -> Vec<CardNumber> {
        match self.category() {
            HandCategory::HighCard => self.card_numbers(),
            HandCategory::OnePair => [self.pairs(), self.kickers()].concat(),
            HandCategory::TwoPairs => [self.pairs().to_vec(), self.kickers()].concat(),
            HandCategory::ThreeOfAKind => [vec![self.three().unwrap()], self.kickers()].concat(),
            HandCategory::Straight => vec![self.middle().unwrap()],
            HandCategory::Flush => self.card_numbers().to_vec(),
            HandCategory::FullHouse => {
                [vec![self.three().unwrap()], vec![self.pairs()[0]]].concat()
            }
            HandCategory::FourOfAKind => [vec![self.four().unwrap()], self.kickers()].concat(),
            HandCategory::StraightFlush => vec![self.middle().unwrap()],
        }
    }

    fn pairs(&self) -> Vec<CardNumber> {
        self.counts()
            .iter()
            .filter(|(_, &count)| count == 2)
            .map(|(&card_number, _)| card_number)
            .sorted()
            .rev()
            .collect()
    }

    fn counts(&self) -> HashMap<CardNumber, usize> {
        self.card_numbers().iter().copied().counts()
    }

    fn card_numbers(&self) -> [CardNumber; 5] {
        self.cards()
            .iter()
            .map(|Card(card_number, _)| *card_number)
            .sorted()
            .rev()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

// TODO: Guarantee that singles in HighCard and Flush are sorted
// TODO: Guarantee that pairs are sorted
// TODO: Consider sorting from greater to lesser
#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum HandCategory {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

//TODO Check if this can be blanket implementation
//TODO Impl Iterator or IntoIterator
fn cmp_singles<'a, T>(a: T, b: T) -> Ordering
where
    T: Iterator<Item = &'a CardNumber> + DoubleEndedIterator + ExactSizeIterator,
{
    a.zip(b)
        .find(|(a, b)| a != b)
        .map(|(a, b)| a.cmp(&b))
        .unwrap_or(Ordering::Equal)
}

// impl Ord for HandCategory {
//     fn cmp(&self, other: &Self) -> Ordering {
//         //let self_category_int = std::mem::discriminant(self).
//         use HandCategory::*;
//         match (self, other) {
//             (x, y) if std::mem::discriminant(x) == std::mem::discriminant(y) => {
//                 let self_discriminants = self.discriminants();
//                 let other_discriminants = other.discriminants();
//                 cmp_singles(self_discriminants.iter(), other_discriminants.iter())
//             }
//             (StraightFlush { .. }, _) => Ordering::Greater,
//             (_, StraightFlush { .. }) => Ordering::Less,
//             (FourOfAKind { .. }, _) => Ordering::Greater,
//             (_, FourOfAKind { .. }) => Ordering::Less,
//             (FullHouse { .. }, _) => Ordering::Greater,
//             (_, FullHouse { .. }) => Ordering::Less,
//             (Flush { .. }, _) => Ordering::Greater,
//             (_, Flush { .. }) => Ordering::Less,
//             (Straight { .. }, _) => Ordering::Greater,
//             (_, Straight { .. }) => Ordering::Less,
//             (ThreeOfAKind { .. }, _) => Ordering::Greater,
//             (_, ThreeOfAKind { .. }) => Ordering::Less,
//             (TwoPairs { .. }, _) => Ordering::Greater,
//             (_, TwoPairs { .. }) => Ordering::Less,
//             (OnePair { .. }, HighCard { .. }) => Ordering::Greater,
//             (HighCard { .. }, OnePair { .. }) => Ordering::Less,
//             _ => unreachable!(),
//         }
//     }
// }

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.category().partial_cmp(&other.category())
    }
}

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = ();

    fn try_from(original: &'a str) -> Result<Hand<'a>, Self::Error> {
        Self::parse_five_cards(original)?;
        Ok(Hand { original })
    }
}

impl<'a> Hand<'a> {
    fn parse_five_cards(input: &str) -> Result<[Card; 5], ()> {
        input
            .split(" ")
            .map(|card| card.parse())
            .collect::<Result<Vec<_>, ()>>()?
            .as_slice()
            .try_into()
            .map_err(|_| ())
    }
    fn cards(&self) -> [Card; 5] {
        Self::parse_five_cards(self.original).unwrap()
    }
}

impl FromStr for Card {
    type Err = ();
    fn from_str(original: &str) -> Result<Self, Self::Err> {
        if let &[ref card_number @ .., card_suit] = original.as_bytes() {
            Ok(Self(card_number.try_into()?, card_suit.try_into()?))
        } else {
            Err(())
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl TryFrom<&[u8]> for CardNumber {
    type Error = ();
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        match input {
            b"2" => Ok(CardNumber::Two),
            b"3" => Ok(CardNumber::Three),
            b"4" => Ok(CardNumber::Four),
            b"5" => Ok(CardNumber::Five),
            b"6" => Ok(CardNumber::Six),
            b"7" => Ok(CardNumber::Seven),
            b"8" => Ok(CardNumber::Eight),
            b"9" => Ok(CardNumber::Nine),
            b"10" => Ok(CardNumber::Ten),
            b"J" => Ok(CardNumber::Jack),
            b"Q" => Ok(CardNumber::Queen),
            b"K" => Ok(CardNumber::King),
            b"A" => Ok(CardNumber::Ace),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for CardSuit {
    type Error = ();
    fn try_from(input: u8) -> Result<Self, Self::Error> {
        match input {
            b'D' => Ok(CardSuit::Diamonds),
            b'H' => Ok(CardSuit::Hearts),
            b'C' => Ok(CardSuit::Clubs),
            b'S' => Ok(CardSuit::Spades),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
struct Card(CardNumber, CardSuit);

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Copy, Clone, Debug)]
enum CardNumber {
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

#[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Debug)]
enum CardSuit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}
