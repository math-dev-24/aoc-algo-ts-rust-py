# from srcp.utils.input import get_data
import re
from math import floor

data = """
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"""

commandes = data.strip().split('\n')


def get_info(tmp_c: str) -> tuple[int, int]:
    match = re.findall(r'\d+', tmp_c)
    return int(match[0]), int(match[1])


def solve_machine(buttonA: tuple[int, int], buttonB: tuple[int, int], prize: tuple[int, int]) -> int:
    Ax, Ay = buttonA
    Bx, By = buttonB
    Px, Py = prize

    min_cost = float('inf')
    for nA in range(101):
        for nB in range(101):
            move_x = nA * Ax + nB * Bx
            move_y = nA * Ay + nB * By
            if move_x == Px and move_y == Py:
                print(f"nA = {nA}, nB = {nB}")
                cost = 3 * nA + 1 * nB
                min_cost = min(min_cost, cost)

    return min_cost if min_cost != float('inf') else -1


def solve_machine_optimized(buttonA: tuple[int, int], buttonB: tuple[int, int], prize: tuple[int, int]) -> int:
    Ax, Ay = buttonA
    Bx, By = buttonB
    Px, Py = prize

    b = (Ay * Px - Ax * Py) / (Ay * Bx - Ax * By)
    a = (Py - By * b) / Ay


    b = floor(b)
    a = floor(a)
    print(f"a = {a}, b = {b}")

    xOk = (Ax * a + Bx * b) == Px
    yOk = (Ay * a + By * b) == Py

    print(f"xOk = {xOk}, yOk = {yOk}")
    if not xOk or not yOk:
        return -1

    return 3 * a + 1 * b

OFFSET = 10 ** 13

results_part_1 = []
results_part_2 = []
for i in range(0, len(commandes), 4):
    buttonA = get_info(commandes[i])
    buttonB = get_info(commandes[i + 1])
    prize = get_info(commandes[i + 2])
    prize_part_2 = (prize[0] + OFFSET, prize[1] + OFFSET)

    print(f"Commande {(i / 4) + 1} : {commandes[i]}")

    cost = solve_machine(buttonA, buttonB, prize)
    cost_m2 = solve_machine_optimized(buttonA, buttonB, prize_part_2)

    print(f"Cost (Part 1): {cost}")
    print(f"Cost (Part 2): {cost_m2}")
    if cost != -1:
        results_part_1.append(cost)
    if cost_m2 != -1:
        results_part_2.append(cost_m2)

print(f"Partie 1 : Nombre de prix gagnés : {len(results_part_1)} - Coût total minimal : {sum(results_part_1)}")
print(f"Partie 2 : Nombre de prix gagnés : {len(results_part_2)} - Coût total minimal : {sum(results_part_2)}")