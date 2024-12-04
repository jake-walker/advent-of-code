use std::error::Error;

use aocutils;

type WordSearch = Vec<Vec<char>>;

// only_diagonal will only search in the diagonal directions for part 2
// offset will start searching that amount backwards to begin with. for part 2 this is set to 1 so
// that the two words overlap for the middle letter
fn check_position(m: &WordSearch, x: usize, y: usize, target_word: &str, only_diagonal: bool, offset: usize) -> usize {
    let to_check: Vec<char> = target_word.chars().collect();

    let mut match_count = 0;

    // loop over possible directions (e.g. (xi=1, yi=1) is south east)
    for xi in -1..2 {
        for yi in -1..2 {
            if xi == 0 && yi == 0 {
                continue
            }

            // if we're only checking diagonal, discard any coords that contain a 0
            if only_diagonal && (xi == 0 || yi == 0) {
                continue
            }

//            println!("search direction {},{} for {:?}", xi, yi, to_check);

            let mut is_match = true;

            // loop over each character in the word, skipping the first as it's already been checked
            for (i, char) in to_check.iter().enumerate() {
                // calculate the new search position
                let cur_x: i32 = x as i32 + (xi * (i as i32 - offset as i32));
                let cur_y: i32 = y as i32 + (yi * (i as i32 - offset as i32));

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

fn check_all_part1(m: &WordSearch) -> usize {
    let mut count = 0;

    for y in 0..m.len() {
        for x in 0..m[y].len() {
            count += check_position(m, x, y, "XMAS", false, 0);
        }
    }

    count
}

fn check_all_part2(m: &WordSearch) -> usize {
    // part 2 is slower than i'd like but i wanted to make the solution work for both parts, i
    // reckon a dedicated algorithm could make it faster

    let mut count = 0;

    for y in 0..m.len() {
        for x in 0..m[y].len() {
            let res = check_position(m, x, y, "MAS", true, 1);
            // only count two matches (to make an x)
            if res == 2 {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aocutils::read_input_grid("input")?;

    println!("part 1: {}", check_all_part1(&input));
    println!("part 2: {}", check_all_part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_position_part1_example() {
        let input = aocutils::read_input_grid("example").unwrap();
        assert_eq!(check_position(&input, 3, 4, "XMAS", false, 0), 0);
        assert_eq!(check_position(&input, 3, 4, "SAMX", false, 0), 2);
    }

    #[test]
    fn test_check_all_part1_example() {
        let input = aocutils::read_input_grid("example").unwrap();
        assert_eq!(check_all_part1(&input), 18);
    }

    #[test]
    fn test_check_position_part2_example() {
        let input = aocutils::read_input_grid("example").unwrap();
        assert_eq!(check_position(&input, 2, 1, "MAS", true, 1), 2);
    }

    #[test]
    fn test_check_all_part2_example() {
        let input = aocutils::read_input_grid("example").unwrap();
        assert_eq!(check_all_part2(&input), 9);
    }
}
