use petgraph::{algo::all_simple_paths, prelude::DiGraphMap};

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

fn get_trailhead_scores_and_ratings(g: &DiGraphMap<Coords, ()>) -> (usize, usize) {
    let trailhead = g.nodes().filter(|(_, _, z)| *z == 0);
    let peaks = g.nodes().filter(|(_, _, z)| *z == 9);

    let mut total_score = 0;
    let mut total_rating = 0;

    for trailhead in trailhead {
        for peak in peaks.clone() {
            let res =
                all_simple_paths::<Vec<_>, _>(g, trailhead, peak, 0, None).collect::<Vec<_>>();

            total_rating += res.len();

            if res.len() > 0 {
                total_score += 1;
            }
        }
    }

    (total_score, total_rating)
}

fn main() {
    let input = aocutils::read_input("input").unwrap();
    let g = parse_input(&input);

    let (total_score, total_rating) = get_trailhead_scores_and_ratings(&g);

    println!("part 1: {}", total_score);
    println!("part 2: {}", total_rating);
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

    const EXAMPLE_6: &str = ".....0.\n..4321.\n..5..2.\n..6543.\n..7..4.\n..8765.\n..9....";
    const EXAMPLE_7: &str = "012345\n123456\n234567\n345678\n4.6789\n56789.";

    #[test]
    fn test_get_trailhead_scores() {
        let scores = [EXAMPLE_1, EXAMPLE_2, EXAMPLE_3, EXAMPLE_4, EXAMPLE_5]
            .iter()
            .map(|input| get_trailhead_scores_and_ratings(&parse_input(input)).0)
            .collect::<Vec<_>>();

        assert_eq!(scores, vec![1, 2, 4, 3, 36]);
    }

    #[test]
    fn test_get_trailhead_ratings() {
        let scores = [EXAMPLE_6, EXAMPLE_3, EXAMPLE_7, EXAMPLE_5]
            .iter()
            .map(|input| get_trailhead_scores_and_ratings(&parse_input(input)).1)
            .collect::<Vec<_>>();

        assert_eq!(scores, vec![3, 13, 227, 81]);
    }
}
