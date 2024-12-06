use std::collections::HashSet;

type Coords = (i32, i32);
type Map = Vec<Vec<bool>>;

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

fn print_map(map: &Map, pos: Coords) -> () {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if pos.0 as usize == x && pos.1 as usize == y {
                print!("%");
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

fn pathfind(map: &Map, start_pos: Coords) -> usize {
    let mut history = HashSet::from([start_pos]);
    let mut pos = start_pos.clone();
    let mut direction = Direction::North;

    loop {
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

        if map[new_pos.1 as usize][new_pos.0 as usize] == true {
            // turn
            direction = direction.rotate_cw();
            continue;
        }

        pos = new_pos;
        history.insert(new_pos);
        // print_map(map, new_pos);
    }

    history.len()
}

fn main() {
    let (map, guard_pos) = parse_map(&aocutils::read_input("input").unwrap());

    println!("part 1: {}", pathfind(&map, guard_pos));
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
    fn test_pathfind() {
        let (map, guard_pos) = parse_map(EXAMPLE_INPUT);
        assert_eq!(pathfind(&map, guard_pos), 41);
    }
}
