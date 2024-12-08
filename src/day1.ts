const data_day_1 = "1\n9\n8\n2\n3\n7\n6\n4\n5".split('\n');

const sorted = (a: number, b: number) => a - b;

console.log(`nombre de lignes :`, data_day_1.length);

const deck_1: number[] = [];
const deck_2: number[] = [];

for (const line of data_day_1) {
    const [tmp_1, tmp_2] = line.trim().split(/\s+/);
    deck_1.push(parseInt(tmp_1, 10));
    deck_2.push(parseInt(tmp_2, 10));
}

// Algo : O(n)
function part_1(deck_1: number[], deck_2: number[]): number {
    return deck_1.reduce((total: number, num: number, index: number) => {
        return total + Math.abs(num - deck_2[index]);
    }, 0);
}

// Algo : O(n)
function part_2(deck_1: number[], deck_2: number[]): number {
    const count_map: Map<number, number> = new Map();
    for (const num of deck_2) {
        count_map.set(num, (count_map.get(num) || 0) + 1);
    }
    return deck_1.reduce((total: number, num: number) => {
        return total + ((count_map.get(num) || 0) * num);
    }, 0);
}

deck_1.sort(sorted);
deck_2.sort(sorted);

const result_1: number = part_1(deck_1, deck_2);
const result_2: number = part_2(deck_1, deck_2);

console.log("Part 1 :", result_1);
console.log("Part 2 :", result_2);