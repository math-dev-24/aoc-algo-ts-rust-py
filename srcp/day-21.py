from typing import Optional

from srcp.utils.input import get_data

numeric_pad = [
    ["7", "8", "9"],
    ["4", "5", "6"],
    ["1", "2", "3"],
    [None, "0", "A"]
]

arrow_pad = [
    [None, "^", "A"],
    ["<", "v", ">"]
]

int_pad = [
    [None, None, "A"],
    ["<", "v", ">"],
    ["<", None, None]
]


def manhattan_distance(pos1, pos2):
    return abs(pos1[0] - pos2[0]) + abs(pos1[1] - pos2[1])


def find_position(target: str, keyboard: list[list[str]]) -> Optional[dict]:
    for ri, row in enumerate(keyboard):
        for ci, val in enumerate(row):
            if val == target:
                return {
                    "position": (ri, ci),
                    "value": val
                }
    return None


def move_on_pad(start: tuple[int, int], end: tuple[int, int]) -> list[str]:
    moves = []
    si, sj = start
    ei, ej = end
    while si < ei:
        moves.append("v")
        si += 1
    while si > ei:
        moves.append("^")
        si -= 1
    while sj < ej:
        moves.append(">")
        sj += 1
    while sj > ej:
        moves.append("<")
        sj -= 1
    return moves


def generate_sequence_for_code(code: str, start_position="A"):
    current_position = find_position(start_position, numeric_pad)
    sequence = []
    for char in code:
        target_position = find_position(char, numeric_pad)
        moves = move_on_pad(current_position['position'], target_position['position'])
        sequence.extend(moves)
        sequence.append("A")
        current_position = target_position
    return sequence


def get_arrow_dir_int(initial_position: tuple[int, int], target: str, pad: list[list[str]]) -> list[str]:
    x, y = initial_position
    x_target, y_target = find_position(target, pad)["position"]
    result = []

    while x != x_target or y != y_target:
        if x_target < x:
            result.append("^")
            x -= 1
        if x_target > x:
            result.append("v")
            x += 1
        if y_target < y:
            result.append("<")
            y -= 1
        if y_target > y:
            result.append(">")
            y += 1
    result.append("A")
    return result


def transform_command(list_sequence: list[str], pad: list[list[str]], start_position="A") -> list[str]:
    current_position = find_position(start_position, pad)

    x, y = current_position["position"]
    sequences = []

    for i, command in enumerate(list_sequence):
        # print(f"Commande : {i + 1}: {command}")
        new_command = get_arrow_dir_int((x, y), command, arrow_pad)
        # print(new_command)
        sequences.extend(new_command)
        x, y = find_position(command, arrow_pad)["position"]

    return sequences


code = "029A"
numeric_sequence = generate_sequence_for_code(code)
print("".join(numeric_sequence))
print("<A^A>^^AvvvA")

intermediate_sequence = transform_command(numeric_sequence, arrow_pad)
print("".join(intermediate_sequence))
print("v<<A>>^A<A>AvA<^AA>A<vAAA>^A")

human_sequence = transform_command(intermediate_sequence, int_pad)
print("".join(human_sequence))
print("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A")

print(len(human_sequence))

total = 0

for code in get_data(2024, 21).splitlines():
    sequence = generate_sequence_for_code(code)
    intermediate_sequence = transform_command(sequence, arrow_pad)
    human_sequence = transform_command(intermediate_sequence, arrow_pad)
    total += len(human_sequence) * int(code[:-1])
print(total)
