use std::{error::Error, ops::Mul};

use regex::Regex;

fn extract_instructions(s: &str) -> Vec<(i32, i32)> {
    // use regex to extract mul instructions that match the format
    // each number in the operation is in a capture group
    let mul_re = Regex::new(r"mul\((-?\d{1,3}),(-?\d{1,3})\)").unwrap();

    mul_re
        // loop over regex captures
        .captures_iter(s)
        .map(|c| {
            // extract the captures
            let captures = c.extract::<2>();
            // parse the first and second numbers as integers
            (
                captures.1[0].parse().unwrap(),
                captures.1[1].parse().unwrap(),
            )
        })
        // bring together our map into a vec/list
        .collect::<Vec<(i32, i32)>>()
}

fn sum_mul_instructions(mul_ops: &Vec<(i32, i32)>) -> i32 {
    mul_ops.iter().map(|(a, b)| a.mul(b)).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aocutils::read_input_lines("input")?;

    // loop over each line of the input, extracting instructions, then flatten into a single list
    let mul_ops: Vec<(i32, i32)> = input
        .iter()
        .map(|line| extract_instructions(line))
        .flatten()
        .collect();

    println!("part 1: {}", sum_mul_instructions(&mul_ops));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_instructions_example() {
        assert_eq!(
            extract_instructions(
                aocutils::read_input_lines("example")
                    .unwrap()
                    .first()
                    .unwrap(),
            ),
            Vec::from([(2, 4), (5, 5), (11, 8), (8, 5)])
        );
    }

    #[test]
    fn test_sum_mul_ops_example() {
        assert_eq!(
            sum_mul_instructions(&Vec::from([(2, 4), (5, 5), (11, 8), (8, 5)])),
            161
        )
    }
}
