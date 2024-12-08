use std::collections::HashSet;

pub fn solve_day_4(input: &str) {
    let mut result_1 = 0;
    let mut result_2 = 0;

    let lines: Vec<&str> = input.trim().lines().collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let word = "XMAS";
    let mut list_x: Vec<(usize, usize)> = Vec::new();
    let mut list_center: Vec<(usize, usize)> = Vec::new();
    let list_dir: [isize; 3] = [-1, 0, 1];

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'X' {
                list_x.push((r, c));
            }
            if grid[r][c] == 'A' {
                list_center.push((r, c));
            }
        }
    }

    for (rx, cx) in list_x {
        for &di in &list_dir {
            for &dj in &list_dir {
                if di == 0 && dj == 0 {
                    continue;
                }
                if word.chars().enumerate().all(|(k, c)| {
                    let ty = rx as isize + k as isize * di;
                    let tx = cx as isize + k as isize * dj;
                    ty >= 0 && ty < grid.len() as isize &&
                        tx >= 0 && tx < grid[0].len() as isize &&
                        grid[ty as usize][tx as usize] == c
                }) {
                    result_1 += 1;
                }
            }
        }
    }

    for (ca, ra) in list_center {
        let xl = ra as isize - 1;
        let xr = ra as isize + 1;
        let yt = ca as isize - 1;
        let yb = ca as isize + 1;

        if xl >= 0 && xr < grid[0].len() as isize && yt >= 0 && yb < grid.len() as isize {
            let check = ['M', 'S'];
            let mut diag_1 = [
                grid[yt as usize][xl as usize],
                grid[yb as usize][xr as usize],
            ];
            let mut diag_2 = [
                grid[yt as usize][xr as usize],
                grid[yb as usize][xl as usize],
            ];
            diag_1.sort();
            diag_2.sort();

            if diag_1 == check && diag_2 == check {
                result_2 += 1;
            }
        }
    }

    println!("Result 1: {}", result_1);
    println!("Result 2: {}", result_2);
}