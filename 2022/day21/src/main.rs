use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Job<'a> {
    Number(i32),
    Add(&'a str, &'a str),
    Subtract(&'a str, &'a str),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str)
}

fn parse_input(input: &str) -> HashMap<&str, Job> {
    let mut map: HashMap<&str, Job> = HashMap::new();

    for line in input.lines() {
        let (name, job) = line.split_once(": ").unwrap();
        let mut job_parts = job.split_whitespace();

        map.insert(name, {
            if job_parts.clone().count() == 1 {
                Job::Number(job_parts.next().unwrap().parse::<i32>().unwrap())
            } else {
                let n1 = job_parts.next().unwrap();
                let op = job_parts.next().unwrap();
                let n2 = job_parts.next().unwrap();

                match op {
                    "+" => Job::Add(n1, n2),
                    "-" => Job::Subtract(n1, n2),
                    "*" => Job::Multiply(n1, n2),
                    "/" => Job::Divide(n1, n2),
                    _ => continue
                }
            }
        });
    }

    map
}

fn get_number(map: &HashMap<&str, Job>, target: &str) -> i64 {
    let x = map.get(target).unwrap();

    match &x {
        Job::Number(n) => (*n).into(),
        Job::Add(a, b) => get_number(map, a) + get_number(map, b),
        Job::Subtract(a, b) => get_number(map, a) - get_number(map, b),
        Job::Multiply(a, b) => get_number(map, a) * get_number(map, b),
        Job::Divide(a, b) => get_number(map, a) / get_number(map, b)
    }
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let map = parse_input(&input);

    println!("Part 1: {}", get_number(&map, "root"));
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn parse_input_example() {
        assert_eq!(parse_input("root: pppw + sjmn\ndbpl: 5\nptdq: humn - dvpt\nsjmn: drzm * dbpl\npppw: cczh / lfqf"), HashMap::from([
            ("root", Job::Add("pppw", "sjmn")),
            ("dbpl", Job::Number(5)),
            ("ptdq", Job::Subtract("humn", "dvpt")),
            ("sjmn", Job::Multiply("drzm", "dbpl")),
            ("pppw", Job::Divide("cczh", "lfqf"))
        ]))
    }

    #[test]
    fn get_number_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let map = parse_input(&input);
        assert_eq!(get_number(&map, "root"), 152);
    }
}
