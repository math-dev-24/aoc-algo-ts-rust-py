from srcp.utils.input import get_data

data: str = get_data(2024, 23)
data: list[tuple] = [tuple(line.split("-")) for line in data.strip().splitlines()]


def bron_kerbosch(graph, r, p, x, cliques):
    if not p and not x:
        cliques.append(r)
        return
    for node in list(p):
        bron_kerbosch(
            graph,
            r.union([node]),
            p.intersection(graph[node]),
            x.intersection(graph[node]),
            cliques,
        )
        p.remove(node)
        x.add(node)


network = {}
for a, b in data:
    network.setdefault(a, set()).add(b)
    network.setdefault(b, set()).add(a)

all_cliques = []
bron_kerbosch(network, set(), set(network.keys()), set(), all_cliques)

largest_clique = max(all_cliques, key=len)

password = ",".join(sorted(largest_clique))

print("Password for LAN party:", password)
