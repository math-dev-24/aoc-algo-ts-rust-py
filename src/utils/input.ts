import * as dotenv from 'dotenv';
dotenv.config();

const request = require('sync-request');

export async function getInput(year: number, day: number) {
    console.log(`Récupération de l'entrée pour le jour ${day} de l'année ${year}...`);
    const url: string = `${process.env.URL}/${year}/day/${day}/input`;
    const res = request('GET', url, {
        headers: {
            'Cookie': `session=${process.env.SESSION}`,
        },
    });
    return res.getBody("utf8").trim();
}