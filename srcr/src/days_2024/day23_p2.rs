use std::collections::{HashMap, HashSet};

pub fn solve_day_23_p2(input: &str) -> usize {
    let data: Vec<&str> = input.trim().lines().collect();
    let mut network: HashMap<String, HashSet<String>> = HashMap::new();

    for line in data {
        let mut split = line.split("-");
        let first: String = split.next().unwrap().to_string();
        let second: String = split.next().unwrap().to_string();
        network.entry(first.clone()).or_insert_with(HashSet::new).insert(second.clone());
        network.entry(second).or_insert_with(HashSet::new).insert(first);
    }

    let mut all_cliques: Vec<HashSet<String>> = Vec::new();
    let mut r: HashSet<String> = HashSet::new();
    let mut p: HashSet<String> = network.keys().cloned().collect();
    let mut x: HashSet<String> = HashSet::new();

    bron_kerbosch(&network, &mut r, &mut p, &mut x, &mut all_cliques);

    let largest_clique = all_cliques.iter().max_by_key(|clique| clique.len()).unwrap();

    let mut sorted_clique: Vec<&String> = largest_clique.iter().collect();
    sorted_clique.sort();

    println!("{:?}", sorted_clique);

    0
}

fn bron_kerbosch(
    graph: &HashMap<String, HashSet<String>>,
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    let p_clone = p.clone();
    for node in p_clone.iter() {
        let mut new_r = r.clone();
        new_r.insert(node.clone());

        let mut new_p: HashSet<String> = p
            .intersection(&graph[node])
            .cloned()
            .collect();

        let mut new_x: HashSet<String> = x
            .intersection(&graph[node])
            .cloned()
            .collect();

        bron_kerbosch(graph, &mut new_r, &mut new_p, &mut new_x, cliques);

        p.remove(node);
        x.insert(node.clone());
    }
}