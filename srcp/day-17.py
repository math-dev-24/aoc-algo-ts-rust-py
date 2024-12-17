from srcp.utils.input import get_data
import re
from concurrent.futures import ProcessPoolExecutor

data = get_data(2024, 17)

lines: list[str] = data.splitlines()

register_1: int = int(re.compile(r'Register A: (\d+)').findall(lines[0])[0])
register_2: int = int(re.compile(r'Register B: (\d+)').findall(lines[1])[0])
register_3: int = int(re.compile(r'Register C: (\d+)').findall(lines[2])[0])
# Programme : [2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 2, 5, 5, 3, 0]
program = list(map(int, re.compile(r'\d+').findall(lines[4])))
target_output = ",".join(map(str, program))

opcode: dict = {0: 'adv', 1: 'bxl', 2: 'bst', 3: 'jnz', 4: 'bxc', 5: 'out', 6: 'bdv', 7: 'cdv'}


def get_combo(op: int, a: int, b: int, c: int) -> int:
    if op == 4:
        return a
    elif op == 5:
        return b
    elif op == 6:
        return c
    return op


def dv(register: int, op: int, a, b, c) -> int:
    return register // (2 ** get_combo(op, a, b, c))


def get_output(prog: list[int], a: int = 0, b: int = 0, c: int = 0, part_2: bool = False) -> str:
    pointer: int = 0
    outs: list[int] = []
    visited_states = set()
    while pointer < len(prog):
        state = (pointer, a, b, c, len(outs))
        if state in visited_states:
            return ",".join(map(str, outs))

        visited_states.add(state)
        instruction: str = opcode[prog[pointer]]
        operand = prog[pointer + 1]
        combo = get_combo(operand, a, b, c)
        match instruction:
            case 'adv':
                a = dv(a, operand, a, b, c)
            case 'bxl':
                b ^= operand
            case 'bst':
                b = combo % 8
            case 'jnz':
                if a != 0:
                    pointer = operand
                    continue
            case 'bxc':
                b ^= c
            case 'out':
                outs.append(combo % 8)
                if part_2 and outs[-1] != int(prog[len(outs) - 1]):
                    return ",".join(map(str, outs))
            case 'bdv':
                b = dv(a, operand, a, b, c)
            case 'cdv':
                c = dv(a, operand, a, b, c)
        pointer += 2
    return ",".join(map(str, outs))


# partie 1
outputs = get_output(program, register_1, register_2, register_3)
print(outputs)


# Partie 2
# brut force
def find_potential_a(s: int, e: int) -> list[int]:
    results = []
    for a in range(s, e + 1):
        b2 = (a % 8) ^ 3
        c = a // (2 ** b2)
        b_val = (((a % 8) ^ 3) ^ 5) ^ c
        if b_val % 8 == program[0]:
            results.append(a)
    return results


is_ok: bool = False
start: int = 0
end: int = 1000
delta: int = 100
f: int = 0

while True:
    potential_a: list[int] = find_potential_a(start + f*delta, end + f*delta)
    print(len(potential_a))
    for a in potential_a:
        output: str = get_output(program, a, register_2, register_3, part_2=True)
        if output == target_output:
            is_ok = True
            print(f"A = {a}")
            break


if not is_ok:
    print("Pas trouvé")
