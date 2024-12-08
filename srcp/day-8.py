from itertools import combinations
from srcp.utils.input import get_data

test = get_data(2024, 8)

lines = test.strip().replace("#", ".").splitlines()

grid = [list(map(str, line)) for line in lines]

list = {}

for row in range(len(lines)):
    chars = [(col, char) for (col, char) in enumerate(lines[row]) if char != '.']
    if chars:
        for col, char in chars:
            if char not in list:
                list[char] = []
            list[char].append((row, col))


anti_nodes = set()

#print(list)

for frequency, antennes in list.items():
    # Pour la partie 2 compter les entennes
    if len(antennes) > 1:
        anti_nodes.update(antennes)

    for a1, a2 in combinations(antennes, 2):
        y1, x1 = a1
        y2, x2 = a2
        # DX et DY sont la distance entre les deux points
        dx, dy = x2 - x1, y2 - y1

        #print(f"dx: {dx}, dy: {dy}")

        factor = 1
        while True:
            scaled_dx = dx * factor
            scaled_dy = dy * factor

            n1 = (y1 - scaled_dy, x1 - scaled_dx)
            n2 = (y2 + scaled_dy, x2 + scaled_dx)

            # print(f"n1: {n1}, n2: {n2}")

            if (
                not (0 <= n1[0] < len(grid) and 0 <= n1[1] < len(grid[0]))
                and not (0 <= n2[0] < len(grid) and 0 <= n2[1] < len(grid[0]))
            ):
                break

            if (
                0 <= n1[0] < len(grid) and
                0 <= n1[1] < len(grid[0]) and
                n1 not in antennes
            ):
                anti_nodes.add(n1)
            if (
                0 <= n2[0] < len(grid) and
                0 <= n2[1] < len(grid[0]) and
                n2 not in antennes
            ):
                anti_nodes.add(n2)
            factor += 1

print(len(anti_nodes))


