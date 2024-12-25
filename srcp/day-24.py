from srcp.utils.input import get_data
import re


def simulate_gates(input_data: str) -> int:
    init, gates = input_data.split("\n\n")
    init = init.splitlines()
    gates = gates.splitlines()


    wires = {}
    for line in init:
        wire, bit = line.split(": ")
        wires[wire] = lambda bit=bit: int(bit)

    for line in gates:
        in0, op, in1, _, out = line.split(" ")
        wires[out] = {
            "AND": lambda in0=in0, in1=in1: wires[in0]() & wires[in1](),
            "OR": lambda in0=in0, in1=in1: wires[in0]() | wires[in1](),
            "XOR": lambda in0=in0, in1=in1: wires[in0]() ^ wires[in1]()
        }[op]

    z = ""
    bit = 0

    while True:
        key = f"z{bit:02}"
        if key not in wires:
            break
        z = str(wires[key]()) + z
        bit += 1

    return int(z, 2)


data = get_data(2024, 24)
print(simulate_gates(data))
