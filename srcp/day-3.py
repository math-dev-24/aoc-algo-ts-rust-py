import re
import time

start_time: float = time.time()
score: int = 0
state: bool = True
test: str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
pattern: str = r"mul\(\d+,\d+\)|don't\(\)|do\(\)"

matches = re.findall(pattern, test)

for match in matches:
    if match.startswith("mul(") and state:
        num1, num2 = re.findall(r"\d+", match)
        score += int(num1) * int(num2)
    else:
        state = match == "do()"

print(f"Regex - Time: {time.time() - start_time} - Score: {score}")

