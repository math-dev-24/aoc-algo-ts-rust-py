
export default function solve_day_11(input: string): number {
    const initialStones: number[] = input.split(" ").map(Number);


    const result: number = simulateStones_part_1(initialStones, 25);
    const result_part_2: number = simulateStonesOptimized(initialStones, 75);

    console.log(`Nombre de pierres après 25 clignements : ${result}`);
    console.log(`Nombre de pierres après 75 clignements : ${result_part_2}`);
    return 0;
}

function simulateStones_part_1(initialStones: number[], blinks: number): number {
    let stones: number[] = [...initialStones];

    for (let i = 0; i < blinks; i++) {
        const newStones: number[] = [];

        for (const stone of stones) {
            if (stone === 0) {
                newStones.push(1);
            } else if (stone.toString().length % 2 === 0) {
                const stoneStr = stone.toString();
                const half: number = stoneStr.length / 2;
                const left: number = parseInt(stoneStr.slice(0, half), 10);
                const right: number = parseInt(stoneStr.slice(half), 10);
                newStones.push(left, right);
            } else {
                newStones.push(stone * 2024);
            }
        }
        stones = newStones;
    }
    return stones.length;
}

function simulateStonesOptimized(initialStones: number[], blinks: number): number {
    let stoneCounts: Record<number, number> = {};

    for (const stone of initialStones) {
        stoneCounts[stone] = (stoneCounts[stone] || 0) + 1;
    }

    for (let i = 0; i < blinks; i++) {
        const newStoneCounts: Record<number, number> = {};

        for (const [stoneStr, count] of Object.entries(stoneCounts)) {
            const stone: number = parseInt(stoneStr, 10);

            if (stone === 0) {
                newStoneCounts[1] = (newStoneCounts[1] || 0) + count;
            } else if (stone.toString().length % 2 === 0) {
                const stoneStr: string = stone.toString();
                const half: number = stoneStr.length / 2;
                const left: number = parseInt(stoneStr.slice(0, half), 10);
                const right: number = parseInt(stoneStr.slice(half), 10);

                newStoneCounts[left] = (newStoneCounts[left] || 0) + count;
                newStoneCounts[right] = (newStoneCounts[right] || 0) + count;
            } else {
                const newStone = stone * 2024;
                newStoneCounts[newStone] = (newStoneCounts[newStone] || 0) + count;
            }
        }
        stoneCounts = newStoneCounts;
    }
    return Object.values(stoneCounts).reduce((sum, count) => sum + count, 0);
}