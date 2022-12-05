use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct MoveInstruction {
    quantity: usize,
    from: usize,
    to: usize
}

fn parse_input(input: &String) -> (Vec<Vec<char>>, Vec<MoveInstruction>) {
    let (raw_crates, raw_instructions) = input.split_once("\n\n").unwrap();
    let raw_crates = raw_crates.lines();

    // get the last line of crates (number labels) then split by whitespace and get the last
    let stacks = raw_crates.clone().rev().next().unwrap().split_whitespace()
        .rev().next().unwrap().parse::<usize>().unwrap();

    let mut crates: Vec<Vec<char>> = vec![vec![]; stacks];
    let mut instructions: Vec<MoveInstruction> = Vec::new();

    // iterate through each crate line, starting from the bottom but skipping the number labels
    for line in raw_crates.rev().skip(1) {
        for stack in 0..stacks {
            // get the character at 4n+1 for the character at the given stack
            if let Some(c) = line.clone().chars().nth((4*stack)+1) {
                if c != ' ' {
                    // add the character to the crates if it is not none or a space
                    crates[stack].push(c);
                }
            }
        }
    }

    for line in raw_instructions.lines() {
        let parts = line.split_whitespace();
        instructions.push(MoveInstruction {
            quantity: parts.clone().skip(1).next().unwrap().parse::<usize>().unwrap(),
            from: parts.clone().skip(3).next().unwrap().parse::<usize>().unwrap(),
            to: parts.skip(5).next().unwrap().parse::<usize>().unwrap()
        });
    }

    (crates, instructions)
}

fn rearrange_stacks(crates: &mut Vec<Vec<char>>, instructions: &Vec<MoveInstruction>, reverse: bool) {
    for instruction in instructions {
        // calculate the length of the from stack minus the quantity to move (i.e. final length of stack)
        let from_len = crates[instruction.from - 1].len().saturating_sub(instruction.quantity);
        // take the items past the above length into a buffer
        let mut buf = crates[instruction.from - 1].split_off(from_len);

        // part 2 doesn't have them reversed
        if reverse {
            // reverse the buffer so that the item removed first is added first
            buf.reverse();
        }

        // add the buffer onto the destination stack
        crates[instruction.to - 1].extend(buf);
    }
}

fn top_crates(crates: &Vec<Vec<char>>) -> Vec<char> {
    crates.iter().map(|stack| *stack.last().unwrap()).collect()
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let (crates, instructions) = parse_input(&input);

    let mut part1_crates = crates.clone();
    rearrange_stacks(&mut part1_crates, &instructions, true);
    println!("Part 1: {}", top_crates(&part1_crates).iter().collect::<String>());

    let mut part2_crates = crates.clone();
    rearrange_stacks(&mut part2_crates, &instructions, false);
    println!("Part 2: {}", top_crates(&part2_crates).iter().collect::<String>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_input_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let (crates, instructions) = parse_input(&input);
        assert_eq!(crates, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        assert_eq!(instructions, vec![
            MoveInstruction { quantity: 1, from: 2, to: 1 },
            MoveInstruction { quantity: 3, from: 1, to: 3 },
            MoveInstruction { quantity: 2, from: 2, to: 1 },
            MoveInstruction { quantity: 1, from: 1, to: 2 },
        ])
    }

    #[test]
    fn rearrange_stacks_part1_example() {
        let mut crates = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        rearrange_stacks(&mut crates, &vec![
                MoveInstruction { quantity: 1, from: 2, to: 1 },
                MoveInstruction { quantity: 3, from: 1, to: 3 },
                MoveInstruction { quantity: 2, from: 2, to: 1 },
                MoveInstruction { quantity: 1, from: 1, to: 2 },
        ], true);

        assert_eq!(crates, vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']]);
    }

    #[test]
    fn rearrange_stacks_part2_example() {
        let mut crates = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        rearrange_stacks(&mut crates, &vec![
            MoveInstruction { quantity: 1, from: 2, to: 1 },
            MoveInstruction { quantity: 3, from: 1, to: 3 },
            MoveInstruction { quantity: 2, from: 2, to: 1 },
            MoveInstruction { quantity: 1, from: 1, to: 2 },
        ], false);

        assert_eq!(crates, vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']]);
    }

    #[test]
    fn top_crates_example() {
        assert_eq!(top_crates(&vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]), vec!['N', 'D', 'P']);
    }
}