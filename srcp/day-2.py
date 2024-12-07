from srcp.utils.input import get_data
import time

start_time = time.time()


def is_safe(arr: tuple[int, ...]) -> bool:
    directions = [a - b for a, b in zip(arr, arr[1:])]
    return all(1 <= d <= 3 for d in directions) or all(-3 <= d <= -1 for d in directions)


list_line: list[str] = get_data(2024, 2).strip().split("\n")
score_part_1: int = 0
score_part_2: int = 0


for line in list_line:
    line = tuple(map(int, line.strip().split(" ")))
    score_part_1 += is_safe(line)
    # une des boucles est valide en enlevant un item
    score_part_2 += any(is_safe(line[:i] + line[i+1:]) for i in range(len(line)))


print(f"Score : {score_part_1}")
print(f"Score : {score_part_2} en {time.time() - start_time}s")
