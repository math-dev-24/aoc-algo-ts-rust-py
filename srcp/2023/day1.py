from srcp.utils.input import get_data

data = get_data(2023, 1)
lines: list[str] = data.strip().splitlines()
total: int = 0

numbers_text: tuple[tuple[str, int], ...] = (
    ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
    ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
)


def get_total_line(n: list[int]) -> int:
    if len(n) == 1:
        return n[0] * 11
    elif len(n) > 1:
        return n[0] * 10 + n[-1]
    return 0


for line in lines:
    numbers: list[int] = []

    for i, c in enumerate(line):
        if c.isdigit():
            numbers.append(int(c))
        else:
            for text in numbers_text:
                if line[i:i+len(text[0])] == text[0]:
                    numbers.append(text[1])
                    break
    total += get_total_line(numbers)


print(total)
