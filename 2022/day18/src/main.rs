use std::{collections::HashSet, str::FromStr, num::ParseIntError, fs};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point { x: i32, y: i32, z: i32 }

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, yz) = s.split_once(',').unwrap();
        let (y, z) = yz.split_once(',').unwrap();

        Ok(Point { x: x.parse()?, y: y.parse()?, z: z.parse()? })
    }
}

fn parse_input(input: &str) -> HashSet<Point> {
    input.lines().map(|l| Point::from_str(l).unwrap()).collect::<HashSet<Point>>()
}

fn get_surface_area(points: &HashSet<Point>) -> i32 {
    points.iter().map(|point| [-1, 1].iter().flat_map(|n| {
        [
            Point { x: point.x + n, y: point.y, z: point.z },
            Point { x: point.x, y: point.y + n, z: point.z },
            Point { x: point.x, y: point.y, z: point.z + n }
        ]
    }).filter(|p| !points.contains(p)).count() as i32).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let points = parse_input(&input);

    println!("Part 1: {}", get_surface_area(&points));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parse_input_example() {
        assert_eq!(parse_input("1,1,1\n2,1,1"), HashSet::from([
            Point { x: 1, y: 1, z: 1 },
            Point { x: 2, y: 1, z: 1 }
        ]));
    }

    #[test]
    fn get_surface_area_small_example() {
        assert_eq!(get_surface_area(&HashSet::from([
            Point { x: 1, y: 1, z: 1 },
            Point { x: 2, y: 1, z: 1 }
        ])), 10);
    }

    #[test]
    fn get_surface_area_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let points = parse_input(&input);
        assert_eq!(get_surface_area(&points), 64);
    }
}


