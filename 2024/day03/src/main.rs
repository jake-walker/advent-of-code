use regex::Regex;
use std::{error::Error, ops::Mul};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Disable,
    Enable,
    Mul(i64, i64),
}

fn parse_instructions(s: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    // use regex to extract mul instructions that match the format
    let re = Regex::new(r"(mul\(-?\d{1,3},-?\d{1,3}\))|(do\(\))|(don't\(\))")?;

    Ok(re
        // loop over regex captures
        .captures_iter(s)
        .map(|c| {
            // extract the capture
            let (instruction_str, _) = c.extract::<1>();

            match instruction_str {
                // if this is a do instruction
                "do()" => Ok(Instruction::Enable),
                // if this is a don't instruction
                "don't()" => Ok(Instruction::Disable),
                _ => {
                    if instruction_str.starts_with("mul(") {
                        // get the characters between the brackets in the instruction
                        let numbers = &instruction_str[4..instruction_str.len() - 1];
                        // split our numbers string once at a comma to get both numbers
                        let (a, b) = numbers.split_once(",").ok_or("")?;
                        // parse the numbers and put into a mul instruction
                        Ok(Instruction::Mul(a.parse().unwrap(), b.parse().unwrap()))
                    } else {
                        Err(format!("unexpected instruction {}", instruction_str).into())
                    }
                }
            }
        })
        // bring together our map into a vec/list
        .collect::<Result<Vec<Instruction>, Box<dyn Error>>>()?)
}

fn process_instructions(instructions: &Vec<Instruction>) -> i64 {
    let mut result = 0;
    let mut execution_enabled = true;

    for i in instructions {
        match (execution_enabled, i) {
            // disable execution for don't instruction
            (_, Instruction::Disable) => execution_enabled = false,
            // enable execution for do instruction
            (_, Instruction::Enable) => execution_enabled = true,
            // if execution is enabled and there's a mul instruction, add it to the result
            (true, Instruction::Mul(a, b)) => result += a.mul(b),
            _ => continue,
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aocutils::read_input_lines("input")?.join("");
    let instructions = parse_instructions(&input)?;

    println!(
        "part 1: {}",
        process_instructions(
            // filter out only mul instructions
            &instructions
                .clone()
                .into_iter()
                .filter(|i| matches!(i, Instruction::Mul(_, _)))
                .collect()
        )
    );

    println!("part 2: {}", process_instructions(&instructions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions_example_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            parse_instructions(&aocutils::read_input_lines("example")?.join(""))?,
            Vec::from([
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5)
            ])
        );

        Ok(())
    }

    #[test]
    fn test_parse_instructions_example_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            parse_instructions(&aocutils::read_input_lines("example2")?.join(""))?,
            Vec::from([
                Instruction::Mul(2, 4),
                Instruction::Disable,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Enable,
                Instruction::Mul(8, 5)
            ])
        );

        Ok(())
    }

    #[test]
    fn test_process_instructions_example_part1() {
        assert_eq!(
            process_instructions(&Vec::from([
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5)
            ])),
            161
        )
    }

    #[test]
    fn test_process_instructions_example_part2() {
        assert_eq!(
            process_instructions(&Vec::from([
                Instruction::Mul(2, 4),
                Instruction::Disable,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Enable,
                Instruction::Mul(8, 5)
            ])),
            48
        )
    }
}
