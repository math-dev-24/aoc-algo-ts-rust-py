use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;


#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug, Hash)]
struct Coordinate {
    row: usize,
    col: usize
}

#[derive(Debug)]
struct Dij {
    grid: Vec<Vec<char>>,
    q_row: usize,
    q_col: usize,
    start: Coordinate,
    end: Coordinate
}


impl Dij {
    fn new(s: &str) -> Self{
        let input: String = s.to_string();
        let grid: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let q_row = grid.len();
        let q_col = grid[0].len();

        Self {
            grid,
            q_row: q_row,
            q_col: q_col,
            start: Coordinate{row: 0, col: 0}, end: Coordinate {row: 0, col: 0}
        }
    }
    fn find_position(&mut self){
        for r in 0..self.q_row {
            for c in 0..self.q_col {
                let tmp_val = self.grid[r][c];
            
                if tmp_val == 'S' {
                    self.start = Coordinate {
                        row: r, col: c
                    }
                }else if tmp_val == 'E' {
                    self.end = Coordinate {
                        row: r, col: c
                    }
                }
            }
        }
    }
    fn is_valid (&self, row: &usize, col: &usize) -> bool {
        let row = *row;
        let col = *col;
        return row < self.q_row && col < self.q_col && self.grid[row][col] != '#';
    }
}

pub fn solve_day_16(input: &str)-> u32 {

    let mut solver: Dij = Dij::new(input);
    solver.find_position();
    let result = solver.find_cost_path();
    println!("Résultat : {:?}", &result.0);
    println!("partie 2 : {}", &result.1.len());
    0
}



impl Dij {
    fn find_cost_path(&mut self) -> (u32, HashSet<Coordinate>) {

        let directions = vec![
            Coordinate { row: 0, col: 1 },
            Coordinate { row: 1, col: 0 },
            Coordinate { row: 0, col: usize::MAX },
            Coordinate { row: usize::MAX, col: 0 },
        ];

        let mut queue = BinaryHeap::new();
        let mut visited: HashMap<(usize, usize, usize), u32> = HashMap::new();
        let mut successful_paths_cells: HashSet<Coordinate> = HashSet::new();
        let mut best_paths: Vec<Vec<Coordinate>> = vec![];
        let mut min_cost = u32::MAX;

        queue.push(Reverse((0, self.start.row, self.start.col, 0, vec![])));

        while let Some(Reverse((cost, row, col, direction, path))) = queue.pop() {

            if let Some(&prev_cost) = visited.get(&(row, col, direction)) {
                if prev_cost < cost {
                    continue;
                }
            }

            visited.insert((row, col, direction), cost);

            let mut new_path = path.clone();
            new_path.push(Coordinate { row, col });

            if row == self.end.row && col == self.end.col {
                if cost < min_cost {
                    min_cost = cost;
                    best_paths = vec![new_path.clone()];
                } else if cost == min_cost {
                    best_paths.push(new_path.clone());
                }
                continue;
            }

            let dir = &directions[direction];
            let new_row = if dir.row == usize::MAX { row.wrapping_sub(1) } else { row + dir.row };
            let new_col = if dir.col == usize::MAX { col.wrapping_sub(1) } else { col + dir.col };

            if self.is_valid(&new_row, &new_col) {
                queue.push(Reverse((cost + 1, new_row, new_col, direction, new_path.clone())));
            }

            queue.push(Reverse((cost + 1000, row, col, (direction + 1) % 4, new_path.clone())));
            queue.push(Reverse((cost + 1000, row, col, (direction + 3) % 4, new_path.clone())));
        }

        for path in best_paths.iter() {
            for coord in path {
                successful_paths_cells.insert(coord.clone());
            }
        }

        (min_cost, successful_paths_cells)
    }
}