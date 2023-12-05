use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64
}

impl Mapping {
    fn destination_range(&self) -> (u64, u64) {
        (self.destination_range_start, self.destination_range_start+self.range_length-1)
    }
    
    fn source_range(&self) -> (u64, u64) {
        (self.source_range_start, self.source_range_start+self.range_length-1)
    }
    
    /// map from the source range to the destination range
    fn map_forward(&self, src: u64) -> Option<u64> {
        let (src_min, src_max) = self.source_range();
        let (dst_min, _)  = self.destination_range();

        if src < src_min || src > src_max {
            return None;
        }

        Some((src - src_min) + dst_min)
    }

    /// map from the destination range to the source range
    fn map_backward(&self, dst: u64) -> Option<u64> {
        let (dst_min, dst_max) = self.destination_range();
        let (src_min, _) = self.source_range();

        if dst < dst_min || dst > dst_max {
            return None;
        }

        Some((dst - dst_min) + src_min)
    }
}

fn is_location_possible(mapping_groups: &Vec<Vec<Mapping>>, seed_ranges: &Vec<(u64, u64)>, location: u64) -> bool {
    let mut current = location;

    for group in mapping_groups.iter().rev() {
        for mapping in group {
            if let Some(source) = mapping.map_backward(current) {
                current = source;
                break;
            }
        }
    }

    // check if final seed is in ranges
    for range in seed_ranges {
        if current >= range.0 && current <= range.1 {
            return true;
        }
    }

    false
}

fn lowest_location(mapping_groups: Vec<Vec<Mapping>>, seed_ranges: &Vec<(u64, u64)>) -> Option<u64> {
    let mut last_mappings: &mut Vec<Mapping> = mapping_groups.last().unwrap().clone();
    last_mappings.sort_by_key(|x| x.destination_range().0);

    for mapping in last_mappings {
        let (min, max) = mapping.destination_range();
        for i in min..max {
            if is_location_possible(&mapping_groups, seed_ranges, i) {
                return Some(i);
            }
        }
    }

    None
}

fn map_through(mapping_groups: &Vec<Vec<Mapping>>, seed: &u64) -> u64 {
    let mut seed = *seed;
    
    for group in mapping_groups {
        for mapping in group {
            if let Some(new_seed) = mapping.map_forward(seed) {
                seed = new_seed;
                break;
            }
        }
    }
    
    seed
}

fn parse_mappings(s: &str) -> (Vec<u64>, Vec<Vec<Mapping>>) {
    let mut seeds: Vec<u64> = Vec::new();
    let mut mapping_groups: Vec<Vec<Mapping>> = Vec::new();

    // parse seeds
    for x in s.lines().next().unwrap().split_once(": ").unwrap().1.split(" ") {
        seeds.push(x.parse::<u64>().unwrap());
    }

    // parse groups    
    for group in s.split("\n\n").skip(1) {
        let mut mapping_group: Vec<Mapping> = Vec::new();

        for line in group.lines().skip(1) {
            if line.trim() == "" {
                continue;
            }

            let mut parts = line.split(" ");
            mapping_group.push(Mapping {
                destination_range_start: parts.next().unwrap().parse::<u64>().unwrap(),
                source_range_start: parts.next().unwrap().parse::<u64>().unwrap(),
                range_length: parts.next().unwrap().parse::<u64>().unwrap()
            })
        }
        
        mapping_groups.push(mapping_group);
    }
    
    (seeds, mapping_groups)
}

fn seeds_to_ranges(seeds: Vec<u64>) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for group in seeds.chunks(2) {
        ranges.push((group[0], group[0] + group[1] - 1));
    }

    ranges
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let (seeds, mapping_groups) = parse_mappings(&input);
    
    let part1 = seeds.iter().map(|s| map_through(&mapping_groups, s)).min().unwrap();
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    
    #[test]
    fn test_parse_mappings() {
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15";
        
        let actual = parse_mappings(input);
        let expected = (vec![79, 14, 55, 13], vec![
            vec![
                Mapping { destination_range_start: 50, source_range_start: 98, range_length: 2 },
                Mapping { destination_range_start: 52, source_range_start: 50, range_length: 48 }
            ],
            vec![
                Mapping { destination_range_start: 0, source_range_start: 15, range_length: 37 },
                Mapping { destination_range_start: 37, source_range_start: 52, range_length: 2 },
                Mapping { destination_range_start: 39, source_range_start: 0, range_length: 15 }
            ]
        ]);
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mapping_map_number() {
        let mapping = Mapping { destination_range_start: 50, source_range_start: 98, range_length: 2 };
        
        assert_eq!(
            vec![97_u64, 98_u64, 99_u64, 100_u64].into_iter().map(|n| mapping.map_forward(n)).collect::<Vec<Option<u64>>>(),
            vec![None, Some(50_u64), Some(51_u64), None]
        );
    }

    #[test]
    fn test_map_through_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let (seeds, mapping_groups) = parse_mappings(&input);
        
        let actual = seeds.iter().map(|s| map_through(&mapping_groups, s)).collect::<Vec<u64>>();
        let expected = vec![82, 43, 86, 35];
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_seed_to_ranges_example() {
        assert_eq!(seeds_to_ranges(vec![79, 14]), vec![(79, 92)]);
    }

    #[test]
    fn test_is_location_possible() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let (seeds, mapping_groups) = parse_mappings(&input);
        let seed_ranges = seeds_to_ranges(seeds);

        assert!(is_location_possible(&mapping_groups, &seed_ranges, 46));
        assert!(!is_location_possible(&mapping_groups, &seed_ranges, 1));
    }

    #[test]
    fn test_lowest_location_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let (seeds, mapping_groups) = parse_mappings(&input);
        let seed_ranges = seeds_to_ranges(seeds);

        assert_eq!(lowest_location(&mapping_groups, &seed_ranges).unwrap(), 46);
    }
}