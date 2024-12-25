from srcp.utils.input import get_data
from collections import defaultdict

data = get_data(2024, 19)
lines = data.strip().splitlines()

# Extraction des des motifs disponibles et des serviettes disponnibles
motifs_list: list[str] = [l.strip() for l in lines[0].split(',')]
serviettes: list[str] = [s.strip() for s in lines[2:]]


def s_is_valid(s: str, motifs_available: list[str], memo: defaultdict) -> bool:
    if s in memo: return memo[s]
    if s == "": return True

    for motif_available in motifs_available:
        if s.startswith(motif_available):
            rest = s[len(motif_available):]
            if s_is_valid(rest, motifs_available, memo):
                memo[s] = True
                return True
    memo[s] = False
    return False


def count_combinations(s: str, motifs_available: list[str], memo: defaultdict) -> int:
    if s in memo:
        return memo[s]
    if s == "":
        return 1

    total_comb: int = 0
    for motif_available in motifs_available:
        if s.startswith(motif_available):
            rest = s[len(motif_available):]
            total_comb += count_combinations(rest, motifs_available, memo)

    memo[s] = total_comb
    return total_comb


# Étape 1 : Validation des serviettes
memo_validation = defaultdict(lambda: None)
valid_serviettes = [serviette for serviette in serviettes if s_is_valid(serviette, motifs_list, memo_validation)]
print(f"Nombre de serviettes valides : {len(valid_serviettes)} - (Part 1)")

# Étape 2 : Calculer toutes les combinaisons pour les serviettes valides
memo_combinations = defaultdict(int)
part_2: int = sum(count_combinations(serviette, motifs_list, memo_combinations) for serviette in valid_serviettes)
print(f"Nombre total de combinaisons : {part_2} - (Part 2)")
