use std::iter::zip;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn variants() -> Vec<Self> {
        vec![Operation::Add, Operation::Multiply]
    }
}

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

fn operation_combinations(l: usize) -> Vec<Vec<Operation>> {
    std::iter::repeat(Operation::variants())
        .take(l)
        .multi_cartesian_product()
        .collect()
}

fn calculate_brute_force(eq: &Equation) -> bool {
    for operations in operation_combinations(eq.numbers.len()) {
        let mut numbers_iter = eq.numbers.clone().into_iter();
        let mut result = numbers_iter.next().unwrap();

        for (n, op) in zip(numbers_iter, operations) {
            match op {
                Operation::Add => result += n,
                Operation::Multiply => result *= n,
            }
        }

        if result == eq.result {
            return true;
        }
    }

    false
}

fn part1(eqs: Vec<Equation>) -> i64 {
    eqs.iter()
        .filter(|eq| calculate_brute_force(eq))
        .map(|eq| eq.result)
        .sum()
}

fn main() {
    let eqs = parse_input(&aocutils::read_input("input").unwrap());

    println!("part 1: {}", part1(eqs));
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
    fn test_operation_combinations() {
        assert_eq!(
            operation_combinations(2),
            Vec::from([
                Vec::from([Operation::Add, Operation::Add]),
                Vec::from([Operation::Add, Operation::Multiply]),
                Vec::from([Operation::Multiply, Operation::Add]),
                Vec::from([Operation::Multiply, Operation::Multiply])
            ])
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
        assert_eq!(part1(parse_input(EXAMPLE_INPUT)), 3749);
    }
}
