export const solve_day_2 = (input: string): number => {
    const data_day_2: string[] = input.trim().split("\n");


    let result_ok: number = 0;

// Algo : O(n2)
    for (const line of data_day_2) {
        const tmp_line: number[] = line.split(" ").map((num: string) => parseInt(num, 10));

        const is_valid = (arr: number[]): {validity: boolean, error: number} => {
            const direction: 1|-1 = arr[1] > arr[0] ? 1 : -1;
            let error: number = 0;

            for (let i = 1; i < arr.length; i++) {
                const delta: number = Math.abs(arr[i] - arr[i - 1]);
                const currentDirection: 1|-1 = arr[i] > arr[i - 1] ? 1 : -1;

                if (delta < 1 || delta > 3 || currentDirection !== direction) {
                    error++;
                    if (error > 1) return {validity: false, error}
                }
            }
            return {validity: error == 0, error}
        };

        const state = is_valid(tmp_line);

        if(state.validity || state.error == 1 ) {
            result_ok++;
        }
    }
    console.log("Quantité de safe : ", result_ok);

    return result_ok;
}

