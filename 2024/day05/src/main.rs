fn parse_input(input: String) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    // split the input by two new lines to get each section
    let (rules, updates) = input.split_once("\n\n").unwrap();

    (
        // for the rules section, take each line, split it by pipe, then parse each side as a number
        rules
            .split("\n")
            .map(|l| {
                let (a, b) = l.split_once("|").unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect(),
        // for the updates section, take each line, split it by comma, then bring into a list
        updates
            .split("\n")
            .map(|l| l.split(",").map(|n| n.parse::<i64>().unwrap()).collect())
            .collect(),
    )
}

fn sort_update(update: &mut Vec<i64>, rules: &Vec<(i64, i64)>, exit_early: bool) -> bool {
    // has the update been changed at all?
    let mut overall_changed = false;
    // has the update been changed this iteration?
    let mut changed_this_iteration = true;

    // loop until no changes have been made
    while changed_this_iteration {
        changed_this_iteration = false;

        // loop over each page number in the update
        for i in 0..update.len() {
            // create a read-only copy of update for filtering rules
            let update_ro = update.clone();
            // loop over each rule that contains the number that is being operated on
            for rule in rules
                .iter()
                .filter(|r| r.0 == update_ro[i] || r.1 == update_ro[i])
            {
                // get the indexes in the update list of each of the numbers in the rule
                let idx_a_opt = update.iter().position(|n| n == &rule.0);
                let idx_b_opt = update.iter().position(|n| n == &rule.1);

                // make sure both indexes exist
                if let (Some(idx_a), Some(idx_b)) = (idx_a_opt, idx_b_opt) {
                    // if the first index is smaller than the second, then they are in the correct order
                    if idx_a < idx_b {
                        continue;
                    }

                    // for part 1 we don't actually need to sort the lists, so we can exit early
                    if exit_early {
                        return true;
                    }

                    // swap the items over in the list
                    update.swap(idx_a, idx_b);

                    overall_changed = true;
                    changed_this_iteration = true;
                    // now stop processing any more rules for this number, we want to start from the beginning
                    break;
                }
            }

            // process rules from the beginning if something has changed
            if changed_this_iteration {
                break;
            }
        }
    }

    return overall_changed;
}

// for part 1, exit early will not sort any lists, so it gets the part 1 solution faster
fn process(updates: &mut Vec<Vec<i64>>, rules: &Vec<(i64, i64)>, exit_early: bool) -> (i64, i64) {
    // for part 1, the sum of the middle numbers of all the lists that are sorted properly
    let mut unchanged_sum = 0;
    // for part 2, the sum of the middle numbers of all the lists that have been sorted
    let mut changed_sum = 0;

    for i in 0..updates.len() {
        let sorted = sort_update(&mut updates[i], rules, exit_early);
        let mid = updates[i][updates[i].len() / 2];

        if sorted {
            changed_sum += mid;
        } else {
            unchanged_sum += mid;
        }
    }

    (unchanged_sum, changed_sum)
}

fn main() {
    let input = aocutils::read_input("input").unwrap();
    let (rules, mut updates) = parse_input(input);

    let (part1, part2) = process(&mut updates, &rules, false);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
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
        let (rules, mut updates) = parse_input(input);

        assert_eq!(process(&mut updates, &rules, true).0, 143);
    }

    #[test]
    fn test_process_part_2() {
        let input = aocutils::read_input("example").unwrap();
        let (rules, mut updates) = parse_input(input);

        assert_eq!(process(&mut updates, &rules, false).1, 123);
    }
}
