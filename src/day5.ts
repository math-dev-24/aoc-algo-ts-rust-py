// const fs = require('fs');
// const path = require('path');

const data_day_5 = fs.readFileSync(path.join(__dirname, './data.txt'), 'utf8').split("\n");
const emptyLineIndex = data_day_5.indexOf('');

const rules = data_day_5.slice(0, emptyLineIndex);
const updates = data_day_5.slice(emptyLineIndex + 1);

function isValidUpdate(update: number[], rules: string[]): boolean {
    for (const rule of rules) {
        const [page_1, page_2] = rule.split('|').map(Number);
        const index_1: number = update.indexOf(page_1);
        const index_2: number = update.indexOf(page_2);

        if (index_1 !== -1 && index_2 !== -1 && index_1 > index_2) {
            return false;
        }
    }
    return true;
}

function buildGraph(rules: string[], pages: number[]): Map<number, number[]> {
    const graph: Map<number, number[]> = new Map();

    for (const page of pages) {
        graph.set(page, []);
    }
    for (const rule of rules) {
        const [page_1, page_2] = rule.split('|').map(Number);
        if (pages.includes(page_1) && pages.includes(page_2)) {
            graph.get(page_1)!.push(page_2);
        }
    }
    return graph;
}

function topologicalSort(graph: Map<number, number[]>): number[] {
    const visited = new Set<number>();
    const result: number[] = [];
    const temp = new Set<number>();

    function visit(node: number): void {
        if (temp.has(node)) {
            throw new Error("Cycle detected!");
        }
        if (!visited.has(node)) {
            temp.add(node);
            for (const neighbor of graph.get(node) || []) {
                visit(neighbor);
            }
            temp.delete(node);
            visited.add(node);
            result.push(node);
        }
    }

    for (const node of graph.keys()) {
        if (!visited.has(node)) {
            visit(node);
        }
    }

    return result.reverse();
}

function reorderUpdate(update: number[], rules: string[]): number[] {
    const graph = buildGraph(rules, update);
    return topologicalSort(graph);
}

function getMiddlePage(update: number[]): number {
    const middleIndex = Math.floor(update.length / 2);
    return update[middleIndex];
}

// Analyse des mises à jour
let total: number = 0;

for (const update of updates) {
    const pages = update.split(',').map(Number); // Convertir en tableau de nombres

    if (!isValidUpdate(pages, rules)) {
        const reordered: number[] = reorderUpdate(pages, rules);
        const middlePage: number = getMiddlePage(reordered);
        total += middlePage;
    }
}

console.log("Résultat (Partie 2) :", total);