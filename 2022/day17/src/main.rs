use std::{fs, str::Chars, iter::Cycle, collections::HashMap};

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

fn drop_many(rocks: &Vec<Vec<Coordinate>>, jet_pattern: &str, n: i64) -> i64 {
    let mut rock_cycle = rocks.iter().cycle();
    let mut jet_pattern_cycle = jet_pattern.chars().cycle();
    let mut map = Vec::new();
    let mut truncate_heights: HashMap<usize, (i64, i64)> = HashMap::new();
    let mut full_lines: i64 = 0;
    let mut pattern_used = false;


    let mut i = 0;

    loop {
        if i >= n {
            break;
        }

        drop_rock(&mut map, rock_cycle.next().expect("should have another rock"), &mut jet_pattern_cycle);

        i += 1;

        for j in (1..map.len()).rev() {
            if map.get(j) == Some(&[true; 7]) {
                full_lines += j as i64;
                map = map[j..].to_vec();

                if !pattern_used {
                    if let Some(th) = truncate_heights.clone().get(&j) {
                        truncate_heights.insert(j, (i, th.1 + 1));

                        if th.1 > 2 {
                            let i_increment = i - th.0;
                            let lines_increment = j as i64;
                            println!("found pattern: i=+{}, height=+{}", i_increment, lines_increment);

                            while i + i_increment < n {
                                i += i_increment;
                                full_lines += lines_increment;
                            }
                            println!("finished pattern: i={}, height={}", i, full_lines);
                            pattern_used = true;
                        }
                    } else {
                        truncate_heights.insert(j, (i, 1));
                    }
                }

                break;
            }
        }
    }

    (map.len() as i64) + full_lines
}

fn main() {
    // let rocks = parse_rocks(&fs::read_to_string("inputs/rocks.txt").expect("should be able to read input"));
    // let jet_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    // let height = drop_many(&rocks, jet_pattern, 1000000000000);
    // println!("{}", height);

    //     return;

    let raw_rocks = fs::read_to_string("inputs/rocks.txt").expect("should be able to read input");
    let jet_pattern = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let rocks = parse_rocks(&raw_rocks);

    let height1 = drop_many(&rocks, &jet_pattern.trim(), 2022);
    println!("Part 1: {}", height1);

    let height2 = drop_many(&rocks, &jet_pattern.trim(), 1000000000000);
    println!("Part 2: {}", height2);
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
        let height = drop_many(&rocks, jet_pattern, 2022);

        assert_eq!(height, 3068);
    }

    #[test]
    #[ignore]
    fn drop_many_example_long() {
        let rocks = parse_rocks(&fs::read_to_string("inputs/rocks.txt").expect("should be able to read input"));
        let jet_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let height = drop_many(&rocks, jet_pattern, 1000000000000);

        assert_eq!(height, 1514285714288);
    }
}
