export const solve_day_1 = (input: string): number => {
    const lines: string[] = input.split("\n");

    let pointer: number = 50;
    let count: number = 0;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        if (!line) continue;

        const indicator = line[0];
        const angle = parseInt(line.slice(1));

        if (indicator == "L") {
            pointer -= angle;
        } else {
            pointer += angle;
        }

        pointer = rem_euclid(pointer);

        if (pointer == 0) {
            count += 1;
        }
    }

    console.log(count);
    return count;
}

export const solve_day_1_part_2 = (input: string): number => {
    const lines: string[] = input.split("\n");

    let pointer: number = 50;
    let count: number = 0;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        if (!line) continue;

        const indicator = line[0];
        const angle = parseInt(line.slice(1));

        const start = pointer;
        const end = indicator === "L" ? pointer - angle : pointer + angle;

        count += count_zeros(start, end);

        pointer = rem_euclid(end);
    }

    console.log(count);
    return count;
}

const count_zeros = (start: number, end: number): number => {
    if (end > start) {
        const first_cross = Math.floor(start / 100 + 1) * 100;
        if (first_cross > end) {
            return 0;
        }
        return Math.floor((end - first_cross) / 100) + 1;
    } else if (end < start) {
        const first_cross = start % 100 === 0 ? start - 100 : Math.floor(start / 100) * 100;
        if (first_cross < end) {
            return 0;
        }
        return Math.floor((first_cross - end) / 100) + 1;
    }
    return 0;
}

const rem_euclid = (value: number, limit: number = 100): number => {
    return ((value % limit) + limit) % limit;
}