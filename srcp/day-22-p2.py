from srcp.utils.input import get_data


def mix(secret_number: int, value: int) -> int:
    return secret_number ^ value


def prune(secret_number: int) -> int:
    modulus = 16777216
    modded = secret_number % modulus
    return modded + modulus if modded < 0 else modded


def get_next_secret_number(secret_number: int) -> int:
    result = prune(mix(secret_number, secret_number * 64))
    result = prune(mix(result, result // 32))
    return prune(mix(result, result * 2048))


def get_final_secret_number(initial_number: int, iterations: int) -> int:
    secret_number: int = initial_number
    for _ in range(iterations):
        secret_number = get_next_secret_number(secret_number)
    return secret_number


def solve_puzzle(data: str):
    sequence_map = {}
    largest_sum = 0
    initial_numbers = list(map(int, data.strip().splitlines()))

    for current_index, initial_number in enumerate(initial_numbers):
        current_sequence = [0, 0, 0, 0]
        sequence_index = 0

        for _ in range(2000):
            next_num: int = get_final_secret_number(initial_number, 1)
            current_sequence_part: int = (next_num % 10) - (initial_number % 10) + 9

            if sequence_index >= 4:
                current_sequence.pop(0)
                current_sequence.append(current_sequence_part)

                sequence_key = ",".join(map(str, current_sequence))
                sequence = sequence_map.get(sequence_key, {"sum": 0, "last_processed": -1})

                if sequence["last_processed"] != current_index:
                    sequence["last_processed"] = current_index
                    sequence["sum"] += next_num % 10

                    if sequence["sum"] > largest_sum:
                        largest_sum = sequence["sum"]

                sequence_map[sequence_key] = sequence
            else:
                current_sequence[sequence_index] = current_sequence_part
                sequence_index += 1

            initial_number = next_num

    return largest_sum


input_data = get_data(2024, 22)
print(solve_puzzle(input_data))
