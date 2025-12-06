
class Day1_2025 {

  int pointer = 50;
  int count = 0;
  final List<String> lines;

  Day1_2025._({
    required this.lines
  });

  factory Day1_2025(String input) {
    final List<String> lines = input.split("\n");
    return Day1_2025._(lines: lines);
  }

  (String, int) getDirAndAngle(String s) {
      final String dir = s[0];
      final String idx = s.substring(1);
      final int angle = int.parse(idx);
      return (dir, angle);
  }

  int getResult() {
    pointer = 50;
    count = 0;

    for (var line in lines) {
      if(line.isEmpty) continue;
      final (dir, angle) = getDirAndAngle(line);
      final int tmp = dir == "L" ? pointer - angle : pointer + angle;
      pointer = remEuclid(tmp);
      if (pointer == 0)  count++;
    }
    return count;
  }

  int getResult2() {
    pointer = 50;
    count = 0;

    for (var line in lines) {
      if (line.isEmpty) continue;
      final (dir, angle) = getDirAndAngle(line);

      int start = pointer;
      int end = dir == "L" ? pointer - angle : pointer + angle;

      count += countZeros(start, end);
      pointer = remEuclid(end);
    }

    return count;
  }

  int remEuclid (int value, {int limit = 100}) {
    return ((value % limit) + limit) % limit;
  }

  int countZeros(int s, int e) {
    if (e > s) {
      final firstCross = (s ~/ 100 + 1) * 100;
      if (firstCross > e) {
        return 0;
      }
      return (e - firstCross) ~/ 100 + 1;
    } else if (e < s) {
      final firstCross = s % 100 == 0 
          ? s - 100 
          : (s ~/ 100) * 100;
      if (firstCross < e) {
        return 0;
      }
      return (firstCross - e) ~/ 100 + 1;
    }
    return 0;
  }
}