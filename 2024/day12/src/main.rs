use std::{
    collections::{HashMap, HashSet},
    sync::atomic::AtomicUsize,
};

// not sure whether the plant types are guarenteed to be unique, so this is for uniquely renumbering them
static PLOT_COUNTER: AtomicUsize = AtomicUsize::new(1);

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

fn find_all_plots(map: &Vec<Vec<char>>) -> HashMap<usize, HashSet<Coords>> {
    let mut plots = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, plant_type) in row.iter().enumerate() {
            if plots
                .values()
                .any(|v: &HashSet<Coords>| v.contains(&(x, y)))
            {
                continue;
            }

            let mut plot_coords = HashSet::new();
            find_plot((x, y), map, plant_type, &mut plot_coords);

            plots.insert(
                PLOT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                plot_coords,
            );
        }
    }

    plots
}

fn main() {
    let map = parse_input(&aocutils::read_input("input").unwrap());

    let _ = find_all_plots(&map);
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

        println!("{:#?}", plots);

        assert!(false);
    }
}
