input = """11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"""

list_range = [[int(x) for x in i.split("-")] for i in input.split(',')]

invalid_p_1 = 0
invalid_p_2 = 0

def is_invalid_p1(n):
    n_txt = str(n)
    long = len(n_txt)
    if long % 2 != 0:
        return False
    mid = long // 2
    return n_txt[:mid] == n_txt[mid:]


def is_invalid_p2(n):
    s = str(n)
    length = len(s)

    if length < 2:
        return False

    for pattern_len in range(1, length // 2 + 1):

        if length % pattern_len != 0:
            continue

        pattern = s[:pattern_len]
        is_valid_pattern = True

        for i in range(pattern_len, length, pattern_len):
            if s[i:i + pattern_len] != pattern:
                is_valid_pattern = False
                break

        if is_valid_pattern:
            return True

    return False


for r in list_range:
    s, e = r
    rng = range(s, e+1)
    invalid_p_1 += sum(i for i in rng if is_invalid_p1(i))
    invalid_p_2 += sum(i for i in rng if is_invalid_p2(i))

print(invalid_p_1)
print(invalid_p_2)
