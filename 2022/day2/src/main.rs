use std::fs;

fn main() {
    let input = parse_guide(&fs::read_to_string("inputs/input.txt")
            .expect("should be able to read input"));

    println!("Part 1: {}", score_part_1(&input));
    println!("Part 2: {}", score_part_2(&input));
}

fn parse_guide(guide: &String) -> Vec<(char, char)> {
    // split guide by each line, then convert each line to a tuple with the
    // first and second move
    guide.lines().map(|line| {
        let mut chars = line.chars();
        (chars.next().unwrap(), chars.skip(1).next().unwrap())
    }).collect()
}

fn shape_score(shape: &char) -> i32 {
    match shape {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0
    }
}

fn outcome_score(round: &(char, char)) -> i32 {
    // convert the second character to be A, B or C for comparisons
    let p1 = ((round.1 as u8) - 23) as char;

    if round.0 == p1 {
        // draw
        3
    } else if (p1 == 'B' && round.0 == 'A') ||
            (p1 == 'C' && round.0 == 'B') ||
            (p1 == 'A' && round.0 == 'C') {
        // p1 wins
        6
    } else {
        // p0 wins
        0
    }
}

fn get_shape(round: &(char, char)) -> char {
    if round.1 == 'Y' {
        return ((round.0 as u8) + 23) as char;
    }

    match round {
        ('A', 'X') => 'Z', // rock, lose
        ('B', 'X') => 'X', // paper, lose
        ('C', 'X') => 'Y', // scissors, lose
        ('A', 'Z') => 'Y', // rock, win
        ('B', 'Z') => 'Z', // paper, win
        ('C', 'Z') => 'X', // scissors, win
        _ => ' '
    }
}

fn score_part_1(guide: &Vec<(char, char)>) -> i32 {
    guide.iter().map(|round| shape_score(&round.1) + outcome_score(round)).sum()
}

fn score_part_2(guide: &Vec<(char, char)>) -> i32 {
    guide.iter().map(|round| {
        let p1 = get_shape(&round);
        let outcome = {
            match round.1 {
                'Y' => 3,
                'Z' => 6,
                _ => 0
            }
        };

        shape_score(&p1) + outcome
    }).sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parse_guide_example() {
        let example = fs::read_to_string("inputs/example.txt")
            .expect("should be able to read input");
        assert_eq!(parse_guide(&example), vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')])
    }

    #[test]
    fn shape_score_1() {
        assert_eq!(shape_score(&'Y'), 2)
    }

    #[test]
    fn shape_score_2() {
        assert_eq!(shape_score(&'X'), 1)
    }

    #[test]
    fn shape_score_3() {
        assert_eq!(shape_score(&'Z'), 3)
    }

    #[test]
    fn outcome_score_1() {
        assert_eq!(outcome_score(&('A', 'Y')), 6)
    }

    #[test]
    fn outcome_score_2() {
        assert_eq!(outcome_score(&('B', 'X')), 0)
    }

    #[test]
    fn outcome_score_3() {
        assert_eq!(outcome_score(&('C', 'Z')), 3)
    }

    #[test]
    fn score_part_1_example() {
        assert_eq!(score_part_1(&vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')]), 15)
    }

    #[test]
    fn get_shape_1() {
        assert_eq!(get_shape(&('A', 'Y')), 'X')
    }

    #[test]
    fn get_shape_2() {
        assert_eq!(get_shape(&('B', 'X')), 'X')
    }

    #[test]
    fn get_shape_3() {
        assert_eq!(get_shape(&('C', 'Z')), 'X')
    }

    #[test]
    fn score_part_2_example() {
        assert_eq!(score_part_2(&vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')]), 12)
    }
}
