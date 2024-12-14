interface Robot {
    position: Coordinate;
    velocity: Coordinate;
}

interface Coordinate {
    x: number;
    y: number;
}


export function solve_day_14(input: string): number {

    const parse_robot = (s: string): Robot => {
        const regex: RegExp = /-?\d+,-?\d+/g;
        const match = [...s.matchAll(regex)];
        console.log(match);
        return {
            position: {
                x: parseInt(match[0][0].split(",")[0], 10),
                y: parseInt(match[0][0].split(",")[1], 10),
            },
            velocity: {
                x: parseInt(match[1][0].split(",")[0], 10),
                y: parseInt(match[1][0].split(",")[1], 10),
            }
        };
    }



    const robots: Robot[] = input.trim().split("\n").map(parse_robot);

    console.log(robots);

    return 0;
}