import combinations from "../utils/iterTools";

export function solve_day_8(data: string): string {
    const grid: string[][] = data.split("\n").map((r: string) => r.trim().split(""));

    type ChartMap = {
        [key: string]: [number, number][];
    };

    const isInGrid = (node: [number, number]): boolean => {
        const [y, x] = node;
        return y >= 0 && y < grid.length && x >= 0 && x < grid[0].length;
    };

    const chartMap: ChartMap = {};
    let antiNodes = new Set<string>();

    for (let row = 0; row < grid.length; row++) {
        for (let col = 0; col < grid[row].length; col++) {
            if (grid[row][col] !== ".") {
                const chart: string = grid[row][col];
                if (!chartMap[chart]) {
                    chartMap[chart] = [];
                }
                chartMap[chart].push([row, col]);
            }
        }
    }

    for (const [_, antennes] of Object.entries(chartMap)) {
        if (antennes.length > 1) {
            antennes.forEach(([y, x]) => {
                antiNodes.add(`${y},${x}`);
            });
            }

        for (const [a1, a2] of combinations(antennes, 2)) {
            const [y1, x1] = a1;
            const [y2, x2] = a2;
            const dy: number = y2 - y1;
            const dx: number = x2 - x1;

            let factor: number = 1;

            while (true) {
                const scaled_dx: number = dx * factor;
                const scaled_dy: number = dy * factor;
                const n1: [number, number] = [y1 - scaled_dy, x1 - scaled_dx];
                const n2: [number, number] = [y2 + scaled_dy, x2 + scaled_dx];

                if (!isInGrid(n1) && !isInGrid(n2)) break;

                if (isInGrid(n1)) antiNodes.add(`${n1[0]},${n1[1]}`);
                if (isInGrid(n2)) antiNodes.add(`${n2[0]},${n2[1]}`);

                factor++;
            }
        }
    }
    console.log(antiNodes.size);
    return antiNodes.size.toString();
}