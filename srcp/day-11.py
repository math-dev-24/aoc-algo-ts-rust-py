from collections import defaultdict

from srcp.utils.input import get_data


def extract_stone(tmp_stone: int) -> list[int]:
    if tmp_stone == 0:
        return [1]
    if len(str(tmp_stone)) % 2 == 0:
        middle = len(str(tmp_stone)) // 2
        left = str(tmp_stone)[:middle]
        right = str(tmp_stone)[middle:]
        return [int(left), int(right)]
    else:
        return [tmp_stone * 2024]


data = get_data(day=11, year=2024)

stones = list(map(int, data.split(" ")))

clicks = 75
result = defaultdict(int)

for stone in stones:
    result[stone] += 1


for _ in range(clicks):
    new_stones = defaultdict(int)
    for stone, count in result.items():
        tmp = extract_stone(stone)
        for s in tmp:
            new_stones[s] += count
    result = new_stones

print(sum(result.values()))




