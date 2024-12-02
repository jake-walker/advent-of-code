use std::ops::Sub;

fn convert_report(report: Vec<String>) -> Vec<i64> {
    report.iter().map(|v| v.parse().unwrap()).collect()
}

fn check_report(report: &Vec<i64>, min: i64, max: i64) -> bool {
    let diff: Vec<i64> = report
        .windows(2)
        .into_iter()
        .map(|c| c[0].sub(c[1]))
        .collect();

    let ascending = diff.first().unwrap().is_positive();

    diff.iter()
        // check this difference is increasing/decreasing like the first difference and the
        // difference is at least the min and at most the max
        .all(|n| n.is_positive() == ascending && n.abs() >= min && n.abs() <= max)
}

fn main() {
    let reports: Vec<Vec<i64>> = aocutils::read_input_lines_whitespace("input")
        .unwrap()
        .into_iter()
        .map(|line| convert_report(line))
        .collect();

    println!(
        "part 1: {}",
        reports
            .iter()
            .filter(|report| check_report(report, 1, 3))
            .count()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_check_report() {
        let expected: Vec<bool> = Vec::from([true, false, false, false, false, true]);
        let actual = aocutils::read_input_lines_whitespace("example")
            .unwrap()
            .into_iter()
            .map(|line| check_report(&convert_report(line), 1, 3))
            .collect::<Vec<bool>>();

        assert_eq!(expected, actual);
    }
}
