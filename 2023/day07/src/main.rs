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

#[derive(Debug, PartialEq, Eq, Clone)]
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

fn get_hand_type(hand: &Vec<CardLabel>, part2: bool) -> HandType {
    if hand.len() != 5 {
        panic!("expected hand of length 5");
    }

    let mut counts = hand.iter().collect::<Counter<_>>().most_common_ordered().iter().map(|x| x.1).collect::<Vec<usize>>();

    // add the number of jokers to each count if this is part 2
    if part2 {
        let jokers = hand.iter().filter(|&&x| x == CardLabel::J).count();
        if jokers > 0 {
            counts[0] += jokers;
        }
    }

    let counts1 = *counts.get(0).unwrap_or_else(|| &0);
    let counts2 = *counts.get(1).unwrap_or_else(|| &0);
    let mut hand_type = HandType::HighCard;

    if counts1 >= 5 {
        hand_type = HandType::FiveOfAKind;
    } else if counts1 >= 4 {
        hand_type = HandType::FourOfAKind;
    } else if counts1 >= 3 && counts2 >= 2 {
        hand_type = HandType::FullHouse;
    } else if counts1 >= 3 {
        hand_type = HandType::ThreeOfAKind;
    } else if counts1 >= 2 && counts2 >= 2 {
        hand_type = HandType::TwoPair;
    } else if counts1 >= 2 {
        hand_type = HandType::OnePair;
    }

    #[cfg(test)]
    println!("{:?} => counts={:?} => {:?}", hand, counts, hand_type);

    hand_type
}

fn compare_hands(a: &Vec<CardLabel>, b: &Vec<CardLabel>, part2: bool) -> Ordering {
    let a_type = get_hand_type(a, part2) as usize;
    let b_type = get_hand_type(b, part2) as usize;

    let type_cmp = b_type.cmp(&a_type);

    #[cfg(test)]
    println!("Comparing {:?} ({}), {:?} ({}) => {:?}", a, a_type, b, b_type, type_cmp);

    if type_cmp != Ordering::Equal {
        return type_cmp;
    }

    #[cfg(test)]
    println!(" - Same types, comparing labels");

    for (&al, &bl) in a.iter().zip(b) {
        let mut aln = al as isize;
        let mut bln = bl as isize;

        if part2 {
            if al == CardLabel::J {
                aln = 100;
            }
            if bl == CardLabel::J {
                bln = 100;
            }
        }

        let l_cmp = (bln).cmp(&aln);

        #[cfg(test)]
        println!("   - {:?} ({}), {:?} ({}) = {:?}", al, aln, bl, bln, l_cmp);

        if l_cmp != Ordering::Equal {
            return l_cmp;
        }
    }

    return Ordering::Equal;
}

fn total_winnings(values: Vec<(Vec<CardLabel>, u32)>, part2: bool) -> u32 {
    let mut values = values;
    values.sort_by(|a, b| compare_hands(&a.0, &b.0, part2));

    values.iter().enumerate().map(|(i, (_, bid_amount))| (i as u32 + 1) * bid_amount).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let values = input.lines().map(|l| parse_hand(l)).collect::<Vec<(Vec<CardLabel>, u32)>>();

    let part1 = total_winnings(values.clone(), false);
    println!("Part 1: {}", part1);

    let part2 = total_winnings(values, true);
    println!("Part 2: {}", part2);
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

        assert_eq!(hands.into_iter().map(|h| get_hand_type(&h, false)).collect::<Vec<HandType>>(), vec![
            HandType::OnePair, HandType::ThreeOfAKind, HandType::TwoPair, HandType::TwoPair, HandType::ThreeOfAKind
        ]);
    }

    #[test]
    fn test_sort_hands_example() {
        let mut hands = EXAMPLE_INPUT.lines().map(|l| parse_hand(l).0).collect::<Vec<Vec<CardLabel>>>();
        hands.sort_by(|a, b| compare_hands(a, b, false));

        assert_eq!(hands, vec![
            vec![CardLabel::Three, CardLabel::Two, CardLabel::T, CardLabel::Three, CardLabel::K],
            vec![CardLabel::K, CardLabel::T, CardLabel::J, CardLabel::J, CardLabel::T],
            vec![CardLabel::K, CardLabel::K, CardLabel::Six, CardLabel::Seven, CardLabel::Seven],
            vec![CardLabel::T, CardLabel::Five, CardLabel::Five, CardLabel::J, CardLabel::Five],
            vec![CardLabel::Q, CardLabel::Q, CardLabel::Q, CardLabel::J, CardLabel::A]
        ])
    }

    #[test]
    fn test_sort_hands_example_part2() {
        let mut hands = EXAMPLE_INPUT.lines().map(|l| parse_hand(l).0).collect::<Vec<Vec<CardLabel>>>();
        hands.sort_by(|a, b| compare_hands(a, b, true));

        assert_eq!(hands, vec![
            vec![CardLabel::Three, CardLabel::Two, CardLabel::T, CardLabel::Three, CardLabel::K],
            vec![CardLabel::K, CardLabel::K, CardLabel::Six, CardLabel::Seven, CardLabel::Seven],
            vec![CardLabel::T, CardLabel::Five, CardLabel::Five, CardLabel::J, CardLabel::Five],
            vec![CardLabel::Q, CardLabel::Q, CardLabel::Q, CardLabel::J, CardLabel::A],
            vec![CardLabel::K, CardLabel::T, CardLabel::J, CardLabel::J, CardLabel::T]
        ])
    }

    #[test]
    fn test_total_winnings() {
        let input = EXAMPLE_INPUT.lines().map(|l| parse_hand(l)).collect::<Vec<(Vec<CardLabel>, u32)>>();

        assert_eq!(total_winnings(input, false), 6440);
    }

    #[test]
    fn test_hand_types_part2() {
        let hands = EXAMPLE_INPUT.lines().map(|l| parse_hand(l).0).collect::<Vec<Vec<CardLabel>>>();

        assert_eq!(hands.into_iter().map(|h| get_hand_type(&h, true)).collect::<Vec<HandType>>(), vec![
            HandType::OnePair, HandType::FourOfAKind, HandType::TwoPair, HandType::FourOfAKind, HandType::FourOfAKind
        ]);
    }

    #[test]
    fn test_total_winnings_part2() {
        let input = EXAMPLE_INPUT.lines().map(|l| parse_hand(l)).collect::<Vec<(Vec<CardLabel>, u32)>>();

        assert_eq!(total_winnings(input, true), 5905);
    }

    #[test]
    fn test_part_2_ordering() {
        let mut hands = vec![
            vec![CardLabel::Two, CardLabel::Two, CardLabel::Two, CardLabel::Two, CardLabel::Two],
            vec![CardLabel::J, CardLabel::J, CardLabel::J, CardLabel::J, CardLabel::J]
        ];
        hands.sort_by(|a, b| compare_hands(a, b, true));

        assert_eq!(hands, vec![
            vec![CardLabel::J, CardLabel::J, CardLabel::J, CardLabel::J, CardLabel::J],
            vec![CardLabel::Two, CardLabel::Two, CardLabel::Two, CardLabel::Two, CardLabel::Two],
        ]);
    }
}
