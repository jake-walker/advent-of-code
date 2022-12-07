use std::fs;

fn total_calories(list: &String) -> Vec<i32> {
    // first split by double lines for each elf, for each elf split into lines
    // to get each item and sum them
    let mut calories = list.split("\n\n")
        .map(|items| items.lines()
            .map(|x| x.parse::<i32>().unwrap())
            .sum::<i32>())
        .collect::<Vec<i32>>();
    // sort from lowest to highest
    calories.sort();
    calories
}

fn highest_calories(list: &String) -> i32 {
    let calories = total_calories(list);
    // get the last item in the list to get the highest
    *calories.last().unwrap()
}

fn total_top_calories(list: &String, n: usize) -> i32 {
    let calories = total_calories(list);
    // reverse and take the first n, then sum them to get the top n highest
    calories.iter().rev().take(n).sum()
}

fn main() {
    let input1 = fs::read_to_string("inputs/input.txt")
        .expect("should be able to read input");

    println!("Part 1: {}", highest_calories(&input1));
    println!("Part 2: {}", total_top_calories(&input1, 3));
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn highest_calories_example() {
        let example = fs::read_to_string("inputs/example.txt")
            .expect("should be able to read input");
        assert_eq!(highest_calories(&example), 24000);
    }

    #[test]
    fn total_top_calories_example() {
        let example = fs::read_to_string("inputs/example.txt")
            .expect("should be able to read input");
        assert_eq!(total_top_calories(&example, 3), 45000);
    }
}
