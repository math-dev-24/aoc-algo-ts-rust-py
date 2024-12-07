import time
from srcp.utils.input import get_data

data = get_data(2024, 4).strip()
start = time.time()

result_1 = result_2 = 0
grid: list[list[str]] = [list(line) for line in data.split("\n")]
rows, cols = len(lines := data.split()), len(lines)

list_x: list[tuple[int, int]] = []
list_center: list[tuple[int, int]] = []
list_dir: tuple[int, ...] = (-1, 0, 1)
word: str = "XMAS"

for r in range(rows):
    for c in range(cols):
        list_x += [(r, c)] if grid[r][c] == "X" else []
        list_center += [(r, c)] if grid[r][c] == "A" else []

for rx, cx in list_x:
    for di in list_dir:
        for dj in list_dir:
            result_1 += all(
                0 <= (ty := rx + k * di) < rows and
                0 <= (tx := cx + k * dj) < cols and
                grid[ty][tx] == c
                for k, c in enumerate(word)
            )

print(f"Result 1: {result_1} in {time.time() - start:.2f} s")
time_int = time.time()

for ca, ra in list_center:
    if (
            (xl := ra - 1) >= 0 and # L pour left
            (xr := ra + 1) < cols and # R pour right
            (yt := ca - 1) >= 0 and # T pour top
            (yb := ca + 1) < rows # B pour bottom
    ):
        diag_1 = {grid[yt][xl], grid[yb][xr]}
        diag_2 = {grid[yt][xr], grid[yb][xl]}

        result_2 += (diag_1 == diag_2 == set("MS"))

print(f"Result 2: {result_2} in {time.time() - time_int:.2f} s")
