import re
from collections import defaultdict
from srcp.utils.input import get_data

data = get_data(2023, 2)

games = data.strip().splitlines()
total: int = 0
max_cubes: dict[str, int] = {"red": 12, "green": 13, "blue": 14}


def parse_game(g: str):
    id_part, str_cubes = g.split(":")
    g_id: int = int(id_part.strip().split(" ")[1])
    observations = []

    for cube in str_cubes.split(";"):
        tmp_cubes = re.findall(r"(\d+) (\w+)", cube)
        observation = defaultdict(int)
        for count, color in tmp_cubes:
            observation[color] += int(count)
        observations.append(observation)
    return g_id, observations


def is_game_possible(obs: list[dict[str, int]], max_cs: dict[str, int]):
    for observation in obs:
        for color, max_count in max_cs.items():
            if observation[color] > max_count:
                return False
    return True


for game in games:
    game_id, observations = parse_game(game)
    if is_game_possible(observations, max_cubes):
        total += game_id

print(total)