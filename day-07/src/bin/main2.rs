use std::{cmp::Ordering, collections::HashMap, fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandCategory {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}
impl HandCategory {
    fn from_histogram(histogram: HashMap<usize, usize>, jokers: usize) -> Self {
        let Some(top_count) = histogram
            .iter()
            .max_by(|(_, amount1), (_, amount2)| amount1.cmp(amount2))
        else {
            // Five jokers, empty histogram
            return HandCategory::Five;
        };

        let before_jokers = match top_count.1 {
            5 => return HandCategory::Five,
            4 => HandCategory::Four,
            3 => {
                if histogram.iter().any(|(_, amount)| *amount == 2) {
                    return HandCategory::FullHouse;
                }
                HandCategory::Three
            }
            2 => {
                if histogram.iter().filter(|(_, amount)| **amount == 2).count() == 2 {
                    HandCategory::TwoPair
                } else {
                    HandCategory::Pair
                }
            }
            1 => HandCategory::HighCard,
            _ => panic!("{:?}", top_count),
        };

        // Joker upgrades
        if jokers == 0 {
            return before_jokers;
        }

        match before_jokers {
            HandCategory::HighCard if jokers == 4 => HandCategory::Five,
            HandCategory::HighCard if jokers == 3 => HandCategory::Four,
            HandCategory::HighCard if jokers == 2 => HandCategory::Three,
            HandCategory::HighCard if jokers == 1 => HandCategory::Pair,
            HandCategory::Pair if jokers == 3 => HandCategory::Five,
            HandCategory::Pair if jokers == 2 => HandCategory::Four,
            HandCategory::Pair => HandCategory::Three,
            HandCategory::TwoPair => HandCategory::FullHouse,
            HandCategory::Three if jokers == 2 => HandCategory::Five,
            HandCategory::Three => HandCategory::Four,
            HandCategory::Four => HandCategory::Five,
            other => {
                panic!("Can't upgrade {:?}", other)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct ParseCardError;

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
struct Hand {
    cards: Vec<usize>,
    category: HandCategory,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.category != other.category {
            self.category.partial_cmp(&other.category)
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                if a != b {
                    return a.partial_cmp(b);
                }
            }

            Some(Ordering::Equal)
        }
    }
}
impl FromStr for Hand {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let cards: Vec<usize> = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                Ok(if c.is_digit(10) {
                    c.to_digit(10).unwrap()
                } else {
                    match c {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 0, // Joker, there should be a better mapping
                        'T' => 10,
                        _ => return Err(ParseCardError),
                    }
                } as usize)
            })
            .collect::<Result<Vec<_>, ParseCardError>>()?;

        let jokers = cards.iter().filter(|val| **val == 0).count();
        let histogram = cards
            .iter()
            .fold(HashMap::<usize, usize>::new(), |mut acc, new| {
                if *new == 0 {
                    return acc;
                }

                let current = acc.get(new).map(|val| *val).unwrap_or_default();
                acc.insert(*new, current + 1);
                acc
            });

        let bid = parts.next().unwrap().parse().unwrap();

        Ok(Self {
            cards,
            category: HandCategory::from_histogram(histogram, jokers),
            bid,
        })
    }
}

fn compute(input: String) -> String {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<Hand>, ParseCardError>>()
        .unwrap();

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").unwrap();
        assert_eq!("5905", compute(input));
    }

    #[test]
    fn test_hand_parsing() {
        assert_eq!(
            Hand::from_str("AKQJT 69"),
            Ok(Hand {
                cards: vec![14, 13, 12, 0, 10],
                category: HandCategory::Pair,
                bid: 69
            })
        );
    }

    #[test]
    fn test_category_parsing() {
        for (to_parse, expected_category) in vec![
            ("AKQT9 1", HandCategory::HighCard),
            ("AKQJT 1", HandCategory::Pair), // Joker
            ("AKQTT 1", HandCategory::Pair),
            ("AQQTT 1", HandCategory::TwoPair),
            ("AKTTT 1", HandCategory::Three),
            ("ATTTT 1", HandCategory::Four),
            ("QQQTT 1", HandCategory::FullHouse),
            ("TTTTT 1", HandCategory::Five),
        ] {
            assert_eq!(
                Hand::from_str(to_parse).unwrap().category,
                expected_category
            );
        }
    }

    #[test]
    fn test_category_ord() {
        // Can never remember if the derive makes asc or desc
        assert!(HandCategory::Five > HandCategory::Four);
        assert!(HandCategory::Four > HandCategory::FullHouse);
        assert!(HandCategory::FullHouse > HandCategory::Three);
        assert!(HandCategory::Three > HandCategory::TwoPair);
        assert!(HandCategory::TwoPair > HandCategory::Pair);
        assert!(HandCategory::Pair > HandCategory::HighCard);
    }

    #[test]
    fn test_secondary_sort() {
        assert!(Hand::from_str("33332 1").unwrap() > Hand::from_str("2AAAA 1").unwrap());
    }

    #[test]
    fn test_sort_order() {
        assert!(Hand::from_str("AKQJT 1").unwrap() < Hand::from_str("2AAAA 1").unwrap());
    }
}
