import * as dotenv from 'dotenv';

dotenv.config();

const BASE_URL: string = 'https://adventofcode.com';

export async function postAnswer(day: number, level: number, answer: string): Promise<void> {
    const year: number = new Date().getFullYear();

    try {
        const body: string = new URLSearchParams({
            level: level.toString(),
            answer,
        }).toString();

        const response: Response = await fetch(
            `${BASE_URL}/${year}/day/${day}/answer`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                    'Cookie': `session=${process.env.SESSION_COOKIE}`,
                },
                body,
            }
        );

        if (response.ok) {
            const text: string = await response.text();
            console.log('Réponse soumise avec succès.');
            console.log("Réponse : ",text);
        } else {
            console.error(`Erreur lors de l'envoi de la réponse : ${response.status}`);
            console.error(await response.text());
        }


    } catch (error: any) {
        console.error('Erreur:', error.message || error);
    }
}
