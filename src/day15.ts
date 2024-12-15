interface Coordinate {
    x: number;
    y: number;
}

export function solve_day_15(input: string): number {

    const lines: string[] = input.split(/\r?\n/);
    const emptyLineIndex: number = lines.findIndex(line => line.trim() === "");
    const grid_txt: string[] = lines.slice(0, emptyLineIndex);
    const movements: string[] = lines.slice(emptyLineIndex + 1).join("").split("");

    const grid: string[][] = grid_txt.map(line => line.split(""));
    let robot_position: Coordinate = get_position_robot(grid);


    for (const movement of movements) {
        const { dx, dy } = get_movement_delta(movement);
        const targetX: number = robot_position.x + dx;
        const targetY: number = robot_position.y + dy;

        if (grid[targetY][targetX] === ".") {
            update_position(grid, robot_position, targetX, targetY);
        } else if (grid[targetY][targetX] === "O") {
            if (can_push_boxes(grid, targetX, targetY, dx, dy)) {
                push_boxes(grid, targetX, targetY, dx, dy);
                update_position(grid, robot_position, targetX, targetY);
            }
        }
    }

    const result: number = calculate_gps_sum(grid);

    const map = grid.map(row => row.join("")).join("\n");
    console.log(map);
    console.log(`Résultat : ${result}`);

    return result;
}


function update_position(grid: string[][], robot_position: { x: number, y: number }, newX: number, newY: number): void {
    grid[robot_position.y][robot_position.x] = ".";
    grid[newY][newX] = "@";
    robot_position.x = newX;
    robot_position.y = newY;
}

function get_movement_delta(direction: string): { dx: number, dy: number } {
    switch (direction) {
        case "^": return { dx: 0, dy: -1 };
        case "v": return { dx: 0, dy: 1 };
        case "<": return { dx: -1, dy: 0 };
        case ">": return { dx: 1, dy: 0 };
        default: return { dx: 0, dy: 0 };
    }
}

function get_position_robot(grid: string[][]): { x: number, y: number } {
    for (let y = 0; y < grid.length; y++) {
        for (let x = 0; x < grid[y].length; x++) {
            if (grid[y][x] == "@") {
                return { x, y };
            }
        }
    }
    throw new Error("Robot not found");
}

function can_push_boxes(grid: string[][], startX: number, startY: number, dx: number, dy: number): boolean {
    let x: number = startX;
    let y: number = startY;

    while (true) {
        x += dx;
        y += dy;
        if (grid[y][x] === "#") {
            return false;
        } else if (grid[y][x] === "O") {
            continue;
        } else if (grid[y][x] === ".") {
            return true;
        } else {
            return false;
        }
    }
}


function push_boxes(
    grid: string[][],
    startX: number,
    startY: number,
    dx: number,
    dy: number
): void {
    const positions: { x: number, y: number }[] = [];
    let x: number = startX;
    let y: number = startY;

    while (grid[y][x] === "O") {
        positions.push({ x, y });
        x += dx;
        y += dy;
    }

    for (let i = positions.length - 1; i >= 0; i--) {
        const { x, y } = positions[i];
        grid[y][x] = ".";
        grid[y + dy][x + dx] = "O";
    }
}

function calculate_gps_sum(grid: string[][]): number {
    let sum = 0;
    for (let y = 0; y < grid.length; y++) {
        for (let x = 0; x < grid[y].length; x++) {
            if (grid[y][x] === "O") {
                sum += 100 * y + x;
            }
        }
    }
    return sum;
}