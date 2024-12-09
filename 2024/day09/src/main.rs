use std::io::{self, Write};

fn expand(map: &str) -> Vec<Option<usize>> {
    let mut output = Vec::new();

    for (i, c) in map.chars().enumerate() {
        let free_space = i % 2 != 0;
        let id = i / 2;
        let size: usize = c.to_string().parse().unwrap();

        for _ in 0..size {
            output.push({
                if free_space {
                    None
                } else {
                    Some(id)
                }
            })
        }
    }

    output
}

fn move_blocks_single(map: &mut Vec<Option<usize>>) -> () {
    let mut free_ptr;
    let mut block_ptr;

    loop {
        free_ptr = map.iter().position(|c| c.is_none()).unwrap();
        block_ptr = map.iter().rposition(|c| c.is_some()).unwrap();

        if free_ptr > block_ptr {
            break;
        }

        map.swap(free_ptr, block_ptr);
    }
}

fn move_blocks_whole(map: &mut Vec<Option<usize>>, exclude: &Vec<usize>) -> (bool, Option<usize>) {
    let block_end: usize = {
        match map
            .iter()
            .rposition(|c| c.is_some() && !exclude.contains(&c.unwrap()))
        {
            Some(n) => n,
            None => return (false, None),
        }
    };
    let mut block_start: usize = block_end;

    // search for the start of the current block
    loop {
        if block_start == 0 || map[block_start - 1] != map[block_end] {
            break;
        }

        block_start -= 1;
    }

    let block_size = block_end - block_start;
    let mut free_start: usize = 0;
    let mut free_end: usize;

    // search from the start for a length of free space that matches
    loop {
        // search to the next free space
        while map[free_start].is_some() {
            free_start += 1;
        }

        // if our free point is after the block then we can't move it
        if free_start >= block_start {
            return (false, map[block_end]);
        }

        free_end = free_start;
        while map[free_end + 1].is_none() {
            free_end += 1;
        }

        if (free_end - free_start) >= block_size {
            break;
        }

        free_start = free_end + 1;
    }

    // perform swap
    for i in 0..block_size + 1 {
        map.swap(free_start + i, block_start + i);
    }

    (true, map[block_end])
}

fn move_blocks_whole_all(map: &mut Vec<Option<usize>>) -> () {
    let mut exclusions = Vec::new();

    loop {
        match move_blocks_whole(map, &exclusions) {
            (false, None) => break,
            (_, Some(n)) => exclusions.push(n),
            (_, _) => continue,
        }
    }
}

fn checksum(map: &Vec<Option<usize>>) -> usize {
    map.iter()
        .enumerate()
        .filter(|(_, c)| c.is_some())
        .map(|(pos, id)| pos * id.unwrap())
        .sum()
}

fn main() {
    let input = aocutils::read_input("input").unwrap();

    let mut map1 = expand(&input);
    let mut map2 = map1.clone();

    move_blocks_single(&mut map1);
    println!("part 1: {}", checksum(&map1));

    move_blocks_whole_all(&mut map2);
    println!("part 2: {}", checksum(&map2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "12345";
    const EXAMPLE_2: &str = "2333133121414131402";

    fn parse_str_map(input: &str) -> Vec<Option<usize>> {
        input
            .chars()
            .into_iter()
            .map(|c| {
                if c == '.' {
                    None
                } else {
                    Some(c.to_string().parse::<usize>().unwrap())
                }
            })
            .collect()
    }

    #[test]
    fn test_expand_example_1() {
        assert_eq!(expand(EXAMPLE_1), parse_str_map("0..111....22222"));
    }

    #[test]
    fn test_expand_example_2() {
        assert_eq!(
            expand(EXAMPLE_2),
            parse_str_map("00...111...2...333.44.5555.6666.777.888899")
        );
    }

    #[test]
    fn test_move_blocks_single_example_1() {
        let mut map = parse_str_map("0..111....22222");
        move_blocks_single(&mut map);

        assert_eq!(map, parse_str_map("022111222......"));
    }

    #[test]
    fn test_move_blocks_single_example_2() {
        let mut map = parse_str_map("00...111...2...333.44.5555.6666.777.888899");
        move_blocks_single(&mut map);

        assert_eq!(
            map,
            parse_str_map("0099811188827773336446555566..............")
        );
    }

    #[test]
    fn test_move_blocks_whole_example_2() {
        let mut map = parse_str_map("00...111...2...333.44.5555.6666.777.888899");
        move_blocks_whole_all(&mut map);

        assert_eq!(
            map,
            parse_str_map("00992111777.44.333....5555.6666.....8888..")
        );
    }

    #[test]
    fn test_checksum_example_1() {
        let map = parse_str_map("0099811188827773336446555566..............");
        assert_eq!(checksum(&map), 1928);
    }
}
