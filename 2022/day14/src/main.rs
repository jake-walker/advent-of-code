use std::{collections::HashMap, error::Error, fs};
use itertools::Itertools;

type Coordinate = (i32, i32);
type ScanMap = HashMap<Coordinate, UnitType>;
type DropFunc = fn (&mut ScanMap, &Coordinate, &i32) -> Option<Coordinate>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum UnitType {
    Rock,
    Sand
}

fn parse_coordinate(input: &str) -> Result<Coordinate, Box<dyn Error>> {
    if let Some((x, y)) = input.split_once(',') {
        Ok((x.parse::<i32>()?, y.parse::<i32>()?))
    } else {
        Err(Box::from("failed to split coordinate"))
    }
}

fn parse_scan(input: &str) -> Result<(ScanMap, i32), Box<dyn Error>> {
    let mut map = ScanMap::new();
    let mut lowest = 0;

    for line in input.lines() {
        let groups = line.split(" -> ").into_iter().tuple_windows::<(_, _)>();

        for group in groups {
            let (mut x1, mut y1) = parse_coordinate(group.0)?;
            let (mut x2, mut y2) = parse_coordinate(group.1)?;

            if y1 > lowest {
                lowest = y1;
            }

            if y2 > lowest {
                lowest = y2;
            }

            if x1 > x2 {
                (x1, x2) = (x2, x1);
            }

            if y1 > y2 {
                (y1, y2) = (y2, y1);
            }

            for x in x1..x2+1 {
                for y in y1..y2+1 {
                    map.insert((x, y), UnitType::Rock);
                }
            }
        }
    }

    Ok((map, lowest))
}

fn drop_sand(map: &mut ScanMap, source: &Coordinate, lowest: &i32) -> Option<Coordinate> {
    let (mut sand_x, mut sand_y) = source.clone();

    loop {
        if sand_y > *lowest {
            return None
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

    map.insert((sand_x, sand_y), UnitType::Sand);
    Some((sand_x, sand_y))
}

fn drop_sand_with_floor(map: &mut ScanMap, source: &Coordinate, lowest: &i32) -> Option<Coordinate> {
    let (mut sand_x, mut sand_y) = source.clone();

    loop {
        if sand_y > *lowest {
            break;
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

    if &(sand_x, sand_y) == source {
        return None;
    }

    map.insert((sand_x, sand_y), UnitType::Sand);
    Some((sand_x, sand_y))
}

fn drop_sand_loop(func: DropFunc, map: &mut ScanMap, source: &Coordinate, lowest: &i32) -> i32 {
    let mut count = 0;

    while func(map, source, lowest) != None {
        count += 1;
    }

    count
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let (map, lowest) = parse_scan(&input).expect("should be able to parse map");
    let source = (500, 0);

    let mut map1 = map.clone();
    println!("Part 1: {}", drop_sand_loop(drop_sand, &mut map1, &source, &lowest));

    let mut map2 = map.clone();
    println!("Part 2: {}", drop_sand_loop(drop_sand_with_floor, &mut map2, &source, &lowest) + 1);
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
        assert_eq!(drop_sand(&mut map, &(500, 0), &lowest), Some((500, 8)));
    }

    #[test]
    fn drop_sand_with_floor_example() {
        let (mut map, lowest) = parse_scan(&EXAMPLE_SCAN).unwrap();
        assert_eq!(drop_sand_with_floor(&mut map, &(490, 0), &lowest), Some((490, 10)));
    }

    #[test]
    fn drop_sand_loop_example() {
        let (mut map, lowest) = parse_scan(&EXAMPLE_SCAN).unwrap();
        assert_eq!(drop_sand_loop(drop_sand, &mut map, &(500, 0), &lowest), 24);
    }

    #[test]
    fn drop_sand_with_floor_loop_example() {
        let (mut map, lowest) = parse_scan(&EXAMPLE_SCAN).unwrap();
        assert_eq!(drop_sand_loop(drop_sand_with_floor, &mut map, &(500, 0), &lowest) + 1, 93);
    }
}
