use std::fs;

use itertools::Itertools;

type Platform = Vec<Vec<char>>;

const MOVE_NORTH: (isize, isize) = (0, -1);

fn parse_platform(s: &str) -> Platform {
    s.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn move_rock_step(p: &mut Platform, pos: (usize, usize), move_by: (isize, isize)) -> Option<(usize, usize)> {
    let dest = ((pos.0 as isize + move_by.0) as usize, (pos.1 as isize + move_by.1) as usize);

    if dest.0 < 0 || dest.0 > p.len() || dest.1 < 0 || dest.1 > p[0].len() {
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

fn tilt_platform(p: &mut Platform, move_by: (isize, isize)) -> () {
    for y in 0..p.len() {
        for x in 0..p[y].len() {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    static INITIAL_EXAMPLE: &str = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";
    static FINISHED_EXAMPLE: &str = "OOOO.#.O..\nOO..#....#\nOO..O##..O\nO..#.OO...\n........#.\n..#....#.#\n..O..#.O.O\n..O.......\n#....###..\n#....#....";
    
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
}