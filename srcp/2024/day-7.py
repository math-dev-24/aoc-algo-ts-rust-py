from srcp.utils.input import get_data
from itertools import product
from operator import add, mul
import time

start_time = time.time()
unresolved = []

data = get_data(2024, 7)

equations = tuple(tuple(map(int, line.split())) for line in data.strip().replace(":", "").splitlines())


def solve(equation, operator_set, retain_unsolvable=False):
    result, *operands = equation
    for operand in product(operator_set, repeat=len(operands) - 1):
        a = operands[0]
        for b, op in zip(operands[1:], operand):
            a = op(a, b)
        if a == result:
            return result
    if retain_unsolvable:
        unresolved.append(equation)
    return 0


def join(a, b):
    # return int(str(a) + str(b)) fait perdre 2 secondes
    k = 10
    while k < b:
        k *= 10
    return k * a + b


part_1 = sum(solve(e, operator_set=(add, mul), retain_unsolvable=True) for e in equations)
print(f"Partie 1: {part_1} en {time.time() - start_time}s")

part_2 = part_1 + sum(solve(e, operator_set=(add, mul, join)) for e in unresolved)
print(f"Partie 2: {part_2} en {time.time() - start_time}s")
