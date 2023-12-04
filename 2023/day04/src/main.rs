use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Scratchcard {
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>
}

impl Scratchcard {
    fn matching_numbers(&self) -> u32 {
        self.numbers.iter().map(|i| self.winning_numbers.contains(i)).filter(|x| *x).count() as u32
    }

    fn score(&self) -> i32 {
        let num_winning = self.matching_numbers();
        if num_winning == 0 {
            0
        } else {
            1 * i32::pow(2, num_winning as u32 - 1)
        }
    }
}

fn parse_scratchcards(s: &str) -> Vec<(i32, Scratchcard)> {
    let mut scratchcards: Vec<(i32, Scratchcard)> = Vec::new();

    for l in s.lines() {
        let (header, all_numbers_str) = l.split_once(": ").unwrap();
        let (_, id) = header.split_once(" ").unwrap();
        let (winning_numbers_str, numbers_str) = all_numbers_str.split_once(" | ").unwrap();

        let winning_numbers = winning_numbers_str.split(" ").map(|s| s.trim()).filter(|s| s != &"").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let numbers = numbers_str.split(" ").map(|s| s.trim()).filter(|s| s != &"").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        
        scratchcards.push((id.trim().parse::<i32>().unwrap(), Scratchcard { winning_numbers, numbers }))
    }
    
    scratchcards
}

fn process_scratchcard_wins(cards: Vec<(i32, Scratchcard)>) -> Vec<i32> {
    let mut inventory: Vec<i32> = cards.iter().map(|x| x.0).collect();
    let scratchcard_map: HashMap<i32, Scratchcard> = cards.into_iter().collect();

    let mut index = 0;

    while index < inventory.len() {
        let card_id = inventory.get(index).unwrap();
        let card = scratchcard_map.get(card_id).expect("should have scratchcard by id");
        let matching = card.matching_numbers();
//        println!("Processing #{} (Card {}) -> {} matching", index, card_id, matching);

        for i in (*card_id+1)..(*card_id + (matching as i32) + 1) {
//            println!(" - Won card {}", i);
            inventory.push(i);
        }

        index += 1;
    }

    inventory
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let cards = parse_scratchcards(&input);
    
    let part1: i32 = cards.iter().map(|c| c.1.score()).sum();
    println!("Part 1: {}", part1);

    let part2 = process_scratchcard_wins(cards).len();
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    
    #[test]
    fn test_parse_scratchcards() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        
        let actual = parse_scratchcards(&input);
        let expected = vec![(1, Scratchcard { winning_numbers: vec![41, 48, 83, 86, 17], numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] })];
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_score() {
        let card = Scratchcard { winning_numbers: vec![41, 48, 83, 86, 17], numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] };
        
        assert_eq!(card.score(), 8);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let cards = parse_scratchcards(&input);
        
        let scores = cards.iter().map(|c| c.1.score()).collect::<Vec<i32>>();
        
        assert_eq!(scores, vec![8, 2, 2, 1, 0, 0]);
    }

    #[test]
    fn test_process_scratchcard_wins_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let cards = parse_scratchcards(&input);

        let mut processed = process_scratchcard_wins(cards);
        processed.sort();

        assert_eq!(processed, vec![1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6])
    }
}