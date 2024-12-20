from srcp.utils.input import get_data
from collections import deque


data = get_data(2024, 20)

lines = data.splitlines()


def parse_map(racetrack: list[str]):
    start, end = None, None
    grid: list[list[str]] = []
    for y, line in enumerate(racetrack):
        grid.append(list(line))
        if "S" in line:
            start = (y, line.index("S"))
        if "E" in line:
            end = (y, line.index("E"))
    return grid, start, end


def is_within_bounds(grid: list[list[str]], x: int, y: int):
    return 0 <= x < len(grid) and 0 <= y < len(grid[0])


def bfs(grid: list[list[str]], start: tuple[int, int], end: tuple[int, int], cheat=False):
    directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
    queue = deque([(start, 0, False)])  # (position, steps, cheat_used)
    visited = set()
    visited.add((start, False))

    while queue:
        (x, y), steps, cheat_used = queue.popleft()
        if (x, y) == end:
            return steps

        for dx, dy in directions:
            nx, ny = x + dx, y + dy
            if is_within_bounds(grid, nx, ny):
                # Déplacement normal
                if grid[nx][ny] in (".", "E") and ((nx, ny), cheat_used) not in visited:
                    visited.add(((nx, ny), cheat_used))
                    queue.append(((nx, ny), steps + 1, cheat_used))
                # Traversée de mur (uniquement si on a encore la possibilité de tricher)
                elif grid[nx][ny] == "#" and not cheat_used and cheat:
                    for ddx, ddy in directions:
                        nnx, nny = nx + ddx, ny + ddy
                        if is_within_bounds(grid, nnx, nny) and grid[nnx][nny] in (".", "E"):
                            if ((nnx, nny), True) not in visited:
                                visited.add(((nnx, nny), True))
                                queue.append(((nnx, nny), steps + 2, True))
    return float("inf")


def calculate_cheats(racetrack: list[str]):
    grid, start, end = parse_map(racetrack)
    no_cheat_time = bfs(grid, start, end)
    cheat_savings = []

    for x in range(len(grid)):
        for y in range(len(grid[0])):
            if grid[x][y] == "#":
                grid[x][y] = "."  # Temporarily remove the wall
                cheat_time = bfs(grid, start, end, cheat=True)
                if cheat_time < no_cheat_time:
                    cheat_savings.append(no_cheat_time - cheat_time)
                grid[x][y] = "#"
    return cheat_savings


def count_cheats_saving_time(racetrack, min_saving):
    cheat_savings = calculate_cheats(racetrack)
    return sum(1 for saving in cheat_savings if saving >= min_saving)


result = count_cheats_saving_time(lines, 100)
print("Nombre de triches économisant au moins 100 picosecondes :", result)