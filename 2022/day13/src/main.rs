use serde_json::{Value, json};
use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;

fn parse_data(input: &str) -> Vec<(Value, Value)> {
    input.split("\n\n").map(|group| {
        group.lines().map(|line| {
            serde_json::from_str(line).unwrap()
        }).collect_tuple().unwrap()
    }).collect::<Vec<_>>()
}

fn parse_data_flat(input: &str) -> Vec<Value> {
    input.split("\n").flat_map(|l| {
        if l.is_empty() {
            None
        } else {
            serde_json::from_str(l).unwrap()
        }
    }).collect::<Vec<_>>()
}

fn compare_packets(pair: &(Value, Value)) -> Ordering {
    // convert both values in the pair into iters
    let mut pair = (pair.0.as_array().unwrap().iter(), pair.1.as_array().unwrap().iter());

    loop {
        match (pair.0.next(), pair.1.next()) {
            // if we have a value on both sides
            (Some(a), Some(b)) => {
                match (a, b) {
                    // if both values are numbers
                    (Value::Number(ai), Value::Number(bi)) => {
                        let ab_cmp = ai.as_i64().unwrap().cmp(&bi.as_i64().unwrap());
                        if ab_cmp != Ordering::Equal {
                            return ab_cmp;
                        }
                    },
                    // if both values are arrays, feed them back into this function recursively
                    (Value::Array(ai), Value::Array(bi)) => {
                        let ab_cmp = compare_packets(&(Value::Array(ai.clone()), Value::Array(bi.clone())));
                        if ab_cmp != Ordering::Equal {
                            return ab_cmp;
                        }
                    },
                    // if one value is an array and other is number, convert the number to an array, then feed them back into this function recursively
                    (Value::Array(ai), Value::Number(bi)) => {
                        let ab_cmp = compare_packets(&(Value::Array(ai.clone()), Value::Array(vec![Value::Number(bi.clone())])));
                        if ab_cmp != Ordering::Equal {
                            return ab_cmp;
                        }
                    },
                    (Value::Number(ai), Value::Array(bi)) => {
                        let ab_cmp = compare_packets(&(Value::Array(vec![Value::Number(ai.clone())]), Value::Array(bi.clone())));
                        if ab_cmp != Ordering::Equal {
                            return ab_cmp;
                        }
                    },
                    // something has gone wrong if we aren't getting an array or number
                    (_, _) => panic!("expected array or number")
                }
            },
            // if we run out of values on the left side, then the left side is less than the right side
            (None, Some(_)) => {
                return Ordering::Less;
            },
            // if we run out of values on the right side, then the left side is greater than the right side
            (Some(_), None) => {
                return Ordering::Greater;
            },
            // if both sides run out of values, then they are equal
            _ => return Ordering::Equal
        }
    }
}

fn get_decoder_keys(packets: &Vec<Value>) -> (usize, usize) {
    let mut packets = packets.clone();

    let (d1, d2) = (json!([[2]]), json!([[6]]));

    // add divider packets
    packets.push(d1.clone());
    packets.push(d2.clone());

    // sort packets
    packets.sort_by(|a, b| compare_packets(&(a.clone(), b.clone())));

    let d1_pos = packets.iter().position(|x| x == &d1).unwrap();
    let d2_pos = packets.iter().position(|x| x == &d2).unwrap();

    (d1_pos + 1, d2_pos + 1)
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let parsed = parse_data(&input);

    let part1 = parsed.iter()
        .enumerate() // number each item
        .filter(|(_, p)| compare_packets(p) != Ordering::Greater) // filter out items that are
        .map(|(i, _)| i+1)
        .sum::<usize>();

    println!("Part 1: {}", part1);

    let part2 = get_decoder_keys(&parse_data_flat(&input));
    println!("Part 2: {}", part2.0 * part2.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn compare_packets_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let parsed = parse_data(&input);

        assert_eq!(
            parsed.iter().map(|p| compare_packets(p)).collect::<Vec<_>>(),
            vec![Ordering::Less, Ordering::Less, Ordering::Greater, Ordering::Less, Ordering::Greater, Ordering::Less, Ordering::Greater, Ordering::Greater]);
    }

    #[test]
    fn get_decoder_keys_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let parsed = parse_data_flat(&input);

        assert_eq!(get_decoder_keys(&parsed), (10, 14));
    }
}
