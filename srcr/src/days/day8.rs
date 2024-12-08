use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve_day_8(input: &str) {
    let processed_input = input.trim().replace("#", ".");
    let lines: Vec<&str> = processed_input.lines().collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut char_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                char_map.entry(char).or_insert_with(Vec::new).push((row, col));
            }
        }
    }

    let mut anti_nodes: HashSet<(isize, isize)> = HashSet::new();

    for (_char, antennes) in char_map.iter() {
        if antennes.len() > 1 {
            anti_nodes.extend(antennes.iter().map(|&(y, x)| (y as isize, x as isize)));
        }

        for (&(y1, x1), &(y2, x2)) in antennes.iter().tuple_combinations() {
            let dx = (x2 as isize) - (x1 as isize);
            let dy = (y2 as isize) - (y1 as isize);

            let mut factor = 1;
            loop {
                let scaled_dx = dx * factor;
                let scaled_dy = dy * factor;

                let n1 = ((y1 as isize) - scaled_dy, (x1 as isize) - scaled_dx);
                let n2 = ((y2 as isize) + scaled_dy, (x2 as isize) + scaled_dx);

                let in_bounds = |(y, x): (isize, isize)| {
                    y >= 0 && y < grid.len() as isize && x >= 0 && x < grid[0].len() as isize
                };

                if !in_bounds(n1) && !in_bounds(n2) {
                    break;
                }

                if in_bounds(n1) && !antennes.contains(&(n1.0 as usize, n1.1 as usize)) {
                    anti_nodes.insert(n1);
                }

                if in_bounds(n2) && !antennes.contains(&(n2.0 as usize, n2.1 as usize)) {
                    anti_nodes.insert(n2);
                }

                factor += 1;
            }
        }
    }
    println!("Nombre de anti-nodes : {}", anti_nodes.len());
}