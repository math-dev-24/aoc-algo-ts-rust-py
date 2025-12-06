class Day5_2025 {
  List<(int, int)> listIds;
  List<int> listId;
  int resultat_1 = 0;
  int resultat_2 = 0;

  Day5_2025._({
    required this.listIds,
    required this.listId,
  });

  factory Day5_2025(String input) {
    final List<String> group = input.split("\n\n").toList();

    final List<(int, int)> listIds =
        group.first.split("\n").where((r) => r.trim().isNotEmpty).map((r) {
      final parts = r.split("-");
      final min = int.parse(parts[0].trim());
      final max = int.parse(parts[1].trim());
      return (min, max);
    }).toList();

    final List<int> listId = group[1]
        .split("\n")
        .where((i) => i.trim().isNotEmpty)
        .map((i) => int.parse(i.trim()))
        .toList();

    return Day5_2025._(listIds: listIds, listId: listId);
  }

  (int, int) getResult() {
    for (var id in listId) {
      for (var i = 0; i < listIds.length; i++) {
        final (min, max) = listIds[i];

        if (id >= min && id <= max) {
          resultat_1 += 1;
          break;
        }
      }
    }

    resultat_2 = part_2();

    return (resultat_1, resultat_2);
  }

  int part_2() {
    final intervals = listIds;
    intervals.sort((a, b) => a.$1.compareTo(b.$1));
    List<(int, int)> merged = [];

    for (var (min, max) in intervals) {
      if (merged.isEmpty) {
        merged.add((min, max));
      } else {
        var lastIdx = merged.length - 1;
        var (lastMin, lastMax) = merged[lastIdx];

        if (min <= lastMax + 1) {
          merged[lastIdx] = (lastMin, max > lastMax ? max : lastMax);
        } else {
          merged.add((min, max));
        }
      }
    }
    return merged.fold(
        0, (sum, interval) => sum + (interval.$2 - interval.$1 + 1));
  }
}
