use std::fs;

fn convert_schematic(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>()
}

fn is_schematic_symbol(c: &char) -> bool {
    !c.is_numeric() && c.ne(&'.')
}

fn extract_part_number(line: &Vec<char>, pos: usize) -> i32 {
    let mut start_pos = pos;
    let mut end_pos = pos;
    
    // work backwards to start pos
    for i in (0..pos).rev() {
        if !line.get(i).unwrap().is_numeric() {
            break
        }
        
        start_pos = i;
    }
    
    // work forwards to end pos
    for i in pos..line.len() {
        if !line.get(i).unwrap().is_numeric() {
            break
        }
        
        end_pos = i;
    }
    
//    println!("{:?} ({}) -> {}:{}", line, pos, start_pos, end_pos);
    
    line.iter().skip(start_pos).take((end_pos+1)-start_pos).collect::<String>().parse::<i32>().unwrap()
}

fn find_part_numbers(schematic: &Vec<Vec<char>>) -> Vec<i32> {
    let mut part_numbers: Vec<i32> = Vec::new();

    for (y, elements) in schematic.iter().enumerate() {
        for (x, element) in elements.iter().enumerate() {
            if !is_schematic_symbol(element) {
                continue
            }

            // search adjacent squares for part numbers
            for y_delta in (-1 as isize)..(2 as isize) {
                for x_delta in (-1 as isize)..(2 as isize) {
                    let search_x = (x as isize).checked_add(x_delta).unwrap() as usize;
                    let search_y = (y as isize).checked_add(y_delta).unwrap() as usize;
                    
                    if search_y >= schematic.len() || search_y < 0 || search_x >= elements.len() || search_x < 0 {
                        println!("Adjacent indexes out of bounds ({},{})", search_x, search_y);
                        continue;
                    }
                    
                    let adjacent_element = schematic.get(search_y).unwrap().get(search_x).unwrap();
                    
                    if adjacent_element.is_numeric() {
                        // extract part number
                        let part_number = extract_part_number(schematic.get(search_y).unwrap(), search_x);
                        part_numbers.push(part_number);
                    }
                }
            }
        }
    }
    
    part_numbers
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let schematic = convert_schematic(&input);
    
    let part_numbers = find_part_numbers(&schematic);
    println!("Part 1: {}", part_numbers.iter().sum::<i32>());
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    
    #[test]
    fn test_extract_part_number_example() {
        let actual = extract_part_number(&".664.598..".chars().collect::<Vec<char>>(), 2);
        let expected = 664;
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_sum_part_numbers() {
        let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let schematic = convert_schematic(&example);
        
        let actual: i32 = find_part_numbers(&schematic).iter().sum();
        let expected = 4361;
        
        assert_eq!(actual, expected);
    }
}
