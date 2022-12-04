use itertools::Itertools;
use std::fs;

fn full_overlap(a: (i32, i32), b: (i32, i32)) -> bool {
    // does a fully contain b, or vice-versa
    (a.0 >= b.0 && a.1 <= b.1) || (a.0 <= b.0 && a.1 >= b.1)
}

fn any_overlap(a: (i32, i32), b: (i32, i32)) -> bool {
    // same as above but whether either starts/ends overlap too
    full_overlap(a, b) || (a.1 >= b.0 && a.0 <= b.0) || (a.0 <= b.1 && a.1 >= b.1)
}

fn parse_input(input: &String) -> Vec<((i32, i32), (i32, i32))> {
    // split into lines, then by comma to get each section
    input.lines().map(|line| line.split(',')
        .map(|assignment| {
            // split by hyphen to get start and end of range
            let split = assignment.split_once('-').unwrap();
            // parse each number as integer and put in tuple
            (split.0.parse::<i32>().unwrap(), split.1.parse::<i32>().unwrap())
        }).collect_tuple().unwrap()
    ).collect::<Vec<_>>()
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let assignments = parse_input(&input);
    // apply overlap functions to all assignments, filter by true values then count items
    let full_overlaps = assignments.iter()
        .map(|assignment| full_overlap(assignment.0, assignment.1))
        .filter(|x| *x).count();
    let any_overlaps = assignments.iter()
        .map(|assignment| any_overlap(assignment.0, assignment.1))
        .filter(|x| *x).count();

    println!("Part 1: {}", full_overlaps);
    println!("Part 2: {}", any_overlaps);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_input_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        assert_eq!(parse_input(&input), vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8))
        ]);
    }

    #[test]
    fn full_overlap_example() {
        assert_eq!(vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8))
        ].iter().map(|x| full_overlap(x.0, x.1)).collect::<Vec<_>>(), vec![
            false,
            false,
            false,
            true,
            true,
            false
        ]);
    }

    #[test]
    fn any_overlap_example() {
        assert_eq!(vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8))
        ].iter().map(|x| any_overlap(x.0, x.1)).collect::<Vec<_>>(), vec![
            false,
            false,
            true,
            true,
            true,
            true
        ]);
    }
}