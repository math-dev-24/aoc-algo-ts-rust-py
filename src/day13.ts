interface Button {
    x: number;
    y: number;
}


export function solve_day_13(input: string): number {
    const commandes: string[] = input.trim().split("\n");

    const parse_data = (s: string, isPrize: boolean = false): Button => {
        if (isPrize) {
            const prize: RegExp = /=\d+/g;
            const text = [...s.matchAll(prize)];
            const x: number = parseInt(text[0][0].replace("=", ""), 10);
            const y: number = parseInt(text[1][0].replace("=", ""), 10);
            return { x, y };
        }
        const button: RegExp = /\+\d+/g;
        const text = [...s.matchAll(button)];
        const x: number = parseInt(text[0][0].replace("+", ""), 10);
        const y: number = parseInt(text[1][0].replace("+", ""), 10);
        return { x, y };
    }

    const method_substitute = (buttonA: Button, buttonB: Button, prize: Button): number => {
        const Ax: number = buttonA.x;
        const Ay: number = buttonA.y;
        const Bx: number = buttonB.x;
        const By: number = buttonB.y;
        const Px: number = prize.x;
        const Py: number = prize.y;

        const b: number = Math.floor((Ay * Px - Ax * Py) / (Ay * Bx - Ax * By));
        const a: number = Math.floor((Py - By * b) / Ay);

        const xOk: boolean = (Ax * a + Bx * b) === Px;
        const yOk: boolean = (Ay * a + By * b) === Py;

        if (!xOk || !yOk) {
            return -1;
        }
        return 3 * a + b;
    }

    let result: number = 0;
    let result2: number = 0;
    const offset: number = 10 ** 13;

    for (let i = 0; i < commandes.length; i+= 4) {
        const button1: Button = parse_data(commandes[i]);
        const button2: Button = parse_data(commandes[i + 1]);
        const price: Button = parse_data(commandes[i + 2], true);

        result += method_substitute(button1, button2, price);
        result2 += method_substitute(button2, button1, { x: price.x + offset, y: price.y + offset });
    }

    console.log(`Partie 1 : ${result}`);
    console.log(`Partie 2 : ${result2}`);

    return result;
}