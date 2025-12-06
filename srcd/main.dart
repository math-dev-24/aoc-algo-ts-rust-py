import 'package:aoc/input.dart';

import 'bin/2025/day3.dart';



void main() async {
  final inputFetcher = Input();
  final String input = await inputFetcher.getInput(2025, 5);

  inputFetcher.close();

  final day_1 = Day3_2025(input);
  
  final r1 = day_1.getResult(2);
  final r2 = day_1.getResult(12);

  print("Résultat part 1 :" + r1.toString() +" Résultat part 2 :" + r2.toString());

  return;
}