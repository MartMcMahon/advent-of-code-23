use std::collections::HashMap;

// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FourOfAKind = 4,
    FiveOfAKind = 5,
}
#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: Vec<u8>,
    hand_type: HandType,
    bid: i32,
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type.cmp(&other.hand_type) == std::cmp::Ordering::Equal {
            let mut i = 0;
            loop {
                if self.cards[i].cmp(&other.cards[i]) == std::cmp::Ordering::Equal {
                    i += 1;
                    continue;
                } else {
                    return self.cards[0].cmp(&other.cards[0]);
                }
            }
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}
impl Hand {
    fn from_lines(lines: impl Iterator<Item = String>) -> anyhow::Result<Vec<Hand>> {
        let mut hands = Vec::new();
        for l in lines {
            let mut cards = [0; 5];

            for (i, c) in l.chars().enumerate() {
                match c {
                    '0'..='9' => {
                        cards[i] = c as u8 - '0' as u8;
                    }
                    'T' => {
                        cards[i] = 10;
                        // counts[10] += 1;
                    }
                    'J' => {
                        cards[i] = 11;
                        // counts[11] += 1;
                    }
                    'Q' => {
                        cards[i] = 12;
                        // counts[12] += 1;
                    }
                    'K' => {
                        cards[i] = 13;
                        // counts[13] += 1;
                    }
                    'A' => {
                        cards[i] = 14;
                        // counts[14] += 1;
                    }
                    ' ' => break,
                    _ => {
                        panic!("unexpected char: {}", c);
                    }
                }
            }
            assert_eq!(cards.len(), 5);

            let mut has_pair = false;
            let mut has_two_pair = false;
            let mut counts: HashMap<u8, u8> = HashMap::new();
            for c in cards {
                if counts.contains_key(&c) && has_pair == false {
                    has_pair = true;
                }
                if counts.contains_key(&c) && has_pair == true {
                    has_two_pair = true;
                }
                // counts.entry(c).and_modify(|e| *e += 1);
                if let Some(count) = counts.get_mut(&c) {
                    *count += 1;
                } else {
                    counts.insert(c, 1);
                }
                // counts.get(&c)or_insert.(1).unwrap();
            }

            println!("counts: {:?}", counts);

            let hand_type = match counts.values().max() {
                Some(5) => HandType::FiveOfAKind,
                Some(4) => HandType::FourOfAKind,
                Some(3) => HandType::ThreeOfAKind,
                Some(2) => {
                    if has_two_pair {
                        HandType::TwoPair
                    } else {
                        HandType::OnePair
                    }
                }
                _ => HandType::HighCard,
            };

            let bid = l[6..].parse::<i32>()?;

            hands.push(Hand {
                cards: cards.to_vec(),
                hand_type,
                bid,
            });
        }
        Ok(hands)
    }
}

pub fn star1(lines: impl Iterator<Item = String>) -> anyhow::Result<i32> {
    let total = 1;
    let mut hands = Hand::from_lines(lines)?;
    hands.sort();
    let players_num = hands.sort();

    // if hands.len() != players_num {
    //     panic!("hands.len() != players_num");
    // }
    // for (i, h) in hands.iter().enumerate() {
    // total += hands.len() - i;
    // }
    // total += hands.len();
    Ok(total as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_star1() {
        let lines = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        assert_eq!(star1(lines.into_iter()).unwrap(), 6440);
    }
}
