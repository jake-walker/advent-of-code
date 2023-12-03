use std::fs;
//use colored::Colorize;
use std::collections::HashSet;

/// Convert the puzzle input to a 2d char array to make indexing eaiser
fn convert_schematic(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>()
}

/// Simple function to check whether a given symbol on the schematic is one we care about for part 1 (not a number or dot)
fn is_schematic_symbol(c: &char) -> bool {
    !c.is_numeric() && c.ne(&'.')
}

/// Simple function to check whether a given symbol on the schematic is one we care about for part 2 (a cog / asterisk)
fn is_gear_symbol(c: &char) -> bool {
    c.eq(&'*')
}

/// With some chars and a position, search to the left and right of the position to find the start and end position of the number
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

/// Flatten (because locations are in groups by the symbol they are by) and deduplicate locations for use with part 1
fn flatten_dedup_part_number_locations(locations: Vec<Vec<((usize, usize), (usize, usize))>>) -> HashSet<((usize, usize), (usize, usize))> {
    dedup_part_number_locations(locations.into_iter().flatten().collect::<Vec<((usize, usize), (usize, usize))>>())
}

/// Deduplicate locations for use with part 2
fn dedup_part_number_locations(locations: Vec<((usize, usize), (usize, usize))>) -> HashSet<((usize, usize), (usize, usize))> {
    HashSet::from_iter(locations.into_iter())
}

/// Loop through all locations on the schematic, searching for valid symbols, then searching adjacent squares for part numbers
fn find_part_numbers(schematic: &Vec<Vec<char>>, symbol_selector: fn(&char) -> bool) -> Vec<Vec<((usize, usize), (usize, usize))>> {
    let mut part_number_locations: Vec<Vec<((usize, usize), (usize, usize))>> = Vec::new();

    for (y, elements) in schematic.iter().enumerate() {
        for (x, element) in elements.iter().enumerate() {
            // skip this square if it's not a symbol we care about
            if !symbol_selector(element) {
                continue
            }

            // store locations for just this symbol, this allows them to be grouped by symbol for part 2
            let mut symbol_locations: Vec<((usize, usize), (usize, usize))> = Vec::new();

            // search adjacent squares for part numbers
            for y_delta in (-1 as isize)..(2 as isize) {
                for x_delta in (-1 as isize)..(2 as isize) {
                    let search_x = (x as isize).checked_add(x_delta).unwrap() as usize;
                    let search_y = (y as isize).checked_add(y_delta).unwrap() as usize;
                    
                    if search_y >= schematic.len() || search_x >= elements.len() {
//                        println!("Adjacent indexes out of bounds ({},{})", search_x, search_y);
                        continue;
                    }
                    
                    let adjacent_element = schematic.get(search_y).unwrap().get(search_x).unwrap();
                    
                    // To improve this, any locations currently inside `symbol_locations` should be skipped so they don't have to be put into a set later on

                    if adjacent_element.is_numeric() {
                        // extract part number
                        let (start_x, end_x) = extract_part_number(schematic.get(search_y).unwrap(), search_x);
                        symbol_locations.push(((start_x, search_y), (end_x, search_y)));
                    }
                }
            }

            part_number_locations.push(symbol_locations);
        }
    }

    part_number_locations
}

/// given a list of locations, extract the part numbers for each
fn get_part_number_values(schematic: &Vec<Vec<char>>, part_number_locations: &HashSet<((usize, usize), (usize, usize))>) -> Vec<i32> {
    part_number_locations.iter().map(|((x1, y1), (x2, _))| {
        schematic.get(*y1).unwrap().iter().skip(*x1).take((x2+1)-x1).collect::<String>().parse::<i32>().unwrap()
    }).collect()
}

/// a useful debug function for printing out a schematic and highlighting where part numbers are found
//fn pretty_print_schematic(schematic: &Vec<Vec<char>>, part_number_locations: &HashSet<((usize, usize), (usize, usize))>) {
//    for (y, elements) in schematic.iter().enumerate() {
//        for (x, element) in elements.iter().enumerate() {
//            let mut highlighted = false;
//
//            for ((x1, y1), (x2, y2)) in part_number_locations {
//                if x >= *x1 && x <= *x2 && y >= *y1 && y <= *y2 {
//                    highlighted = true;
//                    break
//                }
//            }
//
//            if highlighted {
//                print!("{}", element.to_string().on_green());
//            } else {
//                print!("{}", element);
//            }
//        }
//
//        println!("")
//    }
//}

fn main() {
//    let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
//    let example_schematic = convert_schematic(&example);
//    let example_locations = find_part_numbers(&example_schematic);
//    println!("{:?}", example_locations);
//    pretty_print_schematic(&example_schematic, &example_locations);

    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let schematic = convert_schematic(&input);
    
    let locations1 = find_part_numbers(&schematic, is_schematic_symbol);
    let values = get_part_number_values(&schematic, &flatten_dedup_part_number_locations(locations1));
    println!("Part 1: {}", values.iter().sum::<i32>());

    let locations2 = find_part_numbers(&schematic, is_gear_symbol);
    let gear_ratios_summed = locations2.into_iter().map(|gear_number_locations| {
        let dedup_locations = dedup_part_number_locations(gear_number_locations);
        if dedup_locations.len() != 2 {
            return 0;
        }

        let values = get_part_number_values(&schematic, &dedup_locations);
        values.iter().product::<i32>()
    }).sum::<i32>();
    println!("Part 2: {}", gear_ratios_summed);
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
        let locations = find_part_numbers(&schematic, is_schematic_symbol);
        
        let actual: i32 = get_part_number_values(&schematic, &flatten_dedup_part_number_locations(locations)).iter().sum();
        let expected = 4361;
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_gear_ratios() {
        let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let schematic = convert_schematic(&example);
        let locations = find_part_numbers(&schematic, is_gear_symbol);

        let actual = locations.into_iter().map(|gear_number_locations| {
            let dedup_locations = dedup_part_number_locations(gear_number_locations);
            if dedup_locations.len() != 2 {
                return 0;
            }

            let values = get_part_number_values(&schematic, &dedup_locations);
            values.iter().product::<i32>()
        }).sum::<i32>();
        let expected = 467835;

        assert_eq!(actual, expected);
    }
}
