// this code is long and not very good, but i just wanted to get it done
// i'd very much like to come back and improve this another time :)

use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::astar;
use std::fs;

struct ConvertedMap {
    graph: Graph<usize, ()>,
    x_len: usize,
    start: Option<usize>,
    end: Option<usize>
}

fn coords_to_index(x: usize, y: usize, x_len: usize) -> usize {
    (x_len * y) + x
}

fn index_to_coords(index: usize, x_len: usize) -> (usize, usize) {
    let y = index / x_len;
    let x = index.rem_euclid(x_len);

    (x, y)
}

fn convert_height(height: &char) -> u8 {
    match height {
        'S' => 'a' as u8,
        'E' => 'z' as u8,
        _ => *height as u8
    }
}

fn convert_map(map: &str) -> ConvertedMap {
    let mut graph = Graph::<usize, ()>::new();
    let lines = map.lines();
    let y_len = map.lines().count();
    let x_len = map.lines().next().unwrap().chars().count();

    for index in 0..(y_len*x_len) {
        graph.add_node(index);
    }

    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            let value = convert_height(&c);

            if c == 'S' {
                start = Some(coords_to_index(x, y, x_len));
            }

            if c == 'E' {
                end = Some(coords_to_index(x, y, x_len));
            }
            // if there is a point on the right
            if x+1 < line.len() {
                let right_value = convert_height(&line.chars().skip(x+1).next().unwrap());

                if right_value - 1 == value || right_value <= value {
                    graph.add_edge(NodeIndex::new(coords_to_index(x, y, x_len)), NodeIndex::new(coords_to_index(x+1, y, x_len)), ());
                }

                if value - 1 == right_value || value <= right_value {
                    graph.add_edge(NodeIndex::new(coords_to_index(x+1, y, x_len)), NodeIndex::new(coords_to_index(x, y, x_len)), ());
                }
            }

            // if there is a point on the bottom
            if y+1 < y_len {
                let bottom_value = convert_height(&map.lines().skip(y+1).next().unwrap().chars().skip(x).next().unwrap());

                if bottom_value - 1 == value || bottom_value <= value {
                    graph.add_edge(NodeIndex::new(coords_to_index(x, y, x_len)), NodeIndex::new(coords_to_index(x, y+1, x_len)), ());
                }

                if value - 1 == bottom_value || value <= bottom_value {
                    graph.add_edge(NodeIndex::new(coords_to_index(x, y+1, x_len)), NodeIndex::new(coords_to_index(x, y, x_len)), ());
                }
            }
        }
    }

    ConvertedMap { graph, x_len, start, end }
}

fn best_path_len(map: &ConvertedMap) -> i32 {
    astar(&map.graph,
        NodeIndex::new(map.start.unwrap()),
        |n| n == NodeIndex::new(map.end.unwrap()),
        |_| 1,
        |_| 0).unwrap().0
}

fn best_scenic_len(raw_map: &str, map: &ConvertedMap) -> i32 {
    let mut least: Option<i32> = None;

    for node in map.graph.raw_nodes().iter().filter(|n| {
        let (x, y) = index_to_coords(n.weight, map.x_len);

        'a' == raw_map.lines().skip(y).next().unwrap().chars().skip(x).next().unwrap()
    }) {
        if let Some((len, _)) = astar(&map.graph,
            NodeIndex::new(node.weight),
            |n| n == NodeIndex::new(map.end.unwrap()),
            |_| 1,
            |_| 0) {
            if least == None || len < least.unwrap() {
                least = Some(len);
            }
        }
    }

    least.unwrap()
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let map = convert_map(&input);

    println!("Part 1: {}", best_path_len(&map));
    println!("Part 2: {}", best_scenic_len(&input, &map));
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;
    use super::*;

    // I made up this small one because it would take ages to manually write all the connections for
    // the actual example
    static SMALL_MAP: &str = "Sbc\nedd\nacb";
    static EXAMPLE_MAP: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    #[test]
    fn convert_map_example_graph() {
        let actual = convert_map(&SMALL_MAP).graph;
        let mut expected = Graph::<usize, ()>::new();

        let n0 = expected.add_node(0);
        let n1 = expected.add_node(1);
        let n2 = expected.add_node(2);
        let n3 = expected.add_node(3);
        let n4 = expected.add_node(4);
        let n5 = expected.add_node(5);
        let n6 = expected.add_node(6);
        let n7 = expected.add_node(7);
        let n8 = expected.add_node(8);

        expected.extend_with_edges(&[
            (n0, n1), (n1, n0),
            (n1, n2), (n2, n1),
            (n2, n5), (n5, n2),
            (n3, n4), (n4, n3), (n3, n0),
            (n4, n5), (n4, n7), (n5, n4), (n7, n4), (n4, n1),
            (n7, n8), (n8, n7),
            (n3, n6), (n7, n6),
            (n5, n8)
        ]);

        assert_eq_unordered!(
            actual.raw_nodes().iter().map(|n| &n.weight).collect::<Vec<_>>(),
            expected.raw_nodes().iter().map(|n| &n.weight).collect::<Vec<_>>(),
            "compare graph nodes");
        assert_eq_unordered!(
            actual.raw_edges().iter().map(move |e| (e.source(), e.target())).collect::<Vec<_>>(),
            expected.raw_edges().iter().map(move |e| (e.source(), e.target())).collect::<Vec<_>>(),
            "compare graph edges");
    }

    #[test]
    fn convert_map_example_start_end() {
        let map = convert_map(&EXAMPLE_MAP);

        assert_eq!(map.start, Some(0));
        assert_eq!(map.end, Some(21));
    }

    #[test]
    fn coords_conversion() {
        let input: (usize, usize) = (3, 1);
        let x_max: usize = 8;

        let index = coords_to_index(input.0, input.1, x_max);
        assert_eq!(index, 11, "testing coordinates to index");

        let coords = index_to_coords(index, x_max);
        assert_eq!(coords, input, "testing index to coordinates");
    }

    #[test]
    fn best_path_example() {
        let map = convert_map(&EXAMPLE_MAP);

        assert_eq!(best_path_len(&map), 31);
    }

    #[test]
    fn best_scenic_example() {
        let map = convert_map(&EXAMPLE_MAP);

        assert_eq!(best_scenic_len(&EXAMPLE_MAP, &map), 29);
    }
}
