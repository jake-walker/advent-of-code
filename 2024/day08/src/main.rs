use colored::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Coords = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
    antennas: HashMap<Coords, char>,
    antinodes: HashMap<Coords, char>,
    bounds: Coords,
}

impl Map {
    fn draw(&self) -> () {
        print!("   ");
        for x in 0..self.bounds.0 {
            print!("{}", x % 10);
        }
        println!("");
        for y in 0..self.bounds.1 {
            print!("{:>2} ", y);
            for x in 0..self.bounds.0 {
                if let Some(c) = self.antennas.get(&(x, y)) {
                    print!("{}", c.to_string().red());
                } else if let Some(c) = self.antinodes.get(&(x, y)) {
                    print!("{}", c.to_string().green());
                } else {
                    print!("{}", ".".white());
                }
            }
            println!("");
        }
    }

    fn process_antinodes(&mut self, part2: bool) -> () {
        // create a list of coordinates where each frequency has antennas
        let mut freq_antennas: HashMap<char, Vec<Coords>> = HashMap::new();

        for (k, v) in self.antennas.iter() {
            freq_antennas.entry(*v).or_insert(Vec::new()).push(*k);
        }

        // for each frequency
        for (freq, antennas) in freq_antennas {
            // loop over combinations of the frequency antennas
            for (a, b) in antennas.into_iter().tuple_combinations() {
                // the distance between the two antennas in x and y components
                let (xi, yi) = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);

                // this is really lazy, but it works
                // the good plan was to work out the origin of the antenna using (a.0 % xi, a.1 % yi)
                // this would be where the *first* antenna is on the grid, then we add xi and yi
                // respectively until we reach out out bounds. but this doesn't work with part 1
                // as well, meaning this would need to have a separate function and i didn't want any
                // duplicated code
                let range = {
                    if !part2 {
                        [-1, 2].to_vec()
                    } else {
                        Vec::from_iter(-1000..1000)
                    }
                };

                // we want to place an antinode 1 distance the first antenna, and 2 in front
                // 0 would be the first antenna itself, and 1 would be the second antenna
                for i in range {
                    // calculate position of antinode
                    let (x, y) = (a.0 as i32 + (xi * i), a.1 as i32 + (yi * i));

                    // check the antinode is within the bounds
                    if x < 0 || x >= self.bounds.0 as i32 || y < 0 || y >= self.bounds.1 as i32 {
                        continue;
                    }

                    self.antinodes.insert((x as usize, y as usize), freq);
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Map {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut antennas = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }

            antennas.insert((x, y), char);
        }
    }

    Map {
        antinodes: HashMap::new(),
        antennas,
        bounds: (lines[0].len(), lines.len()),
    }
}

fn main() {
    let m = parse_input(&aocutils::read_input("input").unwrap());

    let mut m1 = m.clone();
    m1.process_antinodes(false);
    println!(
        "part 1: {}",
        m1.antinodes.keys().collect::<HashSet<_>>().len()
    );

    let mut m2 = m;
    m2.process_antinodes(true);
    println!(
        "part 2: {}",
        m2.antinodes.keys().collect::<HashSet<_>>().len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE_INPUT: &str = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            Map {
                antinodes: HashMap::new(),
                antennas: HashMap::from([
                    ((8, 1), '0'),
                    ((5, 2), '0'),
                    ((7, 3), '0'),
                    ((4, 4), '0'),
                    ((6, 5), 'A'),
                    ((8, 8), 'A'),
                    ((9, 9), 'A')
                ]),
                bounds: (12, 12)
            }
        );
    }

    #[test]
    fn test_process_antinodes_p1() {
        let mut m = parse_input(EXAMPLE_INPUT);
        m.process_antinodes(false);
        m.draw();

        let mut actual_antinodes = m.antinodes.keys().unique().collect::<Vec<&Coords>>();
        let mut expected_antinodes: Vec<&Coords> = Vec::from([
            &(6, 0),
            &(11, 0),
            &(3, 1),
            &(4, 2),
            &(10, 2),
            &(2, 3),
            &(9, 4),
            &(1, 5),
            &(3, 6),
            &(0, 7),
            &(7, 7),
            &(10, 10),
            &(10, 11),
            &(6, 5),
        ]);

        actual_antinodes.sort();
        expected_antinodes.sort();

        assert_eq!(actual_antinodes, expected_antinodes);
    }

    #[test]
    fn test_process_antinodes_p2() {
        let mut m = parse_input(EXAMPLE_INPUT);
        m.process_antinodes(true);
        m.draw();

        let actual_antinodes = m.antinodes.keys().collect::<HashSet<_>>().len();

        assert_eq!(actual_antinodes, 34);
    }
}
