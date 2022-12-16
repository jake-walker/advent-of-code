use petgraph::graphmap::{UnGraphMap};
use petgraph::algo::astar;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct FullMap<'a> {
    graph: UnGraphMap<&'a str, ()>,
    flow_rates: HashMap<&'a str, i32>
}

fn parse_input(input: &str) -> FullMap {
    let mut graph = UnGraphMap::<&str, ()>::new();
    let mut flow_rates = HashMap::<&str, i32>::new();
    let re = Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]{2,})$").unwrap();

    for line in input.lines() {
        let cap = re.captures(line).unwrap();

        let index = cap.get(1).unwrap().as_str();
        let flow_rate = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let tunnels = cap.get(3).unwrap().as_str().split(", ").collect::<Vec<&str>>();

        flow_rates.insert(index, flow_rate);

        for tunnel in tunnels {
            graph.add_edge(index, tunnel, ());
        }
    }

    FullMap { graph, flow_rates }
}

fn calculate_path<'a>(map: &'a FullMap) -> (Vec<&'a str>, i32) {
    let mut current_pos = "AA";
    let mut open_valves: Vec<&str> = Vec::new();
    let mut pressure_released = 0;
    let mut target: Vec<&str> = vec![];
    let mut to_open = false;

    let max_time = 30;

    for t in 1..max_time+1 {
        // calculate pressure released
        pressure_released += open_valves.iter().map(|v| map.flow_rates.get(v).unwrap_or(&0)).sum::<i32>();

        println!("time={}, pos={}, open={:?}, pres={}, target={:?}", t, current_pos, open_valves, pressure_released, target);

        if to_open {
            println!(" - opening valve {}", current_pos);
            open_valves.push(current_pos);
            to_open = false;
            continue;
        }

        if target.len() == 0 {
            target = map.graph.nodes().filter(|v| !open_valves.contains(v) && *v != current_pos && *map.flow_rates.get(v).unwrap() != 0).filter_map(|v| {
                let (dist, path) = astar(&map.graph, current_pos, |x| x == v, |_| 1, |_| 0).expect("should be able to calculate distance");
                let path = path.iter().skip(1).map(|x| x.to_owned()).rev().collect::<Vec<&str>>();

                let time_left = max_time - t - dist;
                let flow = map.flow_rates.get(v).expect("should have flow rate");
                let score = (time_left * flow) / dist.pow(2);

                println!(" - {} -> dist={}, flow={}, time_left={}, can_be_made={}, score={}", v, dist, flow, time_left, time_left * flow, score);

                Some((v, path, score))
            }).max_by(|(_, _, a), (_, _, b)| a.cmp(b)).and_then(|v| Some(v.1)).unwrap_or_default();

            println!(" - new target: {:?}", target.first());
        }

        if let Some(next) = target.pop() {
            println!(" - moving to {}", next);
            current_pos = next;

            if target.len() == 0 {
                to_open = true;
            }
            continue;
        }
    }

    (open_valves, pressure_released)
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let map = parse_input(&input);

    println!("Part 1: {}", calculate_path(&map).1);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn calculate_path_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        let map = parse_input(&input);

        assert_eq!(calculate_path(&map), (
            vec!["DD", "BB", "JJ", "HH", "EE", "CC"],
            1651
        ));
    }
}
