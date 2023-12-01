use std::fs;

fn calibration_value(s: &str) -> i32 {
    let digits = s.chars().filter(|c| c.is_digit(10));
    format!("{}{}", digits.clone().next().unwrap(), digits.last().unwrap()).parse::<i32>().expect("should be able to parse digits")
}

fn calibration_values_sum(s: &str) -> i32 {
    s.lines().map(|l| calibration_value(l)).sum()
}

fn calibration_values_sum_replaced_words(s: &str) -> i32 {
    s.lines().map(|l| calibration_value(replace_words(l).as_str())).sum()
}

fn replace_words(s: &str) -> String {
    // the numbers are replaced by putting a digit in the middle so characters aren't 'used up'
    // this is a super janky way of doing this but if it works, it works ¯\_(ツ)_/¯
    // for example, normally "nineight" would become "nin8" but 9 is actually the first digit that we want
    s.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero")
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt")
            .expect("should be able to read input");
    
    println!("Part 1: {}", calibration_values_sum(&input));
    
    println!("Part 2: {}", calibration_values_sum_replaced_words(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn calibration_value_example() {
        let input = "pqr3stu8vwx";
        assert_eq!(calibration_value(input), 38);
    }

    #[test]
    fn calibration_values_sum_example() {
        let example = fs::read_to_string("inputs/example.txt")
            .expect("should be able to read input");
        assert_eq!(calibration_values_sum(&example), 142);
    }

    #[test]
    #[ignore]
    fn replace_words_example() {
        let input = "two1nine";
        assert_eq!(replace_words(input), "219");
    }

    #[test]
    fn calibration_values_sum_replaced_words_example() {
        let example = fs::read_to_string("inputs/example1.txt")
            .expect("should be able to read input");
        assert_eq!(calibration_values_sum_replaced_words(&example), 281);
    }
}
