from srcp.utils.input import get_data

data = get_data(2024, 15)

def part2(data):
    grid = []
    moves = ""
    for line in data.splitlines():
        if line.startswith("#"):
            grid.append(line)
        else:
            moves += line.strip()

    rows = len(grid)
    cols = len(grid[0])

    # Scale the grid
    for r in range(rows):
        nl = []
        for c in grid[r]:
            if c == "#":
                nl.append("#")
                nl.append("#")
            elif c == "O":
                nl.append("[")
                nl.append("]")
            elif c == ".":
                nl.append(".")
                nl.append(".")
            else:  # '@'
                nl.append("@")
                nl.append(".")
        grid[r] = nl

    pos = None
    for r in range(rows):
        for c in range(cols * 2):
            if grid[r][c] == "@":
                pos = (r, c)
                grid[r][c] = "."
                break
        if pos is not None:
            break

    rows = len(grid)
    cols = len(grid[0])

    dir = {"^": (-1, 0), "v": (1, 0), "<": (0, -1), ">": (0, 1)}

    def try_move(grid, pos, dpos):
        x, y = pos
        dx, dy = dpos
        nx, ny = x + dx, y + dy
        if grid[nx][ny] == "#":
            return False
        elif grid[nx][ny] == ".":
            return True
        if dy == 0: # push up or down
            if grid[nx][ny] == "]":
                return try_move(grid, (nx, ny), dpos) and try_move(
                    grid, (nx, ny - 1), dpos
                )
            elif grid[nx][ny] == "[":
                return try_move(grid, (nx, ny), dpos) and try_move(
                    grid, (nx, ny + 1), dpos
                )
        elif dy == -1:  # push left
            if grid[nx][ny] == "]":
                return try_move(grid, (nx, ny - 1), dpos)
        elif dy == 1:  # push right
            if grid[nx][ny] == "[":
                return try_move(grid, (nx, ny + 1), dpos)
        return False

    def move(grid, pos, dpos):
        x, y = pos
        dx, dy = dpos
        nx, ny = x + dx, y + dy
        if grid[nx][ny] == "#":
            return
        elif grid[nx][ny] == ".":
            grid[x][y], grid[nx][ny] = grid[nx][ny], grid[x][y]
            return
        if dy == 0:
            if grid[nx][ny] == "]":
                move(grid, (nx, ny), dpos)
                move(grid, (nx, ny - 1), dpos)
                grid[x][y], grid[nx][ny] = grid[nx][ny], grid[x][y]
            elif grid[nx][ny] == "[":
                move(grid, (nx, ny), dpos)
                move(grid, (nx, ny + 1), dpos)
                grid[x][y], grid[nx][ny] = grid[nx][ny], grid[x][y]
        elif dy == -1:  # push left
            if grid[nx][ny] == "]":
                move(grid, (nx, ny - 1), dpos)
                grid[nx][ny - 1], grid[nx][ny], grid[x][y] = (
                    grid[nx][ny],
                    grid[x][y],
                    grid[nx][ny - 1],
                )
        elif dy == 1:  # push right
            if grid[nx][ny] == "[":
                move(grid, (nx, ny + 1), dpos)
                grid[nx][ny + 1], grid[nx][ny], grid[x][y] = (
                    grid[nx][ny],
                    grid[x][y],
                    grid[nx][ny + 1],
                )

    for mv in moves:
        dpos = dir[mv]
        if try_move(grid, pos, dpos):
            move(grid, pos, dpos)
            pos = (pos[0] + dpos[0], pos[1] + dpos[1])

    total = 0
    for r in range(rows):
        for c in range(cols):
            if grid[r][c] == "[":
                total += 100 * r + c
    return total


print(part2(data))