use itertools::Itertools;
use regex::Regex;

type Coords = (isize, isize);

static BOUNDARY: Coords = (101, 103);

#[derive(Debug, PartialEq, Eq)]
struct Robot {
    pub position: Coords,
    pub velocity: Coords,
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"(?m)^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();

    re.captures_iter(input)
        .map(|c| {
            let (_, [start_x, start_y, velocity_x, velocity_y]) = c.extract();
            Robot {
                position: (start_x.parse().unwrap(), start_y.parse().unwrap()),
                velocity: (velocity_x.parse().unwrap(), velocity_y.parse().unwrap()),
            }
        })
        .collect()
}

fn gen_map(robots: &Vec<Robot>, boundary: Coords) -> String {
    let mut map = String::new();

    for y in 0..boundary.1 {
        for x in 0..boundary.0 {
            let robot_count = robots
                .iter()
                .filter(|r| r.position.0 == x && r.position.1 == y)
                .count();

            if robot_count > 0 {
                map += &(robot_count % 10).to_string();
            } else {
                map += ".";
            }
        }
        map += "\n";
    }

    map.trim().to_string()
}

fn move_robots(robots: &mut Vec<Robot>, boundary: Coords) {
    for r in robots.iter_mut() {
        r.position.0 = (r.position.0 + r.velocity.0) % boundary.0;
        r.position.1 = (r.position.1 + r.velocity.1) % boundary.1;

        // wrap around if negative
        if r.position.0 < 0 {
            r.position.0 = boundary.0 - r.position.0.abs();
        }
        if r.position.1 < 0 {
            r.position.1 = boundary.1 - r.position.1.abs();
        }
    }
}

fn count_robots(robots: &Vec<Robot>, boundary: Coords) -> (usize, usize, usize, usize) {
    let horizontal = boundary.0 / 2;
    let vertical = boundary.1 / 2;

    // if there's no remainders, use >= rather than >
    let horizontal_remainder = boundary.0 % 2 != 0;
    let vertical_remainder = boundary.1 % 2 != 0;

    (
        // top left
        robots
            .iter()
            .filter(|r| r.position.0 < horizontal && r.position.1 < vertical)
            .count(),
        // top right
        robots
            .iter()
            .filter(|r| {
                ((horizontal_remainder && r.position.0 > horizontal)
                    || (!horizontal_remainder && r.position.0 >= horizontal))
                    && r.position.1 < vertical
            })
            .count(),
        // bottom left
        robots
            .iter()
            .filter(|r| {
                r.position.0 < horizontal
                    && ((vertical_remainder && r.position.1 > vertical)
                        || (!vertical_remainder && r.position.1 >= vertical))
            })
            .count(),
        // bottom right
        robots
            .iter()
            .filter(|r| {
                ((horizontal_remainder && r.position.0 > horizontal)
                    || (!horizontal_remainder && r.position.0 >= horizontal))
                    && ((vertical_remainder && r.position.1 > vertical)
                        || (!vertical_remainder && r.position.1 >= vertical))
            })
            .count(),
    )
}

/// Scan for lines with `threshold` items in a row
fn scan_for_grouped(robots: &Vec<Robot>, boundary: Coords, i: usize, threshold: usize) -> bool {
    for y in 0..boundary.1 {
        // get all the robot x positions in this row
        let mut x_positions = robots
            .iter()
            .filter(|r| r.position.1 == y)
            .map(|r| r.position.0)
            .unique()
            .collect::<Vec<isize>>();

        if x_positions.len() == 0 {
            continue;
        }

        x_positions.sort();

        let mut max_len = 0;
        let mut current_len = 0;

        for i in 0..(x_positions.len() - 1) {
            // get the previous, current and next positions in the line
            let prev = {
                if i <= 0 {
                    &-100
                } else {
                    &x_positions[i - 1]
                }
            };
            let cur = &x_positions[i];
            let next = &x_positions[i + 1];

            // if the previous is not 1 position before the current, reset the current length counter
            if cur - prev != 1 {
                current_len = 0;
            }

            // if the next position is 1 in front, increment the current length
            if next - cur == 1 {
                current_len += 1;
            }

            // if the current length is the highest, update the max length
            if current_len > max_len {
                max_len = current_len;
            }
        }

        // if this length is over the threshold, print the map for review
        if max_len > threshold {
            println!(
                "-=-=-=-=-=-=-=- t={} -=-=-=-=-=-=-=-\n{}",
                i,
                gen_map(robots, boundary)
            );
            return true;
        }
    }

    false
}

