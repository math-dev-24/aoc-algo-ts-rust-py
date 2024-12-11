use std::collections::HashSet;

pub fn solve_day_10(input: &str) -> u128 {
    let grid: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut start_trails: Vec<(usize, usize)> = Vec::new();
    let mut result: u128 = 0;

    for (row, line) in grid.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            if c == 0 {
                start_trails.push((row, col));
            }
        }
    }

    for &(x, y) in &start_trails {
        result += explore_distinct_trails(&grid, x, y, &mut HashSet::new());
    }

    println!("Nombre de trails : {}", start_trails.len());
    println!("Row : {} - Col : {}", grid.len(), grid[0].len());
    println!("Total de chemins : {}", result);
    result
}

fn explore_distinct_trails(
    grid: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    visited: &mut HashSet<String>,
) -> u128 {
    if grid[x][y] == 9 {
        return 1;
    }

    visited.insert(format!("{},{}", x, y));
    let mut total_paths: u128 = 0;

    let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (dx, dy) in directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0
            && new_x < grid.len() as isize
            && new_y >= 0
            && new_y < grid[0].len() as isize
        {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if grid[new_x][new_y] == grid[x][y] + 1
                && !visited.contains(&format!("{},{}", new_x, new_y))
            {
                total_paths += explore_distinct_trails(grid, new_x, new_y, visited);
            }
        }
    }

    visited.remove(&format!("{},{}", x, y));
    total_paths
}