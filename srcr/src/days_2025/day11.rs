use std::collections::HashMap;

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(":").collect();
        let device_name = parts[0].trim().to_string();
        let connections: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        graph.insert(device_name, connections);
    }
    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
) -> usize {
    if current == target {
        return 1;
    }

    // Get the neighbors of the current node
    if let Some(neighbors) = graph.get(current) {
        // Sum up all paths from each neighbor to the target
        neighbors
            .iter()
            .map(|next| count_paths(graph, next, target))
            .sum()
    } else {
        // Dead end: no outgoing connections
        0
    }
}

fn count_the_paths(
    graph: &HashMap<String, Vec<String>>,
    from: &str,
    to: &str,
    avoid: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    // We want to avoid this one
    if from == avoid {
        return 0;
    }

    // we've reached our goal
    if from == to {
        return 1;
    }

    // Do we already have a memoized count for this node?
    if let Some(&count) = memo.get(from) {
        return count;
    }

    // We need to start counting paths from here
    let mut path_count = 0;
    if let Some(outputs) = graph.get(from) {
        for output in outputs {
            path_count += count_the_paths(graph, output, to, avoid, memo);
        }
    }

    // Memoize the count for this node
    memo.insert(from.to_string(), path_count);
    path_count
}

pub fn solve_day_11(input: &str) -> u64 {
    let graph = parse_graph(input);
    let nb_path = count_paths(&graph, "you", "out") as u64;

    let part_2 = _part_2(input);

    println!("Résultat 1 : {}", nb_path);
    println!("Résultat 2 : {}", part_2);

    nb_path
}

fn _part_2(input: &str) -> u64 {
    let graph = parse_graph(input);

    // We want to get from svr to out, but via the special devices fft and dac (in any order)
    // So we count for svr->fft->dac->out and svr->dac->fft->out and add them together
    // When counting each of those, we need to avoid going through the other of the special devices

    // svr -> fft -> dac -> out
    let svr_to_fft = count_the_paths(&graph, "svr", "fft", "dac", &mut HashMap::new());
    let fft_to_dac = count_the_paths(&graph, "fft", "dac", " ", &mut HashMap::new());
    let dac_to_out = count_the_paths(&graph, "dac", "out", "fft", &mut HashMap::new());
    let count_svr_fft_dac_out = svr_to_fft * fft_to_dac * dac_to_out;

    // svr -> dac -> fft -> out
    let svr_to_dac = count_the_paths(&graph, "svr", "dac", "fft", &mut HashMap::new());
    let dac_to_fft = count_the_paths(&graph, "dac", "fft", " ", &mut HashMap::new());
    let fft_to_out = count_the_paths(&graph, "fft", "out", "dac", &mut HashMap::new());
    let count_svr_dac_fft_out = svr_to_dac * dac_to_fft * fft_to_out;

    count_svr_fft_dac_out + count_svr_dac_fft_out
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1_exemple() {
        let input = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;
        assert_eq!(solve_day_11(input), 5);
    }

    #[test]
    fn test_part2_exemple() {
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

        assert_eq!(_part_2(input), 2);
    }
}