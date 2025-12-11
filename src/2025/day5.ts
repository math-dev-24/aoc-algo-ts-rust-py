
const solve_day_5_2025 = (input: string): number => {

    const split_input: string[] = input.split("\n\n");

    const list_range: string[] = split_input[0].split('\n');
    const list_number: number[] = split_input[1].split("\n").map(el => parseFloat(el));

    let part_1: number = 0;
    
    for (const n of list_number) {
        if (in_range(list_range, n)) part_1++;
    }

    console.log(part_1)
    console.log(part_1 == 643);

    const result_part_2: number = part_2(list_range);
    console.log(result_part_2);
    console.log(result_part_2 == 342018167474526)
    return 1;
}

export default solve_day_5_2025;


function in_range(list_range: string[], n: number): boolean {
    for (const range of list_range) {
        const tmp_range: number[] = range.split('-').map(el => parseFloat(el));
        if (tmp_range[0] <= n && n <= tmp_range[1]) return true;
    }
    return false;
}

function part_2(list_range: string[]): number {

    let intervals = list_range.map(el => {
        const spl = el.split("-").map(e => parseFloat(e))
        return {min: spl[0], max: spl[1]}
    });

    intervals.sort((a, b) => a.min - b.min);

    let merged_range: number[][] = [];

    for (const i of intervals) {
        if (merged_range.length == 0) {
            merged_range.push([i.min, i.max]);
        } else {
            const last_idx = merged_range.length - 1;
            const [last_min, last_max] = merged_range[last_idx];

            if (i.min <= last_max + 1) {
                merged_range[last_idx] = [last_min, Math.max(last_max, i.max)]
            } else {
                merged_range.push([i.min, i.max])
            }
        }
    }

    return merged_range.reduce((acc, el) => {
        const min = el[0];
        const max = el[1];
        return acc + (max - min + 1)
    }, 0)
}