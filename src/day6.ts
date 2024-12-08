// const fs = require('fs');
// const path = require('path');

const data_day_6 = fs.readFileSync(path.join(__dirname, './data.txt'), 'utf8');
const time_start = new Date().getTime();

const grid: string[][] = data_day_6.split("\n").map((r: string) => r.trim().split(""));
const q_row: number = grid.length;
const q_col: number = grid[0].length;
let guardInitPos: [number, number] | null;
let guardInitDir: Direction | null;

type Direction = '^' | '>' | 'v' | '<';
const turnOrder: Direction[] = ['^', '>', 'v', '<'];


const getInitPos = (grid: string[][], rows: number, cols: number): [[number, number], Direction] => {
    let tmpGuardPos: [number, number] | null = null;
    let tmpGuardDir: Direction | null = null;

    for (let r = 0; r < rows; r++) {
        for (let c = 0; c < cols; c++) {
            const cell = grid[r][c] as Direction | string;
            if (turnOrder.includes(cell as Direction)) {
                tmpGuardPos = [r, c];
                tmpGuardDir = cell as Direction;
                break;
            }
        }
        if (tmpGuardPos) break;
    }
    if (!tmpGuardPos || !tmpGuardDir) {
        throw new Error("Guard position or direction not found on the map");
    }
    return [tmpGuardPos, tmpGuardDir]

}

const simulateGuardPatrol = (
    grid: string[][],
    rows: number, cols: number,
    iniPosition: [number, number],
    initDir: Direction,
    detectLoop: boolean = false
): [ boolean, Set<string> ] => {

    const directions: Record<Direction, [number, number]> = {
        '^': [-1, 0],  // Haut
        '>': [0, 1],   // Droite
        'v': [1, 0],   // Bas
        '<': [0, -1],  // Gauche
    };

    let guardPos: [number, number] = iniPosition;
    let guardDir: Direction = initDir;

    const visited = new Set<string>();

    while (true) {
        const [r, c] = guardPos;
        const state = detectLoop ? `${r},${c},${guardDir}` : `${r},${c}`;

        if (visited.has(state) && detectLoop) {
            return [true, visited];
        }

        visited.add(state);

        const [dr, dc] = directions[guardDir];
        const [fr, fc] = [r + dr, c + dc];

        if (fr < 0 || fr >= rows || fc < 0 || fc >= cols) {
            return [false, visited]
        }
        const frontCell: string = grid[fr][fc];

        if (frontCell === "#" || frontCell == "O") {
            const currentDirIdx: number = turnOrder.indexOf(guardDir);
            guardDir = turnOrder[(currentDirIdx + 1) % 4];
        } else {
            guardPos = [fr, fc];
        }
    }
};

const initPosition = getInitPos(grid, q_row, q_col);
guardInitPos = initPosition[0];
guardInitDir = initPosition[1];

const time_int: number = new Date().getTime();
const totalPart1 = simulateGuardPatrol(grid, q_row, q_col, guardInitPos, guardInitDir)
const time_end_part1: number = new Date().getTime();
console.log("Partie 1 :", totalPart1[1].size, ", en :", (time_end_part1 - time_start) / 1000, "s");

let totalPart2: number = 0;

for (const cell of totalPart1[1]) {
    const [r, c] = cell.split(",").map(Number);
    if (grid[r][c] == "."){
        grid[r][c] = "O";
        const [isLooping] = simulateGuardPatrol(grid, q_row, q_col, guardInitPos, guardInitDir, true);
        if (isLooping) totalPart2++;
        grid[r][c] = ".";
    }
}
const time_end_part2: number = new Date().getTime();
console.log("Partie 2 :", totalPart2, " en : ", ((time_end_part2 - time_start) - (time_end_part1 - time_int)) / 1000, "s");