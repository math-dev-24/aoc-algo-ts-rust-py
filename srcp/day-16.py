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

    def print_grid(self):
        for row in self.grid:
            print("".join(row))

    def find_shortest_path(self) -> (int, set):
        # queue : tuple(cost, row, col, direction, path)
        queue = [(0, self.start[0], self.start[1], 0, [])]
        visited = {}
        successful_paths_cells = set()
        best_paths = []
        min_cost = float('inf')

        while queue:
            cost, row, col, direction, path = heappop(queue)

            if (row, col, direction) in visited and visited[(row, col, direction)] < cost:
                continue

            visited[(row, col, direction)] = cost
            new_path = [*path, (row, col)]

            if (row, col) == self.end:
                if cost < min_cost:
                    min_cost = cost
                    best_paths = [new_path]
                elif cost == min_cost:
                    best_paths.append(new_path)
                continue

            dr, dc = self.directions[direction]
            new_row, new_col = row + dr, col + dc

            if self.is_valid(new_row, new_col):
                heappush(queue, (cost + 1, new_row, new_col, direction, new_path))

            # check rotate
            heappush(queue, (cost + 1000, row, col, (direction + 1) % 4, new_path))
            heappush(queue, (cost + 1000, row, col, (direction - 1) % 4, new_path))

        for path in best_paths:
            successful_paths_cells.update(path)
        # Je dessine ma grille avec les postes de repos
        for (row, col) in successful_paths_cells:
            self.grid[row][col] = "O"

        return min_cost, successful_paths_cells


if __name__ == "__main__":
    input = get_data(2024, 16)
    grid = [list(row) for row in input.strip().splitlines()]
    solver = Dijkstra(grid)
    result = solver.find_shortest_path()
    print(f"Partie 1 : {result[0]}")
    print(f"Partie 2 : {len(result[1])}")
    solver.print_grid()
