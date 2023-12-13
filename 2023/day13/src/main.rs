use std::fs;

use itertools::Itertools;

type Pattern = Vec<Vec<char>>;

fn parse_pattern(s: &str) -> Pattern {
    s.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn find_horizonal_symmetry_lines(p: &Pattern) -> Vec<usize> {
    let mut lines: Vec<usize> = Vec::new();
    
    for i in 1..p.len() {
        let (top, bottom) = p.split_at(i);
        
        let mut top_iter = top.iter().rev();
        let mut bottom_iter = bottom.iter();
        let mut is_symmetrical = true;
        
        loop {
            if let (Some(next_top), Some(next_bottom)) = (top_iter.next(), bottom_iter.next()) {
                #[cfg(test)]
                println!("{} -> top {}: {:?}, bottom {}: {:?}", i, top_iter.len(), next_top, bottom_iter.len(), next_bottom);
                
                if next_top != next_bottom {
                    #[cfg(test)]
                    println!("  not symmetrical");
                    is_symmetrical = false;
                    break;
                }
            } else {
                #[cfg(test)]
                println!("  completed check");
                break;
            }
        }
        
        if is_symmetrical {
            lines.push(i);
        }
    }
    
    lines
}

fn find_vertical_symmetry_lines(p: &Pattern) -> Vec<usize> {
    // transpose
    let mut transposed_pattern = vec![Vec::with_capacity(p.len()); p[0].len()];
    for item in p {
        for i in 0..item.len() {
            transposed_pattern[i].push(item[i]);
        }
    }
    
    find_horizonal_symmetry_lines(&transposed_pattern)
}

fn find_symmetry_lines(p: &Pattern) -> (Vec<usize>, Vec<usize>) {
    (find_horizonal_symmetry_lines(p), find_vertical_symmetry_lines(p))
}

fn summarize(patterns: &Vec<Pattern>) -> usize {
    let mut cols_left_of_vertical = 0;
    let mut rows_above_horizontal = 0;
    
    for pattern in patterns {
        let symmetry_lines = find_symmetry_lines(pattern);
        cols_left_of_vertical += symmetry_lines.1.iter().sum::<usize>();
        rows_above_horizontal += symmetry_lines.0.iter().sum::<usize>();
    }
    
    cols_left_of_vertical + (100 * rows_above_horizontal)
}

fn main() {
    let input = fs::read_to_string("inputs/input").expect("should be able to read input");
    let patterns: Vec<Pattern> = input.split("\n\n").map(|l| parse_pattern(l)).collect_vec();
    
    let part1 = summarize(&patterns);
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    
    static EXAMPLE_PATTERN1: Lazy<Pattern> = Lazy::new(|| parse_pattern("#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#."));
    static EXAMPLE_PATTERN2: Lazy<Pattern> = Lazy::new(|| parse_pattern("#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#"));
    
    #[test]
    fn test_find_symmetry_lines_ex2() {
        let actual = find_symmetry_lines(&EXAMPLE_PATTERN2);
        let expected = (vec![4], vec![]);
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_symmetry_lines_ex1() {
        let actual = find_symmetry_lines(&EXAMPLE_PATTERN1);
        let expected = (vec![], vec![5]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_summarize_examples() {
        let patterns: Vec<Pattern> = vec![EXAMPLE_PATTERN1.to_vec(), EXAMPLE_PATTERN2.to_vec()];
        
        let actual = summarize(&patterns);
        let expected = 405;
        
        assert_eq!(actual, expected);
    }
}