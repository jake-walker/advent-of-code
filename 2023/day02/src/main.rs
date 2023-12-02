use std::{collections::HashMap, fs};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum CubeColor {
    Red,
    Green,
    Blue
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: i32,
    sets: Vec<HashMap<CubeColor, i32>>
}

fn parse_game(s: &str) -> Game {
    let pattern = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    let captures = pattern.captures(s).expect("should be able to parse game");
    let mut sets: Vec<HashMap<CubeColor, i32>> = Vec::new();
    
    for group in captures.get(2).unwrap().as_str().split("; ") {
        let mut set: HashMap<CubeColor, i32> = HashMap::new();

        for item in group.split(", ") {
            let mut parts = item.split(" ");
            let qty = parts.next().unwrap().parse::<i32>().unwrap();
            let color = {
                match parts.next().unwrap() {
                    "red" => CubeColor::Red,
                    "green" => CubeColor::Green,
                    "blue" => CubeColor::Blue,
                    _ => panic!("unexpected color")
                }
            };
            
            set.insert(color, qty);
        }
        
        sets.push(set);
    }
    
    Game {
        id: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        sets: sets
    }
}

fn is_game_possible(g: &Game, maxes: &HashMap<CubeColor, i32>) -> bool {
    for set in g.sets.iter() {
        for (color, qty) in set {
            let max = maxes.get(&color).unwrap_or(&0);
            if qty > max {
                return false;
            }
        }
    }
    
    true
}

fn minimum_cubes(g: &Game) -> HashMap<CubeColor, i32> {
    let mut mins = HashMap::new();

    for set in g.sets.iter() {
        for (color, qty) in set {
            if !mins.contains_key(color) || qty > mins.get(color).unwrap_or(&0) {
                mins.insert(*color, *qty);
            }
        }
    }
    
    mins
}

fn part1(games: &Vec<Game>, maxes: &HashMap<CubeColor, i32>) -> i32 {
    games.iter().filter(|g| is_game_possible(g, maxes)).map(|g| g.id).sum()
}

fn part2(games: &Vec<Game>) -> i32 {
    games.iter().map(|g| minimum_cubes(g).values().product::<i32>()).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let games = input.lines().map(|l| parse_game(l)).collect::<Vec<Game>>();
    let part_1_maxes = HashMap::from([
        (CubeColor::Red, 12),
        (CubeColor::Green, 13),
        (CubeColor::Blue, 14)
    ]);
    
    let part1: i32 = part1(&games, &part_1_maxes);
    println!("Part 1: {}", part1);
    
    let part2: i32 = part2(&games);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    
    #[test]
    fn test_parse_game_example1() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(parse_game(s), Game {
            id: 1,
            sets: vec![
                HashMap::from([(CubeColor::Blue, 3), (CubeColor::Red, 4)]),
                HashMap::from([(CubeColor::Red, 1), (CubeColor::Green, 2), (CubeColor::Blue, 6)]),
                HashMap::from([(CubeColor::Green, 2)])
            ]
        })
    }

    #[test]
    fn test_minimum_cubes_example1() {
        assert_eq!(minimum_cubes(&Game {
            id: 1,
            sets: vec![
                HashMap::from([(CubeColor::Blue, 3), (CubeColor::Red, 4)]),
                HashMap::from([(CubeColor::Red, 1), (CubeColor::Green, 2), (CubeColor::Blue, 6)]),
                HashMap::from([(CubeColor::Green, 2)])
            ]
        }), HashMap::from([(CubeColor::Red, 4), (CubeColor::Green, 2), (CubeColor::Blue, 6)]))
    }

    #[test]
    fn test_is_game_possible_example() {
        let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let games = example.lines().map(|l| parse_game(l)).collect::<Vec<Game>>();
        
        let part_1_maxes = HashMap::from([
            (CubeColor::Red, 12),
            (CubeColor::Green, 13),
            (CubeColor::Blue, 14)
        ]);
        
        assert_eq!(
            games.iter().map(|g| is_game_possible(g, &part_1_maxes)).collect::<Vec<bool>>(),
            vec![true, true, false, false, true]
        )
    }

    #[test]
    fn test_part_2_example() {
        let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let games = example.lines().map(|l| parse_game(l)).collect::<Vec<Game>>();
        
        assert_eq!(part2(&games), 2286);
    }
}