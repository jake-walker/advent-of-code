use std::{ops::Rem, fs};

fn mix(v: &mut Vec<i32>, n: i32) {
    let current_index = v.iter().position(|&x| x == n).unwrap() as i32;
    let mut new_index = (current_index + n).rem_euclid(v.len() as i32);

    v.remove(current_index as usize);
    v.insert(new_index as usize, n);
}

fn mix_all(input: &Vec<i32>) -> Vec<i32> {
    let mut mixed = input.clone();

    for &n in input {
        mix(&mut mixed, n);
    }

    mixed
}

fn get_grove_coordinates(v: &Vec<i32>) -> [i32; 3] {
    let offset = v.iter().position(|&x| x == 0).unwrap();

    [1000_usize, 2000_usize, 3000_usize].map(|n| {
        let index = (n + offset).rem_euclid(v.len());
        *v.get(index).unwrap()
    })
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

fn main() {
    let raw_input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let input = parse_input(&raw_input);
    let part1 = mix_all(&input);

    println!("Part 1: {}", get_grove_coordinates(&part1).iter().sum::<i32>()); // ! not -11553
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "1\n2\n-3\n3\n-2\n0\n4";

    #[test]
    fn mix_example() {
        let mut input = parse_input(EXAMPLE_INPUT);
        assert_eq!(input, vec![1, 2, -3, 3, -2, 0, 4]);

        mix(&mut input, 1);
        assert_eq!(input, vec![2, 1, -3, 3, -2, 0, 4], "mix element 1");

        mix(&mut input, 2);
        assert_eq!(input, vec![1, -3, 2, 3, -2, 0, 4], "mix element 2");

        mix(&mut input, -3);
        assert_eq!(input, vec![1, 2, 3, -2, -3, 0, 4], "mix element -3");

        mix(&mut input, 3);
        assert_eq!(input, vec![1, 2, -2, -3, 0, 3, 4], "mix element 3");

        mix(&mut input, -2);
        assert_eq!(input, vec![1, 2, -3, 0, 3, 4, -2], "mix element -2");

        mix(&mut input, 0);
        assert_eq!(input, vec![1, 2, -3, 0, 3, 4, -2], "mix element 0");

        mix(&mut input, 4);
        assert_eq!(input, vec![1, 2, -3, 4, 0, 3, -2], "mix element 4");
    }

    #[test]
    fn grove_coordinates_example() {
        let mixed = mix_all(&parse_input(EXAMPLE_INPUT));
        let grove = get_grove_coordinates(&mixed);

        assert_eq!(grove, [4, -3, 2]);
    }
}
