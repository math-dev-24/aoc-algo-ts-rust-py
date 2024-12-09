
export default function solve_day_9(input: string): string {

    const {disk, files, free_blocks} = parse_disk(input);
    const compacted_disk = move_data(disk, files, free_blocks);
    const result = get_result(compacted_disk);
    console.log(`Résultat : ${result}`);
    return result.toString();
}

interface FileInterface {
    id: number,
    position: number,
    len: number
}

interface FreeBlockInterface {
    position: number,
    len: number
}

interface ParseDiskResult {
    disk: string[],
    files: FileInterface[],
    free_blocks: FreeBlockInterface[]
}

function parse_disk(disk: string): ParseDiskResult {
    let file_data: FileInterface[] = [];
    let tmp_disk: string[] = [];
    let free_blocks: FreeBlockInterface[] = [];

    let current_pos: number = 0
    
    disk.split("").forEach((val, i) => {
        const tmp_l = parseInt(val);
        if (i % 2 == 0) {
            const file_id: number = Math.floor(i/2);
            file_data.push({id: file_id, position: current_pos, len: tmp_l});
            tmp_disk.push(...Array(tmp_l).fill(file_id.toString()));
        } else {
            free_blocks.push({position: current_pos, len: tmp_l})
            tmp_disk.push(...Array(tmp_l).fill("."))
        }
        current_pos += tmp_l;
    })

    return {
        disk: tmp_disk,
        files: file_data,
        free_blocks: free_blocks
    }

}

function move_data(disk: string[], files: FileInterface[], free_blocks: FreeBlockInterface[]): string[] {
    files.sort((a: FileInterface,b: FileInterface) => b.id - a.id);
    
    for (let fi = 0; fi < files.length; fi++){

        let free_start: number|null = null;
        const current_file: FileInterface = files[fi];

        for (let b = 0; b < free_blocks.length; b++){
            const current_free: FreeBlockInterface = free_blocks[b];
            if (current_free.len >= current_file.len && current_free.position <= current_file.position){
                free_start = current_free.position;
                if (current_free.len == current_file.len) {
                    free_blocks.splice(b, 1);
                }else {
                    free_blocks[b] = {
                        position: current_free.position + current_file.len, 
                        len: current_free.len - current_file.len
                    };
                }
                break;
            }
        }
        if(free_start) {
            disk.fill(".", current_file.position, current_file.position + current_file.len)
            disk.fill(current_file.id.toString(), free_start, free_start + current_file.len)
        }
    }
    return disk;
}

function get_result(disk: string[]): number {
    return disk.reduce((total, val, idx) => total + (val !== "." ? idx * parseInt(val) : 0), 0);
}