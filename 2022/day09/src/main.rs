use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Eq)]
struct HeadTail {
    head: Coordinate,
    tail: Coordinate
}

fn do_move(i: &mut HeadTail, instruction: &str) -> HashSet<Coordinate> {
    let mut parts = instruction.split_whitespace();
    let dir = parts.next().unwrap();
    let amount = parts.next().unwrap().parse::<usize>().unwrap();

    let mut tail_history: HashSet<Coordinate> = HashSet::from([i.tail.clone()]);
    let mut last_head: Coordinate;

    for _ in 0..amount {
        last_head = i.head.clone();

        match dir {
            "U" => i.head.y += 1,
            "D" => i.head.y -= 1,
            "L" => i.head.x -= 1,
            "R" => i.head.x += 1,
            _ => {}
        }

        if i32::abs(i.head.x - i.tail.x) > 1 || i32::abs(i.head.y - i.tail.y) > 1 {
            // tail is too far away
            i.tail = last_head;
            tail_history.insert(i.tail.clone());
        }
    }

    tail_history
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");

    let mut i = HeadTail {
        head: Coordinate { x: 0, y: 0 },
        tail: Coordinate { x: 0, y: 0 }
    };

    let mut tail_histories: HashSet<Coordinate> = HashSet::new();

    for instruction in input.lines() {
        let tail_history = do_move(&mut i, instruction);
        tail_histories = tail_histories.union(&tail_history).map(|x| x.clone()).collect::<HashSet<_>>();
    }

    println!("Part 1: {}", tail_histories.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_unordered::assert_eq_unordered;

    static EXAMPLE_INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

    #[test]
    fn do_move_example() {
        let mut i = HeadTail {
            head: Coordinate { x: 0, y: 0 },
            tail: Coordinate { x: 0, y: 0 }
        };

        do_move(&mut i, "R 4");

        assert_eq!(i.head, Coordinate { x: 4, y: 0 });
    }

    #[test]
    fn do_multiple_move_example() {
        let mut i = HeadTail {
            head: Coordinate { x: 0, y: 0 },
            tail: Coordinate { x: 0, y: 0 }
        };

        let mut tail_histories: HashSet<Coordinate> = HashSet::new();

        for instruction in EXAMPLE_INPUT.lines() {
            let tail_history = do_move(&mut i, instruction);
            tail_histories = tail_histories.union(&tail_history).map(|x| x.clone()).collect::<HashSet<_>>();
        }

        assert_eq!(i.head, Coordinate { x: 2, y: 2 });
        assert_eq!(i.tail, Coordinate { x: 1, y: 2 });
        assert_eq_unordered!(tail_histories.iter().map(|x| x.clone()).collect::<Vec<_>>(), vec![
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 1, y: 0 },
            Coordinate { x: 2, y: 0 },
            Coordinate { x: 3, y: 0 },
            Coordinate { x: 4, y: 1 },
            Coordinate { x: 1, y: 2 },
            Coordinate { x: 2, y: 2 },
            Coordinate { x: 3, y: 2 },
            Coordinate { x: 4, y: 2 },
            Coordinate { x: 3, y: 3 },
            Coordinate { x: 4, y: 3 },
            Coordinate { x: 2, y: 4 },
            Coordinate { x: 3, y: 4 },
        ]);
    }
}
