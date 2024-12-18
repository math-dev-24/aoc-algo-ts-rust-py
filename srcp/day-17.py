from srcp.utils.input import get_data
import re

data = get_data(2024, 17)

lines: list[str] = data.splitlines()

register_1: int = int(re.compile(r'Register A: (\d+)').findall(lines[0])[0])
register_2: int = int(re.compile(r'Register B: (\d+)').findall(lines[1])[0])
register_3: int = int(re.compile(r'Register C: (\d+)').findall(lines[2])[0])
# Programme toujours : [2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 2, 5, 5, 3, 0]
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


def dv(register: int, op: int, a: int, b: int, c: int) -> int:
    return register >> get_combo(op, a, b, c)


def get_output(prog: list[int], a: int = 0, b: int = 0, c: int = 0) -> list[int]:
    pointer: int = 0
    outs: list[int] = []

    while pointer < len(prog):
        instruction: str = opcode[prog[pointer]]
        operand = prog[pointer + 1]
        combo = get_combo(operand, a, b, c)
        match instruction:
            case 'adv':
                a = dv(a, operand, a, b, c)
            case 'bxl':
                b ^= operand
            case 'bst':
                b = combo & 7
            case 'jnz':
                if a != 0:
                    pointer = operand
                    continue
            case 'bxc':
                b ^= c
            case 'out':
                outs.append(combo & 7)
            case 'bdv':
                b = dv(a, operand, a, b, c)
            case 'cdv':
                c = dv(a, operand, a, b, c)
        # print(f"Pointer: {pointer}, A: {a}, B: {b}, C: {c}, Output: {outs}")
        pointer += 2
    return outs


# partie 1
outputs = get_output(program, register_1, register_2, register_3)
print(",".join(map(str, outputs)))

# Partie 2

candidates: list[int] = [0]
print(program)
for i in range(1, len(program) + 1):
    print("------")
    print(f"Pour {program[-i:]}")
    out = []
    for c in candidates:
        print(f"Candidat : {c}")
        for offset in range(2 ** 3):
            #  2**3 -> 8 dernier bits
            a = (2 ** 3) * c + offset
            outs = get_output(program, a, register_2, register_3)
            if outs == program[-i:]:
                print(f"A valid pour {program[-i:]} == {outs} -> {a}")
                out.append(a)
    candidates = out

print(min(candidates))