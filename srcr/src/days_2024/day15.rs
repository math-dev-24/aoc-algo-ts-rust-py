
#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn can_move(&self, pos: &Point, d_pos: &Direction) -> bool {
        let new_pos: Point = pos.get_next(&d_pos);

        if self.grid[new_pos.row][new_pos.col] == '#' {
            return false;
        }
        if self.grid[new_pos.row][new_pos.col] == '.' {
            return true;
        }
        self.can_move(&Point { row: new_pos.row, col: new_pos.col }, &d_pos)
    }

    fn push_block(&mut self, pos: &Point, d_pos: &Direction) {
        let mut current_pos: Point = pos.clone();
        let next_pos: Point = current_pos.get_next(&d_pos);
        let mut move_to: Point = Point { row: 0, col: 0 };

        loop {
            let next_pos: Point = current_pos.get_next(&d_pos);

            if next_pos.row >= self.rows || next_pos.col >= self.cols || self.grid[next_pos.row][next_pos.col] == '#' {
                break;
            }

            match self.grid[next_pos.row][next_pos.col] {
                '.' => {
                    move_to = next_pos;
                    break;
                }
                _ => {
                    return;
                }
            }
        }

        if move_to.row != 0 || move_to.col != 0 {
            self.grid[current_pos.row][current_pos.col] = '.';
            self.grid[next_pos.row][next_pos.col] = d_pos.char;
            self.grid[move_to.row][move_to.col] = 'O';
        }
    }

    fn print_grid(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{}", self.grid[row][col]);
            }
            println!();
        }
    }
}

#[derive(Debug, Clone)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn get_next(&self, d_pos: &Direction) -> Point {
        let new_row: isize = self.row as isize + d_pos.d_row;
        let new_col: isize = self.col as isize + d_pos.d_col;
        Point {
            row: new_row as usize,
            col: new_col as usize,
        }
    }
}

#[derive(Debug)]
struct Direction {
    d_col: isize,
    d_row: isize,
    char: char,
}

pub fn solve_run_day_15(input: &str) -> u8 {
    let data_in = input.trim().split("\n\n").collect::<Vec<&str>>();
    let tmp_grid = data_in[0]
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut grid = Grid {
        rows: tmp_grid.len(),
        cols: tmp_grid[0].len(),
        grid: tmp_grid,
    };

    let mut positions: Point = Point { row: 0, col: 0 };

    let commands = data_in[1].trim().split("").collect::<Vec<&str>>();

    for row in 0..grid.rows {
        for col in  0..grid.cols {
            let tmp = grid.grid[row][col];
            if tmp == '@' {
                positions = Point { row, col };
                break;
            }
        }
    }
    println!("positions initial: {:?}", &positions);

    for command in commands {
        if let Some(tmp_dirs) = get_direction(command) {

            println!("Direction : {:?}", &tmp_dirs);

            if grid.can_move(&positions, &tmp_dirs) {
                grid.push_block(&positions, &tmp_dirs);
            }
        }
    }

    grid.print_grid();
    0
}


fn get_direction(command: &str) -> Option<Direction> {
    match command {
        ">" => Some(Direction { d_col: 0, d_row: 1, char: '>' }),
        "v" => Some(Direction { d_col: 1, d_row: 0, char: 'v' }),
        "<" => Some(Direction { d_col: 0, d_row: -1, char: '<' }),
        "^" => Some(Direction { d_col: -1, d_row: 0, char: '^' }),
        _ => None,
    }
}