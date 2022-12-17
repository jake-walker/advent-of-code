use std::{fs, slice::Iter, str::Chars, iter::Cycle};

type Coordinate = (usize, usize);

fn get_bounds(shape: &Vec<Coordinate>) -> (usize, usize) {
    (
        shape.iter().max_by_key(|c| c.0).unwrap().0 + 1,
        shape.iter().max_by_key(|c| c.1).unwrap().1 + 1
    )
}

fn parse_rocks(input: &str) -> Vec<Vec<Coordinate>> {
    input.split("\n\n").map(|r| {
        let lines = r.lines();
        let y_max = r.lines().count();
        let mut shape = Vec::new();

        for (y, row) in lines.enumerate() {
            for (x, char) in row.chars().enumerate() {
                if char == '#' {
                    shape.push((x, y_max - y - 1));
                }
            }
        }

        shape
    }).collect()
}

fn find_neighbours(map: &Vec<[bool; 7]>, (x, y): Coordinate) -> [bool; 4] {
    [
        {
            if y+1 >= map.len() {
                false
            } else {
                map[y + 1][x]
            }
        },
        {
            if y < 1 {
                true
            } else if map.len() < y || map.len() == 0 {
                false
            } else {
                map[y - 1][x]
            }
        },
        {
            if x <= 0 {
                true
            } else if map.len() < y+1 || map.len() == 0 {
                false
            } else {
                map[y][x-1]
            }
        },
        {
            if x >= 6 {
                true
            } else if map.len() < y+1 || map.len() == 0 {
                false
            } else {
                map[y][x+1]
            }
        }
    ]
}

fn drop_rock(map: &mut Vec<[bool; 7]>, rock: &Vec<Coordinate>, jet_pattern: &mut Cycle<Chars>) {
    let (rock_x_bound, rock_y_bound) = get_bounds(rock);
    let (mut x, mut y) = (2, map.len() + 3);

    'outer: loop {
        let gas_dir = jet_pattern.next().expect("should have jet pattern left");
        if gas_dir != '>' && gas_dir != '<' {
            panic!("bad gas dir");
        }

        let mut move_ok = true;

        for r in rock {
            let (rx, ry) = (x + r.0, y + r.1);
            let neighbours = find_neighbours(map, (rx, ry));

            if gas_dir == '>' && neighbours[3] {
                move_ok = false;
                break;
            }

            if gas_dir == '<' && neighbours[2] {
                move_ok = false;
                break;
            }
        }

        match (move_ok, gas_dir) {
            (true, '>') => {
                if x < (7 - rock_x_bound) {
                    x += 1;
                }
            },
            (true, '<') => {
                if x > 0 {
                    x -= 1;
                }
            }
            (_, _) => {}
        }

        // check each part of rock for something underneath
        for r in rock {
            let (rx, ry) = (x + r.0, y + r.1);

            let neighbours = find_neighbours(map, (rx, ry));

            if neighbours[1] {
                break 'outer;
            }
        }

        y -= 1;
    }

    for _ in 0..((y+rock_y_bound).saturating_sub(map.len())) {
        map.push([false; 7]);
    }

    for r in rock {
        let (rx, ry) = (x + r.0, y + r.1);
        map[ry][rx] = true;
    }
}

fn drop_many(rocks: &Vec<Vec<Coordinate>>, jet_pattern: &str, n: i64) -> Vec<[bool; 7]> {
    let mut rock_cycle = rocks.iter().cycle();
    let mut jet_pattern_cycle = jet_pattern.chars().cycle();
    let mut map = Vec::new();

    for _ in 0..n {
        drop_rock(&mut map, rock_cycle.next().expect("should have another rock"), &mut jet_pattern_cycle);
    }

    map
}

fn main() {
    let raw_rocks = fs::read_to_string("inputs/rocks.txt").expect("should be able to read input");
    let jet_pattern = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let rocks = parse_rocks(&raw_rocks);

    let map1 = drop_many(&rocks, &jet_pattern.trim(), 2022);
    println!("Part 1: {}", map1.len());

    let map2 = drop_many(&rocks, &jet_pattern.trim(), 1000000000000);
    println!("Part 1: {}", map2.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_maps(a: &Vec<[bool; 7]>, b: &Vec<[bool; 7]>) {
        let height = {
            if a.len() > b.len() {
                a.len()
            } else {
                b.len()
            }
        };

        for y in (0..height).rev() {
            print!("|");

            if a.len() <= y {
                print!("       ");
            } else {
                for ax in a[y] {
                    if ax {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }

            print!("|  |");

            if b.len() <= y {
                print!("       ");
            } else {
                for bx in b[y] {
                    if bx {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }

            println!("|");
        }

        println!("+-------+  +-------+");
    }

    #[test]
    fn parse_rocks_example() {
        let rocks = fs::read_to_string("inputs/rocks.txt").expect("should be able to read input");

        assert_eq!(parse_rocks(&rocks), vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
            vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
            vec![(0, 3), (0, 2), (0, 1), (0, 0)],
            vec![(0, 1), (1, 1), (0, 0), (1, 0)]
        ]);
    }

    #[test]
    fn drop_rocks_example() {
        let mut jet_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".chars().cycle();
        let mut map: Vec<[bool; 7]> = Vec::new();

        drop_rock(&mut map, &vec![(0, 0), (1, 0), (2, 0), (3, 0)], &mut jet_pattern);
        let expected1 = vec![
            [false, false, true, true, true, true, false]
        ];
        print_maps(&map, &expected1);
        assert_eq!(map, expected1, "first drop");

        println!("---");

        drop_rock(&mut map, &vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)], &mut jet_pattern);
        let expected2 = vec![
            [false, false, true, true, true, true, false],
            [false, false, false, true, false, false, false],
            [false, false, true, true, true, false, false],
            [false, false, false, true, false, false, false]
        ];
        print_maps(&map, &expected2);
        assert_eq!(map, expected2, "second drop");

        println!("---");

        drop_rock(&mut map, &vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)], &mut jet_pattern);
        let expected3 = vec![
            [false, false, true, true, true, true, false],
            [false, false, false, true, false, false, false],
            [false, false, true, true, true, false, false],
            [true, true, true, true, false, false, false],
            [false, false, true, false, false, false, false],
            [false, false, true, false, false, false, false]
        ];
        print_maps(&map, &expected3);
        assert_eq!(map, expected3, "third drop");

        println!("---");

        drop_rock(&mut map, &vec![(0, 3), (0, 2), (0, 1), (0, 0)], &mut jet_pattern);
        let expected4 = vec![
            [false, false, true, true, true, true, false],
            [false, false, false, true, false, false, false],
            [false, false, true, true, true, false, false],
            [true, true, true, true, true, false, false],
            [false, false, true, false, true, false, false],
            [false, false, true, false, true, false, false],
            [false, false, false, false, true, false, false]
        ];
        print_maps(&map, &expected4);
        assert_eq!(map, expected4, "fourth drop");
    }

    #[test]
    fn drop_many_example() {
        let rocks = parse_rocks(&fs::read_to_string("inputs/rocks.txt").expect("should be able to read input"));
        let jet_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let map = drop_many(&rocks, jet_pattern, 2022);

        assert_eq!(map.len(), 3068);
    }
}
