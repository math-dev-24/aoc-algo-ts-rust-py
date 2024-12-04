import time

test: str = """
3   4
4   3
2   5
1   3
3   9
3   3
"""

start_time = time.time()
data = test.strip().split("\n")

deck_1, deck_2 = map(list, zip(*[map(int, line.split()) for line in data]))

deck_1.sort()
deck_2.sort()

score: int = sum(abs(deck_1[i] - deck_2[i]) for i in range(len(deck_1)))
part_2: int = sum((sum(1 if deck_1[i] == deck_2[j] else 0 for j in range(len(deck_2))) * deck_1[i]) for i in range(len(deck_1)))

print(f"Sortie - Time: {time.time() - start_time} - Score: {score}")
print(f"Sortie - Time: {time.time() - start_time} - Score: {part_2}")