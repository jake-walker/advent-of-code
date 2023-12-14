use std::fs;

use itertools::Itertools;

type Platform = Vec<Vec<char>>;

const MOVE_NORTH: (isize, isize) = (0, -1);
const MOVE_EAST: (isize, isize) = (1, 0);
const MOVE_SOUTH: (isize, isize) = (0, 1);
const MOVE_WEST: (isize, isize) = (-1, 0);

fn parse_platform(s: &str) -> Platform {
    s.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn move_rock_step(p: &mut Platform, pos: (usize, usize), move_by: (isize, isize)) -> Option<(usize, usize)> {
    let dest = ((pos.0 as isize + move_by.0) as usize, (pos.1 as isize + move_by.1) as usize);

    if dest.0 < 0 || dest.0 >= p.len() || dest.1 < 0 || dest.1 >= p[0].len() {
        return None;
    }
    
    if p[dest.1][dest.0] != '.' {
        return None;
    }
    
    let src_char = p[pos.1][pos.0];
    
    p[pos.1][pos.0] = '.';
    p[dest.1][dest.0] = src_char;
    
    Some(dest)
}

fn transform_pos(i: usize, max_dimensions: (usize, usize), move_by: (isize, isize)) -> (usize, usize) {
    match move_by {
        MOVE_SOUTH => (i % max_dimensions.0, max_dimensions.1 - (i / max_dimensions.0) - 1),
        MOVE_WEST => (i / max_dimensions.1, i % max_dimensions.1),
        MOVE_EAST => (max_dimensions.0 - (i / max_dimensions.1) - 1, i % max_dimensions.1),
        _ => (i % max_dimensions.0, i / max_dimensions.0)
    }
}

fn tilt_platform(p: &mut Platform, move_by: (isize, isize)) -> () {
    let max_dimensions = (p[0].len(), p.len());

    for i in 0..(max_dimensions.0 * max_dimensions.1) {
        let (x, y) = transform_pos(i, (max_dimensions.0, max_dimensions.1), move_by);

        if p[y][x] != 'O' {
            continue;
        }

        let mut pos = (x, y);

        loop {
            if let Some(new_pos) = move_rock_step(p, pos, move_by) {
                pos = new_pos
            } else {
                break;
            }
        }
    }
}

fn cycle_platform(p: &mut Platform, repeat: usize) -> () {
    for i in 0..repeat {
        if i % 100000 == 0 {
            println!("cycling {}%...", (i / repeat) * 100);
        }

        for direction in vec![MOVE_NORTH, MOVE_WEST, MOVE_SOUTH, MOVE_EAST] {
            tilt_platform(p, direction);
        }
    }
}

fn calculate_load(p: &Platform) -> usize {
    let mut load = 0;
    
    for (y, line) in p.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            if value != &'O' {
                continue;
            }
            
            load += p.len() - y;
        }
    }
    
    load
}

fn main() {
    let input = fs::read_to_string("inputs/input").expect("should be able to read input");
    let mut platform = parse_platform(&input);
    tilt_platform(&mut platform, MOVE_NORTH);
    println!("Part 1: {}", calculate_load(&platform));

//    platform = parse_platform(&input);
//    cycle_platform(&mut platform, 1000000000);
//    println!("Part 2: {}", calculate_load(&platform));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    static INITIAL_EXAMPLE: &str = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";
    static FINISHED_EXAMPLE: &str = "OOOO.#.O..\nOO..#....#\nOO..O##..O\nO..#.OO...\n........#.\n..#....#.#\n..O..#.O.O\n..O.......\n#....###..\n#....#....";
    static ONE_CYCLE_EXAMPLE: &str = ".....#....\n....#...O#\n...OO##...\n.OO#......\n.....OOO#.\n.O#...O#.#\n....O#....\n......OOOO\n#...O###..\n#..OO#....";
    static TWO_CYCLES_EXAMPLE: &str = ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#..OO###..\n#.OOO#...O";
    static THREE_CYCLES_EXAMPLE: &str = ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#...O###.O\n#.OOO#...O";
    
    fn print_platform(p: &Platform) -> () {
        for line in p {
            println!("{}", line.iter().collect::<String>());
        }
    }
    
    #[test]
    fn test_tilt_platform() {
        let mut actual = parse_platform(INITIAL_EXAMPLE);
        tilt_platform(&mut actual, MOVE_NORTH);
        
        let expected = parse_platform(FINISHED_EXAMPLE);
        
        print_platform(&actual);
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_load() {
        let actual = calculate_load(&parse_platform(FINISHED_EXAMPLE));
        let expected = 136;
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_transform_pos() {
        assert_eq!(transform_pos(5, (10, 10), MOVE_NORTH), (5, 0));
        assert_eq!(transform_pos(5, (10, 10), MOVE_EAST), (9, 5));
        assert_eq!(transform_pos(5, (10, 10), MOVE_SOUTH), (5, 9));
        assert_eq!(transform_pos(5, (10, 10), MOVE_WEST), (0, 5));

        assert_eq!(transform_pos(10, (10, 10), MOVE_NORTH), (0, 1));
        assert_eq!(transform_pos(10, (10, 10), MOVE_EAST), (8, 0));
        assert_eq!(transform_pos(10, (10, 10), MOVE_SOUTH), (0, 8));
        assert_eq!(transform_pos(10, (10, 10), MOVE_WEST), (1, 0));
    }

    #[test]
    fn test_cycle_platform() {
        let mut actual = parse_platform(INITIAL_EXAMPLE);

        cycle_platform(&mut actual, 1);
        assert_eq!(actual, parse_platform(ONE_CYCLE_EXAMPLE));

        cycle_platform(&mut actual, 1);
        assert_eq!(actual, parse_platform(TWO_CYCLES_EXAMPLE));

        cycle_platform(&mut actual, 1);
        assert_eq!(actual, parse_platform(THREE_CYCLES_EXAMPLE));
    }
}