from heapq import heappop, heappush
from srcp.utils.input import get_data


class Dijkstra:
    def __init__(self, grid: list[list[str]]):
        self.grid = grid
        self.rows = len(grid)
        self.cols = len(grid[0])
        self.start, self.end = self.find_points()
        self.directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]

    def find_points(self) -> tuple[tuple[int, int], tuple[int, int]]:
        start = end = None
        for r, row in enumerate(self.grid):
            for c, cell in enumerate(row):
                if cell == "S":
                    start = (r, c)
                elif cell == "E":
                    end = (r, c)
        return start, end

    def is_valid(self, row: int, col: int) -> bool:
        return 0 <= row < self.rows and 0 <= col < self.cols and self.grid[row][col] != "#"

    def find_shortest_path(self) -> int:
        queue = [(0, self.start[0], self.start[1], 0)]
        visited = {}

        while queue:
            cost, row, col, direction = heappop(queue)

            if (row, col, direction) in visited and visited[(row, col, direction)] <= cost:
                continue

            visited[(row, col, direction)] = cost

            if (row, col) == self.end:
                return cost

            dr, dc = self.directions[direction]
            new_row, new_col = row + dr, col + dc
            if self.is_valid(new_row, new_col):
                heappush(queue, (cost + 1, new_row, new_col, direction))

            # check rotate
            heappush(queue, (cost + 1000, row, col, (direction + 1) % 4))
            heappush(queue, (cost + 1000, row, col, (direction - 1) % 4))


if __name__ == "__main__":
    grid = [list(row) for row in get_data(2024, 16).splitlines()]
    solver = Dijkstra(grid)
    result = solver.find_shortest_path()
    print(f"Partie 1 : {result}")