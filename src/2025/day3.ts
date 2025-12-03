

const solve_day_3 = (input: string): number => {
    const TARGET = 12;

    let result = 0;

    for (const line of input.split("\n")) {
        if (line.length < TARGET) continue;

        const chars: number[] = line.split("").map(c => parseInt(c));
        const n = chars.length;

        let selected: number[] = [];
        let start = 0;

        for(let pos = 0; pos < TARGET; pos++) {
            const window_end = n - (TARGET - pos - 1);

            let best_digit = 0;
            let best_index = start;

            for(let i = start; i < window_end; i++) {
                if (chars[i] > best_digit) {
                    best_digit = chars[i];
                    best_index = i;
                }
            }

            selected.push(best_digit);
            start = best_index + 1;
        }

        // Former le nombre à partir des chiffres sélectionnés
        const joltage = selected.reduce((acc, c) => acc * 10 + c, 0);
        result += joltage;
    }

    console.log("Résultat : ", result);

    return result;

}

export default solve_day_3;