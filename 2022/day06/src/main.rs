use std::fs;
use std::collections::HashSet;

fn get_marker(buf: &str, n: usize) -> Option<usize> {
    let chars = buf.chars().collect::<Vec<char>>();
    for (i, window) in chars.windows(n).enumerate() {
        let hs: HashSet<&char> = HashSet::from_iter(window.iter());
        if hs.len() >= n {
            return Some(i+n);
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");

    println!("Part 1: {}", get_marker(&input, 4).unwrap());
    println!("Part 2: {}", get_marker(&input, 14).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_start_of_packet_marker_examples() {
        let n = 4;
        assert_eq!(get_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", n), Some(7));
        assert_eq!(get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", n), Some(5));
        assert_eq!(get_marker("nppdvjthqldpwncqszvftbrmjlhg", n), Some(6));
        assert_eq!(get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", n), Some(10));
        assert_eq!(get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", n), Some(11));
    }

    #[test]
    fn get_start_of_message_marker_examples() {
        let n = 14;
        assert_eq!(get_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", n), Some(19));
        assert_eq!(get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", n), Some(23));
        assert_eq!(get_marker("nppdvjthqldpwncqszvftbrmjlhg", n), Some(23));
        assert_eq!(get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", n), Some(29));
        assert_eq!(get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", n), Some(26));
    }
}