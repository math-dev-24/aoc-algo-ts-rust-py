const fs = require('fs');
const path = require('path');

const data_day_6 = fs.readFileSync(path.join(__dirname, './data.txt'), 'utf8');

const grid: string[][] = data_day_6.split("\n").map((r: string) => r.trim().split(""));

type Direction = '^' | '>' | 'v' | '<';

const simulateGuardPatrol = (mapData: string): [boolean, number] => {
    const grid = mapData.split("\n").map(row => row.split(""));
    const rows = grid.length;
    const cols = grid[0].length;
    const directions: Record<Direction, [number, number]> = {
        '^': [-1, 0],  // Haut
        '>': [0, 1],   // Droite
        'v': [1, 0],   // Bas
        '<': [0, -1],  // Gauche
    };

    const turnOrder: Direction[] = ['^', '>', 'v', '<'];
    let guardPos: [number, number] | null = null;
    let guardDir: Direction | null = null;

    for (let r = 0; r < rows; r++) {
        for (let c = 0; c < cols; c++) {
            const cell = grid[r][c] as Direction | string;
            if (turnOrder.includes(cell as Direction)) {
                guardPos = [r, c];
                guardDir = cell as Direction;
                break;
            }
        }
        if (guardPos) break;
    }
    if (!guardPos || !guardDir) {
        throw new Error("Guard position or direction not found on the map");
    }

    const visited = new Set<string>();

    while (true) {
        const [r, c] = guardPos;
        visited.add(`${r},${c}`);
        const [dr, dc] = directions[guardDir];
        const frontPos: [number, number] = [r + dr, c + dc];

        if (frontPos[0] < 0 || frontPos[0] >= rows || frontPos[1] < 0 || frontPos[1] >= cols) {
            break;
        }


        const [fr, fc] = frontPos;
        const frontCell = grid[fr][fc];

        if (visited.has(`${frontPos[0]},${frontPos[1]}`) && (frontCell === "#" || frontCell === "O")) {
            return [true, visited.size];
        }
        if (frontCell === "#") {
            const currentDirIdx = turnOrder.indexOf(guardDir);
            guardDir = turnOrder[(currentDirIdx + 1) % 4];
        } else {
            guardPos = frontPos;
        }
    }
    return [false, visited.size];
};


console.log("Partie 1 :", simulateGuardPatrol(data_day_6)[1]);

const [isLooping, xCount] = simulateGuardPatrol(data_day_6);

console.log("Partie 2 :", isLooping ? "Oui" : "Non", xCount);