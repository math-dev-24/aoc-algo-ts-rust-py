import re
import matplotlib
import matplotlib.pyplot as plt
from srcp.utils.input import get_data

matplotlib.use('macOSX')


def solve(robots):
    w, h = 101, 103
    states = set()
    answer = 0

    while True:
        answer += 1

        for robot in robots:
            p = robot["p"]
            v = robot["v"]

            p[0] += v[0]
            while p[0] < 0:
                p[0] += w
            while p[0] >= w:
                p[0] -= w

            p[1] += v[1]
            while p[1] < 0:
                p[1] += h
            while p[1] >= h:
                p[1] -= h

        pt = [tuple(robot["p"]) for robot in robots]
        if len(set(pt)) == len(pt):
            break

        s = frozenset(pt)
        if s in states:
            break

        states.add(s)

    fig, ax = plt.subplots(figsize=(10, 6))
    for i in range(h):
        for j in range(w):
            if (j, i) in pt:
                ax.plot(j, i, "s", color="blue")

    ax.set_xlim(0, w)
    ax.set_ylim(0, h)
    ax.invert_yaxis()
    ax.set_title(f"{answer} seconds")
    plt.savefig("rp.png")
    plt.close()

    return answer


data: str = get_data(2024, 14)

robots = []


def parse_robot(r: str):
    match = re.findall(r'-?\d+,-?\d+', r)
    position = match[0].split(',')
    velocity = match[1].split(',')
    x, y = int(position[0]), int(position[1])
    vx, vy = int(velocity[0]), int(velocity[1])
    return {"p": [x, y], "v": [vx, vy]}


for line in data.strip().split('\n'):
    robots.append(parse_robot(line))

steps = solve(robots)
print(f"Part 2: {steps}")