from collections import defaultdict
from itertools import combinations
from srcp.utils.input import get_data

test = get_data(2024, 8)

lines = test.strip().replace("#", ".").splitlines()

# dict (y, x) -> char
grid: dict = {(i, j): c for j, line in enumerate(lines) for i, c in enumerate(line)}


def get_antennas(tmp_grid: dict) -> dict:
    tmp_list = defaultdict(set)
    for yx, c in tmp_grid.items():
        if c != ".":
            tmp_list[c].add(yx)
    return tmp_list


list_antennas: dict = get_antennas(grid)
anti_nodes = set()

for _, antennas in list_antennas.items():
    if len(antennas) > 1:
        anti_nodes.update(antennas)

    for a1, a2 in combinations(antennas, 2):
        y1, x1 = a1
        y2, x2 = a2
        dx, dy = x2 - x1, y2 - y1

        factor = 1
        while True:
            def is_in_grid(node):
                return 0 <= node[0] < len(lines) and 0 <= node[1] < len(lines[0])

            scaled_dx = dx * factor
            scaled_dy = dy * factor

            n1 = (y1 - scaled_dy, x1 - scaled_dx)
            n2 = (y2 + scaled_dy, x2 + scaled_dx)

            if not is_in_grid(n1) and not is_in_grid(n2): break
            if is_in_grid(n1): anti_nodes.add(n1)
            if is_in_grid(n2): anti_nodes.add(n2)
            factor += 1

print(len(anti_nodes))
