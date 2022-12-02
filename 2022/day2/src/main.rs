use std::fs;

// slice with all shapes and outcomes. elements can be found by using 2 values
// to find the missing one
const OUTCOMES: [(char, char, i32); 9] = [
    // p0 shape, p1 shape, outcome (as p1)
    ('A', 'X', 3), // rock + rock = draw
    ('A', 'Y', 6), // rock + paper = win
    ('A', 'Z', 0), // rock + scissors = loss
    ('B', 'X', 0), // paper + rock = loss
    ('B', 'Y', 3), // paper + paper = draw
    ('B', 'Z', 6), // paper + scissors = win
    ('C', 'X', 6), // scissors + rock = win
    ('C', 'Y', 0), // scissors + rock = loss
    ('C', 'Z', 3) // scissors + scissors = draw
];

fn main() {
    let input = parse_guide(&fs::read_to_string("inputs/input.txt")
            .expect("should be able to read input"));

    println!("Part 1: {}", input.iter().map(|round| score_part_1(&round)).sum::<i32>());
    println!("Part 2: {}", input.iter().map(|round| score_part_2(&round)).sum::<i32>());
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
    // convert a shape into the shape score
    match shape {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0
    }
}

fn score_part_1(round: &(char, char)) -> i32 {
    // find the outcome score by looking up the two played moves
    let outcome_score = OUTCOMES.iter()
        .find(|x| x.0 == round.0 && x.1 == round.1)
        .unwrap().2;

    // calculate shape score + outcome score
    shape_score(&round.1) + outcome_score
}

fn score_part_2(round: &(char, char)) -> i32 {
    // convert x/y/z into an outcome score
    let outcome_score = {
        match round.1 {
            'Y' => 3,
            'Z' => 6,
            _ => 0
        }
    };

    // find p1 shape by looking up p0 shape and outcome score
    let p1 = OUTCOMES.iter()
        .find(|x| x.0 == round.0 && x.2 == outcome_score)
        .unwrap().1;

    // calculate shape score + outcome score
    shape_score(&p1) + outcome_score
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parse_guide_example() {
        let example = fs::read_to_string("inputs/example.txt")
            .expect("should be able to read input");
        assert_eq!(parse_guide(&example), vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')]);
    }

    #[test]
    fn shape_score_1() {
        assert_eq!(shape_score(&'Y'), 2);
    }

    #[test]
    fn shape_score_2() {
        assert_eq!(shape_score(&'X'), 1);
    }

    #[test]
    fn shape_score_3() {
        assert_eq!(shape_score(&'Z'), 3);
    }

    #[test]
    fn score_part_1_1() {
        assert_eq!(score_part_1(&('A', 'Y')), 8);
    }

    #[test]
    fn score_part_1_2() {
        assert_eq!(score_part_1(&('B', 'X')), 1);
    }

    #[test]
    fn score_part_1_3() {
        assert_eq!(score_part_1(&('C', 'Z')), 6);
    }

    #[test]
    fn score_part_2_1() {
        assert_eq!(score_part_2(&('A', 'Y')), 4);
    }

    #[test]
    fn score_part_2_2() {
        assert_eq!(score_part_2(&('B', 'X')), 1);
    }

    #[test]
    fn score_part_2_3() {
        assert_eq!(score_part_2(&('C', 'Z')), 7);
    }
}
