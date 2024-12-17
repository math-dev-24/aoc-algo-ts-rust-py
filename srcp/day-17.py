from srcp.utils.input import get_data
import re

data = get_data(2024, 17)

lines: list[str] = data.splitlines()

register_1: int = int(re.compile(r'Register A: (\d+)').findall(lines[0])[0])
register_2: int = int(re.compile(r'Register B: (\d+)').findall(lines[1])[0])
register_3: int = int(re.compile(r'Register C: (\d+)').findall(lines[2])[0])
program = list(map(int, re.compile(r'\d+').findall(lines[4])))

outputs = []
pointer: int = 0
opcode = {0: 'adv', 1: 'bxl', 2: 'bst', 3: 'jnz', 4: 'bxc', 5: 'out', 6: 'bdv', 7: 'cdv'}


def get_combo(op: int, a: int, b: int, c: int) -> int:
    if op == 4:
        return a
    elif op == 5:
        return b
    elif op == 6:
        return c
    return op


def xor(register: int, op: int, a, b, c) -> int:
    return register // (2 ** get_combo(op, a, b, c))


while pointer < len(program):
    instruction: str = opcode[program[pointer]]
    operand = program[pointer + 1]
    print(f"{instruction} -> {operand}")
    match instruction:
        case 'adv':
            register_1 = xor(register_1, operand, register_1, register_2, register_3)
        case 'bxl':
            register_2 ^= operand
        case 'bst':
            register_2 = get_combo(operand, register_1, register_2, register_3) % 8
        case 'jnz':
            if register_1 != 0:
                print(f"Pointer nouvelle position : {operand}")
                pointer = operand
                continue
        case 'bxc':
            register_2 ^= register_3
        case 'out':
            outputs.append(get_combo(operand, register_1, register_2, register_3) % 8)
        case 'bdv':
            register_2 = xor(register_1, operand, register_1, register_2, register_3)
        case 'cdv':
            register_3 = xor(register_1, operand, register_1, register_2, register_3)
    pointer += 2
    print(f"A = {register_1}, B = {register_2}, C = {register_3}")

print(",".join(map(str, outputs)))
