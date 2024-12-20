from collections import deque, defaultdict

from srcp.utils.input import get_data


def parse_map(inpt: str):
    g = [list(r) for r in inpt.strip().splitlines()]
    s, e = None, None
    for ri, row in enumerate(g):
        for ci, cell in enumerate(row):
            if cell == 'S':
                s = (ri, ci)
            elif cell == 'E':
                e = (ri, ci)
    return g, s, e


# Génération d'une grille de distances en partant de la fin
def bfs_from_end(g: list[list[str]], e: tuple[int, int]) -> list[list[float]]:
    rows, cols = len(g), len(g[0])
    distances = [[float('inf')] * cols for _ in range(rows)]
    queue = deque([e])
    distances[e[0]][e[1]] = 0
    directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    while queue:
        x, y = queue.popleft()
        for dx, dy in directions:
            nx, ny = x + dx, y + dy
            if 0 <= nx < rows and 0 <= ny < cols and g[nx][ny] != '#' and distances[nx][ny] == float('inf'):
                distances[nx][ny] = distances[x][y] + 1
                queue.append((nx, ny))
    return distances


def find_path(g, s: tuple[int, int], e_d: list[list[float]], e: tuple[int, int]) -> list[tuple[int, int]]:
    queue = deque([(s, [])])
    visited = set()
    visited.add(s)

    directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]

    while queue:
        (x, y), p = queue.popleft()
        new_p = p + [(x, y)]

        if (x, y) == e:
            return new_p

        for dx, dy in directions:
            nx, ny = x + dx, y + dy
            if 0 <= nx < len(g) and 0 <= ny < len(g[0]) and g[nx][ny] != '#':
                next_pos = (nx, ny)
                if next_pos not in visited and e_d[nx][ny] < e_d[x][y]:
                    visited.add(next_pos)
                    queue.append((next_pos, new_p))
    return []


# Example map
map_text = get_data(2024, 20)

grid, start, end = parse_map(map_text.strip())
end_distances: list[list[float]] = bfs_from_end(grid, end)
path: list[tuple[int, int]] = find_path(grid, start, end_distances, end)

cheats = defaultdict(int)
max_cheat_length = 20
min_benefit = 100

for i in range(len(path)):
    for j in range(i + 1, len(path)):
        a, b = path[i], path[j]
        manhattan_distance = abs(a[0] - b[0]) + abs(a[1] - b[1])

        if manhattan_distance > max_cheat_length:
            continue

        dir_path_length = manhattan_distance
        distance_b_to_end = end_distances[b[0]][b[1]]

        if distance_b_to_end + dir_path_length >= end_distances[a[0]][a[1]]:
            continue

        benefit = end_distances[a[0]][a[1]] - (distance_b_to_end + dir_path_length)

        if benefit >= min_benefit:
            cheats[benefit] += 1

print(sum(cheats.values()))
