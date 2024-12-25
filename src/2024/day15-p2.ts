import {Coordinate, get_position_robot, Direction, get_movement_delta} from "./day15-p1";

export function solve_day_15_p2(input: string): number {
    const lines: string[] = input.split(/\r?\n/);
    const emptyLineIndex: number = lines.findIndex(line => line.trim() === "");
    const grid_txt: string[] = lines.slice(0, emptyLineIndex);

    const movements: string[] = lines.slice(emptyLineIndex + 1).join("").split("");
    const grid: string[][] = grid_txt.map(line => line.split(""));

    const scaledGrid: string[][] = scale_up_grid(grid);
    let robot_position: Coordinate = get_position_robot(scaledGrid);

    for (const movement of movements) {
        const tmp_direction: Direction = get_movement_delta(movement);
        const target_col: number = robot_position.col + tmp_direction.d_col;
        const target_row: number = robot_position.row + tmp_direction.d_row;
        const nextCell: string = scaledGrid[target_row][target_col];
        if (nextCell === ".") {
            scaledGrid[robot_position.row][robot_position.col] = ".";
            scaledGrid[target_row][target_col] = "@";
            robot_position.row = target_row;
            robot_position.col = target_col;
        }else if (nextCell === "[" || nextCell === "]") {
            if (can_push_boxes_part2(scaledGrid, target_row, target_col, tmp_direction)) {
                push_boxes_part2(scaledGrid, target_row, target_col, tmp_direction);
                scaledGrid[robot_position.row][robot_position.col] = ".";
                scaledGrid[target_row][target_col] = "@";
                robot_position.row = target_row;
                robot_position.col = target_col;
            }
        }
    }

    const result: number = calculate_gps_sum_part2(scaledGrid);
    console.log("Résultat :", result);

    return 0;
}

function calculate_gps_sum_part2(grid: string[][]): number {
    return grid.reduce((sum, row, y) =>
        sum + row.reduce((rowSum, cell, x) =>
            rowSum + (cell === "[" ? 100 * y + x : 0), 0), 0);
}

function scale_up_grid(grid: string[][]): string[][] {
    const scaledGrid: string[][] = [];

    for (const row of grid) {
        const scaledRow: string[] = [];
        for (const tile of row) {
            switch (tile) {
                case "#": scaledRow.push("##"); break;
                case "O": scaledRow.push("[]"); break;
                case ".": scaledRow.push(".."); break;
                case "@": scaledRow.push("@."); break;
            }
        }
        scaledGrid.push(scaledRow.join("").split(""));
    }
    return scaledGrid;
}

function can_push_boxes_part2(grid: string[][], row: number, col: number, direction: Direction): boolean {
    const new_row: number = row + direction.d_row;
    const new_col: number = col + direction.d_col;

    if (new_row < 0 || new_row >= grid.length || new_col < 0 || new_col >= grid[0].length) return false;

    const cell: string = grid[new_row][new_col];

    if (cell === "#") return false;
    if (cell === ".") return true;

    if (direction.d_col == 0) {
        if (cell === "]") {
            return can_push_boxes_part2(grid, new_row, new_col, direction) &&
                can_push_boxes_part2(grid, new_row, new_col - 1, direction);
        } else if (cell === "[") {
            return can_push_boxes_part2(grid, new_row, new_col, direction) &&
                can_push_boxes_part2(grid, new_row, new_col + 1, direction);
        }
    } else if (direction.d_row === -1 && cell === "]") {
        return can_push_boxes_part2(grid, new_row, new_col, direction);
    } else if (direction.d_row === 1 && cell === "[") {
        return can_push_boxes_part2(grid, new_row, new_col, direction);
    }
    return false;
}


function push_boxes_part2(grid: string[][], row: number, col: number, direction: Direction): void {
    const new_row: number = row + direction.d_row;
    const new_col: number = col + direction.d_col;
    const new_cell: string = grid[new_row][new_col];

    if (new_row < 0 || new_row >= grid.length || new_col < 0 || new_col >= grid[0].length) return;

    if (new_cell == "#") return;
    if (new_cell === ".") {
        [grid[row][col], grid[new_row][new_col]] = [grid[new_row][new_col], grid[row][col]];
        return;
    }

    if (direction.d_col === 0) { // Mouvement vertical
        if (new_cell === "]") {
            push_boxes_part2(grid, new_row, new_col, direction);
            push_boxes_part2(grid, new_row, new_col - 1, direction);
            [grid[row][col], grid[new_row][new_col]] = [grid[new_row][new_col], grid[row][col]];
        } else if (new_cell === "[") {
            push_boxes_part2(grid, new_row, new_col, direction);
            push_boxes_part2(grid, new_row, new_col + 1, direction);
            [grid[row][col], grid[new_row][new_col]] = [grid[new_row][new_col], grid[row][col]];
        }
    } else if (direction.d_col === -1) { // Mouvement vers la gauche
        if (new_cell === "]") {
            push_boxes_part2(grid, new_row, new_col - 1, direction);
            const temp = grid[new_row][new_col];
            grid[new_row][new_col] = grid[row][col];
            grid[row][col] = grid[new_row][new_col - 1];
            grid[new_row][new_col - 1] = temp;
        }
    } else if (direction.d_col === 1) { // Mouvement vers la droite
        if (new_cell === "[") {
            push_boxes_part2(grid, new_row, new_col + 1, direction);
            const temp = grid[new_row][new_col];
            grid[new_row][new_col] = grid[row][col];
            grid[row][col] = grid[new_row][new_col + 1];
            grid[new_row][new_col + 1] = temp;
        }
    }
}


