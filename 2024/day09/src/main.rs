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

fn move_blocks(map: &mut Vec<Option<usize>>) -> () {
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

fn checksum(map: &Vec<Option<usize>>) -> usize {
    map.iter()
        .enumerate()
        .filter(|(_, c)| c.is_some())
        .map(|(pos, id)| pos * id.unwrap())
        .sum()
}

fn main() {
    let input = aocutils::read_input("input").unwrap();

    let mut map = expand(&input);
    move_blocks(&mut map);

    // > 91380424522
    println!("part 1: {}", checksum(&map));
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
    fn test_move_blocks_example_1() {
        let mut map = parse_str_map("0..111....22222");
        move_blocks(&mut map);

        assert_eq!(map, parse_str_map("022111222......"));
    }

    #[test]
    fn test_move_blocks_example_2() {
        let mut map = parse_str_map("00...111...2...333.44.5555.6666.777.888899");
        move_blocks(&mut map);

        assert_eq!(
            map,
            parse_str_map("0099811188827773336446555566..............")
        );
    }

    #[test]
    fn test_checksum_example_1() {
        let map = parse_str_map("0099811188827773336446555566..............");
        assert_eq!(checksum(&map), 1928);
    }
}
