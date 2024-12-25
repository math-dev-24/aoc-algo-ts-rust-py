export function solve_day_18(input: string): number {
    const bytes: number[][] = input.split("\n").map(line => line.split(",").map(char => parseInt(char)));

    const SIZE: number = 71;
    const grid: number[][] = Array.from({ length: SIZE }, () => Array(SIZE).fill(0));
    const start: number[] = [0, 0];
    const end: number[] = [SIZE - 1, SIZE - 1];

    // partie 1
    for (let i = 0; i < 1025; i++) {
        const [x, y] = bytes[i];
        grid[x][y] = 1;
    }

    const is_valid = (x: number, y: number): boolean => {
        return x >= 0 && x < SIZE && y >= 0 && y < SIZE && grid[x][y] === 0;
    }

    const shortest_path = (start: number[], end: number[]): number => {
        const queue: number[][] = [[start[0], start[1], 0]];
        const visited: Set<string> = new Set();

        while (queue.length > 0) {
            const [x, y, steps] = queue.shift()!;
            const label: string = `${x},${y}`;

            if (visited.has(label)) {
                continue;
            }

            visited.add(label);

            if (x === end[0] && y === end[1]) {
                return steps;
            }

            const directions: number[][] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

            for (let i = 0; i < directions.length; i++) {
                const [dx, dy] = directions[i];
                const nx: number = x + dx;
                const ny: number = y + dy;

                if (is_valid(nx, ny)) {
                    queue.push([nx, ny, steps + 1]);
                }
            }
        }

        return -1;
    }

    const result = shortest_path(start, end);
    console.log("Partie 1 :", result);

    // partie 2
    let blocking_byte: number[]|null = null;
    for (let i = 0; i < bytes.length; i++) {
        const [x, y] = bytes[i];
        grid[x][y] = 1;
        const steps: number = shortest_path(start, end);
        if (steps === -1) {
            blocking_byte = [x, y];
            break;
        }
    }
    console.log("Partie 2 :", blocking_byte);

    return 0;
}