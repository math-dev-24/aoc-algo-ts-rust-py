

const solve_day_2 = (input: string): number[] => {

    const list_of_range = input.split(",");

    let invalid_part_1 = 0;
    let invalid_part_2 = 0;

    for (const range of list_of_range) {
        const [start, end] = range.split("-");

        const s = parseInt(start, 10);
        const e = parseInt(end, 10);

        if (s > e ) {
            throw new Error("S > e");
        }

        for (let i = s; i <= e; i++) {

            if (is_invalid(i)) {
                invalid_part_1 += i;
            }
            if(is_invalid_part_2(i)){
                invalid_part_2 += i;
            }
        }

    }

    console.log("Résultat Part 1 :", invalid_part_1)
    console.log("Résultat Part 2 :", invalid_part_2)

    return [invalid_part_1, invalid_part_2]
}

export default solve_day_2;

const is_invalid = (value: number): boolean => {
    const val_str = value.toString();
    const long = val_str.length;

    if (long % 2 !== 0) {
        return false;
    }

    const mid = long / 2;

    const first_half = val_str.slice(0, mid);
    const second_half = val_str.slice(mid);

    return first_half === second_half;
};

const is_invalid_part_2 = (num: number): boolean => {
    const s = num.toString();
    const len = s.length;

    if (len < 2) return false;

    for (let pattern_len = 1; pattern_len <= len / 2; pattern_len++) {

        if (len % pattern_len !== 0) continue;

        const pattern = s.slice(0, pattern_len);
        let is_valid_pattern = true;

        for (let i = pattern_len; i < len; i += pattern_len) {
            if (s.slice(i, i + pattern_len) !== pattern) {
                is_valid_pattern = false;
                break;
            }
        }

        if (is_valid_pattern) return true;
    }

    return false;
};
