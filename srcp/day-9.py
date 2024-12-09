import time
# from srcp.utils.input import get_data


def parse_disk_map(disk):
    file_data = []
    tmp_disk = []
    free_blocks = []
    current_position = 0
    for i, length in enumerate(map(int, disk)):
        if i % 2 == 0:
            file_id = i // 2
            file_data.append([file_id, current_position, length])
            tmp_disk.extend([file_id] * length)
        else:
            free_blocks.append([current_position, length])
            tmp_disk.extend(["."] * length)
        current_position += length

    return tmp_disk, file_data, free_blocks


def move_whole_files(tmp_disk, tmp_file, tmp_free_blocks):
    tmp_file.sort(key=lambda x: x[0], reverse=True)

    for file_id, start, size in tmp_file:

        free_start = None
        for index, (free_start_pos, free_size) in enumerate(tmp_free_blocks):
            if free_size >= size and free_start_pos <= start:
                free_start = free_start_pos
                if free_size == size:
                    tmp_free_blocks.pop(index)
                else:
                    tmp_free_blocks[index] = [free_start_pos + size, free_size - size]
                break

        if free_start is not None:
            tmp_disk[start:start + size] = ['.'] * size
            tmp_disk[free_start:free_start + size] = [file_id] * size

            # print(f"Moved file {file_id} to {free_start} - Disk state: {''.join(map(str, tmp_disk))}")

    return tmp_disk


time_start = time.time()
disk_map = "2333133121414131402"
# disk_map = get_data(2024, 9)

disk_data, file_data, free_blocks = parse_disk_map(disk_map)
compacted_data = move_whole_files(disk_data, file_data, free_blocks)

# Calcul du checksum
checksum = sum(i * block for (i, block) in enumerate(compacted_data) if block != ".")

print("Resultat :", checksum)
print(f"Time : {time.time() - time_start:.2f}s")
