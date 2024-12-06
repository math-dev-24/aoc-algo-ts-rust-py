use std::collections::HashSet;
use std::time::Instant;
use rayon::prelude::*;


type Direction = char;
const TURN_ORDER: [Direction; 4] = ['^', '>', 'v', '<'];

fn get_init_pos(grid: &Vec<Vec<char>>) -> ([usize; 2], Direction) {
    let rows = grid.len();
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if TURN_ORDER.contains(&grid[r][c]) {
                return ([r, c], grid[r][c]);
            }
        }
    }
    panic!("Guard position or direction not found on the map");
}

fn simulate_guard_patrol(
    grid: &Vec<Vec<char>>,
    init_pos: [usize; 2],
    init_dir: Direction,
    detect_loop: bool,
) -> (bool, HashSet<String>) {
    let directions = [
        ('^', (-1, 0)),
        ('>', (0, 1)),
        ('v', (1, 0)),
        ('<', (0, -1)),
    ]
        .iter()
        .cloned()
        .collect::<std::collections::HashMap<_, _>>();

    let mut guard_pos = init_pos;
    let mut guard_dir = init_dir;
    let mut visited = HashSet::new();

    loop {
        let (r, c) = (guard_pos[0], guard_pos[1]);
        let state = if detect_loop {
            format!("{},{},{}", r, c, guard_dir)
        } else {
            format!("{},{}", r, c)
        };

        if visited.contains(&state) && detect_loop {
            return (true, visited);
        }

        visited.insert(state);

        let (dr, dc) = directions[&guard_dir];
        let (fr, fc) = (r as isize + dr, c as isize + dc);

        if fr < 0
            || fr as usize >= grid.len()
            || fc < 0
            || fc as usize >= grid[0].len()
        {
            return (false, visited);
        }

        let front_cell = grid[fr as usize][fc as usize];

        if front_cell == '#' || front_cell == 'O' {
            let current_dir_idx = TURN_ORDER.iter().position(|&d| d == guard_dir).unwrap();
            guard_dir = TURN_ORDER[(current_dir_idx + 1) % 4];
        } else {
            guard_pos = [fr as usize, fc as usize];
        }
    }
}

pub fn solve_day_6() {
    let data_day_6 = "
grid
    ";

    let start_time: Instant = Instant::now();

    let grid: Vec<Vec<char>> = data_day_6.trim().to_string()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let (guard_init_pos, guard_init_dir) = get_init_pos(&grid);

    let ( _ , visited) =
        simulate_guard_patrol(&grid, guard_init_pos, guard_init_dir, false);

    let time_end_part1 = Instant::now();
    println!(
        "Partie 1 : {} , en : {:.2}s",
        visited.len(),
        (time_end_part1 - start_time).as_secs_f64()
    );

    let total_part3: usize = visited.par_iter()
        .map(|cell| {
            let mut split = cell.split(',');
            let r: usize = split.next().unwrap().parse().unwrap();
            let c: usize = split.next().unwrap().parse().unwrap();

            if grid[r][c] == '.' {
                let mut tmp_grid = grid.clone();
                tmp_grid[r][c] = 'O';
                let (looping, _) = simulate_guard_patrol(&grid, guard_init_pos, guard_init_dir, true);
                if looping { 1 } else { 0 }
            } else {
                0
            }
        })
        .sum();

    let time_end_part2: Instant = Instant::now();
    println!(
        "Partie 2 : {} en : {:.2}s",
        total_part3,
        (time_end_part2 - time_end_part1).as_secs_f64()
    );
}