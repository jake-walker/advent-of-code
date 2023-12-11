use std::fs;

use itertools::Itertools;

type Map = Vec<Vec<char>>;
type Coordinate = (u32, u32);

fn parse_map(s: &str) -> Map {
    s.lines().map(|l| l.chars().collect::<Vec<char>>()).collect()
}

fn expand_map(map: &mut Map) -> () {
    // expand in y direction
    let mut y = 0;
    loop {
        let mut to_expand = true;
        for x in 0..map[0].len() {
            if map[y][x] != '.' {
                to_expand = false;
                break;
            }
        }

        if to_expand {
            map.insert(y, vec!['.'; map[0].len()]);
            y += 1;
        }

        y += 1;
        if y >= map.len() {
            break;
        }
    }

    // expand in x direction
    let mut x = 0;
    loop {
        let mut to_expand = true;
        for y in 0..map.len() {
            if map[y][x] != '.' {
                to_expand = false;
                break;
            }
        }

        if to_expand {
            for y in 0..map.len() {
                map[y].insert(x, '.');
            }
            x += 1;
        }

        x += 1;
        if x >= map[0].len() {
            break;
        }
    }
}

fn print_map(map: &Map) {
    for line in map {
        println!("{}", line.iter().collect::<String>())
    }
}

fn get_points(map: &Map) -> Vec<Coordinate> {
    let mut coordinates = Vec::new();

    for (y, line) in map.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            if value == &'#' {
                coordinates.push((x as u32, y as u32));
            }
        }
    }

    coordinates
}

fn manhattan_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    u32::abs_diff(x1, x2) + u32::abs_diff(y1, y2)
}

fn get_distances(points: &Vec<Coordinate>) -> Vec<u32> {
    let mut distances = Vec::new();

    let pairs = points.iter().permutations(2).map(|p| {
        let mut p = p;
        p.sort();
        p
    }).unique();

    for items in pairs {
        let ((x1, y1), (x2, y2)) = (items[0], items[1]);
        let distance = manhattan_distance(*x1, *y1, *x2, *y2);

        // println!("Distance between ({},{}) and ({},{}) is {}", x1, y1, x2, y2, distance);

        distances.push(distance);
    }

    distances
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let mut map = parse_map(&input);
    expand_map(&mut map);

    let part1 = get_distances(&get_points(&map)).iter().sum::<u32>();
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
    static EXPANDED_EXAMPLE_INPUT: &str = "....#........\n.........#...\n#............\n.............\n.............\n........#....\n.#...........\n............#\n.............\n.............\n.........#...\n#....#.......";

    #[test]
    fn test_expand_map_example() {
        let mut actual = parse_map(EXAMPLE_INPUT);
        expand_map(&mut actual);

        let expected = parse_map(EXPANDED_EXAMPLE_INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_points_example() {
        let map = parse_map(EXPANDED_EXAMPLE_INPUT);

        let actual = get_points(&map).len();
        let expected = 9;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_distances_example() {
        let points = get_points(&parse_map(EXPANDED_EXAMPLE_INPUT));
        let distances = get_distances(&points);

        assert_eq!(distances.len(), 36);
        assert_eq!(distances.iter().sum::<u32>(), 374);
    }
}
