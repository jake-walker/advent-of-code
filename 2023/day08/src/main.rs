use std::{collections::HashMap, fs};

static START_LOCATION: &str = "AAA";
static END_LOCATION: &str = "ZZZ";

#[derive(Debug, PartialEq, Eq)]
struct Map {
    directions: Vec<char>,
    nodes: HashMap<String, (String, String)>
}

fn parse_map(s: &str) -> Map {
    let sections = s.split_once("\n\n").unwrap();
    
    let directions: Vec<char> = sections.0.trim().chars().collect();
    
    let mut nodes = HashMap::new();
    
    for line in sections.1.trim().lines() {
        let (from_node, to_nodes) = line.split_once(" = ").unwrap();
        let (left_node, right_node) = to_nodes.trim_matches(|c| c == '(' || c == ')').split_once(", ").unwrap();
        
        nodes.insert(from_node.to_string(), (left_node.to_string(), right_node.to_string()));
    }
    
    Map { directions, nodes }
}

fn step_to_end(map: &Map, start_location: &str, end_location: &str) -> Vec<String> {
    let mut locations: Vec<String> = vec![start_location.to_string()];
    
    while *locations.last().unwrap() != end_location {
        let (left, right) = map.nodes.get(locations.last().unwrap()).unwrap();
        let step = locations.len() - 1;
        let direction = map.directions.get(step % map.directions.len()).unwrap();

        if direction == &'L' {
            locations.push(left.clone());
        } else if direction == &'R' {
            locations.push(right.clone());
        } else {
            panic!("unexpected direction {}", direction);
        }
    }
    
    locations
}

fn main() {
    let input = fs::read_to_string("inputs/input").expect("should be able to read input");
    let map = parse_map(&input);
    
    let part1 = step_to_end(&map, START_LOCATION, END_LOCATION).len() - 1;
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    static EXAMPLE_MAP1: &str = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
    static EXAMPLE_MAP2: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    
    #[test]
    fn test_parse_example_map() {
        let actual = parse_map(EXAMPLE_MAP2);
        let expected = Map {
            directions: vec!['L', 'L', 'R'],
            nodes: HashMap::from([
                ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
                ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
                ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string()))
            ])
        };
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_step_to_end_example1() {
        let map = parse_map(EXAMPLE_MAP1);
        
        let actual = step_to_end(&map, START_LOCATION, END_LOCATION);
        let expected = vec!["AAA".to_string(), "CCC".to_string(), "ZZZ".to_string()];
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_step_to_end_example2() {
        let map = parse_map(EXAMPLE_MAP2);

        let actual = step_to_end(&map, START_LOCATION, END_LOCATION).len() - 1;
        let expected = 6_usize;

        assert_eq!(actual, expected);
    }
}