use std::fs;

type HeightMap = Vec<Vec<u8>>;

// convert input to 2d array of ints
fn parse_map(input: &str) -> HeightMap {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>()
}

// get the trees in up, down, left, right directions
fn tree_lines(map: &HeightMap, x: usize, y: usize) -> [Vec<u8>; 4] {
    let (left, right) = map[y].split_at(x);
    let col = map.iter().map(|row| row[x]).collect::<Vec<u8>>();
    let (up, down) = col.split_at(y);

    let down_vec = down[1..].to_vec();
    let right_vec = right[1..].to_vec();
    let mut up_vec = up.to_vec();
    let mut left_vec = left.to_vec();

    // reverse the up and left directions as they need to be searched from the outside in
    left_vec.reverse();
    up_vec.reverse();

    [up_vec, down_vec, left_vec, right_vec]
}

// given coordinates of the tree, is it visible from the outside?
fn tree_visible(map: &HeightMap, x: usize, y: usize) -> bool {
    let directions = tree_lines(&map, x, y);
    // height of the given tree
    let target = map[y][x];

    // in any direction see if all trees smaller than the given one
    directions.iter().map(|direction| direction.iter().all(|x| *x < target)).any(|x| x)
}

// count the total number of trees that are visible from the outside
fn count_all_visible(map: &HeightMap) -> i32 {
    let mut count = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if tree_visible(&map, x, y) {
                count += 1;
            }
        }
    }

    count
}

// calculate the scenic score of a given tree
fn scenic_score(map: &HeightMap, x: usize, y: usize) -> i32 {
    let directions = tree_lines(&map, x, y);
    let target = map[y][x];
    let mut product = 1;

    for direction in directions {
        let mut view_distance = 0;

        // get the height of the first tree that blocks view (equal or taller than target)
        for tree in direction.iter() {
            view_distance += 1;
            if *tree >= target {
                break;
            }
        }

        // multiply each directions view distance
        product *= view_distance;
    }

    product
}

// calculate scenic score for every tree and find highest
fn highest_scenic_score(map: &HeightMap) -> Option<i32> {
    let mut highest = None;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let score = scenic_score(&map, x, y);
            if highest == None || score > highest.unwrap() {
                highest = Some(score);
            }
        }
    }

    highest
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let map = parse_map(&input);

    println!("Part 1: {}", count_all_visible(&map));
    println!("Part 2: {}", highest_scenic_score(&map).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn parse_map_example() {
        assert_eq!(parse_map(&EXAMPLE_INPUT), vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ]);
    }

    #[test]
    fn tree_lines_example() {
        assert_eq!(tree_lines(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ], 2, 2), [
            vec![5, 3],
            vec![5, 3],
            vec![5, 6],
            vec![3, 2]
        ]);
    }

    #[test]
    fn tree_visible_example() {
        assert_eq!(tree_visible(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ], 1, 1), true);
    }

    #[test]
    fn count_all_visible_example() {
        assert_eq!(count_all_visible(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ]), 21);
    }

    #[test]
    fn scenic_score_example() {
        assert_eq!(scenic_score(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ], 2, 1), 4);

        assert_eq!(scenic_score(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ], 2, 3), 8);
    }
}