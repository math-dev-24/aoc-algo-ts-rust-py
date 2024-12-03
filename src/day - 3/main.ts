
const fs = require('fs');
const path = require('path');

const data_day_3 = fs.readFileSync(path.join(__dirname, './data.txt'), 'utf8').trim();

const mulPattern: RegExp = /mul\((\d+),(\d+)\)/g;
const controlPattern: RegExp = /do\(\)|don't\(\)/g;

const mulls: any[] = [...data_day_3.matchAll(mulPattern)];
const controls: any[] = [...data_day_3.matchAll(controlPattern)];

let result: number = 0;

mulls.forEach(mull => {
    let tmp_control = controls.filter((control): boolean => control.index < mull.index + 1).pop();
    if(!tmp_control) tmp_control = {index: 0, input: "do()"};
    if(tmp_control[0] == "don't()") return;
    result += parseInt(mull[1], 10) * parseInt(mull[2], 10);
});

console.log("Resultat : ", result);
