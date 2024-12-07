use std::{
    collections::HashSet,
    io::{self, Write},
};

type Coords = (i32, i32);
type Map = Vec<Vec<bool>>;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn to_coords(&self) -> Coords {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

fn parse_map(input: &str) -> (Map, Coords) {
    let mut map = Vec::new();
    let mut guard_pos: Option<Coords> = None;

    for (y, line) in input.split("\n").enumerate() {
        map.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => map[y].push(true),
                '.' => map[y].push(false),
                '^' => {
                    guard_pos = Some((x as i32, y as i32));
                    map[y].push(false);
                }
                _ => continue,
            }
        }
    }

    (map, guard_pos.unwrap())
}

fn in_map_bounds(map: &Map, pos: &Coords) -> bool {
    pos.0 >= 0 && pos.0 < map[0].len() as i32 && pos.1 >= 0 && pos.1 < map.len() as i32
}

fn print_map(map: &Map, pos: Coords, history: Option<Vec<&(Coords, Direction)>>) -> () {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let h = history.clone().and_then(|i| {
                i.into_iter()
                    .filter(|(c, _)| c.0 as usize == x && c.1 as usize == y)
                    .next()
            });

            if pos.0 as usize == x && pos.1 as usize == y {
                print!("%");
            } else if let Some((_, direction)) = h {
                match direction {
                    Direction::North => print!("^"),
                    Direction::East => print!(">"),
                    Direction::South => print!("v"),
                    Direction::West => print!("<"),
                }
            } else if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("----");
}

fn pathfind(
    map: &Map,
    start_pos: Coords,
    start_direction: Direction,
) -> (bool, Vec<(Coords, Direction)>) {
    println!(
        "pathfinding from {},{} {:?}",
        start_pos.0, start_pos.1, start_direction
    );
    let _ = io::stdout().flush();

    let mut history = Vec::from([(start_pos, start_direction)]);
    let mut pos = start_pos.clone();
    let mut direction = start_direction;
    let mut i = 0;

    loop {
        let new_pos = (
            pos.0 + direction.to_coords().0,
            pos.1 + direction.to_coords().1,
        );

        // check bounds
        if !in_map_bounds(map, &new_pos) {
            return (false, history);
        }

        if i >= 10 {
            println!("we've been stuck for a while");
            return (false, history);
        }

        // quit if visited before
        if history.contains(&(new_pos, direction)) {
            return (true, history);
        }

        if map[new_pos.1 as usize][new_pos.0 as usize] {
            // turn
            direction = direction.rotate_cw();
            i += 1;
            continue;
        }

        i = 0;
        pos = new_pos;
        history.push((pos, direction));
    }
}

fn unique_positions(history: &Vec<(Coords, Direction)>) -> usize {
    history
        .iter()
        .map(|x| x.0)
        .collect::<HashSet<Coords>>()
        .len()
}

fn find_loops(map: &Map, history: &Vec<(Coords, Direction)>) -> Vec<Coords> {
    let mut new_blocks = Vec::new();

    for (pos, direction) in history {
        for d in [
            direction,
            &direction.rotate_cw(),
            &direction.rotate_cw().rotate_cw(),
            &direction.rotate_cw().rotate_cw().rotate_cw(),
        ] {
            let block_pos = (pos.0 + d.to_coords().0, pos.1 + d.to_coords().1);

            // check bounds
            if !in_map_bounds(map, &pos) {
                break;
            }
            if !in_map_bounds(map, &block_pos) {
                if d == direction {
                    break;
                } else {
                    continue;
                }
            }

            let mut new_map = map.clone();

            // there's already a block here, rotate and try again
            if new_map[block_pos.1 as usize][block_pos.0 as usize] {
                println!("already block");
                continue;
            }

            // update the map
            new_map[block_pos.1 as usize][block_pos.0 as usize] = true;

            let (loop_detected, _) = pathfind(&new_map, *pos, *d);

            if loop_detected {
                new_blocks.push(block_pos);
            }

            break;
        }
    }

    new_blocks
}

fn main() {
    let (map, guard_pos) = parse_map(&aocutils::read_input("input").unwrap());
    let (_, path) = pathfind(&map, guard_pos, Direction::North);

    println!("part 1: {}", unique_positions(&path));
    println!("part 2: {}", find_loops(&map, &path).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

    #[test]
    fn test_parse_map() {
        let (_, guard_pos) = parse_map(EXAMPLE_INPUT);
        assert_eq!(guard_pos, (4, 6));
    }

    #[test]
    fn test_pathfind_unique_positions() {
        let (map, guard_pos) = parse_map(EXAMPLE_INPUT);
        assert_eq!(
            unique_positions(&pathfind(&map, guard_pos, Direction::North).1),
            41
        );
    }

    #[test]
    fn test_loops() {
        let (map, guard_pos) = parse_map(EXAMPLE_INPUT);
        let (_, history) = pathfind(&map, guard_pos, Direction::North);
        let blocks = find_loops(&map, &history);

        assert!(blocks.contains(&(3, 6)));
        assert!(blocks.contains(&(6, 7)));
        assert!(blocks.contains(&(7, 7)));
        assert!(blocks.contains(&(1, 8)));
        assert!(blocks.contains(&(3, 8)));
        assert!(blocks.contains(&(7, 9)));
        assert_eq!(blocks.len(), 6);
    }

    #[test]
    fn test_custom_case_1() {
        let (map, guard_pos) = parse_map("..#.\n...#\n..^.");
        let (_, history) = pathfind(&map, guard_pos, Direction::North);

        assert_eq!(unique_positions(&history), 2);
        assert_eq!(find_loops(&map, &history).len(), 0);
    }

    #[test]
    fn test_custom_case_2() {
        let (map, guard_pos) = parse_map(".#.\n#.#\n#^.\n...");
        let (_, history) = pathfind(&map, guard_pos, Direction::North);

        assert_eq!(unique_positions(&history), 3);
        assert_eq!(find_loops(&map, &history).len(), 1);
    }

    #[test]
    fn test_custom_case_3() {
        let (map, guard_pos) = parse_map(".#.\n..#\n#^.\n...");
        let (_, history) = pathfind(&map, guard_pos, Direction::North);

        assert_eq!(unique_positions(&history), 3);
        assert_eq!(find_loops(&map, &history).len(), 1);
    }
}
