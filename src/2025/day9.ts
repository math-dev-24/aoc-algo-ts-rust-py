
class Coord {
    line: string
    x: number
    y: number

    constructor (line: string) {
        this.line = line;
        [this.x, this.y] = this._parse(line);
    }

    private _parse(line: string): number[] {
        return line.split(",").map(c => {
            return parseFloat(c)
        })
    }

    public get_size(other: Coord): number {
        const dif_x = Math.abs(this.x - other.x) + 1
        const dif_y = Math.abs(this.y - other.y) + 1
        return dif_x * dif_y
    }
}

class Edge {
    x1: number
    x2: number
    y1: number
    y2: number

    constructor (x1: number, y1: number, x2: number, y2: number) {
        this.x1 = x1;
        this.y1 = y1;
        this.x2 = x2;
        this.y2 = y2;
    }
}


const solve_day_9 = (input: string): number => {
    const list_coord: Coord[] = input.split("\n")
    .filter(l => l.trim() != "")
    .map(l => new Coord(l));

    console.log("Red tiles:", list_coord.length);

    const part_1 = solve_part_1(list_coord);
    console.log("Part 1 - Max area:", part_1);
    console.log("Part 1 check:", part_1 == 4759531084);

    const part_2 = solve_part_2(list_coord);
    console.log("Part 2 - Max area (red/green only):", part_2);
    console.log("Part 2 check :", part_2 == 1539238860)

    return part_2;
}

export default solve_day_9;


function solve_part_1(list_coord: Coord[]): number {
    const len_list = list_coord.length;

    let max: number = 0;

    for (let idx_1 = 0; idx_1 < len_list; idx_1++) {
        for (let idx_2 = idx_1+1; idx_2 < len_list; idx_2++){
            const corner_1 = list_coord[idx_1];
            const corner_2 = list_coord[idx_2];
            const size = corner_1.get_size(corner_2);
            if (size > max) max = size;
        }

    }

    return max
}

function solve_part_2(list_coord: Coord[]): number {
    // Construire les arêtes du polygone
    const edges: Edge[] = [];

    for (let i = 0; i < list_coord.length - 1; i++) {
        edges.push(new Edge(
            list_coord[i].x,
            list_coord[i].y,
            list_coord[i + 1].x,
            list_coord[i + 1].y
        ));
    }

    // Fermer le polygone
    const last_idx = list_coord.length - 1;
    edges.push(new Edge(
        list_coord[last_idx].x,
        list_coord[last_idx].y,
        list_coord[0].x,
        list_coord[0].y
    ));

    const len_list = list_coord.length;

    // Générer toutes les paires avec leur aire potentielle
    interface Pair {
        idx_1: number;
        idx_2: number;
        area: number;
    }

    const pairs: Pair[] = [];
    for (let idx_1 = 0; idx_1 < len_list; idx_1++) {
        for (let idx_2 = idx_1 + 1; idx_2 < len_list; idx_2++) {
            const from_tile = list_coord[idx_1];
            const to_tile = list_coord[idx_2];
            const area = from_tile.get_size(to_tile);
            pairs.push({ idx_1, idx_2, area });
        }
    }

    // Trier par aire décroissante
    pairs.sort((a, b) => b.area - a.area);

    console.log(`Vérification des ${pairs.length} paires de rectangles...`);

    let max_found = 0;

    for (const pair of pairs) {
        // Early exit : si l'aire potentielle est <= au max trouvé, ignorer
        if (pair.area <= max_found) {
            break; // Comme les paires sont triées, on peut sortir complètement
        }

        const from_tile = list_coord[pair.idx_1];
        const to_tile = list_coord[pair.idx_2];

        const min_x = Math.min(from_tile.x, to_tile.x);
        const max_x = Math.max(from_tile.x, to_tile.x);
        const min_y = Math.min(from_tile.y, to_tile.y);
        const max_y = Math.max(from_tile.y, to_tile.y);

        // Optimisation : vérifier la distance de Manhattan au carré
        const manhattan = manhattan_distance(from_tile, to_tile);
        if (manhattan * manhattan <= max_found) {
            continue;
        }

        // Vérifier si le rectangle intersecte avec les arêtes
        if (!rectangles_intersects_edges(min_x, min_y, max_x, max_y, edges)) {
            max_found = pair.area;
        }
    }

    return max_found;
}


function rectangles_intersects_edges(min_x: number, min_y: number, max_x: number, max_y: number, edges: Edge[]): boolean {
    for (const edge of edges) {
        const i_min_x = Math.min(edge.x1, edge.x2);
        const i_max_x = Math.max(edge.x1, edge.x2);
        const i_min_y = Math.min(edge.y1, edge.y2);
        const i_max_y = Math.max(edge.y1, edge.y2);

        // Vérifie si les bounding boxes se chevauchent
        if (min_x < i_max_x && max_x > i_min_x && min_y < i_max_y && max_y > i_min_y) {
            return true;
        }
    }
    return false;
}

function manhattan_distance(a: Coord, b: Coord): number {
    return Math.abs(a.x - b.x) + Math.abs(a.y - b.y);
}