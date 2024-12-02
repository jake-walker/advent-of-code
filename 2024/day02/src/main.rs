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

fn remove_levels(report: &Vec<i64>) -> Vec<Vec<i64>> {
    report
        .into_iter()
        .enumerate()
        .map(|(i, _)| {
            let mut new_report = report.clone();
            new_report.remove(i);
            new_report
        })
        .collect()
}

fn check_report_try_remove_level(report: &Vec<i64>, min: i64, max: i64) -> bool {
    // first try checking with the full report
    if check_report(report, min, max) {
        return true;
    }

    // then try removing a single level, are any of the reports valid?
    remove_levels(report)
        .iter()
        .any(|r| check_report(r, min, max))
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
    );

    println!(
        "part 2: {}",
        reports
            .iter()
            .filter(|report| check_report_try_remove_level(report, 1, 3))
            .count()
    );
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

    #[test]
    fn test_remove_levels() {
        assert_eq!(
            remove_levels(&Vec::from([7, 6, 4, 2, 1])),
            Vec::from([
                Vec::from([6, 4, 2, 1]),
                Vec::from([7, 4, 2, 1]),
                Vec::from([7, 6, 2, 1]),
                Vec::from([7, 6, 4, 1]),
                Vec::from([7, 6, 4, 2])
            ])
        )
    }

    #[test]
    fn test_part2_check_report() {
        let expected: Vec<bool> = Vec::from([true, false, false, true, true, true]);
        let actual = aocutils::read_input_lines_whitespace("example")
            .unwrap()
            .into_iter()
            .map(|line| check_report_try_remove_level(&convert_report(line), 1, 3))
            .collect::<Vec<bool>>();

        assert_eq!(expected, actual);
    }
}
