class Day2 {
  final List<String> listOfRange;

  Day2._({required this.listOfRange});

  factory Day2(String input) {
    final List<String> listOfRange =
        input.split(",").map((String e) => e.trim()).toList();
    return Day2._(listOfRange: listOfRange);
  }

  (int, int) getResult() {
    int invalid = 0;
    int invalid_2 = 0;

    for (var tmpRange in listOfRange) {
      final (s, e) = parseRange(tmpRange);

      if (s > e) {
        throw Exception("Invalid range");
      }

      for (var i = s; i <= e; i++) {
        if (isInvalid(i)) {
          invalid += i;
        }
        if (isInvaldPart2(i)){
          invalid_2 += i;
        }
      }
    }
    return (invalid, invalid_2);
  }

  (int, int) parseRange(String input) {
    final List<String> rangeList = input.split("-");
    final int start = int.parse(rangeList[0]);
    final int end = int.parse(rangeList[1]);

    return (start, end);
  }

  bool isInvalid(int number) {
    final String nString = number.toString();
    final int len = nString.length;

    if (len % 2 != 0) {
      return false;
    }

    final int mid = (len / 2).floor();
    final String firstHalf = nString.substring(0, mid);
    final String secondHalf = nString.substring(mid);

    return firstHalf == secondHalf;
  }

  bool isInvaldPart2(int number) {
    final String s = number.toString();
    final int len = s.length;

    if (len < 2) {
      return false;
    }

    for (var patternLen = 1; patternLen <= (len ~/ 2); patternLen++) {
      if (len % patternLen != 0) {
        continue;
      }

      final pattern = s.substring(0, patternLen);
      bool isValidPattern = true;

      for (var i = patternLen; i < len; i += patternLen) {
        if (s.substring(i, i + patternLen) != pattern) {
          isValidPattern = false;
          break;
        }
      }

      if (isValidPattern) {
        return true;
      }
    }

    return false;
  }
}
