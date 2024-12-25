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

    for (_char, antennas) in char_map.iter() {
        if antennas.len() > 1 {
            anti_nodes.extend(antennas.iter().map(|&(y, x)| (y as isize, x as isize)));
        }

        for (&(y1, x1), &(y2, x2)) in antennas.iter().tuple_combinations() {
            let dx = (x2 as isize) - (x1 as isize);
            let dy = (y2 as isize) - (y1 as isize);

            let mut factor = 1;
            loop {
                let scaled_dx = dx * factor;
                let scaled_dy = dy * factor;

                let n1 = ((y1 as isize) - scaled_dy, (x1 as isize) - scaled_dx);
                let n2 = ((y2 as isize) + scaled_dy, (x2 as isize) + scaled_dx);

                let is_in_grid = |(y, x): (isize, isize)| {
                    y >= 0 && y < grid.len() as isize && x >= 0 && x < grid[0].len() as isize
                };

                if !is_in_grid(n1) && !is_in_grid(n2) { break;}
                if is_in_grid(n1) { anti_nodes.insert(n1); }
                if is_in_grid(n2) { anti_nodes.insert(n2); }
                factor += 1;
            }
        }
    }
    println!("Nombre de anti-nodes : {}", anti_nodes.len());
}