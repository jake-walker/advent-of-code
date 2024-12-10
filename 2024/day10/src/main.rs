use petgraph::{algo::dijkstra, prelude::DiGraphMap};

type Coords = (usize, usize, isize);

fn parse_input(input: &str) -> DiGraphMap<Coords, ()> {
    let mut g = DiGraphMap::new();

    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => -1,
                    _ => char.to_string().parse().unwrap(),
                })
                .collect()
        })
        .collect::<Vec<Vec<isize>>>();

    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == -1 {
                continue;
            }

            // search up for height decrease
            if y > 0 {
                let up_height = map[y - 1][x];
                let diff = up_height - height;
                if diff == 1 {
                    g.add_edge((x, y, *height), (x, y - 1, up_height), ());
                } else if diff == -1 {
                    g.add_edge((x, y - 1, up_height), (x, y, *height), ());
                }
            }

            // search left for height decrease
            if x > 0 {
                let left_height = map[y][x - 1];
                let diff = left_height - height;
                if diff == 1 {
                    g.add_edge((x, y, *height), (x - 1, y, left_height), ());
                } else if diff == -1 {
                    g.add_edge((x - 1, y, left_height), (x, y, *height), ());
                }
            }
        }
    }

    g
}

fn get_trailhead_scores(g: &DiGraphMap<Coords, ()>) -> Option<usize> {
    let trailhead = g.nodes().filter(|(_, _, z)| *z == 0);
    let peaks = g.nodes().filter(|(_, _, z)| *z == 9);
    let mut score = None;

    for trailhead in trailhead {
        for peak in peaks.clone() {
            let res = dijkstra(g, trailhead, Some(peak), |_| 1);

            if res.keys().any(|k| *k == peak) {
                score = Some(score.unwrap_or(0) + 1);
            }
        }
    }

    score
}

fn main() {
    let input = aocutils::read_input("input").unwrap();
    let g = parse_input(&input);

    println!("part 1: {}", get_trailhead_scores(&g).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "0123\n1234\n8765\n9876";
    const EXAMPLE_2: &str = "...0...\n...1...\n...2...\n6543456\n7.....7\n8.....8\n9.....9";
    const EXAMPLE_3: &str = "..90..9\n...1.98\n...2..7\n6543456\n765.987\n876....\n987....";
    const EXAMPLE_4: &str = "10..9..\n2...8..\n3...7..\n4567654\n...8..3\n...9..2\n.....01";
    const EXAMPLE_5: &str =
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";

    #[test]
    fn test_get_trailhead_scores() {
        let scores = [EXAMPLE_1, EXAMPLE_2, EXAMPLE_3, EXAMPLE_4, EXAMPLE_5]
            .iter()
            .map(|input| get_trailhead_scores(&parse_input(input)))
            .collect::<Vec<_>>();

        assert_eq!(
            scores,
            Vec::from([Some(1), Some(2), Some(4), Some(3), Some(36)])
        );
    }
}
