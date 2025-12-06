
class Day3_2025{
  final String input;

  Day3_2025._({
    required this.input
  });

  factory Day3_2025(String input) {
    return Day3_2025._(input: input);
  }

  int getResult(int target) {

    final List<String> lines = input.split("\n");

    int total = lines.map((line) {
      final List<String> chars = line.split('');
      final int n = chars.length;

      final List<String> selected = [];
      int start = 0;

      for (int pos = 0; pos < target; pos++) {
        final int windowEnd = n - (target - pos - 1);

        String bestDigit = '0';
        int bestIdx = start;

        for (int i = start; i < windowEnd; i++) {
          if (chars[i].compareTo(bestDigit) > 0) {
            bestDigit = chars[i];
            bestIdx = i;
          }
        }

        selected.add(bestDigit);
        start = bestIdx + 1;
      }

      final int joltage = selected.fold(0, (acc, ch) {
        return acc * 10 + int.parse(ch);
      });

      return joltage;
    }).fold(0, (a, b) => a + b);

    print('Total : $total');
    return total;
  }

}