from srcp.utils.input import get_data

data: str = get_data(2024, 23)
data: list[tuple] = [tuple(line.split("-")) for line in data.strip().splitlines()]

network = {}
for a, b in data:
    network.setdefault(a, set()).add(b)
    network.setdefault(b, set()).add(a)

listings = set()
for a in network:
    for b in network[a]:
        for c in network[b]:
            if c in network[a] and a != b and b != c and a != c:
                triplet = tuple(sorted([a, b, c]))
                listings.add(triplet)

filtered_listings: list[tuple] = [triplet for triplet in listings if any(comp.startswith("t") for comp in triplet)]

print(filtered_listings)
print(len(filtered_listings))