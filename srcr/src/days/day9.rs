pub fn solve_day_9(input: &str) {
    let current_input: String = input.trim().to_string();
    let mut disk = parse_disk(&current_input);

    disk.sort_file();
    move_data(&mut disk);

    let result = get_result(&disk.disk);
    println!("Résultat : {}", result);

}

#[derive(Clone)]
struct File {
    id: u128,
    position: u128,
    length: u128
}

struct FreeBlock {
    position: u128,
    length: u128
}

struct ParseDiskResult {
    disk: Vec<char>,
    files: Vec<File>,
    free_blocks: Vec<FreeBlock>
}

impl ParseDiskResult {
    fn sort_file(&mut self) {
        self.files.sort_by(|a,b | {b.id.cmp(&a.id)});
    }
    fn remove_free_block(&mut self, id: usize) {
        self.free_blocks.remove(id);
    }
    fn update_free_block(&mut self, id: usize, position: u128, length: u128){
        if let Some(block) = self.free_blocks.get_mut(id) {
            block.position = position;
            block.length = length
        }
    }
    fn update_disk(&mut self, value: char, start: usize, end: usize) {
        if start >= end  || end > self.disk.len() {
            panic!("Tentative impossible - start: {} - end {}", start, end);
        }
        for i in start..end {
            self.disk[i] = value;
        }
    }
}

fn parse_disk(disk: &String) -> ParseDiskResult {
    let mut file_data: Vec<File> = Vec::new();
    let mut tmp_disk: Vec<char> = Vec::new();
    let mut free_blocks: Vec<FreeBlock> = Vec::new();

    let mut current_pos: u128 = 0;

    for (i, val) in disk.chars().enumerate() {
        let tmp_length: usize = val.to_digit(10).expect("Ce n'est pas un nombre") as usize;

        if i%2 == 0 {
            let file_id = i / 2;
            file_data.push(File {
                id: file_id as u128, 
                position: current_pos, 
                length: tmp_length as u128
            });
            tmp_disk.extend(vec![file_id.to_string().chars().next().unwrap(); tmp_length]);
        } else {
            free_blocks.push(FreeBlock {
                position: current_pos,
                length: tmp_length as u128
            });
            tmp_disk.extend(vec!['.'; tmp_length]);
        }
        current_pos += tmp_length as u128;
    }

    return ParseDiskResult {
        disk: tmp_disk,
        files: file_data,
        free_blocks: free_blocks
    };
}


fn move_data(disk: &mut ParseDiskResult) -> Vec<char> {
    let file_count = disk.files.len();

    for file_idx in 0..file_count {
        let file = disk.files[file_idx].clone();
        let mut free_start: Option<u128> = None;

        for free_idx in 0..disk.free_blocks.len() {
            let free_block = &disk.free_blocks[free_idx];

            if free_block.length >= file.length && free_block.position <= file.position {
                free_start = Some(free_block.position);

                if free_block.length == file.length {
                    disk.remove_free_block(free_idx);
                } else {
                    disk.update_free_block(
                        free_idx,
                        free_block.position + file.length,
                        free_block.length - file.length,
                    );
                }
                break;
            }
        }

        if let Some(val) = free_start {
            let start = file.position as usize;
            let end = (file.position + file.length) as usize;
            disk.update_disk('.', start, end);

            let new_start = val as usize;
            let new_end = (val + file.length) as usize;
            disk.update_disk(
                (file.id % 10).to_string().chars().next().unwrap(),
                new_start,
                new_end,
            );
        }
    }
    disk.disk.clone()
}



fn get_result(disk: &Vec<char>) -> u128 {
    disk.iter().enumerate()
    .fold(0, |total, (idx, val)| {
        if *val != '.' {
            total + idx as u128 * val.to_digit(10).unwrap_or(0) as u128
        } else {
            total
        }
    })
}