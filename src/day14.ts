interface Robot {
    position: Coordinate;
    velocity: Coordinate;
}

interface Coordinate {
    x: number;
    y: number;
}

enum Quadrant {
    TOP_LEFT= "top_left",
    TOP_RIGHT= "top_right",
    BOTTOM_LEFT= "bottom_left",
    BOTTOM_RIGHT= "bottom_right",
}

export function solve_day_14(input: string): number {
    const WIDTH: number = 101;
    const MID_X: number = Math.floor(WIDTH / 2);
    const HEIGHT: number = 103;
    const MID_Y: number = Math.floor(HEIGHT / 2);


    const parse_robot = (s: string): Robot => {
        const regex: RegExp = /-?\d+,-?\d+/g;
        const match = [...s.matchAll(regex)];
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

    const new_position = (robot: Robot): Coordinate => {
        let x_final: number = (robot.position.x + robot.velocity.x) % WIDTH;
        let y_final: number = (robot.position.y + robot.velocity.y) % HEIGHT;
        x_final = (x_final + WIDTH) % WIDTH;
        y_final = (y_final + HEIGHT) % HEIGHT;
        return { x: x_final, y: y_final };
    };

    const robots: Robot[] = input.trim().split("\n").map(parse_robot);

    const robots_final: Robot[] = robots.map(robot => {
        return { ...robot, position: new_position(robot) };
    });

    const quadrants: Record<Quadrant, number> = {
        [Quadrant.TOP_LEFT]: 0,
        [Quadrant.TOP_RIGHT]: 0,
        [Quadrant.BOTTOM_LEFT]: 0,
        [Quadrant.BOTTOM_RIGHT]: 0,
    };

    for (const robot of robots_final) {
        const { x, y } = robot.position;
        if (x === MID_X || y === MID_Y) {
            continue;
        }
        if (x < MID_X && y < MID_Y) {
            quadrants[Quadrant.TOP_LEFT] += 1;
        } else if (x >= MID_X && y < MID_Y) {
            quadrants[Quadrant.TOP_RIGHT] += 1;
        } else if (x < MID_X && y >= MID_Y) {
            quadrants[Quadrant.BOTTOM_LEFT] += 1;
        } else if (x >= MID_X && y >= MID_Y) {
            quadrants[Quadrant.BOTTOM_RIGHT] += 1;
        }
    }

    const safety_factor: number = quadrants[Quadrant.TOP_LEFT] * quadrants[Quadrant.TOP_RIGHT] * quadrants[Quadrant.BOTTOM_LEFT] * quadrants[Quadrant.BOTTOM_RIGHT];

    console.log(quadrants)
    console.log(`Résultat : ${safety_factor}`);

    // partie 2, recherche de l'ester eags
    let actual_state: Set<string> = new Set();
    let time_s: number = 0;

    while(true) {
        time_s += 1;
        const points: string[] = [];
        for(const robot of robots) {
            const tmp_pos: Coordinate = new_position(robot);
            robot.position = {...tmp_pos};
            points.push(`${robot.position.x},${robot.position.y}`);
        }

        if(points.length === new Set(points).size) {
            break;
        }
        const state: string = robots
            .map(robot => `${robot.position.x},${robot.position.y}`)
            .join(",");

        if (actual_state.has(state)) {
            break;
        }
        actual_state.add(state);
    }
    console.log(`Temps : ${time_s}`);


    return 0;
}