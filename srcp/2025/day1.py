from math import floor

input = """L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"""

commands = [i for i in input.splitlines()]


# Partie 1
def rem_euclid(value, limit = 100):
    return ((value % limit) + limit) % limit

pos = 50
part_1 = 0

for command in commands:
    dir = command[0]
    q = int(command[1:])
    
    if dir == "R":
        pos += q
    else:
        pos -= q

    pos = rem_euclid(pos)

    if pos == 0:
        part_1+= 1

print(f"Partie 1 {part_1}")

# Partie 2
pointer = 50
part_2 = 0

def count_zeros(s, e):
    if e > s:
        first_cross = floor(s / 100 + 1) * 100
        if first_cross > e:
            return 0
        return floor((e - first_cross) / 100) + 1
    elif e < s:
        first_cross = 0
        if s%100 == 0:
            first_cross = s - 100
        else:
            first_cross = floor(s / 100) * 100
        if first_cross < e:
            return 0
        return floor((first_cross - e) / 100) + 1
    else:
        return 0

for command in commands:
    dir = command[0]
    q = int(command[1:])

    start = pointer
    end = pointer + q

    if dir == "L":
        end = pointer - q

    part_2 += count_zeros(start, end)
    pointer = rem_euclid(end)

print("Partie 2: ",part_2)