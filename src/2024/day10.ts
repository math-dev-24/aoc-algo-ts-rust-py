export default function solve_day_10(input: string): number {
    const grid: number[][] = input.split("\n")
        .map(line => line.split("").map(char => parseInt(char)));


    const trailheads: [number, number][] = grid
        .map((row, x) =>
            row.map((cell, y) => (cell === 0 ? [x, y] : null)).filter(Boolean)
        ).flat() as [number, number][];

    let total: number = 0;

    for (const [x, y] of trailheads) {
        total += exploreDistinctTrails(grid, x, y, new Set<string>());
    }
    console.log(total);
    return total;
}

function exploreDistinctTrails(grid: number[][], x: number, y: number, visited: Set<string>): number {
    if (grid[x][y] === 9) {
        return 1;
    }

    visited.add(`${x},${y}`);
    let totalPaths: number = 0;

    for (const [dx, dy] of [[0, 1], [0, -1], [1, 0], [-1, 0]]) {
        const newX: number = x + dx;
        const newY: number = y + dy;

        if (
            newX >= 0 &&
            newX < grid.length &&
            newY >= 0 &&
            newY < grid[0].length &&
            grid[newX][newY] === grid[x][y] + 1 &&
            !visited.has(`${newX},${newY}`)
        ) {
            totalPaths += exploreDistinctTrails(grid, newX, newY, visited);}
    }

    visited.delete(`${x},${y}`);
    return totalPaths;
}