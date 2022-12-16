use std::collections::HashMap;

use regex::Regex;
use util::read_files::read_file_as_vector;

#[derive(Debug)]
struct Valve {
    _identifier: usize,
    _orig_identifier: String,
    flow_rate: u64,
    edges: Vec<usize>,
}

fn main() {
    let lines = read_file_as_vector("./files/day16.txt").expect("Error reading file.");
    println!("Solution part 1: {}", solve1(&lines));
}

fn solve1(lines: &Vec<String>) -> u64 {
    let valves = create_valves(lines);
    let mut adjacency_mat = create_adjacency_mat_floyd_warshall(&valves);
    let mut flow_rates = create_flow_rates(&valves);
    remove_zero_flow_nodes(&mut adjacency_mat, &mut flow_rates);
    include_valve_opening(&mut adjacency_mat);

    for valve in valves {
        println!("{:?}", valve);
    }

    for adj in &adjacency_mat {
        println!("{:?}", adj);
    }

    println!("FLOW RATES: {:?}", flow_rates);

    find_best_path(&adjacency_mat, &flow_rates)
}

fn create_valves(lines: &Vec<String>) -> HashMap<usize, Valve> {
    let mut valves = HashMap::new();
    let re =
        Regex::new(r"^Valve ([a-zA-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
            .unwrap();

    let mut identifiers = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let caps = re.captures(line).unwrap();
        let identifier = caps.get(1).unwrap().as_str().to_string();
        identifiers.insert(identifier, i);
    }

    for (i, line) in lines.iter().enumerate() {
        let caps = re.captures(line).unwrap();
        let orig_identifier = caps.get(1).unwrap().as_str().to_string();
        let flow_rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let edges = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|el| *identifiers.get(&el.to_string()).unwrap())
            .collect();
        valves.insert(
            i,
            Valve {
                _identifier: i,
                _orig_identifier: orig_identifier,
                flow_rate,
                edges,
            },
        );
    }

    valves
}

fn create_adjacency_mat_floyd_warshall(valves: &HashMap<usize, Valve>) -> Vec<Vec<u64>> {
    let mut adj_mat = vec![vec![u64::MAX / 2; valves.len()]; valves.len()];

    for (vid, valve) in valves {
        adj_mat[*vid][*vid] = 0;
        for edge in &valve.edges {
            adj_mat[*vid][*edge] = 1;
            adj_mat[*edge][*vid] = 1;
        }
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if adj_mat[i][j] > adj_mat[i][k] + adj_mat[k][j] {
                    adj_mat[i][j] = adj_mat[i][k] + adj_mat[k][j];
                }
            }
        }
    }

    adj_mat
}

fn create_flow_rates(valves: &HashMap<usize, Valve>) -> Vec<u64> {
    let mut result = vec![0; valves.len()];

    for (i, valve) in valves {
        result[*i] = valve.flow_rate;
    }

    result
}

fn remove_zero_flow_nodes(adj_mat: &mut Vec<Vec<u64>>, flow_rates: &mut Vec<u64>) {
    let nonzeroes = flow_rates
        .iter()
        .enumerate()
        .map(|(i, el)| if *el != 0 || i == 0 { i as i64 } else { -1 })
        .filter(|el| *el != -1)
        .map(|el| el as usize)
        .collect::<Vec<usize>>();

    *flow_rates = flow_rates
        .iter()
        .enumerate()
        .filter(|(i, _)| nonzeroes.contains(i))
        .map(|(_, &el)| el)
        .collect::<Vec<u64>>();

    for row in adj_mat.iter_mut() {
        *row = row
            .iter()
            .enumerate()
            .filter(|(i, _)| nonzeroes.contains(i))
            .map(|(_, &el)| el)
            .collect::<Vec<u64>>();
    }
    *adj_mat = adj_mat
        .iter()
        .enumerate()
        .filter(|(i, _)| nonzeroes.contains(i))
        .map(|(_, el)| el.clone())
        .collect::<Vec<Vec<u64>>>();
}

fn include_valve_opening(adj_mat: &mut Vec<Vec<u64>>) {
    for i in 0..adj_mat.len() {
        for j in 0..adj_mat[i].len() {
            if i == j {
                continue;
            }
            adj_mat[i][j] += 1;
        }
    }
}

fn find_best_path(adj_mat: &Vec<Vec<u64>>, flow_rates: &Vec<u64>) -> u64 {
    let mut visited = vec![false; flow_rates.len()];
    visited[0] = true;
    let start = (0, 0, 30, visited);

    _find_best_path_helper(start, adj_mat, flow_rates) as u64
}

fn _find_best_path_helper(
    curr_item: (usize, i64, i64, Vec<bool>),
    adj_mat: &Vec<Vec<u64>>,
    flow_rates: &Vec<u64>,
) -> i64 {
    let (curr_valve, curr_flow, remaining_time, visited) = curr_item;

    let mut curr_max = curr_flow;
    for valve in 0..visited.len() {
        if visited[valve] {
            continue;
        }
        let mut new_visited = visited.clone();
        new_visited[valve] = true;
        let time = remaining_time - adj_mat[curr_valve][valve] as i64;
        if time <= 0 {
            continue;
        }
        let flow = curr_flow + time * flow_rates[valve] as i64;
        curr_max = std::cmp::max(
            curr_max,
            _find_best_path_helper((valve, flow, time, new_visited), adj_mat, flow_rates),
        );
    }

    curr_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(&lines), 1651);
    }
}
