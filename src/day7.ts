const fs = require('fs');
const path = require('path');


const data_day_7 = fs.readFileSync(path.join(__dirname, './data.txt'), 'utf8');

const operators: string[] = ["+", "*", "||"];
const stard_D6 = new Date().getTime();

function generateOperatorCombinations(count: number): string[][] {
    if (count === 0) return [[]];
    const smallerCombos: string[][] = generateOperatorCombinations(count - 1);
    return smallerCombos.flatMap(combo =>
        operators.map(op => [...combo, op])
    );
}

function evaluateExpression(numbers: number[], ops: string[]): number {
    let result = numbers[0];
    for (let i = 0; i < ops.length; i++) {
        if (ops[i] === '+') {
            result += numbers[i + 1];
        } else if (ops[i] === '*') {
            result *= numbers[i + 1];
        }else if (ops[i] === '||') {
            result = parseInt(`${result}${numbers[i + 1]}`);
        }else {
            throw new Error("Operator not found");
        }
    }
    return result;
}

function findValidCombinations(numbers: number[], target: number): string[] {
    const operatorCombinations = generateOperatorCombinations(numbers.length - 1);
    const validExpressions: string[] = [];

    for (const ops of operatorCombinations) {
        const result = evaluateExpression(numbers, ops);
        if (result === target) {
            const expression = numbers
                .map((num, i) => (i < ops.length ? `${num} ${ops[i]}` : `${num}`))
                .join(' ');
            validExpressions.push(expression);
        }
    }

    return validExpressions;
}
let totalPar1: number = 0;
for (const line of data_day_7.split('\n')) {
    const total: number = parseInt(line.split(":")[0]);
    const number: number[] = line.split(":")[1].trim().split(" ").map(Number);
    const validExpressions = findValidCombinations(number, total);
    if (validExpressions.length > 0) {
        totalPar1 += total
    }
}
const end_D6 = new Date().getTime();
console.log("total :", totalPar1, " en : ", (end_D6 - stard_D6) / 1000, "s");