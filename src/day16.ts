interface Point {
    row: number;
    col: number;
}

interface Direction {
    row: number;
    col: number;
}

interface Node {
    cost: number;
    row: number;
    col: number;
    directionIndex: number;
    path: Point[];
}

export function solve_day16(input: string): number {
    const grid: string[][] = input.split("\n").map(line => line.split(""));
    const dirs: Direction[] = [
        { row: 0, col: 1 },
        { row: 1, col: 0 },
        { row: 0, col: -1 },
        { row: -1, col: 0 }
    ];

    const isValid = (row: number, col: number): boolean => {
        return row >= 0 && row < grid.length && col >= 0 && col < grid[0].length && grid[row][col] !== "#";
    };

    let start: Point | null = null;
    let end: Point | null = null;

    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[0].length; j++) {
            if (grid[i][j] === "S") {
                start = { row: i, col: j };
            } else if (grid[i][j] === "E") {
                end = { row: i, col: j };
            }
        }
    }

    if (!start || !end) {
        throw new Error("Invalid input");
    }

    const queue: Node[] = [{
        cost: 0,
        row: start.row,
        col: start.col,
        directionIndex: 0,
        path: []
    }];
    const visited: Record<string, number> = {};
    const successfulPathsCells: Set<string> = new Set();
    let bestPaths: Point[][] = [];
    let minCost: number = Infinity;

    while (queue.length > 0) {
        const { cost, row, col, directionIndex, path } = queue.shift()!;

        // Unique key for visited
        const key = `${row},${col},${directionIndex}`;
        if (visited[key] !== undefined && visited[key] < cost) {
            continue;
        }
        visited[key] = cost;

        const newPath = [...path, { row, col }];

        if (row === end.row && col === end.col) {
            if (cost < minCost) {
                minCost = cost;
                bestPaths = [newPath];
            } else if (cost === minCost) {
                bestPaths.push(newPath);
            }
            continue;
        }

        // Move forward in the same direction
        const { row: dr, col: dc } = dirs[directionIndex];
        const newRow = row + dr;
        const newCol = col + dc;

        if (isValid(newRow, newCol)) {
            queue.push({
                cost: cost + 1,
                row: newRow,
                col: newCol,
                directionIndex,
                path: newPath
            });
        }

        queue.push({
            cost: cost + 1000,
            row,
            col,
            directionIndex: (directionIndex + 1) % 4,
            path: newPath
        });

        queue.push({
            cost: cost + 1000,
            row,
            col,
            directionIndex: (directionIndex + 3) % 4,
            path: newPath
        });
    }

    for (const path of bestPaths) {
        for (const { row, col } of path) {
            successfulPathsCells.add(`${row},${col}`);
        }
    }
    for (const cell of successfulPathsCells.values()) {
        const [row, col] = cell.split(",");
        grid[Number(row)][Number(col)] = "O";
    }

    console.log(`Partie 1 : ${minCost}`);
    console.log(`Partie 2 : ${successfulPathsCells.size}`);
    console.log(grid.map(row => row.join("")).join("\n"));

    return successfulPathsCells.size;
}
