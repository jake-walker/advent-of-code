use std::iter::zip;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

const PART1_OPERATIONS: [Operation; 2] = [Operation::Add, Operation::Multiply];
const PART2_OPERATIONS: [Operation; 3] =
    [Operation::Add, Operation::Multiply, Operation::Concatenate];

#[derive(Debug, PartialEq, Eq)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (result, numbers) = line.split_once(": ").unwrap();

            Equation {
                result: result.parse().unwrap(),
                numbers: numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn calculate_brute_force(eq: &Equation, operations: &[Operation]) -> bool {
    let op_combos: Vec<Vec<&Operation>> = std::iter::repeat(operations)
        .take(eq.numbers.len() - 1)
        .multi_cartesian_product()
        .collect();

    for operations in op_combos {
        let mut numbers_iter = eq.numbers.clone().into_iter();
        let mut result = numbers_iter.next().unwrap();

        for (n, op) in zip(numbers_iter, operations) {
            match op {
                Operation::Add => result += n,
                Operation::Multiply => result *= n,
                Operation::Concatenate => result = format!("{}{}", result, n).parse().unwrap(),
            }
        }

        if result == eq.result {
            return true;
        }
    }

    false
}

fn sum_valid_equations(eqs: &Vec<Equation>, operations: &[Operation]) -> i64 {
    eqs.iter()
        .filter(|eq| calculate_brute_force(eq, operations))
        .map(|eq| eq.result)
        .sum()
}

fn main() {
    let eqs = parse_input(&aocutils::read_input("input").unwrap());

    println!("part 1: {}", sum_valid_equations(&eqs, &PART1_OPERATIONS));
    println!("part 2: {}", sum_valid_equations(&eqs, &PART2_OPERATIONS));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(EXAMPLE_INPUT);
        assert_eq!(parsed.len(), 9);
        assert_eq!(
            parsed[0],
            Equation {
                numbers: vec![10, 19],
                result: 190
            }
        )
    }

    #[test]
    fn test_calculate_brute_force_part1() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT)
                .iter()
                .map(|eq| calculate_brute_force(eq, &PART1_OPERATIONS))
                .collect::<Vec<bool>>(),
            vec![true, true, false, false, false, false, false, false, true]
        )
    }

    #[test]
    fn test_calculate_brute_force() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT)
                .iter()
                .map(|eq| calculate_brute_force(eq))
                .collect::<Vec<bool>>(),
            vec![true, true, false, false, false, false, false, false, true]
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            sum_valid_equations(&parse_input(EXAMPLE_INPUT), &PART1_OPERATIONS),
            3749
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            sum_valid_equations(&parse_input(EXAMPLE_INPUT), &PART2_OPERATIONS),
            11387
        );
    }
}
