use std::collections::HashSet;

type Coords = (usize, usize);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_plot(
    start: Coords,
    map: &Vec<Vec<char>>,
    plant_type: &char,
    coords: &mut HashSet<Coords>,
) -> () {
    if &map[start.1][start.0] != plant_type {
        return;
    }

    coords.insert(start);

    for (xi, yi) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let x = start.0 as i32 + xi;
        let y = start.1 as i32 + yi;

        if x < 0
            || x >= map[0].len() as i32
            || y < 0
            || y >= map.len() as i32
            || coords.contains(&(x as usize, y as usize))
        {
            continue;
        }

        find_plot((x as usize, y as usize), map, plant_type, coords);
    }
}

fn count_edges(region: &HashSet<Coords>, x_max: i32, y_max: i32) -> usize {
    let mut sides = 0;

    for plot in region {
        for (xi, yi) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = plot.0 as i32 + xi;
            let y = plot.1 as i32 + yi;

            if x < 0
                || x >= x_max as i32
                || y < 0
                || y >= y_max as i32
                || !region.contains(&(x as usize, y as usize))
            {
                sides += 1;
            }
        }
    }

    sides
}

fn find_all_plots(map: &Vec<Vec<char>>) -> Vec<HashSet<Coords>> {
    let mut plots = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, plant_type) in row.iter().enumerate() {
            if plots.iter().any(|v: &HashSet<Coords>| v.contains(&(x, y))) {
                continue;
            }

            let mut plot_coords = HashSet::new();
            find_plot((x, y), map, plant_type, &mut plot_coords);

            plots.push(plot_coords);
        }
    }

    plots
}

fn calculate_price(region: &HashSet<Coords>, x_max: i32, y_max: i32) -> usize {
    let edges = count_edges(region, x_max, y_max);
    edges * region.len()
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    find_all_plots(map)
        .iter()
        .map(|r| calculate_price(r, map[0].len() as i32, map.len() as i32))
        .sum()
}

fn main() {
    let map = parse_input(&aocutils::read_input("input").unwrap());

    println!("part 1: {}", part1(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_1: &str = "AAAA\nBBCD\nBBCC\nEEEC";

    #[test]
    fn test_find_plot() {
        let map = parse_input(EXAMPLE_1);
        let mut plot_coords = HashSet::new();
        find_plot((0, 0), &map, &'A', &mut plot_coords);

        assert_eq!(plot_coords, HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)]))
    }

    #[test]
    fn test_find_all_plots() {
        let map = parse_input(EXAMPLE_1);
        let plots = find_all_plots(&map);

        assert_eq!(plots.len(), 5);
    }

    #[test]
    fn test_count_edges() {
        assert_eq!(
            count_edges(&HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)]), 4, 4),
            10
        );
    }

    #[test]
    fn test_part1() {
        let map = parse_input(EXAMPLE_1);

        assert_eq!(part1(&map), 140);
    }
}
