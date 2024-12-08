
export function combinations<T>(array: T[], r: number): T[][] {
    const result: T[][] = [];
    const n = array.length;

    if (r > n) return result;

    const indices = Array.from({ length: r }, (_, i) => i);

    while (true) {
        result.push(indices.map((i) => array[i]));

        let i = r - 1;
        while (i >= 0 && indices[i] === i + n - r) {
            i--;
        }
        if (i < 0) break;

        indices[i]++;
        for (let j = i + 1; j < r; j++) {
            indices[j] = indices[j - 1] + 1;
        }
    }
    return result;
}