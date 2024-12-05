fn parse_input(input: String) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    (
        rules
            .split("\n")
            .map(|l| {
                let (a, b) = l.split_once("|").unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect(),
        updates
            .split("\n")
            .map(|l| l.split(",").map(|n| n.parse::<i64>().unwrap()).collect())
            .collect(),
    )
}

fn sort_update(update: Vec<i64>, rules: &Vec<(i64, i64)>, exit_early: bool) -> bool {
    // loop until sorting is completed
    loop {
        // loop over each page number in the update
        for i in 0..update.len() {
            // loop over each rule that contains the number that is being operated on
            for rule in rules
                .iter()
                .filter(|r| r.0 == update[i] || r.1 == update[i])
            {
                // get the indexes in the update list of each of the numbers in the rule
                let idx_a_opt = update.iter().position(|n| n == &rule.0);
                let idx_b_opt = update.iter().position(|n| n == &rule.1);

                // make sure both indexes exist
                if let (Some(idx_a), Some(idx_b)) = (idx_a_opt, idx_b_opt) {
                    // if the first index is larger than the second, then they are in the wrong order
                    if idx_a >= idx_b {
                        // these two would need to be swapped
                        // (i assume the lists need to be sorted for part 2?)
                        if exit_early {
                            return true;
                        }
                    }
                }
            }
        }

        break;
    }

    false
}

fn process_part_1(updates: Vec<Vec<i64>>, rules: Vec<(i64, i64)>) -> i64 {
    updates
        .into_iter()
        .filter(|u| !sort_update(u.clone(), &rules, true))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn main() {
    let input = aocutils::read_input("input").unwrap();
    let (rules, updates) = parse_input(input);

    println!("part 1: {}", process_part_1(updates, rules));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = aocutils::read_input("example").unwrap();
        let (rules, updates) = parse_input(input);
        assert!(rules.contains(&(47, 53)));
        assert!(rules.contains(&(53, 13)));
        assert!(updates.contains(&vec![75, 47, 61, 53, 29]));
        assert!(updates.contains(&vec![97, 13, 75, 29, 47]));
    }

    #[test]
    fn test_process_part_1() {
        let input = aocutils::read_input("example").unwrap();
        let (rules, updates) = parse_input(input);

        assert_eq!(process_part_1(updates, rules), 143);
    }
}
