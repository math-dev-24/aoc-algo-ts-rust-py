
export interface Coordinate {
    row: number;
    col: number;
}

export interface Direction {
    d_row: number;
    d_col: number;
}

export function solve_day_15(input: string): number {
    const lines: string[] = input.split(/\r?\n/);
    const emptyLineIndex: number = lines.findIndex(line => line.trim() === "");
    const grid_txt: string[] = lines.slice(0, emptyLineIndex);

    const movements: string[] = lines.slice(emptyLineIndex + 1).join("").split("");
    const grid: string[][] = grid_txt.map(line => line.split(""));

    let robot_position: Coordinate = get_position_robot(grid);

    for (const movement of movements) {
        const tmp_direction: Direction = get_movement_delta(movement);
        const target_row: number = robot_position.row + tmp_direction.d_row;
        const target_col: number = robot_position.col + tmp_direction.d_col;
        const targetCell: string = grid[target_row][target_col];

        if (targetCell === ".") {
            update_position(grid, robot_position, target_row, target_col);
        } else if (targetCell === "O") {
            if (can_push_boxes(grid, target_row, target_col, tmp_direction)) {
                push_boxes(grid, target_row, target_col, tmp_direction);
                update_position(grid, robot_position, target_row, target_col);
            }
        }
    }
    const result1: number = calculate_gps_sum(grid);
    console.log('Résultat : ', result1);
    return result1;
}

export function get_position_robot(grid: string[][]): Coordinate {
    for (let row = 0; row < grid.length; row++) {
        for (let col = 0; col < grid[row].length; col++) {
            if (grid[row][col] === "@") {
                grid[row][col] = ".";
                return { row, col };
            }
        }
    }
    throw new Error("Robot not found in the grid.");
}

function update_position(grid: string[][], robot_position: Coordinate, row: number, col: number): void {
    grid[robot_position.row][robot_position.col] = ".";
    grid[row][col] = "@";
    robot_position.row = row;
    robot_position.col = col;
}

export function get_movement_delta(direction: string): Direction {
    switch (direction) {
        case "^": return { d_col: 0, d_row: -1 };
        case "v": return { d_col: 0, d_row: 1 };
        case "<": return { d_col: -1, d_row: 0 };
        case ">": return { d_col: 1, d_row: 0 };
        default: throw new Error(`Invalid direction: ${direction}`);
    }
}

function can_push_boxes(grid: string[][], row: number, col: number, direction: Direction): boolean {
    row += direction.d_row;
    col += direction.d_col;
    if (row < 0 || row >= grid.length || col < 0 || col >= grid[0].length) return false;
    const cell: string = grid[row][col];

    if (cell === "#") {
        return false;
    }
    if (cell === ".") {
        return true;
    }
    if (cell === "O") {
        return can_push_boxes(grid, row, col, direction);
    }
    return false;
}

function push_boxes(grid: string[][], startRow: number, startCol: number, direction: Direction): void {
    const chain: Coordinate[] = [];
    let row = startRow, col = startCol;

    while (grid[row][col] === "O") {
        chain.push({ row, col });
        row += direction.d_row;
        col += direction.d_col;
    }

    if (grid[row][col] === ".") {
        for (let i = chain.length - 1; i >= 0; i--) {
            const { row, col } = chain[i];
            grid[row][col] = ".";
            grid[row + direction.d_row][col + direction.d_col] = "O";
        }
    }
}

function calculate_gps_sum(grid: string[][]): number {
    return grid.reduce((sum, row, y) =>
        sum + row.reduce((rowSum, cell, x) =>
            rowSum + (cell === "O" ? 100 * y + x : 0), 0), 0);
}


