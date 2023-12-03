use std::fs;
use colored::Colorize;
use std::collections::HashSet;

fn convert_schematic(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>()
}

fn is_schematic_symbol(c: &char) -> bool {
    !c.is_numeric() && c.ne(&'.')
}

fn extract_part_number(line: &Vec<char>, pos: usize) -> (usize, usize) {
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

    (start_pos, end_pos)
}

fn find_part_numbers(schematic: &Vec<Vec<char>>) -> HashSet<((usize, usize), (usize, usize))> {
    let mut part_number_locations: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

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
                    
                    if search_y >= schematic.len() || search_x >= elements.len() {
                        println!("Adjacent indexes out of bounds ({},{})", search_x, search_y);
                        continue;
                    }
                    
                    let adjacent_element = schematic.get(search_y).unwrap().get(search_x).unwrap();
                    
                    if adjacent_element.is_numeric() {
                        // extract part number
                        let (start_x, end_x) = extract_part_number(schematic.get(search_y).unwrap(), search_x);
                        part_number_locations.insert(((start_x, search_y), (end_x, search_y)));
                    }
                }
            }
        }
    }

    part_number_locations
}

fn get_part_number_values(schematic: &Vec<Vec<char>>, part_number_locations: &HashSet<((usize, usize), (usize, usize))>) -> Vec<i32> {
    part_number_locations.iter().map(|((x1, y1), (x2, _))| {
        schematic.get(*y1).unwrap().iter().skip(*x1).take((x2+1)-x1).collect::<String>().parse::<i32>().unwrap()
    }).collect()
}

fn pretty_print_schematic(schematic: &Vec<Vec<char>>, part_number_locations: &HashSet<((usize, usize), (usize, usize))>) {
    for (y, elements) in schematic.iter().enumerate() {
        for (x, element) in elements.iter().enumerate() {
            let mut highlighted = false;

            for ((x1, y1), (x2, y2)) in part_number_locations {
                if x >= *x1 && x <= *x2 && y >= *y1 && y <= *y2 {
                    highlighted = true;
                    break
                }
            }

            if highlighted {
                print!("{}", element.to_string().on_green());
            } else {
                print!("{}", element);
            }
        }

        println!("")
    }
}

fn main() {
//    let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
//    let example_schematic = convert_schematic(&example);
//    let example_locations = find_part_numbers(&example_schematic);
//    println!("{:?}", example_locations);
//    pretty_print_schematic(&example_schematic, &example_locations);

    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let schematic = convert_schematic(&input);
    let locations = find_part_numbers(&schematic);
    
    let values = get_part_number_values(&schematic, &locations);
    println!("Part 1: {}", values.iter().sum::<i32>());
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    
    #[test]
    fn test_extract_part_number_example() {
        let actual = extract_part_number(&".664.598..".chars().collect::<Vec<char>>(), 2);
        let expected = (1, 3);
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_sum_part_numbers() {
        let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let schematic = convert_schematic(&example);
        let locations = find_part_numbers(&schematic);
        
        let actual: i32 = get_part_number_values(&schematic, &locations).iter().sum();
        let expected = 4361;
        
        assert_eq!(actual, expected);
    }
}
