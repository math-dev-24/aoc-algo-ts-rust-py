data = """
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"""
import time

start = time.time()

data = data.strip().splitlines()
grid: list[list[str]] = list(map(list, data))

turnOrder: list[str] = ['^', '>', 'v', '<']
positionInitGuard: list[int] = [0, 0]
directionInitGuard: str = '^'

for row in range(len(grid)):
    for col in range(len(grid[0])):
        cell = grid[row][col]
        if cell in turnOrder:
            positionInitGuard[0] = row
            positionInitGuard[1] = col
            directionInitGuard = cell
            break


def simulate_guard_patrol(grid: list[list[str]], rows: int, cols: int,
                          ini_position: list[int], init_dir: str,
                          detect_loop: bool = False) -> tuple[bool, set[str]]:
    directions = {
        '^': (-1, 0),
        '>': (0, 1),
        'v': (1, 0),
        '<': (0, -1),
    }

    turn_order = ['^', '>', 'v', '<']
    guard_pos = list(ini_position)
    guard_dir = init_dir

    visited = set()

    while True:
        r, c = guard_pos
        state = f"{r},{c},{guard_dir}" if detect_loop else f"{r},{c}"

        # Vérification de boucle
        if state in visited and detect_loop:
            return True, visited

        visited.add(state)

        dr, dc = directions[guard_dir]
        fr, fc = r + dr, c + dc

        if fr < 0 or fr >= rows or fc < 0 or fc >= cols:
            return False, visited

        front_cell = grid[fr][fc]

        if front_cell == "#" or front_cell == "O":
            current_dir_idx = turn_order.index(guard_dir)
            guard_dir = turn_order[(current_dir_idx + 1) % len(turn_order)]
        else:
            guard_pos = [fr, fc]


q_row: int = len(grid)
q_col: int = len(grid[0])
time_int = time.time() - start
totalPart1: [bool, set[str]] = simulate_guard_patrol(grid, q_row, q_col, positionInitGuard,directionInitGuard)
print(f"Part 1: {len(totalPart1[1])} en {time_int:.2f}s")

totalPart2: int = 0

for cell in totalPart1[1]:
    r, c = cell.split(",")
    if grid[int(r)][int(c)] == ".":
        grid[int(r)][int(c)] = "O"
        (is_loop, visited) = simulate_guard_patrol(grid, q_row, q_col, positionInitGuard, directionInitGuard, True)
        if is_loop:
            totalPart2 += 1
        grid[int(r)][int(c)] = "."


end = time.time()
print(f"Part 2: {totalPart2} en {(time_int + (end - start)):.2f}s")