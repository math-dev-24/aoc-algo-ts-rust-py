import getInput from './utils/input';
import postAnswer from "./utils/post";
import solve_day_5_2025 from "./2025/day5";


async function main() {
    const year: number = 2025;
    const day: number = 5;
    const sendData: boolean = false;
    const level: number = 1;

    const data: string = await getInput(year, day);
    console.log("Données d'entrées récupérées");
    console.log("Calcul en cours...");
    const start: number = new Date().
    getTime();
    const res = solve_day_5_2025(data);
    console.log("Temps de calcul : ", (new Date().getTime() - start) / 1000, "s");

    if (sendData && (level === 1 || level === 2)) {
        console.log("Envoi des données...");
        await postAnswer(day, level, res.toString());
    }
}

main();