use std::fs;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
struct SpringsGroup {
    conditions: Vec<char>,
    group_lengths: Vec<u32>
}

impl SpringsGroup {
    fn is_valid(&self) -> bool {
        let mut groups: Vec<u32> = Vec::new();
        let mut group_length: u32 = 0;

        for condition in self.conditions.iter() {
            if condition == &'.' {
                if group_length != 0 {
                    groups.push(group_length);
                    group_length = 0;
                }
            } else if condition == &'#' {
                group_length += 1
            } else {
                println!("unexpected condition {}", condition);
                return false
            }
        }

        if group_length != 0 {
            groups.push(group_length);
        }

        groups == self.group_lengths
    }

    fn wildcard_locations(&self) -> Vec<usize> {
        self.conditions.iter().enumerate().filter(|(_, &c)| c == '?').map(|(i, _)| i).collect()
    }
}

fn parse_line(s: &str) -> SpringsGroup {
    let (conditions, group_lengths) = s.split_once(" ").unwrap();

    SpringsGroup {
        conditions: conditions.chars().collect::<Vec<char>>(),
        group_lengths: group_lengths.split(",").map(|n| n.parse::<u32>().unwrap()).collect()
    }
}

fn find_combinations(group: &SpringsGroup) -> Vec<SpringsGroup> {
    let wildcards = group.wildcard_locations();
    let mut output = Vec::new();

    #[cfg(test)]
    println!("Finding combinations for {:?}", group.conditions);

    for combination in itertools::repeat_n(vec!['.', '#'], wildcards.len()).multi_cartesian_product() {
        let mut new_spring_group = group.clone();

        for (i, new_char) in combination.into_iter().enumerate() {
            let replace_index = wildcards[i];
            new_spring_group.conditions[replace_index] = new_char;
        }

        if new_spring_group.is_valid() {
            output.push(new_spring_group);
        }
    }

    output
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let groups = input.lines().map(|l| parse_line(l)).collect_vec();

    let part1 = groups.iter().map(|g| find_combinations(g).len()).sum::<usize>();
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("???.### 1,1,3"), SpringsGroup {
            conditions: vec!['?', '?', '?', '.', '#', '#', '#'],
            group_lengths: vec![1, 1, 3]
        })
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(
            vec![
                "#.#.### 1,1,3",
                ".#...#....###. 1,1,3",
                ".#.###.#.###### 1,3,1,6",
                "####.#...#... 4,1,1",
                "#....######..#####. 1,6,5",
                ".###.##....# 3,2,1",
                "....### 1,1,3",
                "##..### 1,1,3"
            ].iter().map(|l| parse_line(l).is_valid()).collect::<Vec<bool>>(),
            vec![true, true, true, true, true, true, false, false]
        )
    }

    #[test]
    fn test_find_combinations() {
        let group = SpringsGroup { conditions: "?###????????".chars().collect::<Vec<char>>(), group_lengths: vec![3, 2, 1] };

        let mut actual = find_combinations(&group).iter().map(|g| g.conditions.iter().collect::<String>()).collect::<Vec<String>>();
        let mut expected = vec![
            ".###.##.#...".to_string(),
            ".###.##..#..".to_string(),
            ".###.##...#.".to_string(),
            ".###.##....#".to_string(),
            ".###..##.#..".to_string(),
            ".###..##..#.".to_string(),
            ".###..##...#".to_string(),
            ".###...##.#.".to_string(),
            ".###...##..#".to_string(),
            ".###....##.#".to_string()
        ];

        actual.sort();
        expected.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_total_arrangements() {
        let actual = EXAMPLE_INPUT.lines().map(|l| find_combinations(&parse_line(l)).len()).sum::<usize>();
        let expected = 21;

        assert_eq!(actual, expected);
    }
}
