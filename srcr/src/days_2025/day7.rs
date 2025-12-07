use colored::Colorize;

pub fn solve_day_7(input: &str) {

    let grid = input.lines()
    .map(|l| l.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let part_1 = part_1(&grid, true);

    let part_2 = count_all_timelines(&grid);

    println!("Partie 1 : {:?}", part_1);
    println!("Partie 2 : {}", part_2);



}


fn search_start(line: Vec<char>) -> usize {
    for (idx, char) in line.iter().enumerate() {
        if *char == 'S' {
            return idx;
        }
    }
    panic!("Start introuvable !")
}

fn draw_tree(grid: &Vec<Vec<char>>) {
    let width = grid[0].len();

    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            let colored_char = match ch {
                '.' => {
                    let has_left = col > 0 && grid[row][col - 1] == '|';
                    let has_right = col + 1 < width && grid[row][col + 1] == '|';

                    if has_left && has_right {
                        ch.to_string().green()
                    } else {
                        ch.to_string().dimmed()
                    }
                },
                '^' | '|' | 'S' => ch.to_string().green().bold(),
                _ => ch.to_string().normal(),
            };
            print!("{}", colored_char);
        }
        println!();
    }
}

fn count_all_timelines(grid: &Vec<Vec<char>>) -> usize {
    use std::collections::HashMap;

    let height = grid.len();
    let width = grid[0].len();

    let mut paths: HashMap<(usize, usize), usize> = HashMap::new();

    let start_col = search_start(grid[0].clone());
    paths.insert((0, start_col), 1);

    for row in 0..height - 1 {
        let mut next_paths: HashMap<(usize, usize), usize> = HashMap::new();

        for ((r, col), count) in paths.iter() {
            if *r != row {
                continue;
            }

            let next_row = row + 1;
            let el = grid[next_row][*col];

            if el == '.' {
                *next_paths.entry((next_row, *col)).or_insert(0) += count;
            } else if el == '^' {
                if *col > 0 {
                    *next_paths.entry((next_row, col - 1)).or_insert(0) += count;
                }
                if *col + 1 < width {
                    *next_paths.entry((next_row, col + 1)).or_insert(0) += count;
                }
            }
        }

        for (pos, count) in next_paths {
            *paths.entry(pos).or_insert(0) += count;
        }
    }

    paths.iter()
        .filter(|((r, _), _)| *r == height - 1)
        .map(|(_, count)| count)
        .sum()
}


fn part_1(grid: &Vec<Vec<char>>, print_result: bool) -> usize {
    let mut grid = grid.clone();

    let mut counter: usize = 0;
    let mut list_idx: Vec<usize> = vec![];
    let height = grid.len();

    for row in 0..height {
        let line: Vec<char> = grid[row].clone();
        let length_row: usize = grid[row].len();


        let mut tmp_list: Vec<usize> = vec![];

        if row == 0 {
            let idx_start = search_start(line);
            list_idx.push(idx_start);
            continue;
        }

        for col in list_idx {
            let el = grid[row][col];
            if el == '.' {
                grid[row][col] = '|';
                tmp_list.push(col);
            } else if el == '^' {
                counter += 1;
                grid[row][col] = '|';  // Marquer la position du ^
                let new_col = [col-1, col+1];
                for new in new_col {
                    if new < length_row {
                        tmp_list.push(new);
                    }
                }
            }
        }
        
        list_idx = tmp_list;
        println!("{:?}", list_idx);
    }

    if print_result {
        draw_tree(&grid);
    }

    counter
}