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

fn step_to_end(map: &Map, start_location: &str, is_end: fn (&str) -> bool, step_offset: usize) -> Vec<String> {
    let mut locations: Vec<String> = vec![start_location.to_string()];
    
    let mut finished = false;

    while !finished {
        let (left, right) = map.nodes.get(locations.last().unwrap()).unwrap();
        let step = locations.len() - 1 + step_offset;
        let direction = map.directions.get(step % map.directions.len()).unwrap();

        #[cfg(test)]
        print!("Step {}: from={}", step, locations.last().unwrap());

        if direction == &'L' {
            locations.push(left.clone());
        } else if direction == &'R' {
            locations.push(right.clone());
        } else {
            panic!("unexpected direction {}", direction);
        }

        finished = is_end(locations.last().unwrap());

        #[cfg(test)]
        println!(", to={}, finished={}", locations.last().unwrap(), finished);
    }
    
    locations
}

fn greatest_common_divisor(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }

    greatest_common_divisor(b % a, a)
}

fn lowest_common_multiple(a: i64, b: i64) -> i64 {
    a / greatest_common_divisor(a, b) * b
}

fn step_to_end_simultaneous(map: &Map) -> i64 {
    let start_locations = map.nodes.keys().filter(|n| n.ends_with("A")).collect::<Vec<&String>>();
    let mut step_counts: Vec<usize> = Vec::new();

    for start_location in start_locations {
        let initial_steps = step_to_end(map, start_location, |c| c.ends_with("Z"), 0);
//        let repeating_steps = step_to_end(map, initial_steps.last().unwrap(), |c| c.ends_with("Z"), initial_steps.len() - 1);

//        #[cfg(test)]
//        println!("{} -> {} + {}", start_location, initial_steps.len() - 1, repeating_steps.len() - 1);

        step_counts.push(initial_steps.len() - 1);
    }

    let mut repeat_lcm = step_counts[0] as i64;

    for repeat_steps in step_counts.into_iter().skip(1) {
        repeat_lcm = lowest_common_multiple(repeat_lcm, repeat_steps as i64);
    }

    repeat_lcm
}

fn main() {
    let input = fs::read_to_string("inputs/input").expect("should be able to read input");
    let map = parse_map(&input);

    let part1 = step_to_end(&map, START_LOCATION, |n| n == END_LOCATION, 0).len() - 1;
    println!("Part 1: {}", part1);

    let part2 = step_to_end_simultaneous(&map);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_MAP1: &str = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
    static EXAMPLE_MAP2: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    static EXAMPLE_MAP3: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

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

        let actual = step_to_end(&map, START_LOCATION, |n| n == END_LOCATION, 0);
        let expected = vec!["AAA".to_string(), "CCC".to_string(), "ZZZ".to_string()];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_step_to_end_example2() {
        let map = parse_map(EXAMPLE_MAP2);

        let actual = step_to_end(&map, START_LOCATION, |n| n == END_LOCATION, 0).len() - 1;
        let expected = 6_usize;

        assert_eq!(actual, expected);
    }
}