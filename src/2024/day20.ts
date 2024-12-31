
export function solve_day_20(data: string): number {
    const grid: string[][] = data.split("\n").map(line => line.split(""));

    const start: [number, number] = [0, 0];
    const end: [number, number] = [0, 0];

    for (let row = 0; row < grid.length; row++) {
        for (let col = 0; col < grid[row].length; col++) {
            if (grid[row][col] === "S") {
                start[0] = row;
                start[1] = col;
            } else if (grid[row][col] === "E") {
                end[0] = row;
                end[1] = col;
            }
        }
    }

    const list_distances: Array<Array<number>> = bfs_from_end(grid, end);
    const path: [number, number][] = find_path(grid, start, list_distances, end);

    console.log(path);

    return 0;
}

function bfs_from_end(grid: string[][], end: [number, number]): number[][] {
    const row: number = grid.length;
    const col: number = grid[0].length;
    const distances: number[][] = Array.from({ length: row }, () => Array(col).fill(Infinity));
    const queue: [number, number][] = [end];
    distances[end[0]][end[1]] = 0;
    const directions: [number, number][] = [[0, -1], [0, 1], [-1, 0], [1, 0]];

    while (queue.length > 0) {
        const current = queue.shift();
        if (!current) continue;

        const [x, y] = current;
        for (const [dx, dy] of directions) {
            const nx: number = x + dx;
            const ny: number = y + dy;
            if (nx >= 0 && nx < row && ny >= 0 && ny < col && grid[nx][ny] !== "#" && distances[nx][ny] === Infinity) {
                distances[nx][ny] = distances[x][y] + 1;
                queue.push([nx, ny]);
            }
        }
    }
    return distances;
}

function find_path(grid: string[][], start: [number, number], end_distances: number[][], end: [number, number]): [number, number][] {
    let queue: [number, number][] = [[start[0], start[1]]];
    const visited: Set<[number, number]> = new Set();
    visited.add(start);

    const directions: [number, number][] = [[0, -1], [0, 1], [-1, 0], [1, 0]];

    while (queue.length > 0) {
        const current = queue.shift();
        if (!current) continue;

        const [x, y] = current;
        const new_p: [number, number][] = queue.map(p => p);
        new_p.push([x, y]);

        if (x === end[0] && y === end[1]) {
            return new_p;
        }

        for (const [dx, dy] of directions) {
            const nx: number = x + dx;
            const ny: number = y + dy;
            if (nx >= 0 && nx < grid.length && ny >= 0 && ny < grid[0].length && grid[nx][ny] !== "#") {
                const next_pos: [number, number] = [nx, ny];
                if (!visited.has(next_pos) && end_distances[nx][ny] < end_distances[x][y]) {
                    visited.add(next_pos);
                    queue.push([next_pos[0], next_pos[1]]);
                }
            }
        }
    }

    return [];
}