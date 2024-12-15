use std::collections::{HashMap, VecDeque};

type Coords = (isize, isize);

#[derive(Debug, PartialEq, Eq)]
struct Puzzle {
    robot: Coords,
    map: HashMap<Coords, Item>,
    movements: VecDeque<Direction>,
}

impl Puzzle {
    fn all_box_gps(&self) -> Vec<isize> {
        self.map
            .iter()
            .filter(|(_, x)| x == &&Item::Box || x == &&Item::BigBoxLeft)
            .map(|((x, y), _)| 100 * y + x)
            .collect()
    }

    fn draw(&self) -> String {
        let y_max = *self.map.iter().map(|((_, y), _)| y).max().unwrap() + 1;
        let x_max = *self.map.iter().map(|((x, _), _)| x).max().unwrap() + 1;
        let mut result = String::new();

        for y in 0..y_max {
            for x in 0..x_max {
                result += match (self.robot == (x, y), self.map.get(&(x, y))) {
                    (true, _) => "@",
                    (_, Some(Item::Box)) => "O",
                    (_, Some(Item::Wall)) => "#",
                    (_, Some(Item::BigBoxLeft)) => "[",
                    (_, Some(Item::BigBoxRight)) => "]",
                    (_, None) => ".",
                };
            }
            result += "\n";
        }

        result.trim().to_string()
    }

    fn move_robot(&mut self) -> bool {
        if let Some(dir) = self.movements.pop_front() {
            let new_pos = dir.apply_to_coords(self.robot);

            match self.map.get(&new_pos) {
                // if the new spot is empty, update the robot position
                None => self.robot = new_pos,
                // if there's a wall, do nothing
                Some(Item::Wall) => (),

                Some(Item::Box | Item::BigBoxLeft | Item::BigBoxRight) => {
                    // can this box be moved?
                    let moveable;
                    let mut next_pos = new_pos;
                    loop {
                        next_pos = dir.apply_to_coords(next_pos);
                        match self.map.get(&next_pos) {
                            None => {
                                moveable = true;
                                break;
                            }
                            Some(Item::Wall) => {
                                moveable = false;
                                break;
                            }
                            // TODO: check next position for left/right side of box for obstructions
                            _ => (),
                        }
                    }

                    if !moveable {
                        return true;
                    }

                    // TODO: change this to shuffle items along
                    // add a box to the empty space
                    self.map.insert(next_pos, Item::Box);
                    // remove the box from robot position
                    self.map.remove(&new_pos);
                    // move the robot
                    self.robot = new_pos;
                }
            }

            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Box,
    Wall,
    BigBoxLeft,
    BigBoxRight,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '#' => Item::Wall,
            'O' => Item::Box,
            '[' => Item::BigBoxLeft,
            ']' => Item::BigBoxRight,
            _ => panic!("unexpected item {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '^' => Direction::Up,
            _ => panic!("unexpected direction {}", value),
        }
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn to_coords(&self) -> Coords {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn apply_to_coords(&self, coords: Coords) -> Coords {
        let d = self.to_coords();
        (coords.0 + d.0, coords.1 + d.1)
    }
}

fn parse_input(input: &str, double: bool) -> Puzzle {
    let (map, movements) = input.split_once("\n\n").unwrap();

    let map_iter = map.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c != &'.')
            .flat_map(move |(x, c)| {
                if double {
                    let (mut left_char, mut right_char) = (c, c);

                    if c == 'O' {
                        (left_char, right_char) = ('[', ']');
                    }

                    Vec::from([
                        ((x as isize * 2, y as isize), left_char),
                        (((x as isize * 2) + 1, y as isize), right_char),
                    ])
                } else {
                    Vec::from([((x as isize, y as isize), c)])
                }
            })
    });

    Puzzle {
        robot: map_iter.clone().find(|(_, c)| c == &'@').unwrap().0,
        map: map_iter
            .filter(|(_, c)| c != &'@')
            .map(|(pos, c)| (pos, c.into()))
            .collect(),
        movements: movements
            .chars()
            .filter(|c| c != &'\n')
            .map(|c| c.into())
            .collect(),
    }
}

