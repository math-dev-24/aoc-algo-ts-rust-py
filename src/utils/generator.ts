const fs = require('fs');

export function generateFolder(path: string, day_number: number): void {
    if(!fs.existsSync(path)) {
        fs.mkdirSync(path);
        generateFile(path, day_number);
    }else {
        console.log('folder already exists')
    }
}

function generateFile(path: string, day_number: number):  void {
    const file_main: string = `${path}/main.ts`;
    const file_txt: string = `${path}/data.txt`;
    fs.writeFileSync(file_main, `console.log('Challenge jour ${day_number}')`);
    fs.writeFileSync(file_txt, "");

}