from srcp.utils.input import get_data

data: list[str] = get_data(day=3, year=2021).splitlines()


def part_1(data: list[str]) -> int:
    length: int = len(data[0])
    gamma = []
    epsilon = []

    for l in range(0, length):
        one_count = 0
        zero_count = 0

        for line in data:
            tmp = line[l]
            if tmp == '1':
                one_count += 1
            else:
                zero_count += 1
        if one_count > zero_count:
            gamma.append(1)
            epsilon.append(0)
        else:
            gamma.append(0)
            epsilon.append(1)

    result_gamma = "".join([str(x) for x in gamma])
    result_epsilon = "".join([str(x) for x in epsilon])
    return int(result_gamma, 2) * int(result_epsilon, 2)


print(f"Partie 1 : {part_1(data)}")
