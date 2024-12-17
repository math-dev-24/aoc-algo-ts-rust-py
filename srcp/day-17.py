from srcp.utils.input import get_data
import re

data = get_data(2024, 17)

lines: list[str] = data.splitlines()

register_1: int = int(re.compile(r'Register A: (\d+)').findall(lines[0])[0])
register_2: int = int(re.compile(r'Register B: (\d+)').findall(lines[1])[0])
register_3: int = int(re.compile(r'Register C: (\d+)').findall(lines[2])[0])
program = list(map(int, re.compile(r'\d+').findall(lines[4])))


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


def get_output(prog: list[int], a: int = 0, b: int = 0, c: int = 0) -> str:
    pointer: int = 0
    outs: list[int] = []
    while pointer < len(prog):
        instruction: str = opcode[prog[pointer]]
        operand = prog[pointer + 1]
        match instruction:
            case 'adv':
                a = dv(a, operand, a, b, c)
            case 'bxl':
                b ^= operand
            case 'bst':
                b = get_combo(operand, a, b, c) % 8
            case 'jnz':
                if a != 0:
                    pointer = operand
                    continue
            case 'bxc':
                b ^= c
            case 'out':
                outs.append(get_combo(operand, a, b, c) % 8)
            case 'bdv':
                b = dv(a, operand, a, b, c)
            case 'cdv':
                c = dv(a, operand, a, b, c)
        pointer += 2
    return ",".join(map(str, outs))


# partie 1
# outputs = get_output(program, register_1, register_2, register_3)
# print(outputs)

target = [0, 3, 5, 4, 3, 0]
