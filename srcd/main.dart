import 'package:aoc/input.dart';

import 'bin/day1.dart';

void main() async {
  final inputFetcher = Input();

  final String input = await inputFetcher.getInput(2025, 1);

  inputFetcher.close();

  final day_1 = Day1(input);
  
  final result = day_1.getResult();
  final result2 = day_1.getResult2();

  print("Résultat 1 :" + result.toString());
  print("Résultat 1 :" + result2.toString());

  return;
}