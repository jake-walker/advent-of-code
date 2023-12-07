use std::{cmp::Ordering, fs};

use counter::Counter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum CardLabel {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

fn char_to_label(c: &char) -> CardLabel {
    match c {
        'A' => CardLabel::A,
        'K' => CardLabel::K,
        'Q' => CardLabel::Q,
        'J' => CardLabel::J,
        'T' => CardLabel::T,
        '9' => CardLabel::Nine,
        '8' => CardLabel::Eight,
        '7' => CardLabel::Seven,
        '6' => CardLabel::Six,
        '5' => CardLabel::Five,
        '4' => CardLabel::Four,
        '3' => CardLabel::Three,
        '2' => CardLabel::Two,
        _ => panic!("unexpected char")
    }
}

fn parse_hand(s: &str) -> (Vec<CardLabel>, u32) {
    let (hand, bid_amount) = s.split_once(" ").unwrap();
    
    (
        hand.chars().map(|c| char_to_label(&c)).collect::<Vec<CardLabel>>(),
        bid_amount.parse::<u32>().unwrap()
    )
}

fn get_hand_type(hand: &Vec<CardLabel>) -> HandType {
    if hand.len() != 5 {
        panic!("expected hand of length 5");
    }

    let counts = hand.iter().collect::<Counter<_>>().most_common_ordered().iter().map(|x| x.1).collect::<Vec<usize>>();
    
    match (counts.get(0).unwrap_or_else(|| &0), counts.get(1).unwrap_or_else(|| &0), counts.get(2).unwrap_or_else(|| &0)) {
        (5, _, _) => HandType::FiveOfAKind,
        (4, _, _) => HandType::FourOfAKind,
        (3, 2, _) => HandType::FullHouse,
        (3, _, _) => HandType::ThreeOfAKind,
        (2, 2, _) => HandType::TwoPair,
        (2, _, _) => HandType::OnePair,
        _ => HandType::HighCard
    }
}

fn compare_hands(a: &Vec<CardLabel>, b: &Vec<CardLabel>) -> Ordering {
    let a_type = get_hand_type(a) as usize;
    let b_type = get_hand_type(b) as usize;
    
    let type_cmp = a_type.cmp(&b_type);
    
    if type_cmp != Ordering::Equal {
        return type_cmp;
    }
    
    for (&al, &bl) in a.iter().zip(b) {
        let l_cmp = (al as usize).cmp(&(bl as usize));
        
        if l_cmp != Ordering::Equal {
            return l_cmp;
        }
    }
    
    return Ordering::Equal;
}

fn total_winnings(values: Vec<(Vec<CardLabel>, u32)>) -> u32 {
    let mut values = values;
    values.sort_by(|a, b| compare_hands(&b.0, &a.0));

    values.iter().enumerate().map(|(i, (_, bid_amount))| (i as u32 + 1) * bid_amount).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let values = input.lines().map(|l| parse_hand(l)).collect::<Vec<(Vec<CardLabel>, u32)>>();
    
    let part1 = total_winnings(values);
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    static EXAMPLE_INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
    
    #[test]
    fn test_parse_hand() {
        assert_eq!(parse_hand("32T3K 0").0, vec![CardLabel::Three, CardLabel::Two, CardLabel::T, CardLabel::Three, CardLabel::K]);
    }

    #[test]
    fn test_hand_types() {
        let hands = EXAMPLE_INPUT.lines().map(|l| parse_hand(l).0).collect::<Vec<Vec<CardLabel>>>();
        
        assert_eq!(hands.into_iter().map(|h| get_hand_type(&h)).collect::<Vec<HandType>>(), vec![
            HandType::OnePair, HandType::ThreeOfAKind, HandType::TwoPair, HandType::TwoPair, HandType::ThreeOfAKind
        ]);
    }

    #[test]
    fn test_sort_hands_example() {
        let mut hands = EXAMPLE_INPUT.lines().map(|l| parse_hand(l).0).collect::<Vec<Vec<CardLabel>>>();
        hands.sort_by(|a, b| compare_hands(a, b));
        
        assert_eq!(hands, vec![
            vec![CardLabel::Q, CardLabel::Q, CardLabel::Q, CardLabel::J, CardLabel::A],
            vec![CardLabel::T, CardLabel::Five, CardLabel::Five, CardLabel::J, CardLabel::Five],
            vec![CardLabel::K, CardLabel::K, CardLabel::Six, CardLabel::Seven, CardLabel::Seven],
            vec![CardLabel::K, CardLabel::T, CardLabel::J, CardLabel::J, CardLabel::T],
            vec![CardLabel::Three, CardLabel::Two, CardLabel::T, CardLabel::Three, CardLabel::K]
        ])
    }

    #[test]
    fn test_total_winnings() {
        let input = EXAMPLE_INPUT.lines().map(|l| parse_hand(l)).collect::<Vec<(Vec<CardLabel>, u32)>>();
        
        assert_eq!(total_winnings(input), 6440);
    }
}