use std::error::Error;

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

fn split_vec<T>(v: Vec<Vec<T>>) -> (Vec<T>, Vec<T>) {
    let mut v_iter = v.into_iter();
    (v_iter.next().unwrap(), v_iter.next().unwrap())
}

fn total_pair_distance(left: Vec<i64>, right: Vec<i64>) -> u64 {
    let mut left = left;
    let mut right = right;

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum()
}

fn product_similarity_score(left: Vec<i64>, right: Vec<i64>) -> i64 {
    left.iter()
        .map(|a| a * (right.iter().filter(|b| *b == a).count() as i64))
        .sum()
}

fn process_part1(input: Vec<Vec<String>>) -> u64 {
    let (left, right) = split_vec(transpose(parse_input(input)));
    total_pair_distance(left, right)
}

fn process_part2(input: Vec<Vec<String>>) -> i64 {
    let (left, right) = split_vec(transpose(parse_input(input)));
    product_similarity_score(left, right)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "example part 1: {}",
        process_part1(aocutils::read_input_lines_whitespace("example")?)
    );
    println!(
        "part 1: {}",
        process_part1(aocutils::read_input_lines_whitespace("input")?)
    );
    println!(
        "example part 2: {}",
        process_part2(aocutils::read_input_lines_whitespace("example")?)
    );
    println!(
        "part 2: {}",
        process_part2(aocutils::read_input_lines_whitespace("input")?)
    );

    Ok(())
}
