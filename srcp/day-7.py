data = """
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"""
from itertools import product
import time

operators = ["+", "*", "||"]


def evaluate_expression(numbers, ops):
    result = numbers[0]
    for i in range(len(ops)):
        if ops[i] == '+':
            result += numbers[i + 1]
        elif ops[i] == '*':
            result *= numbers[i + 1]
        elif ops[i] == '||':
            result = int(f"{result}{numbers[i + 1]}")
        else:
            raise ValueError(f"Unsupported operator: {ops[i]}")
    return result


def find_valid_combinations(numbers, target):
    operator_combinations = generate_operator_combinations(len(numbers) - 1)
    valid_expressions = []

    for ops in operator_combinations:
        try:
            result = evaluate_expression(numbers, ops)
            if result == target:
                expression = ' '.join(
                    f"{numbers[i]} {ops[i]}" if i < len(ops) else str(numbers[i])
                    for i in range(len(numbers))
                )
                valid_expressions.append(expression)
        except ValueError as e:
            # Ignore unsupported operator errors
            pass

    return valid_expressions


def generate_operator_combinations(quantity: int):
    return list(product(operators, repeat=quantity))

start_time = time.time()
total = 0
for line in data.strip().splitlines():
    tmp = line.split(":")
    target = int(tmp[0].strip())
    numbers = list(map(int, tmp[1].strip().split(" ")))

    nb_combinations = find_valid_combinations(numbers, target)
    if len(nb_combinations) > 0:
        total += target

print(f"Part 1: {total} en {time.time() - start_time}s")
