from srcp.utils.input import get_data

data: list[int] = [int(n) for n in get_data(2021, 1).splitlines()]


def part_1(d: list[int]) -> int:
    before: int = 0
    total: int = 0
    for i, line in enumerate(d):
        if i == 0:
            before = line
            continue
        if line > before:
            total += 1
        before = line
    return total


print(f"Partie 1 : {part_1(data)}")


def part_2(d: list[int]) -> int:
    sliding_sums = [
        d[i] + d[i + 1] + d[i + 2]
        for i in range(len(d) - 2)
    ]
    increase_count = sum(
        1 for i in range(1, len(sliding_sums))
        if sliding_sums[i] > sliding_sums[i - 1]
    )
    return increase_count


print(f"Partie 2 : {part_2(data)}")