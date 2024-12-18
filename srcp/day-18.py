from collections import deque

from srcp.utils.input import get_data

data: str = get_data(2024, 18)

# X par rapport au bord de gauche
# Y par rapport au bord du haut
size = 71

bytes_positions: list[list[int]] = [list(map(int, x.split(','))) for x in data.splitlines()]
grid: list[list[str]] = [['.' for _ in range(size)] for _ in range(size)]


def is_valid(x: int, y: int, g: list[list[str]], s: int = 71):
    return 0 <= x < s and 0 <= y < s and g[y][x] == '.'


def shortest_path(grid: list[list[str]], start: tuple[int, int], goal: tuple[int, int]) -> int:
    queue = deque([(start, 0)])  # ma structure => (position (col, row), distance)
    visited = set()
    visited.add(start)

    while queue:
        (x, y), steps = queue.popleft()

        if (x, y) == goal:
            return steps

        for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
            nx, ny = x + dx, y + dy
            if is_valid(nx, ny, grid) and (nx, ny) not in visited:
                visited.add((nx, ny))
                queue.append(((nx, ny), steps + 1))
    return -1


start: tuple[int, int] = (0, 0)
end: tuple[int, int] = (70, 70)
current_position: tuple[int, int] = start

# Partie 1 : Générer la grille des 1024 premier bytes
# objectif même chose que depuis quelques semaile chercher le chemin de sortie
for i, (x, y) in enumerate(bytes_positions[:1024]):
    grid[y][x] = '#'

print(f"Partie 1 : {shortest_path(grid, current_position, end)} pas.")


# Partie 2 pareil que part 1 sauf que des que la steps est -1 j'ajoute le block bloquant
# Pas de blocage part 2 pas besoin de reset la grille
blocking_byte: tuple[int, int]|None = None

for i, (x, y) in enumerate(bytes_positions):
    grid[y][x] = '#'
    steps = shortest_path(grid, start, end)
    if steps == -1:
        blocking_byte = (x, y)
        break

if blocking_byte:
    print(f"Partie 2, le premier block qui bloque : {blocking_byte[0]},{blocking_byte[1]}")
else:
    print("Partie 2, aucun block bloquant.")