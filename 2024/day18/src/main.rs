use petgraph::{algo::dijkstra, prelude::UnGraphMap};

static GRID_SIZE: usize = 70;

type Coords = (usize, usize);

fn parse_input(input: &str) -> Vec<Coords> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn create_graph(coords: Vec<Coords>, grid_size: usize) -> UnGraphMap<Coords, ()> {
    let mut g = UnGraphMap::new();

    for y in 0..grid_size + 1 {
        for x in 0..grid_size + 1 {
            for (dx, dy) in [(1, 0), (0, 1)] {
                let (x1, y1) = (x + dx, y + dy);

                if x1 > grid_size
                    || y1 > grid_size
                    || coords.contains(&(x, y))
                    || coords.contains(&(x1, y1))
                {
                    continue;
                }

                g.add_edge((x, y), (x1, y1), ());
            }
        }
    }

    g
}

fn find_path(graph: &UnGraphMap<Coords, ()>, grid_size: usize) -> Option<usize> {
    let res = dijkstra(graph, (0, 0), Some((grid_size, grid_size)), |_| 1_usize);
    res.get(&(grid_size, grid_size)).copied()
}

fn print_map(coords: Vec<Coords>, grid_size: usize) {
    for y in 0..grid_size + 1 {
        for x in 0..grid_size + 1 {
            print!("{}", {
                if coords.contains(&(x, y)) {
                    "#"
                } else {
                    "."
                }
            })
        }
        print!("\n")
    }
}

fn find_blocking_coord(coords: Vec<Coords>, grid_size: usize) -> Option<Coords> {
    let mut start = 0;
    let mut end = coords.len();

    while start != end {
        let mid = start + ((end - start) / 2);
        println!("{} - {}: searching at {}", start, end, mid);

        let g = create_graph(coords[0..mid].to_vec(), grid_size);
        let path_len = find_path(&g, grid_size);

        if mid == start || mid == end {
            return Some(coords[mid]);
        }

        if path_len.is_none() {
            end = mid;
        } else {
            start = mid;
        }
    }

    None
}

fn main() {
    let coords = parse_input(&aocutils::read_input("input").unwrap());
    let g = create_graph(coords[0..1024].to_vec(), GRID_SIZE);

    println!("part 1: {}", find_path(&g, GRID_SIZE).unwrap());
    println!(
        "part 2: {:?}",
        find_blocking_coord(coords, GRID_SIZE).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_POSITIONS: &str = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";
    static EXAMPLE_GRID_SIZE: usize = 6;

    #[test]
    fn test_find_path() {
        let coords = parse_input(EXAMPLE_POSITIONS);
        let g = create_graph(coords[0..12].to_vec(), EXAMPLE_GRID_SIZE);
        assert_eq!(find_path(&g, EXAMPLE_GRID_SIZE), Some(22));
    }

    #[test]
    fn test_find_blocking_coord() {
        let coords = parse_input(EXAMPLE_POSITIONS);
        assert_eq!(find_blocking_coord(coords, EXAMPLE_GRID_SIZE), Some((6, 1)));
    }
}
