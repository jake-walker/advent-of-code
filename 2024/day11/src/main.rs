fn replace_stone(value: i64) -> Vec<i64> {
    if value == 0 {
        return vec![1];
    }

    let s = value.to_string();
    let digits = s.len();
    if digits % 2 == 0 {
        let (a, b) = s.split_at(digits / 2);
        return vec![a.parse().unwrap(), b.parse().unwrap()];
    }

    return vec![value * 2024];
}

fn replace_all(values: Vec<i64>, n: usize) -> Vec<i64> {
    let mut values = values;
    for _ in 0..n {
        values = values.into_iter().flat_map(|v| replace_stone(v)).collect();
    }
    values
}

fn main() {
    let input: Vec<i64> = aocutils::read_input("input")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    println!("part 1: {}", replace_all(input.clone(), 25).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_stone() {
        assert_eq!(replace_stone(0), vec![1]);
        assert_eq!(replace_stone(1000), vec![10, 0]);
        assert_eq!(replace_stone(5), vec![10120]);
    }

    #[test]
    fn test_replace_all() {
        assert_eq!(
            replace_all(vec![0, 1, 10, 99, 999], 1),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );

        assert_eq!(replace_all(vec![125, 17], 1), vec![253000, 1, 7]);
        assert_eq!(replace_all(vec![125, 17], 2), vec![253, 0, 2024, 14168]);
        assert_eq!(
            replace_all(vec![125, 17], 3),
            vec![512072, 1, 20, 24, 28676032]
        );
        assert_eq!(
            replace_all(vec![125, 17], 4),
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
        );
        assert_eq!(
            replace_all(vec![125, 17], 5),
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        assert_eq!(
            replace_all(vec![125, 17], 6),
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
        assert_eq!(replace_all(vec![125, 17], 25).len(), 55312);
    }
}
