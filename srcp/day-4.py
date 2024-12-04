test = """
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"""

directions: list[tuple[int, int]] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
]

data = test.strip().split("\n")
grid: list[list[str]] = []

for line in data:
    grid.append(list(line))

count: int = 0
searchWord: str = "XMAS"


def is_word_found(start_row: int, start_col: int, dir_row: int, dir_col: int) -> bool:
    for i in range(len(searchWord)):
        new_row = start_row + i * dir_row
        new_col = start_col + i * dir_col

        if not (0 <= new_row < len(grid) and 0 <= new_col < len(grid[0])):
            return False

        if grid[new_row][new_col] != searchWord[i]:
            return False
    return True


for row in range(len(grid)):
    for col in range(len(grid[0])):
        for dir_row, dir_col in directions:
            if is_word_found(row, col, dir_row, dir_col):
                count += 1

print(count)
