def get_direction(n1: str, n2: str) -> int:
    return 1 if int(n1) < int(n2) else -1


def is_safe(arr: list[str]) -> (bool, int):
    d: int = get_direction(arr[1], arr[0])
    q_error: int = 0

    for i in range(1, len(arr)):
        delta: float = abs(int(arr[i]) - int(arr[i - 1]))
        c_d: int = get_direction(arr[i], arr[i - 1])

        if delta < 1 or delta > 3 or c_d != d:
            q_error += 1
            if q_error > 1:
                return False, 2
    return q_error == 0, q_error


list_line: list[str] = reports.strip().split("\n")
score: int = 0

# O(n²)
for line in list_line:
    safe, q_error = is_safe(line.split(" "))
    if safe or q_error == 1:
        score += 1

print(score)
