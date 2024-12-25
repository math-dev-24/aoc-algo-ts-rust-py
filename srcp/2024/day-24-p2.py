from srcp.utils.input import get_data


def simulate_gates(input_data):
    init, gates = input_data.split("\n\n")
    init = init.splitlines()
    gates = gates.splitlines()

    gates_map = {}

    for line in gates:
        in0, op, in1, _, out = line.split(" ")

        gates0 = gates_map.setdefault(in0, {})
        gates1 = gates_map.setdefault(in1, {})

        gates0[op] = {"other": in1, "out": out}
        gates1[op] = {"other": in0, "out": out}

    swaps = set()

    def expect(in0: str, op: str, in1: str):
        res = gates_map.get(in0, {}).get(op)

        if not res:
            res = gates_map[in1][op]
            swaps.add(in0)
            swaps.add(res["other"])
            in0 = res["other"]
        elif res["other"] != in1:
            swaps.add(in1)
            swaps.add(res["other"])
            in1 = res["other"]

        return {"in0": in0, "in1": in1, "out": res["out"]}

    carry = None
    bit = 0

    while len(swaps) < 8:
        x = f"x{bit:02}"
        y = f"y{bit:02}"

        new_carry = gates_map[x]["AND"]["out"]

        if bit > 0:
            low = gates_map[x]["XOR"]["out"]
            carry, low = expect(carry, "XOR", low)["in0"], expect(carry, "XOR", low)["in1"]

            cont = expect(carry, "AND", low)["out"]
            new_carry = expect(new_carry, "OR", cont)["out"]

        carry = new_carry
        bit += 1

    return ",".join(sorted(swaps))



data = get_data(2024, 24)
print(simulate_gates(data))