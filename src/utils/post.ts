import * as dotenv from 'dotenv';
dotenv.config();

export async function postAnswer(day: number, level: number, answer: string): Promise<void> {
    const year: number = new Date().getFullYear();

    try {
        const body: string = new URLSearchParams({
            level: level.toString(),
            answer,
        }).toString();

        const response: Response = await fetch(
            `${process.env.URL}/${year}/day/${day}/answer`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                    'Cookie': `session=${process.env.SESSION}`,
                },
                body,
            }
        );

        if (response.ok) {
            const text: string = await response.text();
            console.log('Réponse soumise avec succès.');
            if (text.includes("That's not the right answer;")) {
                console.log('Mauvaise réponse.');
            }else {
                console.log('Bonne réponse.');
            }
        } else {
            console.error(`Erreur lors de l'envoi de la réponse : ${response.status}`);
            console.error(await response.text());
        }
    } catch (error: any) {
        console.error('Erreur:', error.message || error);
    }
}
