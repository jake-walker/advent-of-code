use std::collections::HashSet;

type Coords = (i32, i32);
type Map = Vec<Vec<bool>>;

#[derive(Clone, Copy, PartialEq)]
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
                '^' => guard_pos = Some((x as i32, y as i32)),
                _ => continue,
            }
        }
    }

    (map, guard_pos.unwrap())
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

fn pathfind(map: &Map, start_pos: Coords) -> Vec<(Coords, Direction)> {
    let mut history = Vec::new();
    let mut pos = start_pos.clone();
    let mut direction = Direction::North;

    loop {
        history.push((pos, direction));
        let new_pos = (
            pos.0 + direction.to_coords().0,
            pos.1 + direction.to_coords().1,
        );

        // check bounds
        if new_pos.0 < 0
            || new_pos.0 >= map[0].len() as i32
            || new_pos.1 < 0
            || new_pos.1 >= map.len() as i32
        {
            break;
        }

        if map[new_pos.1 as usize][new_pos.0 as usize] {
            // turn
            direction = direction.rotate_cw();
            continue;
        }

        pos = new_pos;
        // print_map(map, new_pos);
    }

    history
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

    for (i, (pos, direction)) in history.iter().enumerate() {
        let forward_pos = (
            pos.0 + direction.to_coords().0,
            pos.1 + direction.to_coords().1,
        );

        // position for blockage is out of bounds, skip
        if forward_pos.0 < 0
            || forward_pos.0 >= map[0].len() as i32
            || forward_pos.1 < 0
            || forward_pos.1 >= map.len() as i32
        {
            continue;
        }

        // if this is the start position or there's already a blockage, skip
        if i == 0 || map[forward_pos.1 as usize][forward_pos.0 as usize] {
            continue;
        }

        let direction_cw = direction.rotate_cw();
        let prev_history = history
            .iter()
            .take(i - 1)
            .filter(|(_, d)| d == &direction_cw);

        if (direction_cw == Direction::North
            && prev_history
                .clone()
                .any(|(p, _)| p.0 == pos.0 && p.1 <= pos.1))
            || (direction_cw == Direction::East
                && prev_history
                    .clone()
                    .any(|(p, _)| p.0 >= pos.0 && p.1 == pos.1))
            || (direction_cw == Direction::South
                && prev_history
                    .clone()
                    .any(|(p, _)| p.0 == pos.0 && p.1 >= pos.1))
            || (direction_cw == Direction::West
                && prev_history
                    .clone()
                    .any(|(p, _)| p.0 <= pos.0 && p.1 == pos.1))
        {
            print_map(map, forward_pos, Some(history.iter().take(i - 1).collect()));
            new_blocks.push(forward_pos);
        }
    }

    new_blocks
}

fn main() {
    let (map, guard_pos) = parse_map(&aocutils::read_input("input").unwrap());
    let path = pathfind(&map, guard_pos);

    println!("part 1: {}", unique_positions(&path));
    // > 646
    println!("part 2: {}", find_loops(&map, &path).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    // const EXAMPLE_INPUT: &str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
    const EXAMPLE_INPUT: &str = "....#.....\n.....#..#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n....#.#...";

    #[test]
    fn test_parse_map() {
        let (_, guard_pos) = parse_map(EXAMPLE_INPUT);
        assert_eq!(guard_pos, (4, 6));
    }

    #[test]
    #[ignore]
    fn test_pathfind_unique_positions() {
        let (map, guard_pos) = parse_map(EXAMPLE_INPUT);
        assert_eq!(unique_positions(&pathfind(&map, guard_pos)), 41);
    }

    #[test]
    fn test_loops() {
        let (map, guard_pos) = parse_map(EXAMPLE_INPUT);
        let history = pathfind(&map, guard_pos);
        let blocks = find_loops(&map, &history);

        print_map(&map, (0, 0), Some(history.iter().collect()));

        assert!(blocks.contains(&(3, 6)));
        assert!(blocks.contains(&(6, 7)));
        assert!(blocks.contains(&(7, 7)));
        assert!(blocks.contains(&(1, 8)));
        assert!(blocks.contains(&(3, 8)));
        assert!(blocks.contains(&(7, 9)));
        assert_eq!(blocks.len(), 6);
        assert!(false);
    }
}
