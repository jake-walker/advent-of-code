use std::ops::BitXor;

#[derive(Debug, PartialEq, Eq)]
struct Computer {
    memory: Vec<u8>,
    instruction_pointer: usize,
    register_a: i64,
    register_b: i64,
    register_c: i64,
    output: Vec<i64>,
}

impl Computer {
    fn load_program(input: &str) -> Self {
        let (registers, program) = input.split_once("\n\n").unwrap();
        let mut register_lines = registers
            .lines()
            .map(|line| line.split_once(": ").unwrap().1.parse::<i64>().unwrap());

        let mut comp = Computer {
            memory: Vec::new(),
            register_a: register_lines.next().unwrap(),
            register_b: register_lines.next().unwrap(),
            register_c: register_lines.next().unwrap(),
            instruction_pointer: 0,
            output: Vec::new(),
        };

        comp.memory = program
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(|n| n.parse::<u8>().unwrap())
            .collect();

        comp
    }

    fn combo_operand(&self, operand: u8) -> i64 {
        match operand {
            0 | 1 | 2 | 3 => operand as i64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("unexpected combo operand {}", operand),
        }
    }

    fn cycle(&mut self) -> bool {
        if self.instruction_pointer + 1 >= self.memory.len() {
            return true;
        }

        let opcode = self.memory[self.instruction_pointer];
        let operand = self.memory[self.instruction_pointer + 1];

        match opcode {
            // adv
            0 => {
                self.register_a =
                    self.register_a / (2_i64.pow(self.combo_operand(operand).try_into().unwrap()))
            }
            // bxl
            1 => self.register_b = self.register_b.bitxor(operand as i64),
            // bst
            2 => self.register_b = self.combo_operand(operand) % 8,
            // jnz
            3 => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand as usize;
                    return false;
                }
            }
            // bxc
            4 => self.register_b = self.register_b.bitxor(self.register_c),
            // out
            5 => self.output.push(self.combo_operand(operand) % 8),
            // bdv
            6 => {
                self.register_b =
                    self.register_a / (2_i64.pow(self.combo_operand(operand).try_into().unwrap()))
            }
            // cdv
            7 => {
                self.register_c =
                    self.register_a / (2_i64.pow(self.combo_operand(operand).try_into().unwrap()))
            }
            _ => panic!("unexpected opcode {}", opcode),
        }

        self.instruction_pointer += 2;

        false
    }
}

fn main() {
    let mut comp = Computer::load_program(&aocutils::read_input("input").unwrap());
    while !comp.cycle() {}

    println!(
        "part 1: {}",
        comp.output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str =
        "Register A: 729\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0";

    fn test_program(program: &[u8], register_a: i64, register_b: i64, register_c: i64) -> Computer {
        let mut comp = Computer {
            instruction_pointer: 0,
            memory: program.to_vec(),
            register_a,
            register_b,
            register_c,
            output: Vec::new(),
        };
        while !comp.cycle() {}
        comp
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Computer::load_program(EXAMPLE_INPUT),
            Computer {
                memory: vec![0, 1, 5, 4, 3, 0],
                instruction_pointer: 0,
                register_a: 729,
                register_b: 0,
                register_c: 0,
                output: vec![]
            }
        );
    }

    #[test]
    fn test_program_1() {
        let comp = test_program(&[2, 6], 0, 0, 9);
        assert_eq!(comp.register_b, 1);
    }

    #[test]
    fn test_program_2() {
        let comp = test_program(&[5, 0, 5, 1, 5, 4], 10, 0, 0);
        assert_eq!(comp.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_program_3() {
        let comp = test_program(&[0, 1, 5, 4, 3, 0], 2024, 0, 0);
        assert_eq!(comp.register_a, 0);
        assert_eq!(comp.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_program_4() {
        let comp = test_program(&[1, 7], 0, 29, 0);
        assert_eq!(comp.register_b, 26);
    }

    #[test]
    fn test_program_5() {
        let comp = test_program(&[4, 0], 0, 2024, 43690);
        assert_eq!(comp.register_b, 44354);
    }

    #[test]
    fn test_program_example() {
        let mut comp = Computer::load_program(EXAMPLE_INPUT);
        while !comp.cycle() {}
        assert_eq!(comp.output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
