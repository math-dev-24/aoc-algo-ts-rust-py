export function solve_day_12(input: string): number {
    // Parse the input to create the grid
    const grid: string[][] = input.trim().split("\n").map(line => line.trim().split(""));

    const price = calculateTotalPrice(grid);

    console.log(`Prix : ${price}`);
    return price;
}


type Position = { x: number; y: number };

function calculateTotalPrice(grid: string[][]): number {
    const rows: number = grid.length;
    const cols: number = grid[0].length;
    const visited: boolean[][] = Array(rows).fill(0).map(() => Array(cols).fill(false));

    const directions: Position[] = [
        { x: -1, y: 0 },
        { x: 1, y: 0 },
        { x: 0, y: -1 },
        { x: 0, y: 1 },
    ];

    const isInsideGrid = (x: number, y: number): boolean => {
        return x >= 0 && x < rows && y >= 0 && y < cols;
    }


    function bfs(start: Position, plantType: string): { area: number; sides: number } {
        let queue: Position[] = [start];
        visited[start.x][start.y] = true;

        let area = 0;
        let sides = 0;

        while (queue.length > 0) {
            const { x, y } = queue.shift()!;
            area++;

            for (const dir of directions) {
                const nx = x + dir.x;
                const ny = y + dir.y;

                // Vérifie si le côté est un bord ou une séparation
                if (!isInsideGrid(nx, ny) || grid[nx][ny] !== plantType) {
                    sides++;
                } else if (!visited[nx][ny]) {
                    // Ajoute la cellule voisine à la file pour exploration
                    visited[nx][ny] = true;
                    queue.push({ x: nx, y: ny });
                }
            }
        }

        return { area, sides };
    }

    let totalPrice = 0;

    for (let r = 0; r < rows; r++) {
        for (let c = 0; c < cols; c++) {
            if (!visited[r][c]) {
                const plantType = grid[r][c];
                const { area, sides } = bfs({ x: r, y: c }, plantType);
                totalPrice += area * sides;
            }
        }
    }

    return totalPrice;
}
