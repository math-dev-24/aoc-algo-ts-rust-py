from srcp.utils.input import get_data

data: list[tuple[str, int]] = [(n.split(" ")[0], int(n.split(" ")[1])) for n in get_data(day=2, year=2021).splitlines()]

result_1 = {
    'horizontal': 0,
    'vertical': 0
}


def part_1(result: dict[str, int]) -> dict[str, int]:
    for n in data:
        if n[0] == 'forward':
            result['horizontal'] += n[1]
        elif n[0] == 'up':
            result['vertical'] -= n[1]
        elif n[0] == 'down':
            result['vertical'] += n[1]
    return result


part_1 = part_1(result_1)
print(f"Partie 1 : {part_1['horizontal'] * part_1['vertical']}")

result_2 = {
    'horizontal': 0,
    'vertical': 0,
    'vise': 0
}


def part_2(result: dict[str, int]) -> dict[str, int]:
    for n in data:
        if n[0] == 'forward':
            result['horizontal'] += n[1]
            result['vertical'] += n[1] * result['vise']
        if n[0] == 'up':
            result['vise'] -= n[1]
        if n[0] == 'down':
            result['vise'] += n[1]

    return result

part_2 = part_2(result_2)
print(f"Partie 2 : {part_2['horizontal'] * part_2['vertical']}")