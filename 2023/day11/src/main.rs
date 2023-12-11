use std::fs;

use itertools::Itertools;

type Map = Vec<Vec<char>>;
type Coordinate = (u64, u64);

// the number of rows to replace empty rows with
const PART1_EXPANSION_SCALE: u64 = 2;
const PART2_EXPANSION_SCALE: u64 = 1000000;

fn parse_map(s: &str) -> Map {
    s.lines().map(|l| l.chars().collect::<Vec<char>>()).collect()
}

/// Get the indexes of where the map expands (i.e. the indexes of empty rows and columns)
fn get_map_expansions(map: &Map) -> (Vec<usize>, Vec<usize>) {
    let y_max = map.len();
    let x_max = map[0].len();

    let mut y_expansions: Vec<usize> = Vec::new();
    let mut x_expansions: Vec<usize> = Vec::new();

    // expand in y direction
    for y in 0..y_max {
        let mut to_expand = true;
        for x in 0..x_max {
            if map[y][x] != '.' {
                to_expand = false;
                break;
            }
        }

        if to_expand {
            y_expansions.push(y);
        }
    }

    // expand in x direction
    for x in 0..x_max {
        let mut to_expand = true;
        for y in 0..y_max {
            if map[y][x] != '.' {
                to_expand = false;
                break;
            }
        }

        if to_expand {
            x_expansions.push(x);
        }
    }

    (x_expansions, y_expansions)
}

/// Get a list of coordinates for each point on the map
fn get_coordinates(map: &Map) -> Vec<Coordinate> {
    let mut coordinates = Vec::new();

    for (y, line) in map.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            if value == &'#' {
                coordinates.push((x as u64, y as u64));
            }
        }
    }

    coordinates
}

/// Given a coordinate, expansions and expansion scale, calculate the new coordinates
fn expand_coordinate(c: &Coordinate, expansions: &(Vec<usize>, Vec<usize>), scale: u64) -> Coordinate {
    let (x_expansions, y_expansions) = expansions;

    // Calculate the number of expansions for this coordinate (i.e. how many blank rows come before it)
    let coordinate_x_expansions = {
        // Get the index of the first expansion that is larger than the x coordinate
        // This returns the total number of blank rows before the coordinate
        if let Some(val) = x_expansions.iter().position(|e| *e as u64 >= c.0) {
            val
        } else {
            // If no value is found, ALL the expansions are before the coordinate
            x_expansions.len()
        }
    } as u64;

    // Same as above for the y coordinate
    let coordinate_y_expansions = {
        if let Some(val) = y_expansions.iter().position(|e| *e as u64 >= c.1) {
            val
        } else {
            y_expansions.len()
        }
    } as u64;

    #[cfg(test)]
    print!("{},{} has expansions x={:?}, y={:?}", c.0, c.1, coordinate_x_expansions, coordinate_y_expansions);

    // Add the number of expansions multiplied by the scale. The number of expansions must be subtracted because the original shouldn't be added again
    // For example, with a scale factor of 2 (for part 1), one extra row is added for each expansion
    let new_coords = (c.0 + (coordinate_x_expansions * scale) - coordinate_x_expansions, c.1 + (coordinate_y_expansions * scale) - coordinate_y_expansions);

    #[cfg(test)]
    println!(" -> {},{}", new_coords.0, new_coords.1);

    new_coords
}

/// Calculate manhattan distance between two coordinates
fn manhattan_distance(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    u64::abs_diff(x1, x2) + u64::abs_diff(y1, y2)
}

/// Calculate a list of distances for every point given
fn get_distances(points: &Vec<Coordinate>) -> Vec<u64> {
    let mut distances = Vec::new();

    let pairs = points.iter().permutations(2).map(|p| {
        let mut p = p;
        p.sort();
        p
    }).unique();

    for items in pairs {
        let ((x1, y1), (x2, y2)) = (items[0], items[1]);
        let distance = manhattan_distance(*x1, *y1, *x2, *y2);

        distances.push(distance);
    }

    distances
}

/// Function to do everything in one go
fn all_distances(m: &Map, scale: u64) -> Vec<u64> {
    let coordinates = get_coordinates(m);
    let expansions = get_map_expansions(m);
    let expanded_coordinates = coordinates.iter().map(|c| expand_coordinate(c, &expansions, scale)).collect::<Vec<Coordinate>>();
    get_distances(&expanded_coordinates)
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let map = parse_map(&input);

    let part1 = all_distances(&map, PART1_EXPANSION_SCALE).iter().sum::<u64>();
    println!("Part 1: {}", part1);

    let part2 = all_distances(&map, PART2_EXPANSION_SCALE).iter().sum::<u64>();
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

    #[test]
    fn test_get_map_expansions_example() {
        let map = parse_map(EXAMPLE_INPUT);

        let actual = get_map_expansions(&map);
        let expected = (vec![2, 5, 8], vec![3, 7]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_coordinates_example() {
        let map = parse_map(EXAMPLE_INPUT);

        let actual = get_coordinates(&map);
        let expected = vec![(3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_expand_coordinates_example() {
        let map = parse_map(EXAMPLE_INPUT);
        let expansions = get_map_expansions(&map);

        let actual = vec![(3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9)].iter().map(|c| expand_coordinate(c, &expansions, PART1_EXPANSION_SCALE)).collect::<Vec<Coordinate>>();
        let expected = vec![(4, 0), (9, 1), (0, 2), (8, 5), (1, 6), (12, 7), (9, 10), (0, 11), (5, 11)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_all_distances_example() {
        let map = parse_map(EXAMPLE_INPUT);

        let distances = all_distances(&map, PART1_EXPANSION_SCALE);

        assert_eq!(distances.len(), 36);
        assert_eq!(distances.iter().sum::<u64>(), 374);
    }

    #[test]
    fn test_all_distances_example_part2() {
        let map = parse_map(EXAMPLE_INPUT);

        assert_eq!(all_distances(&map, 10).iter().sum::<u64>(), 1030);
        assert_eq!(all_distances(&map, 100).iter().sum::<u64>(), 8410);
    }
}
