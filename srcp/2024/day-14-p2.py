import re
import matplotlib.pyplot as plt
from srcp.utils.input import get_data

WIDTH: int = 101
HEIGHT: int = 103

data: str = get_data(day=14, year=2024)

# Parsing des données
robots: list[dict[str, list[int]]] = []

for line in data.strip().split('\n'):
    match = re.findall(r'-?\d+,-?\d+', line)
    position: list[int] = list(map(int, match[0].split(',')))
    velocity: list[int] = list(map(int, match[1].split(',')))
    robots.append({"p": position, "v": velocity})


def next_position(r: dict[str, list[int]]):
    x_final = (r["p"][0] + r["v"][0]) % WIDTH
    y_final = (r["p"][1] + r["v"][1]) % HEIGHT
    return x_final, y_final


def part_2(robots: list[dict[str, list[int]]]) -> int:
    states = set()
    time_s = 0

    while True:
        time_s += 1

        # Mettre à jour les positions
        for robot in robots:
            p, v = robot["p"], robot["v"]
            p[0], p[1] = next_position(robot)

        points = [tuple(robot["p"]) for robot in robots]

        # Break si les positions sont uniques
        if len(set(points)) == len(points):
            break

        # Break si on revient à un état précédent
        s = frozenset(points)
        if s in states:
            break

        states.add(s)

    # Tracer les points
    fig, ax = plt.subplots(figsize=(10, 6))
    for x, y in points:
        ax.plot(x, y, "s", color="green")

    ax.set_xlim(0, WIDTH)
    ax.set_ylim(0, HEIGHT)
    ax.invert_yaxis()
    ax.set_title(f"{time_s} seconds")
    plt.savefig("sapin.png")
    plt.close()

    return time_s


steps: int = part_2(robots)
print(f"Part 2: {steps}")
