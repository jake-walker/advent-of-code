// i used a hashmap to store what was at each coordinate, however if i had more time i would check
// for stone in the drop function to save memory space. (e.g. does the coordinate below intersect
// with any of the lines in the puzzle input)

use std::{collections::HashMap, error::Error, fs};
use itertools::Itertools;

type Coordinate = (i32, i32);
type ScanMap = HashMap<Coordinate, UnitType>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum UnitType {
    Rock,
    Sand
}

// parse an x,y coordinate to a tuple of integers
fn parse_coordinate(input: &str) -> Result<Coordinate, Box<dyn Error>> {
    if let Some((x, y)) = input.split_once(',') {
        Ok((x.parse::<i32>()?, y.parse::<i32>()?))
    } else {
        Err(Box::from("failed to split coordinate"))
    }
}

fn parse_scan(input: &str) -> Result<(ScanMap, i32), Box<dyn Error>> {
    let mut map = ScanMap::new();
    // this is the lowest point that has rock (but it is actually the highest because the y coordinates increment downwards)
    let mut lowest = 0;

    for line in input.lines() {
        // split by arrow to get all of the coordinates, then use windows to get each pair (e.g. a,b,c,d -> a+b, b+c, c+d)
        let groups = line.split(" -> ").into_iter().tuple_windows::<(_, _)>();

        for group in groups {
            let (mut x1, mut y1) = parse_coordinate(group.0)?;
            let (mut x2, mut y2) = parse_coordinate(group.1)?;

            // if either of the y coordinates are lower than the current lowest
            if y1 > lowest {
                lowest = y1;
            }
            if y2 > lowest {
                lowest = y2;
            }

            // if one coordinate is higher than the other, swap them around because otherwise the range doesn't work
            if x1 > x2 {
                (x1, x2) = (x2, x1);
            }
            if y1 > y2 {
                (y1, y2) = (y2, y1);
            }

            for x in x1..x2+1 {
                for y in y1..y2+1 {
                    // set the coordinate to rock in the hashmap
                    map.insert((x, y), UnitType::Rock);
                }
            }
        }
    }

    Ok((map, lowest))
}

// drop a single piece of sand from the source
// - map is the hashmap of where stuff is
// - source is where to drop sand from
// - lowest is the lowest piece of stone in the map
// - has_floor is to change logic for part 2
// outputs none if no more sand can be dropped, otherwise where the piece landed
fn drop_sand(map: &mut ScanMap, source: &Coordinate, lowest: &i32, has_floor: bool) -> Option<Coordinate> {
    // initialise the new sand to be from the source
    let (mut sand_x, mut sand_y) = source.clone();

    loop {
        // if the sand is lower than the lowest point (i.e. fallen into the 'void' for part 1, or hit the floor for part 2)
        if sand_y > *lowest {
            if has_floor {
                break;
            } else {
                // return none because we want to stop dropping once one falls into the void
                return None
            }
        }

        // try moving down vertically
        if map.get(&(sand_x, sand_y + 1)) == None {
            sand_y += 1;
            continue;
        }

        // try moving down + left
        if map.get(&(sand_x - 1, sand_y + 1)) == None {
            sand_x -= 1;
            sand_y += 1;
            continue;
        }

        // try moving down + right
        if map.get(&(sand_x + 1, sand_y + 1)) == None {
            sand_x += 1;
            sand_y += 1;
            continue;
        }

        // otherwise the sand can't move
        break;
    }

    // if the sand does not move, stop dropping - condition for part 2
    if &(sand_x, sand_y) == source {
        return None;
    }

    // set the coordinates to sand and return the coordinates
    map.insert((sand_x, sand_y), UnitType::Sand);
    Some((sand_x, sand_y))
}

// drop sand until no more can be dropped
fn drop_sand_loop(map: &mut ScanMap, source: &Coordinate, lowest: &i32, has_floor: bool) -> i32 {
    let mut count = 0;

    while drop_sand(map, source, lowest, has_floor) != None {
        count += 1;
    }

    // count is 1 off with floor because it is not inclusive of the sand covering the source
    if has_floor {
        count += 1;
    }

    count
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let (map, lowest) = parse_scan(&input).expect("should be able to parse map");
    let source = (500, 0);

    let mut map1 = map.clone();
    println!("Part 1: {}", drop_sand_loop(&mut map1, &source, &lowest, false));

    let mut map2 = map.clone();
    println!("Part 2: {}", drop_sand_loop(&mut map2, &source, &lowest, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_SCAN: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn parse_scan_example() {
        let (map, lowest) = parse_scan("498,4 -> 498,6 -> 496,6").unwrap();
        assert_eq!(map, ScanMap::from([
            ((498, 4), UnitType::Rock),
            ((498, 5), UnitType::Rock),
            ((498, 6), UnitType::Rock),
            ((497, 6), UnitType::Rock),
            ((496, 6), UnitType::Rock),
        ]));
        assert_eq!(lowest, 6);
    }

    #[test]
    fn parse_scan_lowest_example() {
        let (_, lowest) = parse_scan(&EXAMPLE_SCAN).unwrap();
        assert_eq!(lowest, 9);
    }

    #[test]
    fn drop_sand_example() {
        let (mut map, lowest) = parse_scan(&EXAMPLE_SCAN).unwrap();
        assert_eq!(drop_sand(&mut map, &(500, 0), &lowest, false), Some((500, 8)));
    }

    #[test]
    fn drop_sand_with_floor_example() {
        let (mut map, lowest) = parse_scan(&EXAMPLE_SCAN).unwrap();
        assert_eq!(drop_sand(&mut map, &(490, 0), &lowest, true), Some((490, 10)));
    }

    #[test]
    fn drop_sand_loop_example() {
        let (mut map, lowest) = parse_scan(&EXAMPLE_SCAN).unwrap();
        assert_eq!(drop_sand_loop(&mut map, &(500, 0), &lowest, false), 24);
    }

    #[test]
    fn drop_sand_with_floor_loop_example() {
        let (mut map, lowest) = parse_scan(&EXAMPLE_SCAN).unwrap();
        assert_eq!(drop_sand_loop(&mut map, &(500, 0), &lowest, true), 93);
    }
}
