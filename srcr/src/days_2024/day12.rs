use std::collections::VecDeque;

pub fn solve_day12(input: &str) -> u32 {

    let mut grid: Vec<Vec<char>> = input.trim().split("\n")
        .map(|line| line.trim().chars().collect()).collect();

    let price = calculate_total_price(&mut grid);

    println!("Prix : {}", price);
    return price;
}


fn calculate_total_price(grid: &mut Vec<Vec<char>>) -> u32 {
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    let mut total_price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let plant_type = grid[r][c];
                let (area, sides) = bfs(Position { x: r, y: c }, plant_type, &mut visited, &grid);
                total_price += area * sides;
            }
        }
    }

    total_price
}


struct Position {
    x: usize,
    y: usize,
}

fn bfs(
    start: Position,
    plant_type: char,
    visited: &mut Vec<Vec<bool>>,
    grid: &Vec<Vec<char>>
) -> (u32, u32)
{
    let mut queue = VecDeque::new();
    visited[*&start.x][*&start.y] = true;
    queue.push_back(start);

    let mut area = 0;
    let mut sides = 0;

    let directions: Vec<Position> = vec![
        Position { x: 0, y: 1 },
        Position { x: 1, y: 0 },
        Position { x: 0, y: usize::MAX },
        Position { x: usize::MAX, y: 0 },
    ];

    while let Some(Position { x, y }) = queue.pop_front() {
        area += 1;

        for dir in &directions {
            let nx = if dir.x == usize::MAX { x.wrapping_sub(1) } else { x + dir.x };
            let ny = if dir.y == usize::MAX { y.wrapping_sub(1) } else { y + dir.y };

            if nx >= grid.len() || ny >= grid[0].len() || grid[nx][ny] != plant_type {
                sides += 1;
            } else if !visited[nx][ny] {
                visited[nx][ny] = true;
                queue.push_back(Position { x: nx, y: ny });
            }
        }
    }
    (area, sides)
}