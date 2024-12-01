use std::convert::TryFrom;
use std::error::Error;
use std::ops::Sub;

fn parse_input(input: Vec<Vec<String>>) -> Vec<Vec<i64>> {
    input
        .iter()
        .map(|v| v.iter().map(|x| x.parse::<i64>().unwrap()).collect())
        .collect()
}

// source: https://stackoverflow.com/a/64499219
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn total_pair_distance(v: Vec<Vec<i64>>) -> u64 {
    let mut v_iter = v.into_iter();
    let mut left = v_iter.next().unwrap();
    let mut right = v_iter.next().unwrap();

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum()
}

fn process_input(input: Vec<Vec<String>>) -> u64 {
    total_pair_distance(transpose(parse_input(input)))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "example: {}",
        process_input(aocutils::read_input_lines_whitespace("example")?)
    );
    println!(
        "part1: {}",
        process_input(aocutils::read_input_lines_whitespace("input")?)
    );

    Ok(())
}
