use std::collections::HashMap;

fn replace_stone(value: i64) -> (Option<i64>, Option<i64>) {
    if value == 0 {
        return (Some(1), None);
    }

    let mut digits = 0;
    let mut num = value;
    while num > 0 {
        num /= 10;
        digits += 1;
    }

    if digits % 2 == 0 {
        let divisor = 10_i64.pow(digits / 2);
        return (Some(value / divisor), Some(value % divisor));
    }

    (Some(value * 2024), None)
}

fn replace_all(values: Vec<i64>, n: usize) -> usize {
    let mut counts: HashMap<i64, usize> = HashMap::new();

    for value in values {
        *counts.entry(value).or_insert(0) += 1;
    }

    for _ in 0..n {
        let new_counts = counts
            .iter()
            .flat_map(|(&value, &count)| {
                let (a, b) = replace_stone(value);
                let mut results = Vec::new();

                if let Some(a_value) = a {
                    results.push((a_value, count));
                }
                if let Some(b_value) = b {
                    results.push((b_value, count));
                }

                results
            })
            .fold(HashMap::new(), |mut acc, (value, count)| {
                *acc.entry(value).or_insert(0) += count;
                acc
            });

        counts = new_counts;
    }

    counts.values().sum()
}

fn main() {
    let input: Vec<i64> = aocutils::read_input("input")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    println!("part 1: {}", replace_all(input.clone(), 25));
    println!("part 2: {}", replace_all(input, 75));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_stone() {
        assert_eq!(replace_stone(0), (Some(1), None));
        assert_eq!(replace_stone(1000), (Some(10), Some(0)));
        assert_eq!(replace_stone(5), (Some(10120), None));
    }

    #[test]
    fn test_replace_all() {
        // assert_eq!(
        //     replace_all(vec![0, 1, 10, 99, 999], 1),
        //     vec![1, 2024, 1, 0, 9, 9, 2021976]
        // );

        // assert_eq!(replace_all(vec![125, 17], 1), vec![253000, 1, 7]);
        // assert_eq!(replace_all(vec![125, 17], 2), vec![253, 0, 2024, 14168]);
        // assert_eq!(
        //     replace_all(vec![125, 17], 3),
        //     vec![512072, 1, 20, 24, 28676032]
        // );
        // assert_eq!(
        //     replace_all(vec![125, 17], 4),
        //     vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
        // );
        // assert_eq!(
        //     replace_all(vec![125, 17], 5),
        //     vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        // );
        // assert_eq!(
        //     replace_all(vec![125, 17], 6),
        //     vec![
        //         2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
        //         0, 3, 2
        //     ]
        // );
        assert_eq!(replace_all(vec![125, 17], 25), 55312);
    }
}
