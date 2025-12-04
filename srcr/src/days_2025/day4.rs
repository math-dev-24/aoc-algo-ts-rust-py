pub fn solve_day_4(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let result = part_1(&grid);
    let result_part_2 = part_2(&grid);

    println!("Résult : {}", result);
    println!("Partie 2: {}", result_part_2);
}

fn part_1(grid: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let char = grid[row][col];
            if (char == '@' || char == 'x') && is_accessible(grid, row, col, true) {
                result += 1;
            }
        }
    }
    result
}

fn part_2(grid: &Vec<Vec<char>>) -> usize {
    let mut grid = grid.clone();
    let mut result = 0;
    let mut removed = true;

    while removed {
        removed = false;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '@' && is_accessible(&grid, row, col, false) {
                    result += 1;
                    grid[row][col] = '.';
                    removed = true;
                }
            }
        }
    }
    result
}

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),   // bas
    (1, 1),   // bas droite
    (0, 1),   // droite
    (-1, 1),  // haut droite
    (-1, 0),  // haut
    (-1, -1), // haut gauche
    (0, -1),  // gauche
    (1, -1),  // bas gauche
];

fn is_accessible(grid: &Vec<Vec<char>>, row: usize, col: usize, include_x: bool) -> bool {
    let mut neighbor_count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for &(dx, dy) in &DIRECTIONS {
        let new_row = row as isize + dx;
        let new_col = col as isize + dy;

        if new_row < 0 || new_col < 0 || new_row >= rows as isize || new_col >= cols as isize {
            continue;
        }

        let neighbor = grid[new_row as usize][new_col as usize];
        if neighbor == '@' || (include_x && neighbor == 'x') {
            neighbor_count += 1;
            if neighbor_count >= 4 {
                return false;
            }
        }
    }
    neighbor_count < 4
}
