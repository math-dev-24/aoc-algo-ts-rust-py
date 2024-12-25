export function solve_day_17(input: string): number {
    const lines: string[] = input.split('\n');

    const register_a: number = parseInt(lines[0].match(/Register A: (\d+)/)![1]);
    const register_b: number = parseInt(lines[1].match(/Register B: (\d+)/)![1]);
    const register_c: number = parseInt(lines[2].match(/Register C: (\d+)/)![1]);
    const program: number[] = lines[4].split(' ')[1].split(',').map(x => parseInt(x));

    const get_combo = (op: number, a: number, b: number, c: number): number => {
        if (op == 4) {
            return a;
        } else if (op == 5) {
            return b;
        } else if (op == 6) {
            return c;
        }
        return op;
    }

    const dv = (register: number, op: number, a: number, b: number, c: number): number => {
        return register >> get_combo(op, a, b, c);
    }

    const get_outputs = (prog: number[], a: number = 0, b: number = 0, c: number = 0): bigint[] => {
        let pointer: number = 0;
        let outputs: bigint[] = [];

        while (pointer < prog.length) {
            const instruction: number = prog[pointer];
            const operand: number = prog[pointer + 1];
            const tmp_combo: number = get_combo(operand, a, b, c);
            const tmp_dv: number = dv(a, operand, a, b, c);
            switch (instruction) {
                case 0:
                    a = tmp_dv;
                    break;
                case 1:
                    b ^= operand;
                    break;
                case 2:
                    b = tmp_combo & 7;
                    break;
                case 3:
                    if (a !== 0) {
                        pointer = operand;
                        continue;
                    }
                    break;
                case 4:
                    b ^= c;
                    break;
                case 5:
                    const val: bigint = BigInt(tmp_combo) % BigInt(8);
                    outputs.push(val);
                    break;
                case 6:
                    c = tmp_dv;
                    break;
                case 7:
                    c = tmp_dv;
                    break;
            }
            //console.log(`Pointer ${pointer}, A ${a}, B ${b}, C ${c}, Output ${outputs.join(',')}`);
            pointer += 2;
        }
        return outputs;
    }

    // Partie 1
    const part_1: bigint[] = get_outputs(program, register_a, register_b, register_c);
    console.log(`Partie 1 : ${part_1.join(',')}`);

    // Partie 2
    let candidates: number[] = [0];

    for (let i = 1; i <= program.length; i++) {
        let out: number[] = [];
        //console.log(`Pour ${program.slice(-i)}`);

        for (let c of candidates) {
            //console.log(`Candidat : ${c}`);
            for (let offset = 0; offset < 2 ** 3; offset++) {
                const a: number = (2 ** 3) * c + offset;
                const tmp_outputs: bigint[] = get_outputs(program, a, register_b, register_c);
                if (tmp_outputs.join(",") == program.slice(-i).join(",")) {
                    //console.log(`A valid pour ${program.slice(-i)} == ${tmp_outputs} -> ${a}`);
                    out.push(a);
                }
            }
        }
        candidates = out;
    }
    console.log(`Partie 2 : ${candidates.join(',')}`);
    return 0;
}