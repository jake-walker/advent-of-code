use std::fs;

use itertools::Itertools;

type Pattern = Vec<Vec<char>>;

fn parse_pattern(s: &str) -> Pattern {
    s.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

/// count the number of differences between to vecs
fn count_differences<T: std::cmp::Eq>(a: &Vec<T>, b: &Vec<T>) -> usize {
    a.into_iter().zip(b).filter(|(ax, bx)| ax != bx).count()
}

fn find_horizonal_symmetry_lines(p: &Pattern, part2: bool) -> Vec<usize> {
    let mut lines: Vec<usize> = Vec::new();

    // loop through each line in the pattern
    for i in 1..p.len() {
        // split the pattern to above and below the symmetry line that is being tested
        let (top, bottom) = p.split_at(i);
        
        let mut top_iter = top.iter().rev();
        let mut bottom_iter = bottom.iter();
        let mut is_symmetrical = true;
        let mut difference_count = 0;  // for part 2

        // loop through all each set of lines until we finish one or both lists
        loop {
            if let (Some(next_top), Some(next_bottom)) = (top_iter.next(), bottom_iter.next()) {
                #[cfg(test)]
                println!("{} -> top {}: {:?}, bottom {}: {:?}", i, top_iter.len(), next_top, bottom_iter.len(), next_bottom);
                
                if part2 {
                    // if we're completing part 2, update the differences count
                    difference_count += count_differences(next_top, next_bottom);

                    // scrap the search if there's too many differences
                    if difference_count > 1 {
                        #[cfg(test)]
                        println!("  difference count {} > 1", difference_count);
                        is_symmetrical = false;
                        break;
                    }
                } else if next_top != next_bottom {
                    // otherwise, if it's *not* part 2, and the lines are not equal, then this is not a line of symmetry
                    #[cfg(test)]
                    println!("  not symmetrical");
                    is_symmetrical = false;
                    break;
                }
            } else {
                // here, we have run out of lines either above or below, so the loop can be exited
                #[cfg(test)]
                println!("  completed check");
                break;
            }
        }
        
        // for part 1, we only care if the line is symmetrical
        // for part 2, it has to be symmetrical with exactly 1 difference
        if (!part2 && is_symmetrical) || (part2 && is_symmetrical && difference_count == 1) {
            lines.push(i);
        }
    }
    
    lines
}

fn find_vertical_symmetry_lines(p: &Pattern, part2: bool) -> Vec<usize> {
    // finding vertical lines is the same, so the vec can be transposed and passed into the horizontal function
    let mut transposed_pattern = vec![Vec::with_capacity(p.len()); p[0].len()];
    for item in p {
        for i in 0..item.len() {
            transposed_pattern[i].push(item[i]);
        }
    }
    
    find_horizonal_symmetry_lines(&transposed_pattern, part2)
}

fn find_symmetry_lines(p: &Pattern, part2: bool) -> (Vec<usize>, Vec<usize>) {
    (find_horizonal_symmetry_lines(p, part2), find_vertical_symmetry_lines(p, part2))
}

fn summarize(patterns: &Vec<Pattern>, part2: bool) -> usize {
    let mut cols_left_of_vertical = 0;
    let mut rows_above_horizontal = 0;
    
    for pattern in patterns {
        let symmetry_lines = find_symmetry_lines(pattern, part2);
        cols_left_of_vertical += symmetry_lines.1.iter().sum::<usize>();
        rows_above_horizontal += symmetry_lines.0.iter().sum::<usize>();
    }
    
    cols_left_of_vertical + (100 * rows_above_horizontal)
}

fn main() {
    let input = fs::read_to_string("inputs/input").expect("should be able to read input");
    let patterns: Vec<Pattern> = input.split("\n\n").map(|l| parse_pattern(l)).collect_vec();
    
    let part1 = summarize(&patterns, false);
    println!("Part 1: {}", part1);

    let part2 = summarize(&patterns, true);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    
    static EXAMPLE_PATTERN1: Lazy<Pattern> = Lazy::new(|| parse_pattern("#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#."));
    static EXAMPLE_PATTERN2: Lazy<Pattern> = Lazy::new(|| parse_pattern("#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#"));
    
    #[test]
    fn test_find_symmetry_lines_ex2() {
        let actual = find_symmetry_lines(&EXAMPLE_PATTERN2, false);
        let expected = (vec![4], vec![]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_symmetry_lines_ex1() {
        let actual = find_symmetry_lines(&EXAMPLE_PATTERN1, false);
        let expected = (vec![], vec![5]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_symmetry_lines_ex2_part2() {
        let actual = find_symmetry_lines(&EXAMPLE_PATTERN2, true);
        let expected = (vec![1], vec![]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_symmetry_lines_ex1_part2() {
        let actual = find_symmetry_lines(&EXAMPLE_PATTERN1, true);
        let expected = (vec![3], vec![]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_summarize_examples() {
        let patterns: Vec<Pattern> = vec![EXAMPLE_PATTERN1.to_vec(), EXAMPLE_PATTERN2.to_vec()];

        let actual = summarize(&patterns, false);
        let expected = 405;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_summarize_examples_part2() {
        let patterns: Vec<Pattern> = vec![EXAMPLE_PATTERN1.to_vec(), EXAMPLE_PATTERN2.to_vec()];

        let actual = summarize(&patterns, true);
        let expected = 400;

        assert_eq!(actual, expected);
    }
}