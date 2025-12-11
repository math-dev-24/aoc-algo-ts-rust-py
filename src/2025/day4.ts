interface Position {
    row: number,
    col: number
}

//  y (line) , x (col)
const POSITIONS: number[][] = [
    [0,1], // a droite
    [0,-1], // a gauche
    [-1 , 0], // en haut
    [1,0], // en bas
    [-1,-1], // haut gauche
    [-1,1], // haut droite
    [1,-1], // bas gauche
    [1,1] // bas droite
]


const solve_day_4_2025 = (input: string): number => {

    const demo = `..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.`;

    const grid: string[][] = demo.split("\n").map(el => el.split(""));
    const result_1 = part_1(grid);
    const result_2 = part_2(grid);

    console.log("Part 1 : ",result_1);
    console.log("Part 2 : ",result_2);
    return 1;
}

function part_1(grid: string[][]): number {
    let result_1: number = 0;

    for (let row = 0; row < grid.length; row++) {
        for (let col = 0; col < grid[0].length; col++){
            if (grid[row][col] == "@" && can_access(grid, {row, col})) {
                result_1++;
            }
        }
    }
    return result_1;
}

function part_2(grid: string[][]): number {
    let result_2: number = 0;
    let removed: boolean = true;

    while (removed) {
        removed = false;
        for (let row = 0; row < grid.length; row++) {
            for (let col = 0; col < grid[0].length; col++){
                if (grid[row][col] == "@" && can_access(grid, {row, col})) {
                    result_2 ++;
                    grid[row][col] = ".";
                    removed = true;
                }
            }
        }

    }
    return result_2
}

function can_access(grid: string[][], position: Position,): boolean {
    let neighbor_count = 0;

    const in_grid = (row: number, col: number): boolean => row >= 0 && row < grid.length && col >= 0 && col < grid[0].length 

    for (const tmp_dx of POSITIONS) {
        const d_row = tmp_dx[0] + position.row
        const d_col = tmp_dx[1] + position.col

        if(!in_grid(d_row, d_col)) continue;

        if (grid[d_row][d_col] == "@"){
            neighbor_count++;
            if (neighbor_count >= 4) return false
        }
        
    }

    return neighbor_count < 4;
}

export default solve_day_4_2025;