const fs = require('fs');
const path = require('path');

const data_day_4 = fs.readFileSync(path.join(__dirname, './data.txt'), 'utf8').trim().split("\n");

const grid: string[][] = [];

data_day_4.forEach((line: string) => {
    const tmp_line: string[] = line.split("");
    grid.push(tmp_line);
});

console.log("Resultat : ", findXMAS(grid));

function findXMAS(grid: string[][]): number {
    const word: string = "XMAS";
    let result: number = 0;
    const rows: number = grid.length;
    const cols: number = grid[0].length;

    const directions: number[][] = [
        [0, 1],   // Droite
        [0, -1],  // Gauche
        [1, 0],   // En bas
        [-1, 0],  // En haut
        [1, 1],   // Diag bas droite
        [1, -1],  // diag bas gauche
        [-1, 1],  // diag haute droite
        [-1, -1]  // dia haut gauche
    ];

    for (let row = 0; row < rows; row++) {
        for (let col = 0; col < cols; col++) {
            for (const [dr, dc] of directions) {
                let match: boolean = true;
                for (let i = 0; i < word.length; i++) {
                    const nr: number = row + dr * i;
                    const nc: number = col + dc * i;

                    if (
                        nr < 0 || nr >= rows ||
                        nc < 0 || nc >= cols ||
                        grid[nr][nc] !== word[i]
                    ) {
                        match = false;
                        break;
                    }
                }

                if (match) {
                    result++;
                }
            }
        }
    }
    return result;
}