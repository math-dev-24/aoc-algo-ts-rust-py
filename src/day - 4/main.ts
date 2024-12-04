const fs = require('fs');
const path = require('path');

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

const data_day_4 = fs.readFileSync(path.join(__dirname, './data.txt'), 'utf8').trim().split("\n");

const grid: string[][] = data_day_4.map((line: string) => line.split(""))


console.log("Resultat : ", findXMAS(grid, "XMAS"));
console.log("Resultat 2 : ", findXMASX(grid))


// PART 1
function findXMAS(grid: string[][], search: string): number {
    let result: number = 0;
    const rows = grid.length;
    const cols = grid[0].length;

    for (let row = 0; row < rows; row++) {
        for (let col = 0; col < cols; col++) {
            for (const [dr, dc] of directions) {
                if (is_word_found(row, col, dr, dc, search)){
                    result++;
                }
            }
        }
    }
    return result;
}

function is_word_found(row: number, col: number, dir_row: number, dir_col: number, search: string): boolean {
    for (let i = 0; i < search.length; i++){
        const tmp_row: number = row + i * dir_row;
        const tmp_col: number = col + i * dir_col;
        
        if (tmp_row < 0 || tmp_row >= grid.length || tmp_col < 0 || tmp_col >= grid[0].length || grid[tmp_row][tmp_col] !== search[i]) {
            return false;
        }
    }
    return true;
}

// PART 2
function findXMASX(grid: string[][]): number {
    let result: number = 0;
    const rows = grid.length;
    const cols = grid[0].length;

    for (let row = 1; row < rows - 1; row++) {
        for (let col = 1; col < cols - 1; col++) {
            if (is_cross(row, col, grid)) {
                result++;
            }
        }
    }
    return result;
}

function is_cross(row: number, col: number, grid: string[][]): boolean {
    const topLeft = `${grid[row - 1][col - 1]}${grid[row][col]}${grid[row + 1][col + 1]}`;
    const topRight = `${grid[row - 1][col + 1]}${grid[row][col]}${grid[row + 1][col - 1]}`;

    return (topLeft === "MAS" || topLeft === "SAM") && (topRight === "MAS" || topRight === "SAM");
}

