macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FilesystemBlock {
    pub size: usize,

    // the file id will be set to none for free space
    pub file_id: Option<usize>,
}

fn parse_input(input: &str) -> Vec<FilesystemBlock> {
    let mut output = Vec::new();

    for (i, c) in input.chars().enumerate() {
        // odd spaces are free space
        let free_space = i % 2 != 0;
        let id = i / 2;
        // the size of the file is the character used
        let size: usize = c.to_string().parse().unwrap();

        // if the size is 0, don't add it to the list
        if size == 0 {
            continue;
        }

        // add this block of size to the list
        output.push(FilesystemBlock {
            file_id: {
                if free_space {
                    None
                } else {
                    Some(id)
                }
            },
            size,
        });
    }

    output
}

fn move_blocks(map: &mut Vec<FilesystemBlock>, move_whole: bool) -> () {
    // pointer for the current block that we're working on, to begin with we want to start from the right side
    let mut block_idx: i32 = map.len() as i32 - 1;

    // loop while we've not reached the left side of the list
    while block_idx >= 0 {
        // create a copy of the block we're on
        let current_block = map[block_idx as usize].clone();

        // keep going backwards through the map until we reach a block that's got a file in
        if current_block.file_id.is_none() {
            block_idx -= 1;
            continue;
        }

        debug_println!("current block @ {} = {:?}", block_idx, current_block);

        // this is how much we still have left to allocate in free space
        let mut left_to_allocate = current_block.size;

        // index for starting search for free blocks
        let mut free_block_idx: i32 = {
            // if this is part 2, we want to find the next free block that is at least the size we need to allocate
            if move_whole {
                if let Some(idx) = map
                    .iter()
                    .position(|b| b.file_id.is_none() && b.size >= left_to_allocate)
                {
                    idx as i32
                } else {
                    // there's no free space that matches the size, so skip this block
                    debug_println!("  unable to move entirely, skipping");
                    block_idx -= 1;
                    continue;
                }
            } else {
                // for part 1, we don't care about the size of the free block so start searching from the left side
                0
            }
        };

        // if there's no free space before the block we're looking for, it needs to be skipped
        if free_block_idx > block_idx {
            block_idx -= 1;
            continue;
        }

        // set the current block to free space
        map[block_idx as usize].file_id = None;

        // set the free blocks
        // if there's still space to allocate and we've not reached the right side of the list
        while left_to_allocate > 0 && free_block_idx < map.len() as i32 {
            let current_free_block = &map[free_block_idx as usize].clone();

            // keep searching forwards until there's free space
            if current_free_block.file_id.is_some() {
                free_block_idx += 1;
                continue;
            }

            debug_println!(
                "  allocating {} to free @ {} = {:?}",
                left_to_allocate,
                free_block_idx,
                current_free_block
            );

            // can the free block be completely written?
            if current_free_block.size <= left_to_allocate {
                debug_println!("  complete write");
                // set the free space to the correct file id
                map[free_block_idx as usize].file_id = current_block.file_id;
                // subtract the space we just set
                left_to_allocate -= current_free_block.size;
            } else {
                // the rest of the bytes fits in less than the free space, so the free space needs to be split
                let (a_size, b_size) =
                    (left_to_allocate, current_free_block.size - left_to_allocate);

                debug_println!("  split write {},{}", a_size, b_size);

                // remove the old free block
                map.remove(free_block_idx as usize);
                // insert the two new ones, the first has the file id, the second has the rest of the free space
                map.insert(
                    free_block_idx as usize,
                    FilesystemBlock {
                        file_id: current_block.file_id,
                        size: a_size,
                    },
                );
                map.insert(
                    free_block_idx as usize + 1,
                    FilesystemBlock {
                        file_id: None,
                        size: b_size,
                    },
                );

                // with a new block added, the pointer needs to be moved over by 1
                block_idx += 1;
                free_block_idx += 1;

                left_to_allocate -= a_size;
            }
        }

        // it shouldn't get to this point, but throw an error if there's not enough free space to move
        // the blocks
        if left_to_allocate > 0 {
            panic!("failed to fully allocate ({} left)", left_to_allocate);
        }

        // move onto the next block
        block_idx -= 1;
    }
}

fn checksum(map: &Vec<FilesystemBlock>) -> usize {
    let mut sum = 0;
    let mut i = 0;

    for item in map {
        for _ in 0..item.size {
            if let Some(file_id) = item.file_id {
                sum += i * file_id;
            }
            i += 1;
        }
    }

    sum
}

fn main() {
    let input = aocutils::read_input("input").unwrap();

    let mut map1 = parse_input(&input);
    let mut map2 = map1.clone();

    move_blocks(&mut map1, false);
    println!("part 1: {}", checksum(&map1));

    move_blocks(&mut map2, true);
    println!("part 2: {}", checksum(&map2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "12345";
    const EXAMPLE_2: &str = "2333133121414131402";

    fn filesystem_to_string(fs: &Vec<FilesystemBlock>) -> String {
        // this expands the filesystem list into the format shown in the puzzle

        let mut buf = String::new();

        for item in fs {
            for _ in 0..item.size {
                match item.file_id {
                    Some(n) => buf += &format!("{}", n % 10).to_string(),
                    None => buf += ".",
                }
            }
        }

        buf
    }

    #[test]
    fn test_expand_example_1() {
        assert_eq!(
            filesystem_to_string(&parse_input(EXAMPLE_1)),
            "0..111....22222"
        );
    }

    #[test]
    fn test_expand_example_2() {
        assert_eq!(
            filesystem_to_string(&parse_input(EXAMPLE_2)),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_move_blocks_single_example_1() {
        let mut map = parse_input(&EXAMPLE_1);
        move_blocks(&mut map, false);

        assert_eq!(filesystem_to_string(&map), "022111222......");
    }

    #[test]
    fn test_move_blocks_single_example_2() {
        let mut map = parse_input(&EXAMPLE_2);
        move_blocks(&mut map, false);

        assert_eq!(
            filesystem_to_string(&map),
            "0099811188827773336446555566.............."
        );
    }

    #[test]
    fn test_move_blocks_whole_example_2() {
        let mut map = parse_input(&EXAMPLE_2);
        move_blocks(&mut map, true);

        assert_eq!(
            filesystem_to_string(&map),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }

    #[test]
    fn test_checksum_example_1() {
        let mut map = parse_input(&EXAMPLE_2);
        move_blocks(&mut map, false);
        assert_eq!(checksum(&map), 1928);
    }
}
