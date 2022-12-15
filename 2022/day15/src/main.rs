use regex::Regex;
use std::collections::HashSet;
use std::fs;

type Coordinate = (i32, i32);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Sensor {
    sensor: Coordinate,
    beacon: Coordinate
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

fn calculate_where_not(sensor: &Sensor, y_target: Option<i32>) -> Vec<Coordinate> {
    let mut coords = Vec::new();

    let manhattan = i32::abs(sensor.sensor.0 - sensor.beacon.0) + i32::abs(sensor.sensor.1 - sensor.beacon.1);
    let mut rel_y_range = (-manhattan)..(manhattan + 1);

    // to speed things up
    if let Some(yt) = y_target {
        let target_rel_y = yt - sensor.sensor.1;

        if !rel_y_range.contains(&target_rel_y) {
            return coords;
        }

        rel_y_range = target_rel_y..(target_rel_y+1);
    }

    for rel_y in rel_y_range {
        for rel_x in (-(manhattan - i32::abs(rel_y)))..(manhattan - i32::abs(rel_y) + 1) {
            coords.push((sensor.sensor.0 + rel_x, sensor.sensor.1 + rel_y));
        }
    }

    coords
}

fn calculate_where_not_row(sensors: &Vec<Sensor>, y_target: i32) -> usize {
    let mut set: HashSet<Coordinate> = HashSet::new();

    for sensor in sensors {
        set.extend(calculate_where_not(sensor, Some(y_target)).iter())
    }

    for sensor in sensors {
        set.remove(&sensor.beacon);
    }

    set.len()
}

// fn print_map(sensors: &Vec<Sensor>, from: &Coordinate, to: &Coordinate) {
//     let where_not = sensors.iter().map(|s| calculate_where_not(s, None)).flatten().collect::<Vec<Coordinate>>();

//     for y in from.1..to.1 {
//         print!("{:>3}>", y);
//         for x in from.0..to.0 {
//             if sensors.iter().any(|s| s.sensor.0 == x && s.sensor.1 == y) {
//                 print!("S");
//             } else if sensors.iter().any(|s| s.beacon.0 == x && s.beacon.1 == y) {
//                 print!("B");
//             } else if where_not.contains(&(x, y)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!()
//     }
// }

fn main() {
    // let example = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
    // let example_sensors = parse_input(&example);

    // print_map(&example_sensors, &(0, 0), &(20, 20));

    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let sensors = parse_input(&input);

    println!("Part 1: {}", calculate_where_not_row(&sensors, 2000000));
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
    fn calculate_where_not_example() {
        assert_eq!(calculate_where_not(&Sensor { sensor: (5, 5), beacon: (6, 7) }, None), vec![
            (5, 2),
            (4, 3), (5, 3), (6, 3),
            (3, 4), (4, 4), (5, 4), (6, 4), (7, 4),
            (2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (7, 5), (8, 5),
            (3, 6), (4, 6), (5, 6), (6, 6), (7, 6),
            (4, 7), (5, 7), (6, 7),
            (5, 8)
        ])
    }

    #[test]
    fn calculate_where_not_with_target_y_example() {
        assert_eq!(calculate_where_not(&Sensor { sensor: (5, 5), beacon: (6, 7) }, Some(4)), vec![
            (3, 4), (4, 4), (5, 4), (6, 4), (7, 4)
        ])
    }

    #[test]
    fn calculate_where_not_row_example1() {
        assert_eq!(calculate_where_not_row(&vec![Sensor { sensor: (8, 7), beacon: (2, 10) }], 10), 12);
    }

    #[test]
    fn calculate_where_not_row_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let sensors = parse_input(&input);

        assert_eq!(calculate_where_not_row(&sensors, 10), 26);
    }
}
