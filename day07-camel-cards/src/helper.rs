use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(Debug, Clone)]
pub(crate) struct Hand {
    pub(crate) cards: String,
    #[allow(dead_code)]
    hand_type: HandType,
    pub(crate) rank: Option<u32>,
    pub(crate) bit: u32,
}

pub(crate) fn parse_input(part_one: bool) -> Vec<Vec<Hand>> {
    let input = read_input();
    let mut type_vec: Vec<Vec<Hand>> = vec![vec![]; 7];
    let cards = if part_one {
        vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
    } else {
        vec![
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ]
    };
    for hand_line in input {
        let hand_info = hand_line
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let hand_cards = &hand_info[0];
        let bit = hand_info[1].parse::<u32>();
        let hand_type = if part_one {
            get_hand_type_one(hand_cards)
        } else {
            get_hand_type_two(hand_cards)
        };
        let hand = Hand {
            cards: hand_cards.to_string(),
            hand_type,
            rank: None,
            bit: bit.unwrap(),
        };
        if let HandType::FiveOfAKind = hand.hand_type {
            type_vec[6].push(hand);
        } else if let HandType::FourOfAKind = hand.hand_type {
            type_vec[5].push(hand);
        } else if let HandType::FullHouse = hand.hand_type {
            type_vec[4].push(hand);
        } else if let HandType::ThreeOfAKind = hand.hand_type {
            type_vec[3].push(hand);
        } else if let HandType::TwoPair = hand.hand_type {
            type_vec[2].push(hand);
        } else if let HandType::OnePair = hand.hand_type {
            type_vec[1].push(hand);
        } else {
            type_vec[0].push(hand);
        }
    }
    let mut prev_len = 0;
    #[allow(unused_mut)]
    for value in type_vec.iter_mut() {
        value.sort_by(|a, b| {
            let a_chars = a.cards.chars().collect::<Vec<char>>();
            let b_chars = b.cards.chars().collect::<Vec<char>>();
            for (c1, c2) in a_chars.iter().zip(b_chars.iter()) {
                if cards.iter().position(|c| c == c1) > cards.iter().position(|c| c == c2) {
                    return std::cmp::Ordering::Greater;
                } else if cards.iter().position(|c| c == c1) < cards.iter().position(|c| c == c2) {
                    return std::cmp::Ordering::Less;
                }
            }
            return std::cmp::Ordering::Equal;
        });
        value.iter_mut().enumerate().for_each(|(i, v)| {
            #[allow(unused_variables)]
            if let None = &mut v.rank {
                v.rank = Option::from(prev_len + i as u32 + 1);
            }
        });
        prev_len += value.len() as u32;
    }
    type_vec
}
fn read_input() -> Vec<String> {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}
fn get_hand_type_one(hand_cards: &str) -> HandType {
    let mut counter: HashMap<char, u32> = HashMap::new();
    for c in hand_cards.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }
    return set_hand_type(counter);
}

fn get_hand_type_two(hand_cards: &str) -> HandType {
    let mut counter: HashMap<char, u32> = HashMap::new();
    for c in hand_cards.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }
    let cards = vec![
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    if hand_cards.contains("J") {
        let mut max_value : u32 = 1;
        for (key, value) in counter.iter() {
            if key != &'J' && *value >= max_value {
                max_value = *value;
            }
        }

        let keys_max: Vec<_> = counter
            .iter()
            .filter_map(|(key, &value)| {
                if key != &'J' && value == max_value {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect();
        let j_val = *counter.get(&'J').unwrap();
        if keys_max.len() == 1 {
            counter.entry(*keys_max.get(0).unwrap()).and_modify(|x| {
                *x += j_val;
            });
        }
        else{
            let mut strongest_card = 'J';
            let mut strongest_card_index: i32 = 0;
            for c in keys_max{
                if cards.iter().position(|x| x == &c).unwrap() as i32 > strongest_card_index {
                    strongest_card = c;
                    strongest_card_index = cards.iter().position(|x| x == &c).unwrap() as i32;
                }
            }
            counter.insert(
                strongest_card,
                j_val + counter.get(&strongest_card).unwrap(),
            );
        }
        counter.remove(&'J');

    }
    return set_hand_type(counter);
}

fn set_hand_type(counter: HashMap<char, u32>) -> HandType {
    if counter.len() == 5 {
        HandType::HighCard
    } else if counter.len() == 4 {
        HandType::OnePair
    } else if counter.len() == 3 {
        let mut count_pairs = 0;
        for (_, v) in counter {
            if v == 2 {
                count_pairs += 1;
            }
        }
        if count_pairs == 2 {
            HandType::TwoPair
        } else {
            HandType::ThreeOfAKind
        }
    } else if counter.len() == 2 {
        let mut count = 0;
        for (_, v) in counter {
            if v == 4 {
                count += 1;
            }
        }
        if count == 1 {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        }
    } else {
        HandType::FiveOfAKind
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let type_vec = parse_input(false);
        assert_eq!(type_vec.len(), 7);
        for (i, v) in type_vec.iter().enumerate() {
            if i == 0 {
                println!("HighCard:\n{:?}", v);
            } else if i == 1 {
                println!("OnePair:\n{:?}", v);
            } else if i == 2 {
                println!("TwoPair:\n{:?}", v);
            } else if i == 3 {
                println!("ThreeOfAKind:\n{:?}", v);
            } else if i == 4 {
                println!("FullHouse:\n{:?}", v);
            } else if i == 5 {
                println!("FourOfAKind:\n{:?}", v);
            } else {
                println!("FiveOfAKind:\n{:?}", v);
            }
        }
    }
}
