export function solve_day_19(input: string): number {

    const lines: string[] = input.trim().split("\n");
    const motifs_list: string[] = lines[0].split((",")).map((line) => line.trim());
    const serviettes: string[] = lines.slice(2);


    const s_is_valid = (s: string, motifs_available: string[], memo: Map<string, boolean>): boolean => {
        if(memo.has(s)) return memo.get(s) as boolean;
        if(s === "") return true;

        for (const motif_available of motifs_available) {
            if(s.startsWith(motif_available)) {
                const rest: string = s.slice(motif_available.length);
                if(s_is_valid(rest, motifs_available, memo)) {
                    memo.set(s, true);
                    return true;
                }
            }
        }
        memo.set(s, false);
        return false;
    }


    const memo_part_1 = new Map<string, boolean>();
    let valid_serviettes: string[] = [];
    for (const serviette of serviettes) {
        if(s_is_valid(serviette, motifs_list, memo_part_1)) {
            valid_serviettes.push(serviette);
        }
    }
    console.log(`Nombre de serviettes valides : ${valid_serviettes.length} - (Part 1)`);

    const count_combinations = (s: string, motifs_available: string[], memo: Map<string, number>): number => {
        if(memo.has(s)) return memo.get(s) as number;
        if(s === "") return 1;

        let total_comb: number = 0;
        for (const motif_available of motifs_available) {
            if(s.startsWith(motif_available)) {
                const rest: string = s.slice(motif_available.length);
                total_comb += count_combinations(rest, motifs_available, memo);
            }
        }
        memo.set(s, total_comb);
        return total_comb;
    }

    const memo_part_2 = new Map<string, number>();
    let part_2: number = 0;
    for (const serviette of valid_serviettes) {
        part_2 += count_combinations(serviette, motifs_list, memo_part_2);
    }

    console.log(`Nombre total de combinaisons : ${part_2} - (Part 2)`);

    return 0;
}