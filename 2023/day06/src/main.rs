use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Record {
    time: u64,
    distance: u64
}

impl Record {
    fn calculate_distance(&self, hold_time: u64) -> u64 {
        let speed = hold_time;
        let remaining_time = self.time - hold_time;
        
        speed * remaining_time
    }
    
    fn ways_to_win(&self) -> Vec<(u64, u64)> {
        let mut ways_to_win: Vec<(u64, u64)> = Vec::new();
        
        for i in 0..self.time {
            let distance = self.calculate_distance(i);
            if distance > self.distance {
                ways_to_win.push((i, distance));
            }
        }
        
        ways_to_win
    }
}

fn parse_input(s: &str) -> Vec<Record> {
    let times = s.lines().nth(0).unwrap();
    let distances = s.lines().nth(1).unwrap();
    
    times.split_whitespace().zip(distances.split_whitespace()).skip(1)
        .map(|(a, b)| Record { time: a.parse::<u64>().unwrap(), distance: b.parse::<u64>().unwrap() })
        .collect::<Vec<Record>>()
}

fn parse_input_part2(s: &str) -> Record {
    let (_, time) = s.lines().nth(0).unwrap().split_once(":").unwrap();
    let (_, distance) = s.lines().nth(1).unwrap().split_once(":").unwrap();
    
    Record {
        time: time.replace(" ", "").trim().parse::<u64>().unwrap(),
        distance: distance.replace(" ", "").trim().parse::<u64>().unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
   
    let part1: usize = parse_input(&input).iter().map(|r| r.ways_to_win().len()).product();
    println!("Part 1: {}", part1);
    
    let part2 = parse_input_part2(&input).ways_to_win().len();
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    
    #[test]
    fn test_parse_input_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        
        let actual = parse_input(&input);
        let expected = vec![Record { time: 7, distance: 9 }, Record { time: 15, distance: 40 }, Record { time: 30, distance: 200 }];
        
        assert_eq!(actual, expected);
    }

#[test]
    fn test_parse_input_part2_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");

        let actual = parse_input_part2(&input);
        let expected = Record { time: 71530, distance: 940200 };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ways_to_win_example() {
        let record = Record { time: 7, distance: 9 };
        
        let actual = record.ways_to_win();
        let expected = vec![(2, 10), (3, 12), (4, 12), (5, 10)];
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_total_ways_to_win_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let records = parse_input(&input);
        
        let actual = records.iter().map(|r| r.ways_to_win().len()).collect::<Vec<usize>>();
        let expected = vec![4, 8, 9];
        
        assert_eq!(actual, expected);
    }
}