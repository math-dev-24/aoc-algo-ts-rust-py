import 'package:aoc/input.dart';

import 'bin/day2.dart';

void main() async {
  final inputFetcher = Input();

  final String input = await inputFetcher.getInput(2025, 2);

  inputFetcher.close();

  final day_1 = Day2(input);
  
  final (r1, r2) = day_1.getResult();

  print("Résultat 1 :" + r1.toString() +" R2 :" + r2.toString());

  return;
}