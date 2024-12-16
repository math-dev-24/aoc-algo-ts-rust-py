interface Point {
    row: number;
    col: number;
}

type Direction = "N" | "E" | "S" | "W";

interface State {
    position: Point;
    direction: Direction;
    score: number;
}

export function solve_day16(input: string): number {
    const grid: string[][] = input.split("\n").map(line => line.split(""));
    const directions: Direction[] = ["N", "E", "S", "W"];
    const deltas: Record<Direction, [number, number]> = {
        "N": [-1, 0],
        "E": [0, 1],
        "S": [1, 0],
        "W": [0, -1],
    };

    let start: Point = { row: 0, col: 0 };
    let end: Point = { row: 0, col: 0 };

    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            if (grid[i][j] === "S") start = { row: i, col: j };
            if (grid[i][j] === "E") end = { row: i, col: j };
        }
    }

    const queue: State[] = [];
    queue.push({ position: start, direction: "N", score: 0 });

    const visited = new Set<string>();
    let bestScore = Infinity;

    while (queue.length > 0) {
        queue.sort((a, b) => a.score - b.score);
        const { position, direction, score } = queue.shift()!;

        const key: string = `${position.row},${position.col},${direction}`;
        if (visited.has(key)) continue;

        visited.add(key);

        if (position.row === end.row && position.col === end.col) {
            bestScore = Math.min(bestScore, score);
            continue;
        }

        for (const dir of directions) {
            const [dRow, dCol] = deltas[dir];
            const newRow = position.row + dRow;
            const newCol = position.col + dCol;

            if (
                newRow >= 0 &&
                newRow < grid.length &&
                newCol >= 0 &&
                newCol < grid[0].length &&
                grid[newRow][newCol] !== "#"
            ) {
                queue.push({
                    position: { row: newRow, col: newCol },
                    direction: dir,
                    score: score + (dir === direction ? 1 : 1000),
                });
            }
        }
    }
    console.log(bestScore);
    return bestScore === Infinity ? -1 : bestScore;
}