from srcp.utils.input import get_data

number_pad = {
    '7': (0, 0),
    '8': (0, 1),
    '9': (0, 2),
    '4': (1, 0),
    '5': (1, 1),
    '6': (1, 2),
    '1': (2, 0),
    '2': (2, 1),
    '3': (2, 2),
    '0': (3, 1),
    'A': (3, 2)
}
arrow_pad = {'^': (0, 1), 'A': (0, 2), '<': (1, 0), 'v': (1, 1), '>': (1, 2)}


def generate_sequences_from_letter_to_letter(keypad, start, end):
    sequences = []
    to_check = [(start, "")]
    while to_check:
        current_position, path = to_check.pop()
        target = keypad[end]

        if current_position == target:
            sequences.append(path)
            continue

        column_move = target[1] - current_position[1]
        if column_move != 0:
            new_point = (current_position[0], current_position[1] + (1 if column_move > 0 else -1))
            if new_point in keypad.values():
                to_check.append((new_point, path + (">" if column_move > 0 else "<")))

        row_move = target[0] - current_position[0]
        if row_move != 0:
            new_point = (current_position[0] + (1 if row_move > 0 else -1), current_position[1])
            if new_point in keypad.values():
                to_check.append((new_point, path + ("v" if row_move > 0 else "^")))

    return sequences


cache = {}


def get_minimal_sequence_length(keypad: dict[str, tuple[int, int]], code: str, robots: int) -> int:
    cache_key: str = f"{len(keypad)}-{code}-{robots}"
    if cache_key in cache:
        return cache[cache_key]

    if robots == 0:
        cache[cache_key] = len(code)
        return len(code)

    current_position = keypad["A"]
    minimal_length: int = 0
    new_robots: int = robots - 1

    for letter in code:
        sequences = generate_sequences_from_letter_to_letter(keypad, current_position, letter)
        minimal_length += min(
            get_minimal_sequence_length(arrow_pad, sequence + "A", new_robots)
            for sequence in sequences
        )
        current_position = keypad[letter]

    cache[cache_key] = minimal_length
    return minimal_length


def solve_puzzle(data: list[str]):
    result: int = 0
    for code in data:
        min_value: int = get_minimal_sequence_length(number_pad, code, 26)
        numeric_value: int = int("".join(c for c in code if c.isdigit()))
        result += min_value * numeric_value
    return result


input_data = get_data(2024, 21).strip().splitlines()
print(solve_puzzle(input_data))
