
#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn can_move(&self, pos: &Point, d_pos: &Direction) -> bool {
        let mut current_pos = pos.clone();

        while let Some(new_pos) = current_pos.get_next(&d_pos, &self.grid) {
            match self.grid[new_pos.row][new_pos.col] {
                '#' => return false,
                '.' => return true,
                _ => current_pos = new_pos,
            }
        }
        false
    }

    fn push_block(&mut self, pos: &Point, d_pos: &Direction) {
        let mut current_pos: Point = pos.clone();
        let mut available_pos: Option<Point> = None;

        while let Some(next_pos) = current_pos.get_next(&d_pos, &self.grid) {
            match self.grid[next_pos.row][next_pos.col] {
                '.' => {
                    available_pos = Some(next_pos);
                    break;
                }
                'O' => {
                    current_pos = next_pos;
                }
                _ => {
                    break;
                }
            }
        }

        if let Some(move_to) = available_pos {
            self.grid[pos.row][pos.col] = '.';
            let next_position = pos.get_next(&d_pos, &self.grid).unwrap();
            self.grid[next_position.row][next_position.col] = d_pos.char;
            self.grid[move_to.row][move_to.col] = 'O';
        }
    }

    fn get_points(&self) -> u32 {
        let mut total: u32 = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.grid[row][col] == 'O' {
                    total += 100 * row as u32 + col as u32;
                }
            }
        }
        total
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
    fn get_next(&self, d_pos: &Direction, grid: &Vec<Vec<char>>) -> Option<Point> {
        let new_row: isize = self.row as isize + d_pos.d_row;
        let new_col: isize = self.col as isize + d_pos.d_col;
        if new_row >= 0 && new_row < grid.len() as isize && new_col >= 0 && new_col < grid[0].len() as isize {
            Some(Point { row: new_row as usize, col: new_col as usize })
        } else {
            None
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

    let commands = data_in[1].trim().chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

    'outer: for row in 0..grid.rows {
        for col in  0..grid.cols {
            let tmp = grid.grid[row][col];
            if tmp == '@' {
                positions = Point { row, col };
                break 'outer;
            }
        }
    }

    println!("Start position: {:?}", positions);

    for command in commands {
        if let Some(tmp_dirs) = get_direction(&command) {
            if grid.can_move(&positions, &tmp_dirs) {
                grid.push_block(&positions, &tmp_dirs);
                positions = positions.get_next(&tmp_dirs, &grid.grid).unwrap();
            }
        }
    }
    grid.print_grid();

    let points =grid.get_points();
    println!("points: {}", points);
    println!("{}", 1552463 == points);
    0
}

fn get_direction(command: &char) -> Option<Direction> {
    match command {
        '>' => Some(Direction { d_col: 1, d_row: 0, char: '>' }),
        'v' => Some(Direction { d_col: 0, d_row: 1, char: 'v' }),
        '<' => Some(Direction { d_col: -1, d_row: 0, char: '<' }),
        '^' => Some(Direction { d_col: 0, d_row: -1, char: '^' }),
        _ => None,
    }
}