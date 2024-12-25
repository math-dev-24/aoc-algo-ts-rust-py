import re
import time
from srcp.utils.input import get_data

start_time: float = time.time()
score: int = 0
state: bool = True
test: str = get_data(2024, 3).strip()
pattern: str = r"mul\(\d+,\d+\)|don't\(\)|do\(\)"

matches = re.findall(pattern, test)

for match in matches:
    if match.startswith("mul(") and state:
        num1, num2 = re.findall(r"\d+", match)
        score += int(num1) * int(num2)
    else:
        state = match == "do()"

print(f"Time: {time.time() - start_time} - Score: {score}")

