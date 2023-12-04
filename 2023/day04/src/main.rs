use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Scratchcard {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>
}

impl Scratchcard {
    fn score(&self) -> i32 {
        let num_winning = self.numbers.iter().map(|i| self.winning_numbers.contains(i)).filter(|x| *x).count();
        if num_winning == 0 {
            0
        } else {
            1 * i32::pow(2, num_winning as u32 - 1)
        }
    }
}

fn parse_scratchcards(s: &str) -> Vec<Scratchcard> {
    let mut scratchcards: Vec<Scratchcard> = Vec::new();

    for l in s.lines() {
        let (header, all_numbers_str) = l.split_once(": ").unwrap();
        let (_, id) = header.split_once(" ").unwrap();
        let (winning_numbers_str, numbers_str) = all_numbers_str.split_once(" | ").unwrap();

        let winning_numbers = winning_numbers_str.split(" ").map(|s| s.trim()).filter(|s| s != &"").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let numbers = numbers_str.split(" ").map(|s| s.trim()).filter(|s| s != &"").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        
        scratchcards.push(Scratchcard { id: id.trim().parse::<i32>().unwrap(), winning_numbers, numbers })
    }
    
    scratchcards
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let cards = parse_scratchcards(&input);
    
    let part1: i32 = cards.iter().map(|c| c.score()).sum();
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    
    #[test]
    fn test_parse_scratchcards() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        
        let actual = parse_scratchcards(&input);
        let expected = vec![Scratchcard { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] }];
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_score() {
        let card = Scratchcard { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] };
        
        assert_eq!(card.score(), 8);
    }

    #[test]
    fn test_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let cards = parse_scratchcards(&input);
        
        let scores = cards.iter().map(|c| c.score()).collect::<Vec<i32>>();
        
        assert_eq!(scores, vec![8, 2, 2, 1, 0, 0]);
    }
}