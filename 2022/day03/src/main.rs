use std::collections::HashSet;
use std::fs;

fn parse_bags(input: &String) -> Vec<(&str, &str)> {
    // read each line and split each in half
    input.lines().map(|line| line.split_at(line.len() / 2)).collect()
}

fn get_priority(c: char) -> u8 {
    // convert a char into priority value
    let mut priority: u8 = c as u8;
    if c.is_ascii_uppercase() {
        priority -= 38;
    } else if c.is_ascii_lowercase() {
        priority -= 96;
    }

    priority
}

fn multi_intersect(items: &Vec<&str>) -> Vec<char> {
    // convert all of the strs into hashsets of chars
    let mut hashsets = items.iter().map(|i| i.chars().collect::<HashSet<char>>());
    // get the first hashset
    let mut intersect = hashsets.next().unwrap();
    // for all the other hashsets, intersect them with the first one
    for hashset in hashsets {
        intersect = intersect.intersection(&hashset).copied().collect();
    }
    // now left with the chars common between all the strs
    intersect.into_iter().collect()
}

fn priority_totals(bags: Vec<(&str, &str)>) -> i32 {
    bags.iter().map(|bag| {
        // convert the tuple to a vec
        let mut bags_vec = Vec::new();
        bags_vec.push(bag.0);
        bags_vec.push(bag.1);

        // get the common characters between both bags and convert them to priorities
        multi_intersect(&bags_vec).iter().map(|c| get_priority(*c) as i32).sum::<i32>()
    }).sum()
}

fn badge_priority_totals(input: &String) -> i32 {
    let lines = input.lines().collect::<Vec<&str>>();
    // group the lines into 3s
    let groups = lines.chunks(3).collect::<Vec<&[&str]>>();

    groups.iter().map(|group| {
        // for each group, find the common characters and convert them to priorities
        multi_intersect(&group.to_vec()).iter().map(|c| get_priority(*c) as i32).sum::<i32>()
    }).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let bags = parse_bags(&input);

    println!("Part 1: {}", priority_totals(bags));
    println!("Part 2: {}", badge_priority_totals(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn parse_bags_example() {
        let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        assert_eq!(parse_bags(&example), vec![
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            ("PmmdzqPrV", "vPwwTWBwg"),
            ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            ("ttgJtRGJ", "QctTZtZT"),
            ("CrZsJsPPZsGz", "wwsLwLmpwMDw")
        ]);
    }

    #[test]
    fn get_priority_example() {
        let input = vec!['p', 'L', 'P', 'v', 't', 's'];
        assert_eq!(input.iter().map(|x| get_priority(*x)).collect::<Vec<_>>(),
            vec![16_u8, 38_u8, 42_u8, 22_u8, 20_u8, 19_u8]);
    }

    #[test]
    fn multi_intersect_example() {
        assert_eq!(vec![
            vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"],
            vec!["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"],
            vec!["PmmdzqPrV", "vPwwTWBwg"],
            vec!["wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"],
            vec!["ttgJtRGJ", "QctTZtZT"],
            vec!["CrZsJsPPZsGz", "wwsLwLmpwMDw"]
        ].iter().map(|group| multi_intersect(group)).collect::<Vec<Vec<char>>>(),
        vec![
            vec!['p'], vec!['L'], vec!['P'], vec!['v'], vec!['t'], vec!['s']
        ]);
    }

    #[test]
    fn priority_totals_example() {
        assert_eq!(priority_totals(vec![
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            ("PmmdzqPrV", "vPwwTWBwg"),
            ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            ("ttgJtRGJ", "QctTZtZT"),
            ("CrZsJsPPZsGz", "wwsLwLmpwMDw")
        ]), 157);
    }

    #[test]
    fn badge_priority_totals_example() {
        let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        assert_eq!(badge_priority_totals(&example), 70);
    }
}