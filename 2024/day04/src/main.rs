use std::error::Error;

use aocutils;

type WordSearch = Vec<Vec<char>>;

const TARGET_WORD: &str = "XMAS";

fn check_position(m: &WordSearch, x: usize, y: usize, reverse: bool) -> usize {
    let to_check: Vec<char> = {
        if !reverse {
            TARGET_WORD.chars().collect()
        } else {
            TARGET_WORD.chars().rev().collect()
        }
    };

    // exit early if the current position isn't part of the word
    if m[y][x] != to_check[0] {
//        println!("early exit, bad start letter");
        return 0
    }

    let mut match_count = 0;

    // loop over possible directions (e.g. (xi=1, yi=1) is south east)
    for xi in -1..2 {
        for yi in -1..2 {
            if xi == 0 && yi == 0 {
                continue
            }

//            println!("search direction {},{} for {:?}", xi, yi, to_check);

            let mut is_match = true;

            // loop over each character in the word, skipping the first as it's already been checked
            for (i, char) in to_check.iter().enumerate().skip(1) {
                let cur_x: i32 = x as i32 + (xi * i as i32);
                let cur_y: i32 = y as i32 + (yi * i as i32);

                // check the bounds of the new position
                if cur_x < 0 || cur_x >= m[0].len() as i32 || cur_y < 0 || cur_y >= m.len() as i32 {
//                    println!("  {},{} out of bounds", cur_x, cur_y);
                    is_match = false;
                    break;
                }

//                println!("  {},{} = {}", cur_x, cur_y, &m[cur_y as usize][cur_x as usize]);

                // check the new position matches the character in the word that is being searched,
                // if not stop searching this one
                if &m[cur_y as usize][cur_x as usize] != char {
//                    println!("    - no match");
                    is_match = false;
                    break;
                }
            }

            // if all the characters matched, increase the counter
            if is_match {
//                println!("    - match");
                match_count += 1;
            }
        }
    }

    match_count
}

fn check_all(m: &WordSearch) -> usize {
    let mut count = 0;

    for y in 0..m.len() {
        for x in 0..m[y].len() {
            count += check_position(m, x, y, false);
        }
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aocutils::read_input_grid("input")?;

    println!("part 1: {}", check_all(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_position_part1_example() {
        let input = aocutils::read_input_grid("example").unwrap();
        assert_eq!(check_position(&input, 3, 4, false), 0);
        assert_eq!(check_position(&input, 3, 4, true), 2);
    }

    #[test]
    fn test_check_all_part1_example() {
        let input = aocutils::read_input_grid("example").unwrap();
        assert_eq!(check_all(&input), 18);
    }
}
