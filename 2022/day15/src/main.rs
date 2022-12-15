// inspiration was taken from https://github.com/tobyink/advent-of-code/blob/main/2022/15/solution.rs

use regex::Regex;
use std::fs;

type Coordinate = (i32, i32);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Sensor {
    sensor: Coordinate,
    beacon: Coordinate
}

impl Sensor {
    fn sensor_beacon_distance(&self) -> i32 {
        manhattan_dist(&self.sensor, &self.beacon)
    }

    fn sensor_distance(&self, p: &Coordinate) -> i32 {
        manhattan_dist(&self.sensor, p)
    }
}

fn manhattan_dist(a: &Coordinate, b: &Coordinate) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1)
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let mut sensors = Vec::new();
    let re = Regex::new(r"^Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)$").unwrap();

    for line in input.lines() {
        let cap = re.captures(line).unwrap();

        let sensor_x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let sensor_y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let beacon_x = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let beacon_y = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();

        sensors.push(Sensor {
            sensor: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y)
        })
    }

    sensors
}

fn calculate_where_not_row(sensors: &Vec<Sensor>, x_min: i32, x_max: i32, y: i32) -> i32 {
    let mut count = 0;

    for x in x_min..x_max {
        if sensors.iter().any(|s| s.sensor_distance(&(x, y)) <= s.sensor_beacon_distance()) {
            count += 1;
        }

        if sensors.iter().any(|s| s.beacon == (x, y)) {
            count -= 1;
        }
    }

    count
}

fn get_missing_beacon(sensors: &Vec<Sensor>, from: Coordinate, to: Coordinate) -> Option<Coordinate> {
    for sensor in sensors {
        let borders = (0..(sensor.sensor_beacon_distance() + 1)).flat_map(|s| vec![
            (sensor.sensor.0 + s, sensor.sensor.1 + s - sensor.sensor_beacon_distance() + 1),
            (sensor.sensor.0 + sensor.sensor_beacon_distance() + 1 - s, sensor.sensor.1 + s),
            (sensor.sensor.0 - s, sensor.sensor.1 + sensor.sensor_beacon_distance() + 1 - s),
            (sensor.sensor.0 + s - sensor.sensor_beacon_distance() + 1, sensor.sensor.1 - s)
        ]).collect::<Vec<Coordinate>>();

        // println!("S({},{}) -> {:?}", sensor.sensor.0, sensor.sensor.1, borders);

        for p in borders {
            if p.0 < from.0 || p.1 < from.1 || p.0 > to.0 || p.1 > to.1 {
                continue;
            }

            if sensors.iter().any(|s1| s1.sensor_distance(&p) <= s1.sensor_beacon_distance()) {
                continue;
            }

            return Some(p);
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let sensors = parse_input(&input);

    println!("Part 1: {}", calculate_where_not_row(&sensors, -2000000, 6000000, 2000000));

    let missing_beacon = get_missing_beacon(&sensors, (0, 0), (4000000, 4000000)).unwrap();
    let tuning_frequency = ((missing_beacon.0 as i64) * 4000000) + (missing_beacon.1 as i64);
    println!("Part 2: {}", tuning_frequency);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_input_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");

        assert_eq!(parse_input(&input.lines().take(4).collect::<Vec<&str>>().join("\n")), vec![
            Sensor { sensor: (2, 18), beacon: (-2, 15) },
            Sensor { sensor: (9, 16), beacon: (10, 16) },
            Sensor { sensor: (13, 2), beacon: (15, 3) },
            Sensor { sensor: (12, 14), beacon: (10, 16) }
        ])
    }

    #[test]
    fn calculate_where_not_row_example1() {
        assert_eq!(calculate_where_not_row(&vec![Sensor { sensor: (8, 7), beacon: (2, 10) }], -20000, 60000, 10), 12);
    }

    #[test]
    fn calculate_where_not_row_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let sensors = parse_input(&input);

        assert_eq!(calculate_where_not_row(&sensors, -20000, 60000, 10), 26);
    }

    #[test]
    fn get_missing_beacon_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let sensors = parse_input(&input);

        assert_eq!(get_missing_beacon(&sensors, (0, 0), (20, 20)), Some((14, 11)));
    }
}
