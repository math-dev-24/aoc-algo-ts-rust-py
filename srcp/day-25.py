from srcp.utils.input import get_data


def parse_schematic(schema: list[str], is_lock: bool, total_height: int = 5) -> list[int]:
    rows: list[str] = [line.strip() for line in schema]
    if is_lock:
        rows_locked = rows[1:]
        column_heights = [col.index(".") if "." in col else total_height for col in zip(*rows_locked)]
    else:
        reversed_rows = rows[::-1]
        rows_key = reversed_rows[1:]
        column_heights = [col.index(".") if "." in col else total_height for col in zip(*rows_key)]

    return column_heights


def count_valid_pairs(locks: list[list[int]], keys: list[list[int]], total_height: int = 5) -> int:
    valid_pairs = 0
    for lock in locks:
        for key in keys:
            if all(lock[i] + key[i] <= total_height for i in range(len(lock))):
                valid_pairs += 1
    return valid_pairs


schematics = get_data(2024, 25).splitlines()

keys: list[list[int]] = []
locks: list[list[int]] = []

for i in range(0, len(schematics), 8):
    schematic = schematics[i: i + 7]
    if schematic[0] == ".....":
        size: list[int] = parse_schematic(schematic, False)
        keys.append(size)
    elif schematic[0] == "#####":
        size: list[int] = parse_schematic(schematic, True)
        locks.append(size)

print(keys)
print(locks)

print(count_valid_pairs(locks, keys))
