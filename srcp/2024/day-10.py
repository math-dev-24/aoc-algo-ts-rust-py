from srcp.utils.input import get_data


def explore_distinct_trails(grid: list[list[int]], x: int, y: int, visited: set) -> int:
    if grid[x][y] == 9:
        return 1

    visited.add((x, y))
    total_paths = 0

    for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        nx, ny = x + dx, y + dy
        if (
                0 <= nx < len(grid) and
                0 <= ny < len(grid[0]) and
                grid[nx][ny] == grid[x][y] + 1 and
                (nx, ny) not in visited
        ):
            total_paths += explore_distinct_trails(grid, nx, ny, visited)

    visited.remove((x, y))
    return total_paths


def calculate_rating(grid: list[list[int]]) -> int:
    trailheads = [(x, y) for x in range(len(grid)) for y in range(len(grid[0])) if grid[x][y] == 0]
    return sum(explore_distinct_trails(grid, x, y, set()) for x, y in trailheads)


grid = [list(map(int, i)) for i in get_data(2024, 10).strip().splitlines()]
print(calculate_rating(grid))
