use std::collections::HashMap;

pub struct Network {
    // Mapping of the id to the original name.
    id_to_name: Vec<String>,
    // connected[a][b] is true if a and b are connected
    connected: Vec<Vec<bool>>,
    // neighbours[a] = [b, c, d]
    neighbours: Vec<Vec<usize>>,
    // is_t_computer[a] is true if name starts with 't'
    is_t_computer: Vec<bool>,
}

pub fn parse_day23(lines: &[String]) -> Network {
    let mut name_to_id = HashMap::new();
    let mut id_to_name = Vec::new();
    let mut is_t_computer = Vec::new();
    let mut connections = Vec::new();

    // Single pass to identify nodes and collect edges
    for line in lines {
        let (a_str, b_str) = line.split_once("-").unwrap();

        // Helper closure to get/create IDs
        let mut get_id = |name: &str| -> usize {
            if let Some(&id) = name_to_id.get(name) {
                id
            } else {
                let id = name_to_id.len();
                id_to_name.push(name.to_string());
                name_to_id.insert(name.to_string(), id);
                is_t_computer.push(name.starts_with('t'));
                id
            }
        };

        let a_idx = get_id(a_str);
        let b_idx = get_id(b_str);
        connections.push((a_idx, b_idx));
    }

    let n = name_to_id.len();
    let mut connected = vec![vec![false; n]; n];
    let mut neighbours = vec![vec![]; n];

    for ((u, v)) in connections {
        connected[u][v] = true;
        connected[v][u] = true;
        neighbours[u].push(v);
        neighbours[v].push(u);
    }

    Network {
        id_to_name,
        connected,
        neighbours,
        is_t_computer,
    }
}

pub fn result_day23_stage1(lan: &Network) -> usize {
    let mut triangles = Vec::new();
    for (a, a_neighbours) in lan.neighbours.iter().enumerate() {
        for &b in a_neighbours.iter() {
            if b <= a {
                continue;
            }
            for &c in lan.neighbours[b].iter() {
                if c <= b {
                    continue;
                }
                if lan.neighbours[a].contains(&c) {
                    triangles.push((a, b, c));
                }
            }
        }
    }

    triangles
        .iter()
        .filter(|(a, b, c)| lan.is_t_computer[*a] || lan.is_t_computer[*b] || lan.is_t_computer[*c])
        .count()
}

pub fn result_day23_stage2(lan: &Network) -> String {
    let mut max_clique: Vec<usize> = Vec::new();
    for cur in 0..lan.neighbours.len() {
        let current_clique = vec![cur];
        let candidates = lan.neighbours[cur].clone();
        find_clique(current_clique, candidates, lan, &mut max_clique);
    }

    let mut names = max_clique
        .iter()
        .map(|&idx| lan.id_to_name[idx].clone())
        .collect::<Vec<_>>();
    names.sort_unstable();

    names.join(",")
}

fn find_clique(
    current_clique: Vec<usize>,
    candidates: Vec<usize>,
    network: &Network,
    max_clique: &mut Vec<usize>,
) {
    if candidates.is_empty() {
        if current_clique.len() > max_clique.len() {
            *max_clique = current_clique;
        }
        return;
    }

    // Optimization: if it's impossible to beat the current max, stop
    if current_clique.len() + candidates.len() <= max_clique.len() {
        return;
    }

    let mut remaining_candidates = candidates.clone();
    for &node in &candidates {
        // Create a new clique with this node
        let mut next_clique = current_clique.clone();
        next_clique.push(node);

        // New candidates must be neighnours of the node we just added
        let next_candidates: Vec<usize> = remaining_candidates
            .iter()
            .filter(|&&c| c != node && network.connected[node][c])
            .copied()
            .collect();

        find_clique(next_clique, next_candidates, network, max_clique);

        // Remove node from candidates so we don't process the same clique twice
        remaining_candidates.retain(|&c| c != node);
    }
}

#[cfg(test)]
mod day23 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("kh-tc"),
            String::from("qp-kh"),
            String::from("de-cg"),
            String::from("ka-co"),
            String::from("yn-aq"),
            String::from("qp-ub"),
            String::from("cg-tb"),
            String::from("vc-aq"),
            String::from("tb-ka"),
            String::from("wh-tc"),
            String::from("yn-cg"),
            String::from("kh-ub"),
            String::from("ta-co"),
            String::from("de-co"),
            String::from("tc-td"),
            String::from("tb-wq"),
            String::from("wh-td"),
            String::from("ta-ka"),
            String::from("td-qp"),
            String::from("aq-cg"),
            String::from("wq-ub"),
            String::from("ub-vc"),
            String::from("de-ta"),
            String::from("wq-aq"),
            String::from("wq-vc"),
            String::from("wh-yn"),
            String::from("ka-de"),
            String::from("kh-ta"),
            String::from("co-tc"),
            String::from("wh-qp"),
            String::from("tb-vc"),
            String::from("td-yn"),
        ]
    }

    #[test]
    fn day23_parse() {
        let example = get_example();
        let lan = parse_day23(&example);
        assert_eq!(lan.id_to_name.len(), 16);
        let connected_qty: u32 = lan
            .connected
            .iter()
            .flatten()
            .map(|&v| if v { 1 } else { 0 })
            .sum();
        assert_eq!(connected_qty, 64);
        let neighbours_qty: usize = lan.neighbours.iter().map(|n| n.len()).sum();
        assert_eq!(neighbours_qty, 64);
    }

    #[test]
    fn day23_stage1() {
        let example = get_example();
        let lan = parse_day23(&example);
        let result = result_day23_stage1(&lan);
        assert_eq!(result, 7);
    }

    #[test]
    fn day23_stage2() {
        let lan = parse_day23(&get_example());
        let result = result_day23_stage2(&lan);
        assert_eq!(result, String::from("co,de,ka,ta"));
    }
}