fn main() {
    let mut robots = parse_input(&aocutils::read_input("input").unwrap());
    for t in 0..100 {
        move_robots(&mut robots, BOUNDARY);
        scan_for_grouped(&robots, BOUNDARY, t, 10);
    }

    let (tl, tr, bl, br) = count_robots(&robots, BOUNDARY);
    println!("part 1: {}", tl * tr * bl * br);

    let mut t = 100;
    let mut found = false;
    while !found {
        t += 1;
        move_robots(&mut robots, BOUNDARY);
        found = scan_for_grouped(&robots, BOUNDARY, t, 10);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "p=0,4 v=3,-3\np=6,3 v=-1,-3\np=10,3 v=-1,2\np=2,0 v=2,-1\np=0,0 v=1,3\np=3,0 v=-2,-2\np=7,6 v=-1,-3\np=3,0 v=-1,-2\np=9,3 v=2,3\np=7,3 v=-1,2\np=2,4 v=2,-3\np=9,5 v=-3,-3";
    static EXAMPLE_BOUNDARY: Coords = (11, 7);

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            Vec::from([
                Robot {
                    position: (0, 4),
                    velocity: (3, -3)
                },
                Robot {
                    position: (6, 3),
                    velocity: (-1, -3)
                },
                Robot {
                    position: (10, 3),
                    velocity: (-1, 2)
                },
                Robot {
                    position: (2, 0),
                    velocity: (2, -1)
                },
                Robot {
                    position: (0, 0),
                    velocity: (1, 3)
                },
                Robot {
                    position: (3, 0),
                    velocity: (-2, -2)
                },
                Robot {
                    position: (7, 6),
                    velocity: (-1, -3)
                },
                Robot {
                    position: (3, 0),
                    velocity: (-1, -2)
                },
                Robot {
                    position: (9, 3),
                    velocity: (2, 3)
                },
                Robot {
                    position: (7, 3),
                    velocity: (-1, 2)
                },
                Robot {
                    position: (2, 4),
                    velocity: (2, -3)
                },
                Robot {
                    position: (9, 5),
                    velocity: (-3, -3)
                },
            ])
        );
    }

    #[test]
    fn test_move_robots() {
        let mut robots = Vec::from([Robot {
            position: (2, 4),
            velocity: (2, -3),
        }]);
        let expected_positions = Vec::from([(2, 4), (4, 1), (6, 5), (8, 2), (10, 6), (1, 3)]);

        for t in 0..expected_positions.len() {
            assert_eq!(
                robots[0].position, expected_positions[t],
                "expected position {:?} at t={}, got {:?}",
                expected_positions[t], t, robots[0].position
            );
            move_robots(&mut robots, EXAMPLE_BOUNDARY);
        }
    }

    #[test]
    fn test_move_edge_cases() {
        for (i, (start_pos, velocity, end_pos)) in [
            // overflow right side
            ((10, 0), (1, 0), (0, 0)),
            // overflow bottom side
            ((0, 6), (0, 1), (0, 0)),
            // underflow left side
            ((0, 0), (-1, 0), (10, 0)),
            // underflow top side
            ((0, 0), (0, -1), (0, 6)),
            // diagonal bottom left
            ((10, 6), (1, 1), (0, 0)),
        ]
        .into_iter()
        .enumerate()
        {
            let mut robots = Vec::from([Robot {
                position: start_pos,
                velocity,
            }]);
            move_robots(&mut robots, EXAMPLE_BOUNDARY);
            assert_eq!(
                robots[0].position, end_pos,
                "expected robot {} to end at {:?}, got {:?}",
                i, robots[0].position, end_pos
            );
        }
    }

    #[test]
    fn test_move_robots_100() {
        let mut robots = parse_input(EXAMPLE_INPUT);
        for _ in 0..100 {
            move_robots(&mut robots, EXAMPLE_BOUNDARY);
        }

        assert_eq!(gen_map(&robots, EXAMPLE_BOUNDARY), "......2..1.\n...........\n1..........\n.11........\n.....1.....\n...12......\n.1....1....")
    }

    #[test]
    fn test_count_robots() {
        let mut robots = parse_input(EXAMPLE_INPUT);
        for _ in 0..100 {
            move_robots(&mut robots, EXAMPLE_BOUNDARY);
        }

        assert_eq!(count_robots(&robots, EXAMPLE_BOUNDARY), (1, 3, 4, 1));
    }

    #[test]
    fn test_count_input_robots() {
        assert_eq!(
            parse_input(&aocutils::read_input("input").unwrap()).len(),
            500
        );
    }
}
