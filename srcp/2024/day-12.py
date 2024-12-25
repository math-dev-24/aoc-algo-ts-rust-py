from collections import deque, defaultdict
from random import randint
from turtle import *

input = """
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"""

lines: list[str] = input.strip().splitlines()
grid: list[list[str]] = [ list(map(str, l)) for l in lines]
h = len(lines)
w = len(lines[0])

total_part_1: int = 0
total_part_2: int = 0

areas: list[list[str]] = [["." for _ in range(w + 2)] for _ in range(h + 2)]
dirs: list[tuple[int, int]] = [(0, 1), (1, 0), (0, -1), (-1, 0)]
diags: list[tuple[int, int]] = [(1, 1), (1, -1), (-1, 1), (-1, -1)]

for y in range(1, h + 1):
    for x in range(1, w + 1):
        areas[y][x] = lines[y - 1][x - 1]

visited: set[tuple[int, int]] = set()

for y in range(1, h + 1):
    for x in range(1, w + 1):
        if (y, x) in visited:
            continue

        area: int = 0
        perimetre: int = 0
        side: int = 0

        queue = deque()
        queue.append((y, x))

        plant: str = areas[y][x]

        while queue:
            cy, cx = queue.popleft()
            if (cy, cx) in visited:
                continue

            visited.add((cy, cx))
            voisin: int = 0

            for dy, dx in dirs:
                ny: int = cy + dy
                nx: int = cx + dx
                if areas[ny][nx] == plant:
                    voisin += 1
                    if (ny, nx) not in visited:
                        queue.append((ny, nx))

            # Vérification des diagonales pour compter les côtés
            for dy, dx in diags:
                ny: int = cy + dy
                nx: int = cx + dx
                # Angle extérieur
                if areas[ny][nx] != plant and areas[ny][cx] != plant and areas[cy][nx] != plant:
                    side += 1
                # Angle intérieur
                if areas[ny][nx] != plant and areas[ny][cx] == plant and areas[cy][nx] == plant:
                    side += 1
                # Diagonal
                if areas[ny][nx] == plant and areas[ny][cx] != plant and areas[cy][nx] != plant:
                    side += 1

            area += 1
            perimetre += 4 - voisin

        total_part_1 += area * perimetre
        total_part_2 += area * side


# Compter les côtés barre par barre des plantes
print(f"Part 1: {total_part_1}")
# Compter que les côtés
print(f"Part 2: {total_part_2}")

#partie affichage avec turtle
dimension: int = 600
size_block: int = dimension / w

screen = Screen()
screen.setup(width=dimension + 50, height=dimension + 50)
screen.tracer(0)

turtle = Turtle()
turtle.speed("fastest")
turtle.penup()

def random_color():
    return f"#{randint(0, 255):02x}{randint(0, 255):02x}{randint(0, 255):02x}"

color_map = defaultdict(random_color)

def draw_square(x: int, y: int, size: int, color: str)-> None:
    turtle.goto(x, y)
    turtle.fillcolor(color)
    turtle.begin_fill()
    for _ in range(4):
        turtle.forward(size)
        turtle.left(90)
    turtle.end_fill()

start_x = -dimension / 2 + (dimension - size_block * w) / 2
start_y = dimension / 2 - (dimension - size_block * h) / 2

for i, row in enumerate(grid):
    for j, cell in enumerate(row):
        color = color_map[cell]
        x = start_x + j * size_block
        y = start_y - i * size_block
        draw_square(x, y, size_block, color)

screen.update()
screen.mainloop()