fn main() {
    let mut puzzle = parse_input(&aocutils::read_input("input").unwrap(), false);
    while puzzle.move_robot() {}

    println!("part 1: {}", puzzle.all_box_gps().iter().sum::<isize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";
    const DOUBLE_EXAMPLE: &str =
        "#######\n#...#.#\n#.....#\n#..OO@#\n#..O..#\n#.....#\n#######\n\n<vv<<^^<<^^";

    #[test]
    fn test_parse_input() {
        let puzzle = parse_input(SMALL_EXAMPLE, false);

        assert_eq!(puzzle.robot, (2, 2));
        assert_eq!(
            puzzle.movements,
            VecDeque::from([
                Direction::Left,
                Direction::Up,
                Direction::Up,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Down,
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left
            ])
        );
        assert_eq!(puzzle.map.values().filter(|v| v == &&Item::Box).count(), 6);
        assert_eq!(
            puzzle.map.values().filter(|v| v == &&Item::Wall).count(),
            30
        );
    }

    #[test]
    fn test_move_robot() {
        let mut puzzle = parse_input(SMALL_EXAMPLE, false);
        let expected_positions: Vec<Coords> = Vec::from([
            (2, 2),
            (2, 2),
            (2, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (4, 1),
            (4, 2),
            (4, 2),
            (3, 2),
            (3, 3),
            (4, 3),
            (5, 3),
            (5, 4),
            (4, 4),
            (4, 4),
        ]);

        for (t, expected_pos) in expected_positions.into_iter().enumerate() {
            println!("t={}\n{}\n\n", t, puzzle.draw());
            assert_eq!(puzzle.robot, expected_pos, "bad position for t={}", t);
            puzzle.move_robot();
        }
    }

    #[test]
    fn test_big_example() {
        let mut puzzle = parse_input(&aocutils::read_input("big_example").unwrap(), false);
        while puzzle.move_robot() {}

        assert_eq!(puzzle.draw(), "##########\n#.O.O.OOO#\n#........#\n#OO......#\n#OO@.....#\n#O#.....O#\n#O.....OO#\n#O.....OO#\n#OO....OO#\n##########");
    }

    #[test]
    fn test_box_gps_example_3() {
        let puzzle = Puzzle {
            map: HashMap::from([
                ((0, 0), Item::Wall),
                ((1, 0), Item::Wall),
                ((2, 0), Item::Wall),
                ((3, 0), Item::Wall),
                ((4, 0), Item::Wall),
                ((5, 0), Item::Wall),
                ((6, 0), Item::Wall),
                ((0, 1), Item::Wall),
                ((0, 1), Item::Wall),
                ((0, 1), Item::Wall),
                ((4, 1), Item::Box),
            ]),
            movements: VecDeque::new(),
            robot: (0, 0),
        };

        assert_eq!(puzzle.all_box_gps(), vec![104]);
    }

    #[test]
    fn test_box_gps_example_small() {
        let mut puzzle = parse_input(SMALL_EXAMPLE, false);
        while puzzle.move_robot() {}

        assert_eq!(puzzle.all_box_gps().iter().sum::<isize>(), 2028);
    }

    #[test]
    fn test_box_gps_example_big() {
        let mut puzzle = parse_input(&aocutils::read_input("big_example").unwrap(), false);
        while puzzle.move_robot() {}

        assert_eq!(puzzle.all_box_gps().iter().sum::<isize>(), 10092);
    }

    #[test]
    fn test_parse_example_big_double() {
        let puzzle = parse_input(&aocutils::read_input("big_example").unwrap(), true);

        assert_eq!(puzzle.draw(), "####################\n##....[]....[]..[]##\n##............[]..##\n##..[][]....[]..[]##\n##....[]@.....[]..##\n##[]##....[]......##\n##[]....[]....[]..##\n##..[][]..[]..[][]##\n##........[]......##\n####################");
        assert_eq!(puzzle.robot, (8, 4));
    }

    #[test]
    fn test_box_gps_example_double() {
        let puzzle = Puzzle {
            map: HashMap::from([
                ((0, 0), Item::Wall),
                ((1, 0), Item::Wall),
                ((2, 0), Item::Wall),
                ((3, 0), Item::Wall),
                ((4, 0), Item::Wall),
                ((5, 0), Item::Wall),
                ((6, 0), Item::Wall),
                ((7, 0), Item::Wall),
                ((8, 0), Item::Wall),
                ((9, 0), Item::Wall),
                ((0, 1), Item::Wall),
                ((1, 1), Item::Wall),
                ((0, 2), Item::Wall),
                ((1, 2), Item::Wall),
                ((5, 1), Item::BigBoxLeft),
                ((6, 1), Item::BigBoxRight),
            ]),
            movements: VecDeque::new(),
            robot: (0, 0),
        };

        assert_eq!(puzzle.all_box_gps(), vec![105]);
    }

    #[test]
    fn test_move_robot_double() {
        let mut puzzle = parse_input(DOUBLE_EXAMPLE, true);
        let expected_positions: Vec<Coords> = Vec::from([
            (10, 3),
            (9, 3),
            (9, 4),
            (9, 5),
            (8, 5),
            (7, 5),
            (7, 4),
            (7, 4),
            (6, 4),
            (5, 4),
            (5, 3),
            (5, 2),
        ]);

        for (t, expected_pos) in expected_positions.into_iter().enumerate() {
            println!("t={}\n{}\n\n", t, puzzle.draw());
            assert_eq!(puzzle.robot, expected_pos, "bad position for t={}", t);
            puzzle.move_robot();
        }
    }

    #[test]
    fn test_big_example_double() {
        let mut puzzle = parse_input(&aocutils::read_input("big_example").unwrap(), true);
        while puzzle.move_robot() {}

        assert_eq!(puzzle.draw(), "####################\n##[].......[].[][]##\n##[]...........[].##\n##[]........[][][]##\n##[]......[]....[]##\n##..##......[]....##\n##..[]............##\n##..@......[].[][]##\n##......[][]..[]..##\n####################");
    }
}
