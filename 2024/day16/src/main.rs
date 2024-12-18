use std::collections::HashSet;

use petgraph::{
    algo::{astar, dijkstra},
    prelude::UnGraphMap,
    visit::EdgeRef,
};

static TURN_COST: usize = 1000;
static MOVE_COST: usize = 1;

type Coords = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    NS,
    EW,
}

struct Maze {
    start: Coords,
    goal: Coords,
    map: UnGraphMap<(Coords, Direction), usize>,
}

impl Maze {
    fn new() -> Self {
        Maze {
            start: (0, 0),
            goal: (0, 0),
            map: UnGraphMap::new(),
        }
    }

    fn best_path_cost(&self) -> Option<(usize, usize)> {
        let res = astar(
            &self.map,
            (self.start, Direction::EW),
            |(pos, _)| pos == self.goal,
            |e| *e.weight(),
            |_| 0,
        );

        res.and_then(|(cost, path)| {
            let unique_path = path
                .iter()
                .map(|(pos, _)| *pos)
                .collect::<HashSet<Coords>>();
            Some((cost, unique_path.len()))
        })
    }
}

fn parse_input(input: &str) -> Maze {
    let mut maze = Maze::new();

    let map_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (y, line) in map_vec.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'#' {
                continue;
            }

            if c == &'S' {
                maze.start = (x, y);
            }

            if c == &'E' {
                maze.goal = (x, y);
            }

            // add node for turning
            maze.map
                .add_edge(((x, y), Direction::NS), ((x, y), Direction::EW), TURN_COST);

            // add nodes for straight lines
            for (dir, dx, dy) in [(Direction::EW, 1, 0), (Direction::NS, 0, 1)] {
                let (x1, y1) = (x + dx, y + dy);

                if x1 >= map_vec[0].len() || y1 >= map_vec.len() || &map_vec[y1][x1] == &'#' {
                    continue;
                }

                maze.map.add_edge(((x, y), dir), ((x1, y1), dir), MOVE_COST);
            }
        }
    }

    maze
}

fn main() {
    let maze = parse_input(&aocutils::read_input("input").unwrap());

    let (part1, part2) = maze.best_path_cost().unwrap();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_1: &str = "###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############";
    static EXAMPLE_2: &str = "#################\n#...#...#...#..E#\n#.#.#.#.#.#.#.#.#\n#.#.#.#...#...#.#\n#.#.#.#.###.#.#.#\n#...#.#.#.....#.#\n#.#.#.#.#.#####.#\n#.#...#.#.#.....#\n#.#.#####.#.###.#\n#.#.#.......#...#\n#.#.###.#####.###\n#.#.#...#.....#.#\n#.#.#.#####.###.#\n#.#.#.........#.#\n#.#.#.#########.#\n#S#.............#\n#################";

    #[test]
    fn test_example_1() {
        let maze = parse_input(EXAMPLE_1);
        assert_eq!(maze.best_path_cost(), Some((7036, 45)));
    }

    #[test]
    fn test_example_2() {
        let maze = parse_input(EXAMPLE_2);
        assert_eq!(maze.best_path_cost(), Some((11048, 64)));
    }
}
