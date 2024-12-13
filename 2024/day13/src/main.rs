use std::cmp;

const BUTTON_A_COST: i32 = 3;
const BUTTON_B_COST: i32 = 1;

fn parse_input(input: &str) -> Vec<(i32, i32, i32, i32, i32, i32)> {
    let mut outputs = Vec::new();

    for prize in input.split("\n\n") {
        let mut output: Vec<i32> = Vec::new();

        for line in prize.lines() {
            let (_, values) = line.split_once(": ").unwrap();
            let (value_x, value_y) = values.split_once(", ").unwrap();
            output.push(value_x[2..].parse().unwrap());
            output.push(value_y[2..].parse().unwrap());
        }

        assert_eq!(output.len(), 6);

        outputs.push((
            output[0], output[1], output[2], output[3], output[4], output[5],
        ));
    }

    outputs
}

fn brute_force_button_press(
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    goal_x: i32,
    goal_y: i32,
) -> Option<(i32, i32, i32)> {
    // get the upper bounds for the number of a presses and b presses
    // this will overshoot by quite a bit, beacuse we could have the maximum number of a presses
    // and the maximum number of b presses
    let a_max = cmp::max(goal_x / ax, goal_y / ay);
    let b_max = cmp::max(goal_x / bx, goal_y / by);

    // what is the currently cheapest way we've found
    let mut current_best: Option<(i32, i32, i32)> = None;

    // loop over possible button press counts
    for a_presses in 0..a_max {
        for b_presses in 0..b_max {
            // what position does this number presses put us at
            let x = (ax * a_presses) + (bx * b_presses);
            let y = (ay * a_presses) + (by * b_presses);
            // what is the cost for this number of presses
            let cost = (a_presses * BUTTON_A_COST) + (b_presses * BUTTON_B_COST);

            // if we're not at the goal, or the current best is cheaper, then skip
            if x != goal_x || y != goal_y || current_best.is_some_and(|c| c.0 > cost) {
                continue;
            }

            // update the current best
            current_best = Some((cost, a_presses, b_presses));
        }
    }

    current_best
}

fn main() {
    let input = parse_input(&aocutils::read_input("input").unwrap());

    let min_button_presses = input.into_iter().map(|(ax, ay, bx, by, goal_x, goal_y)| {
        brute_force_button_press(ax, ay, bx, by, goal_x, goal_y)
    });

    let lowest_cost: i32 = min_button_presses
        .map(|result| result.unwrap_or((0, 0, 0)).0)
        .sum();

    println!("part 1: {}", lowest_cost);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n\nButton A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176\n\nButton A: X+17, Y+86\nButton B: X+84, Y+37\nPrize: X=7870, Y=6450\n\nButton A: X+69, Y+23\nButton B: X+27, Y+71\nPrize: X=18641, Y=10279";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            Vec::from([
                (94, 34, 22, 67, 8400, 5400),
                (26, 66, 67, 21, 12748, 12176),
                (17, 86, 84, 37, 7870, 6450),
                (69, 23, 27, 71, 18641, 10279),
            ])
        );
    }

    #[test]
    fn test_examples() {
        assert_eq!(
            brute_force_button_press(94, 34, 22, 67, 8400, 5400),
            Some((280, 80, 40))
        );
        assert_eq!(brute_force_button_press(26, 66, 67, 21, 12748, 12176), None);
        assert_eq!(
            brute_force_button_press(17, 86, 84, 37, 7870, 6450),
            Some((200, 38, 86))
        );
        assert_eq!(brute_force_button_press(69, 23, 27, 71, 18641, 10279), None);
    }
}
