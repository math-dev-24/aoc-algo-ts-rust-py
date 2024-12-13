from srcp.utils.input import get_data
from functools import lru_cache
import re

data = get_data(2024, 13)

commandes = data.strip().split('\n')


def get_info(tmp_c: str) -> tuple[int, int]:
    match = re.findall(r'\d+', tmp_c)
    return int(match[0]), int(match[1])


@lru_cache(maxsize=None)
def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
    if b == 0:
        return a, 1, 0
    gcd_val, x1, y1 = extended_gcd(b, a % b)
    x = y1
    y = x1 - (a // b) * y1
    return gcd_val, x, y


def solve_machine(buttonA: tuple[int, int], buttonB: tuple[int, int], prize: tuple[int, int]) -> int:
    Ax, Ay = buttonA
    Bx, By = buttonB
    Px, Py = prize

    gcd_x, x0_x, y0_x = extended_gcd(Ax, Bx)
    gcd_y, x0_y, y0_y = extended_gcd(Ay, By)

    # Vérification des divisibilités
    if Px % gcd_x != 0 or Py % gcd_y != 0:
        print(f"Machine invalide : Px={Px}, gcd_x={gcd_x}, Py={Py}, gcd_y={gcd_y}")
        return -1

    Ax, Bx, Px = Ax // gcd_x, Bx // gcd_x, Px // gcd_x
    Ay, By, Py = Ay // gcd_y, By // gcd_y, Py // gcd_y

    print(f"Réduction : Ax={Ax}, Bx={Bx}, Px={Px}, Ay={Ay}, By={By}, Py={Py}")

    x0_x = (Px * x0_x) % Bx
    x0_y = (Py * x0_y) % By

    def find_min_cost(A, B, x0, Px, cost_A, cost_B):
        min_cost = float('inf')
        for k in range(-10000, 10001):  # Étendre la plage
            nA = x0 + k * B
            nB = (Px - nA * A) // B
            if nA >= 0 and nB >= 0:
                min_cost = min(min_cost, nA * cost_A + nB * cost_B)
        return min_cost

    cost_x = find_min_cost(Ax, Bx, x0_x, Px, 3, 1)
    cost_y = find_min_cost(Ay, By, x0_y, Py, 3, 1)

    if cost_x == float('inf') or cost_y == float('inf'):
        return -1

    return cost_x + cost_y


OFFSET = 10 ** 13

results = []
for i in range(0, len(commandes), 4):
    buttonA = get_info(commandes[i])
    buttonB = get_info(commandes[i + 1])
    prize = get_info(commandes[i + 2])
    prize = (prize[0] + OFFSET, prize[1] + OFFSET)

    print(f"Commande {i / 4} : {commandes[i]}")
    print(f"need : {prize}")
    print(f"Button A: {buttonA}")
    print(f"Button B: {buttonB}")

    cost = solve_machine(buttonA, buttonB, prize)

    print(f"Cost: {cost}")
    if cost != -1:
        results.append(cost)

print(f"Nombre de machines : {len(commandes)}")
print(f"Nombre de prix gagnés : {len(results)}")
print(f"Coût total minimal : {sum(results)}")
