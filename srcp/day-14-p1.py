import re
from collections import defaultdict

from srcp.utils.input import get_data

data: str = get_data(2024, 14)

robots = []
position_initial = set()


def parse_robot(r: str):
    match = re.findall(r'-?\d+,-?\d+', r)
    position = match[0].split(',')
    velocity = match[1].split(',')
    x, y = int(position[0]), int(position[1])
    vx, vy = int(velocity[0]), int(velocity[1])
    position_initial.add((x, y))
    return {'x': x, 'y': y, 'vx': vx, 'vy': vy}


for line in data.strip().split('\n'):
    robots.append(parse_robot(line))

width: int = 101
mid_x: int = width // 2

height: int = 103
mid_y: int = height // 2


position_final = defaultdict(int)


def calculate_final_position(r: dict[str, int], tmp_pos: int = 100) -> None:
    x_final = (r['x'] + r['vx'] * tmp_pos) % width
    y_final = (r['y'] + r['vy'] * tmp_pos) % height
    position_final[(x_final, y_final)] += 1


for r in robots:
    calculate_final_position(r)

quadrants = {
    "top_left": 0,
    "top_right": 0,
    "bottom_left": 0,
    "bottom_right": 0,
}

for (x, y), count in position_final.items():
    if x == mid_x or y == mid_y:
        continue
    # je check pour chaques quadrant
    if x < mid_x and y < mid_y:
        quadrants["top_left"] += count
    elif x >= mid_x and y < mid_y:
        quadrants["top_right"] += count
    elif x < mid_x and y >= mid_y:
        quadrants["bottom_left"] += count
    elif x >= mid_x and y >= mid_y:
        quadrants["bottom_right"] += count

safety_factor = (
        quadrants["top_left"]
        * quadrants["top_right"]
        * quadrants["bottom_left"]
        * quadrants["bottom_right"]
)

print("Nombre de robots par quadrant :")
for quadrant, count in quadrants.items():
    print(f"{quadrant}: {count} robots")

print(f"\nFacteur de sécurité : {safety_factor}")
